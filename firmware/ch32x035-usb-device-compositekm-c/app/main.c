/********************************** (C) COPYRIGHT
 ******************************** File Name          : main.c Author : WCH
 * Version            : V1.0.0
 * Date               : 2023/12/26
 * Description        : Main program body.
 *********************************************************************************
 * Copyright (c) 2021 Nanjing Qinheng Microelectronics Co., Ltd.
 * Attention: This software (modified or not) and binary are used for
 * microcontroller manufactured by Nanjing Qinheng Microelectronics.
 *******************************************************************************/

/*
 * @Note
 * Composite Keyboard and Mouse Example:
 *
 * The code for matrix scanning is generated from ncl/codegen_keyboard.ncl,
 * using whatever board.ncl file it was applied with. e.g. by default, with the
 * WeAct CH32X core board: cols: B0, B1, rows: B3, B11.
 */

#include <stdint.h>
#include <string.h>

#include "debug.h"

#include "ch32x035_dbgmcu.h"
#include "ch32x035_gpio.h"
#include "ch32x035_misc.h"

#include "ch32x035_usbfs_device.h"
#include "smart_keymap.h"
#include "system_ch32x035.h"
#include "usbd_composite_km.h"

extern uint8_t KB_Data_Pack[8];
extern uint8_t PREV_KB_Data_Pack[8];
extern uint8_t Consumer_Data_Pack[KEYMAP_HID_REPORT_CONSUMER_LEN];
extern uint8_t PREV_Consumer_Data_Pack[KEYMAP_HID_REPORT_CONSUMER_LEN];
extern uint8_t Mouse_Data_Pack[4];
extern uint8_t PREV_Mouse_Data_Pack[4];

/*********************************************************************
 * @fn      main
 *
 * @brief   Main program.
 *
 * @return  none
 */
int main(void) {

  NVIC_PriorityGroupConfig(NVIC_PriorityGroup_1);
  SystemCoreClockUpdate();
  Delay_Init();
  USART_Printf_Init(115200);
  printf("SystemClk:%d\r\n", SystemCoreClock);
  printf("ChipID:%08x\r\n", DBGMCU_GetCHIPID());

  /* Initialize GPIO for keyboard scan */
  KB_Scan_Init();
  KB_Sleep_Wakeup_Cfg();
  printf("KB Scan Init OK!\r\n");

  /* Initialize timer for Keyboard and mouse scan timing */
  TIM3_Init(47999, 0);
  printf("TIM3 Init OK!\r\n");

  static uint8_t sending_kb = 0;
  static uint8_t sending_mouse = 0;
  static uint8_t sending_consumer = 0;

  /* Usb Init */
  USBFS_RCC_Init();
  USBFS_Device_Init(ENABLE, PWR_VDD_SupplyVoltage());
  USB_Sleep_Wakeup_CFG();
  while (1) {
    if (USBFS_DevEnumStatus) {
      if (memcmp(KB_Data_Pack, PREV_KB_Data_Pack, sizeof(KB_Data_Pack)) != 0) {
        if (sending_kb == 0) {
          USBFS_Endp_DataUp(DEF_UEP1, KB_Data_Pack, sizeof(KB_Data_Pack),
                            DEF_UEP_CPY_LOAD);
          sending_kb = 1;
        } else if (USBFS_Endp_Busy[DEF_UEP1] == 0) {
          memcpy(PREV_KB_Data_Pack, KB_Data_Pack, sizeof(KB_Data_Pack));
          sending_kb = 0;
        }
      }

      if (memcmp(Mouse_Data_Pack, PREV_Mouse_Data_Pack,
                 sizeof(Mouse_Data_Pack)) != 0 ||
          memcmp(Mouse_Data_Pack,
                 (uint8_t[4]){
                     0x00,
                     0x00,
                     0x00,
                     0x00,
                 },
                 sizeof(Mouse_Data_Pack)) != 0) {
        if (sending_mouse == 0) {
          USBFS_Endp_DataUp(DEF_UEP2, Mouse_Data_Pack, sizeof(Mouse_Data_Pack),
                            DEF_UEP_CPY_LOAD);
          sending_mouse = 1;
        } else if (USBFS_Endp_Busy[DEF_UEP2] == 0) {
          memcpy(PREV_Mouse_Data_Pack, Mouse_Data_Pack,
                 sizeof(Mouse_Data_Pack));
          sending_mouse = 0;
        }
      }

      if (memcmp(Consumer_Data_Pack, PREV_Consumer_Data_Pack,
                 sizeof(Consumer_Data_Pack)) != 0) {
        if (sending_consumer == 0) {
          USBFS_Endp_DataUp(DEF_UEP3, Consumer_Data_Pack,
                            sizeof(Consumer_Data_Pack), DEF_UEP_CPY_LOAD);
          sending_consumer = 1;
        } else if (USBFS_Endp_Busy[DEF_UEP3] == 0) {
          memcpy(PREV_Consumer_Data_Pack, Consumer_Data_Pack,
                 sizeof(Consumer_Data_Pack));
          sending_consumer = 0;
        }
      }

      /* Handle keyboard lighting */
      KB_LED_Handle();
    }
  }
}
