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

#include <string.h>

#include "ch32x035.h"
#include "debug.h"

#include "ch32x035_dma.h"
#include "ch32x035_exti.h"
#include "ch32x035_gpio.h"
#include "ch32x035_misc.h"
#include "ch32x035_rcc.h"
#include "ch32x035_tim.h"
#include "ch32x035_usart.h"

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
    (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11);   // Keyboard Keys Last Scan Result
KeymapHidReport hid_report = {0};           // Keyboard HID report
uint8_t KB_Data_Pack[8] = {0x00};           // Keyboard IN Data Packet
uint8_t PREV_KB_Data_Pack[8] = {0x00};      // Keyboard IN Data Packet
volatile uint8_t KB_LED_Last_Status = 0x00; // Keyboard LED Last Result
volatile uint8_t KB_LED_Cur_Status = 0x00;  // Keyboard LED Current Result

static uint32_t transmit_buffer[1]; // DEMO
#define RX_BUFFER_SIZE 64
static uint8_t msg_buffer[MESSAGE_BUFFER_LEN];

static struct {
  volatile uint8_t current_buffer;
  uint8_t rx_buffer[2][RX_BUFFER_SIZE];
} rx_buffers = {
    .current_buffer = 0,
    .rx_buffer = {0},
};

/*******************************************************************************/
/* Interrupt Function Declaration */
void TIM3_IRQHandler(void) __attribute__((interrupt("WCH-Interrupt-fast")));

// DEMO
void USART2_IRQHandler(void) __attribute__((interrupt("WCH-Interrupt-fast")));

// DEMO
void DMA1_Channel6_IRQHandler(void)
    __attribute__((interrupt("WCH-Interrupt-fast")));

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

    if (memcmp(KB_Data_Pack, PREV_KB_Data_Pack, sizeof(KB_Data_Pack)) == 0) {
      keymap_tick(&hid_report);
      memcpy(KB_Data_Pack, hid_report.keyboard, sizeof(KB_Data_Pack));
    }

    /* Clear interrupt flag */
    TIM_ClearITPendingBit(TIM3, TIM_IT_Update);
  }
}

// DEMO
void keymap_split_receive_bytes(uint8_t *buf, uint16_t len) {
  KeymapInputEvent ev;
  for (uint16_t i = 0; i < len; i++) {
    uint8_t recv_byte = buf[i];
    bool received_event =
        keymap_message_buffer_receive_byte(&msg_buffer, recv_byte, &ev);

    if (received_event) {
      keymap_register_input_event(ev);
    }
  }
}

void USART2_IRQHandler(void) {
  if (USART_GetITStatus(USART2, USART_IT_IDLE) != RESET) {
    // IDLE
    uint16_t rx_len = (RX_BUFFER_SIZE - DMA1_Channel6->CNTR);
    uint8_t old_buffer = rx_buffers.current_buffer;

    rx_buffers.current_buffer = !old_buffer;

    DMA_Cmd(DMA1_Channel6, DISABLE);
    DMA_SetCurrDataCounter(DMA1_Channel6, RX_BUFFER_SIZE);
    // Switch buffer
    DMA1_Channel6->MADDR =
        (uint32_t)(rx_buffers.rx_buffer[rx_buffers.current_buffer]);
    DMA_Cmd(DMA1_Channel6, ENABLE);

    USART_ReceiveData(USART2); // clear IDLE flag

    // Process received data
    keymap_split_receive_bytes(rx_buffers.rx_buffer[old_buffer], rx_len);
  }
}

// DEMO
void DMA1_Channel6_IRQHandler(void) {
  KeymapInputEvent ev;

  if (DMA_GetITStatus(DMA1_IT_TC6)) {
    uint16_t rx_len = RX_BUFFER_SIZE;
    uint8_t old_buffer = rx_buffers.current_buffer;

    rx_buffers.current_buffer = !old_buffer;

    DMA_Cmd(DMA1_Channel6, DISABLE);
    DMA_SetCurrDataCounter(DMA1_Channel6, RX_BUFFER_SIZE);
    // Switch buffer
    DMA1_Channel6->MADDR =
        (uint32_t)(rx_buffers.rx_buffer[rx_buffers.current_buffer]);
    DMA_Cmd(DMA1_Channel6, ENABLE);

    // Process received data
    keymap_split_receive_bytes(rx_buffers.rx_buffer[old_buffer], rx_len);

    DMA_ClearITPendingBit(DMA1_IT_TC6);
  }
}

// DEMO
void keyboard_split_init(void) {
  DMA_InitTypeDef DMA_InitStructure;

  // Enable DMA1 clock
  RCC_AHBPeriphClockCmd(RCC_AHBPeriph_DMA1, ENABLE);

  // Configure DMA for USART2 TX
  DMA_DeInit(DMA1_Channel7); // Channel 7 for USART2 TX

  DMA_InitStructure.DMA_PeripheralBaseAddr = (uint32_t)(&USART2->DATAR);
  DMA_InitStructure.DMA_MemoryBaseAddr = (uint32_t)transmit_buffer;
  DMA_InitStructure.DMA_DIR = DMA_DIR_PeripheralDST;
  DMA_InitStructure.DMA_BufferSize = 4; // Number of bytes to send
  DMA_InitStructure.DMA_PeripheralInc = DMA_PeripheralInc_Disable;
  DMA_InitStructure.DMA_MemoryInc = DMA_MemoryInc_Enable;
  DMA_InitStructure.DMA_PeripheralDataSize = DMA_PeripheralDataSize_Byte;
  DMA_InitStructure.DMA_MemoryDataSize = DMA_MemoryDataSize_Byte;
  DMA_InitStructure.DMA_Mode = DMA_Mode_Normal;
  DMA_InitStructure.DMA_Priority = DMA_Priority_Medium;
  DMA_InitStructure.DMA_M2M = DMA_M2M_Disable;

  DMA_Init(DMA1_Channel7, &DMA_InitStructure);

  // Configure DMA for USART2 RX
  DMA_DeInit(DMA1_Channel6); // Channel 6 for USART2 RX

  DMA_InitStructure.DMA_PeripheralBaseAddr = (uint32_t)(&USART2->DATAR);
  DMA_InitStructure.DMA_MemoryBaseAddr = (uint32_t)rx_buffers.rx_buffer[0];
  DMA_InitStructure.DMA_DIR = DMA_DIR_PeripheralSRC;
  DMA_InitStructure.DMA_BufferSize = RX_BUFFER_SIZE;
  DMA_InitStructure.DMA_Mode = DMA_Mode_Normal;

  DMA_Init(DMA1_Channel6, &DMA_InitStructure);

  // Enable DMA1 Channel6 Transfer Complete interrupt
  DMA_ITConfig(DMA1_Channel6, DMA_IT_TC, ENABLE);

  {
    NVIC_InitTypeDef NVIC_InitStructure = {0};
    NVIC_InitStructure.NVIC_IRQChannel = DMA1_Channel6_IRQn;
    NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelSubPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
    NVIC_Init(&NVIC_InitStructure);
  }

  GPIO_InitTypeDef GPIO_InitStructure = {0};
  USART_InitTypeDef USART_InitStructure = {0};
  NVIC_InitTypeDef NVIC_InitStructure = {0};

  RCC_APB1PeriphClockCmd(RCC_APB1Periph_USART2, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);

  /* USART2 TX-->A.2   RX-->A.3 */
  GPIO_InitStructure.GPIO_Pin = GPIO_Pin_2;
  GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
  GPIO_InitStructure.GPIO_Mode = GPIO_Mode_AF_PP;
  GPIO_Init(GPIOA, &GPIO_InitStructure);

  GPIO_InitStructure.GPIO_Pin = GPIO_Pin_3;
  GPIO_InitStructure.GPIO_Mode = GPIO_Mode_IN_FLOATING;
  GPIO_Init(GPIOA, &GPIO_InitStructure);

  USART_InitStructure.USART_BaudRate = 115200;
  USART_InitStructure.USART_WordLength = USART_WordLength_8b;
  USART_InitStructure.USART_StopBits = USART_StopBits_1;
  USART_InitStructure.USART_Parity = USART_Parity_No;
  USART_InitStructure.USART_HardwareFlowControl =
      USART_HardwareFlowControl_None;
  USART_InitStructure.USART_Mode = USART_Mode_Tx | USART_Mode_Rx;

  USART_Init(USART2, &USART_InitStructure);

  USART_Cmd(USART2, ENABLE);

  USART_ITConfig(USART2, USART_IT_IDLE, ENABLE);

  NVIC_InitStructure.NVIC_IRQChannel = USART2_IRQn;
  NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
  NVIC_InitStructure.NVIC_IRQChannelSubPriority = 1;
  NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
  NVIC_Init(&NVIC_InitStructure);

  // Enable USART DMA mode
  DMA_Cmd(DMA1_Channel6, ENABLE); // USART2 RX

  USART_DMACmd(USART2, USART_DMAReq_Tx | USART_DMAReq_Rx, ENABLE);
}

// DEMO
int keyboard_split_write(KeymapInputEvent ev) {
  static uint32_t buf[MESSAGE_BUFFER_LEN / 4];
  keymap_serialize_event((uint8_t *)buf, ev);

  // Wait for any previous DMA transfer to complete
  if ((DMA1_Channel7->CFGR & DMA_CFGR1_EN) > 0) {
    while (DMA_GetFlagStatus(DMA1_FLAG_TC7) == RESET)
      ;
    DMA_ClearFlag(DMA1_FLAG_TC7);
  }

  // Configure DMA for transmission
  DMA_Cmd(DMA1_Channel7, DISABLE);
  DMA1_Channel7->MADDR = (uint32_t)buf;
  DMA1_Channel7->CNTR = MESSAGE_BUFFER_LEN;
  DMA_Cmd(DMA1_Channel7, ENABLE);

  return MESSAGE_BUFFER_LEN;
}

/*********************************************************************
 * @fn      KB_Scan_Init
 *
 * @brief   Initialize IO for keyboard scan.
 *
 * @return  none
 */
void KB_Scan_Init(void) {
  keyboard_split_init(); // DEMO split

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
