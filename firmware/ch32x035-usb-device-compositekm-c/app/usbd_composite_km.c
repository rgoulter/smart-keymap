/********************************** (C) COPYRIGHT
 ******************************** File Name          : usbd_composite_km.c
 * Author             : WCH
 * Version            : V1.0.0
 * Date               : 2023/04/06
 * Description        : USB keyboard and mouse processing.
 *********************************************************************************
 * Copyright (c) 2021 Nanjing Qinheng Microelectronics Co., Ltd.
 * Attention: This software (modified or not) and binary are used for
 * microcontroller manufactured by Nanjing Qinheng Microelectronics.
 *******************************************************************************/

/*******************************************************************************/
/* Header Files */
#include "usbd_composite_km.h"

#include <stdbool.h>
#include <string.h>

#include "ch32x035.h"
#include "debug.h"

#include "ch32x035_exti.h"
#include "ch32x035_gpio.h"
#include "ch32x035_misc.h"
#include "ch32x035_rcc.h"
#include "ch32x035_tim.h"

#include "ch32x035_usbfs_device.h"
#include "system_ch32x035.h"

#include "keyboard.h"
#include "keyboard_matrix.h"
#include "smart_keymap.h"
#ifdef KEYBOARD_LED_ENABLED
#include "keyboard_led.h"
#endif

/*******************************************************************************/
/* Global Variable Definition */

/* Keyboard */
volatile uint8_t KB_Scan_Done = 0x00; // Keyboard Keys Scan Done
volatile uint16_t KB_Scan_Result =
    (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11); // Keyboard Keys Current Scan Result
volatile uint16_t KB_Scan_Last_Result =
    (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11); // Keyboard Keys Last Scan Result
KeymapHidReport hid_report = {0};         // Keyboard HID report
uint8_t KB_Data_Pack[8] = {0x00};         // Keyboard IN Data Packet
uint8_t PREV_KB_Data_Pack[8] = {0x00};    // Keyboard IN Data Packet
uint8_t Consumer_Data_Pack[KEYMAP_HID_REPORT_CONSUMER_LEN] = {
    0x00}; // Consumer IN Data Packet
uint8_t PREV_Consumer_Data_Pack[KEYMAP_HID_REPORT_CONSUMER_LEN] = {
    0x00};                                  // Consumer IN Data Packet
uint8_t Mouse_Data_Pack[4] = {0x00};        // Mouse IN Data Packet
uint8_t PREV_Mouse_Data_Pack[4] = {0x00};   // Mouse IN Data Packet
volatile uint8_t KB_LED_Last_Status = 0x00; // Keyboard LED Last Result
volatile uint8_t KB_LED_Cur_Status = 0x00;  // Keyboard LED Current Result

#define REPORT_QUEUE_CAPACITY 16

typedef struct {
  uint8_t reports[REPORT_QUEUE_CAPACITY][KEYMAP_HID_REPORT_KEYBOARD_LEN];
  uint8_t head;
  uint8_t count;
  uint8_t len;
} ReportQueue;

static ReportQueue keyboard_report_queue = {
    .len = KEYMAP_HID_REPORT_KEYBOARD_LEN,
};
static ReportQueue mouse_report_queue = {
    .len = sizeof(Mouse_Data_Pack),
};
static ReportQueue consumer_report_queue = {
    .len = sizeof(Consumer_Data_Pack),
};

static void report_queue_push_back(ReportQueue *queue, const uint8_t *report) {
  // Preserve the newest snapshots if the queue fills; older snapshots are
  // dropped first so the host still receives the latest state.
  uint8_t tail =
      (uint8_t)((queue->head + queue->count) % REPORT_QUEUE_CAPACITY);

  memcpy(queue->reports[tail], report, queue->len);

  if (queue->count < REPORT_QUEUE_CAPACITY) {
    queue->count++;
  } else {
    queue->head = (uint8_t)((queue->head + 1) % REPORT_QUEUE_CAPACITY);
  }
}

static void report_queue_push_front(ReportQueue *queue, const uint8_t *report) {
  queue->head = (uint8_t)((queue->head + REPORT_QUEUE_CAPACITY - 1) %
                          REPORT_QUEUE_CAPACITY);
  memcpy(queue->reports[queue->head], report, queue->len);

  if (queue->count < REPORT_QUEUE_CAPACITY) {
    queue->count++;
  }
}

static int report_queue_pop_front(ReportQueue *queue, uint8_t *report) {
  if (queue->count == 0) {
    return 0;
  }

  memcpy(report, queue->reports[queue->head], queue->len);
  queue->head = (uint8_t)((queue->head + 1) % REPORT_QUEUE_CAPACITY);
  queue->count--;
  return 1;
}

static void report_queue_send_next(uint8_t endp, ReportQueue *queue) {
  uint8_t report[KEYMAP_HID_REPORT_KEYBOARD_LEN] = {0};

  __disable_irq();
  if (USBFS_Endp_Busy[endp] != 0 || !report_queue_pop_front(queue, report)) {
    __enable_irq();
    return;
  }
  __enable_irq();

  if (USBFS_Endp_DataUp(endp, report, queue->len, DEF_UEP_CPY_LOAD) != 0) {
    __disable_irq();
    report_queue_push_front(queue, report);
    __enable_irq();
  }
}

void USB_ReportQueue_Reset(void) {
  __disable_irq();
  memset(KB_Data_Pack, 0, sizeof(KB_Data_Pack));
  memset(PREV_KB_Data_Pack, 0, sizeof(PREV_KB_Data_Pack));
  memset(Consumer_Data_Pack, 0, sizeof(Consumer_Data_Pack));
  memset(PREV_Consumer_Data_Pack, 0, sizeof(PREV_Consumer_Data_Pack));
  memset(Mouse_Data_Pack, 0, sizeof(Mouse_Data_Pack));
  memset(PREV_Mouse_Data_Pack, 0, sizeof(PREV_Mouse_Data_Pack));

  keyboard_report_queue.head = 0;
  keyboard_report_queue.count = 0;
  mouse_report_queue.head = 0;
  mouse_report_queue.count = 0;
  consumer_report_queue.head = 0;
  consumer_report_queue.count = 0;
  __enable_irq();
}

/*******************************************************************************/
/* Interrupt Function Declaration */
void TIM3_IRQHandler(void) __attribute__((interrupt()));

/*********************************************************************
 * @fn      TIM3_Init
 *
 * @brief   Initialize timer3 for keyboard and mouse scan.
 *
 * @param   arr - The specific period value
 *          psc - The specifies prescaler value
 *
 * @return  none
 */
void TIM3_Init(uint16_t arr, uint16_t psc) {
  TIM_TimeBaseInitTypeDef TIM_TimeBaseStructure = {0};
  NVIC_InitTypeDef NVIC_InitStructure = {0};

  /* Enable Timer3 Clock */
  RCC_APB1PeriphClockCmd(RCC_APB1Periph_TIM3, ENABLE);

  /* Initialize Timer3 */
  TIM_TimeBaseStructure.TIM_Period = arr;
  TIM_TimeBaseStructure.TIM_Prescaler = psc;
  TIM_TimeBaseStructure.TIM_ClockDivision = TIM_CKD_DIV1;
  TIM_TimeBaseStructure.TIM_CounterMode = TIM_CounterMode_Up;
  TIM_TimeBaseInit(TIM3, &TIM_TimeBaseStructure);

  TIM_ITConfig(TIM3, TIM_IT_Update, ENABLE);

  NVIC_InitStructure.NVIC_IRQChannel = TIM3_IRQn;
  NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
  NVIC_InitStructure.NVIC_IRQChannelSubPriority = 2;
  NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
  NVIC_Init(&NVIC_InitStructure);

  /* Enable Timer3 */
  TIM_Cmd(TIM3, ENABLE);
}

/*********************************************************************
 * @fn      TIM3_IRQHandler
 *
 * @brief   This function handles TIM3 global interrupt request.
 *
 * @return  none
 */
void TIM3_IRQHandler(void) {
  if (TIM_GetITStatus(TIM3, TIM_IT_Update) != RESET) {

    /* Handle keyboard scan */
    KB_Scan();

    /* Handle keyboard scan data */
    KB_Scan_Handle();

#ifdef KEYBOARD_LED_ENABLED
    keyboard_led_tick();
#endif

    keymap_tick(&hid_report);
    memcpy(KB_Data_Pack, hid_report.keyboard, sizeof(KB_Data_Pack));
    memcpy(Consumer_Data_Pack, hid_report.consumer, sizeof(Consumer_Data_Pack));
    Mouse_Data_Pack[0] = hid_report.mouse.pressed_buttons;
    Mouse_Data_Pack[1] = hid_report.mouse.x;
    Mouse_Data_Pack[2] = hid_report.mouse.y;
    Mouse_Data_Pack[3] = hid_report.mouse.vertical_scroll;

    USB_ReportQueue_EnqueueCurrent();

    /* Clear interrupt flag */
    TIM_ClearITPendingBit(TIM3, TIM_IT_Update);
  }
}

/*********************************************************************
 * @fn      KB_Scan_Init
 *
 * @brief   Initialize IO for keyboard scan.
 *
 * @return  none
 */
void KB_Scan_Init(void) {
  keyboard_init();

  keymap_init();
}

/*********************************************************************
 * @fn      KB_Sleep_Wakeup_Cfg
 *
 * @brief   Configure keyboard wake up mode.
 *
 * @return  none
 */
void KB_Sleep_Wakeup_Cfg(void) {
  EXTI_InitTypeDef EXTI_InitStructure = {0};

  /* Enable GPIOB clock */
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_AFIO, ENABLE);

  GPIO_EXTILineConfig(GPIO_PortSourceGPIOB, GPIO_PinSource0);
  EXTI_InitStructure.EXTI_Line = EXTI_Line0;
  EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
  EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
  EXTI_InitStructure.EXTI_LineCmd = ENABLE;
  EXTI_Init(&EXTI_InitStructure);

  GPIO_EXTILineConfig(GPIO_PortSourceGPIOB, GPIO_PinSource1);
  EXTI_InitStructure.EXTI_Line = EXTI_Line1;
  EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
  EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
  EXTI_InitStructure.EXTI_LineCmd = ENABLE;
  EXTI_Init(&EXTI_InitStructure);

  GPIO_EXTILineConfig(GPIO_PortSourceGPIOB, GPIO_PinSource3);
  EXTI_InitStructure.EXTI_Line = EXTI_Line3;
  EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
  EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
  EXTI_InitStructure.EXTI_LineCmd = ENABLE;
  EXTI_Init(&EXTI_InitStructure);

  GPIO_EXTILineConfig(GPIO_PortSourceGPIOB, GPIO_PinSource11);
  EXTI_InitStructure.EXTI_Line = EXTI_Line11;
  EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
  EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
  EXTI_InitStructure.EXTI_LineCmd = ENABLE;
  EXTI_Init(&EXTI_InitStructure);

  EXTI->INTENR |=
      EXTI_INTENR_MR0 | EXTI_INTENR_MR1 | EXTI_INTENR_MR3 | EXTI_INTENR_MR11;
}

/*********************************************************************
 * @fn      KB_Scan
 *
 * @brief   Perform keyboard scan.
 *
 * @return  none
 */
void KB_Scan(void) {
  keyboard_matrix_scan();
  KB_Scan_Done = 1;
}

/*********************************************************************
 * @fn      KB_Scan_Handle
 *
 * @brief   Handle keyboard scan data.
 *
 * @return  none
 */
void KB_Scan_Handle(void) {}

/*********************************************************************
 * @fn      KB_LED_Handle
 *
 * @brief   Handle keyboard lighting.
 *
 * @return  none
 */
void KB_LED_Handle(void) {
  if (KB_LED_Cur_Status != KB_LED_Last_Status) {
    if ((KB_LED_Cur_Status & 0x01) != (KB_LED_Last_Status & 0x01)) {
      if (KB_LED_Cur_Status & 0x01) {
        printf("Turn on the NUM LED\r\n");
      } else {
        printf("Turn off the NUM LED\r\n");
      }
    }
    if ((KB_LED_Cur_Status & 0x02) != (KB_LED_Last_Status & 0x02)) {
      if (KB_LED_Cur_Status & 0x02) {
        printf("Turn on the CAPS LED\r\n");
      } else {
        printf("Turn off the CAPS LED\r\n");
      }
    }
    if ((KB_LED_Cur_Status & 0x04) != (KB_LED_Last_Status & 0x04)) {
      if (KB_LED_Cur_Status & 0x04) {
        printf("Turn on the SCROLL LED\r\n");
      } else {
        printf("Turn off the SCROLL LED\r\n");
      }
    }
    KB_LED_Last_Status = KB_LED_Cur_Status;
  }
}

void USB_ReportQueue_EnqueueCurrent(void) {
  if (memcmp(KB_Data_Pack, PREV_KB_Data_Pack, sizeof(KB_Data_Pack)) != 0) {
    report_queue_push_back(&keyboard_report_queue, KB_Data_Pack);
    memcpy(PREV_KB_Data_Pack, KB_Data_Pack, sizeof(KB_Data_Pack));
  }

  if (memcmp(Mouse_Data_Pack, PREV_Mouse_Data_Pack, sizeof(Mouse_Data_Pack)) !=
      0) {
    report_queue_push_back(&mouse_report_queue, Mouse_Data_Pack);
    memcpy(PREV_Mouse_Data_Pack, Mouse_Data_Pack, sizeof(Mouse_Data_Pack));
  }

  if (memcmp(Consumer_Data_Pack, PREV_Consumer_Data_Pack,
             sizeof(Consumer_Data_Pack)) != 0) {
    report_queue_push_back(&consumer_report_queue, Consumer_Data_Pack);
    memcpy(PREV_Consumer_Data_Pack, Consumer_Data_Pack,
           sizeof(Consumer_Data_Pack));
  }
}

void USB_ReportQueue_Tick(void) {
  static uint8_t was_enum = 0;

  if (!USBFS_DevEnumStatus) {
    if (was_enum != 0) {
      USB_ReportQueue_Reset();
      was_enum = 0;
    }
    return;
  }

  if (was_enum == 0) {
    USB_ReportQueue_Reset();
    was_enum = 1;
  }

  report_queue_send_next(DEF_UEP1, &keyboard_report_queue);
  report_queue_send_next(DEF_UEP2, &mouse_report_queue);
  report_queue_send_next(DEF_UEP3, &consumer_report_queue);
}

/*********************************************************************
 * @fn      USB_Sleep_Wakeup_CFG
 *
 * @brief   Configure USB wake up mode
 *
 * @return  none
 */
void USB_Sleep_Wakeup_CFG(void) {
  EXTI_InitTypeDef EXTI_InitStructure = {0};

  EXTI_InitStructure.EXTI_Line = EXTI_Line28;
  EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
  EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Rising;
  EXTI_InitStructure.EXTI_LineCmd = ENABLE;
  EXTI_Init(&EXTI_InitStructure);
}

/*********************************************************************
 * @fn      MCU_Sleep_Wakeup_Operate
 *
 * @brief   Perform sleep operation
 *
 * @return  none
 */
void MCU_Sleep_Wakeup_Operate(void) {
  printf("Sleep\r\n");
  __disable_irq();
  EXTI_ClearFlag(EXTI_Line0 | EXTI_Line1 | EXTI_Line3 | EXTI_Line11);
  EXTI_ClearFlag(EXTI_Line4 | EXTI_Line5 | EXTI_Line6 | EXTI_Line7);

  PWR_EnterSTOPMode(PWR_STOPEntry_WFE);
  SystemInit();
  SystemCoreClockUpdate();
  USBFS_RCC_Init();

  if (EXTI_GetFlagStatus(EXTI_Line0 | EXTI_Line1 | EXTI_Line3 | EXTI_Line11) !=
      RESET) {
    EXTI_ClearFlag(EXTI_Line0 | EXTI_Line1 | EXTI_Line3 | EXTI_Line11);
    USBFS_Send_Resume();
  } else if (EXTI_GetFlagStatus(EXTI_Line4 | EXTI_Line5 | EXTI_Line6 |
                                EXTI_Line7) != RESET) {
    EXTI_ClearFlag(EXTI_Line4 | EXTI_Line5 | EXTI_Line6 | EXTI_Line7);
    USBFS_Send_Resume();
  }
  __enable_irq();
  printf("Wake\r\n");
}
