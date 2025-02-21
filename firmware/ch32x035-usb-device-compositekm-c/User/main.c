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
 * The matrix scan code is generated from ncl/matrix_scan.ncl, using whatever
 *  board.ncl file it was applied with.
 * e.g. by default, with the WeAct CH32X core board: cols: B0, B1, rows: B3,
 * B11.
 */

#include <stdint.h>

#include "debug.h"
#include "usbd_composite_km.h"
#include <ch32x035_usbfs_device.h>

extern uint8_t KB_Data_Pack[8];
extern uint8_t PREV_KB_Data_Pack[8];

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

  static uint8_t sending = 0;

  /* Usb Init */
  USBFS_RCC_Init();
  USBFS_Device_Init(ENABLE, PWR_VDD_SupplyVoltage());
  USB_Sleep_Wakeup_CFG();
  while (1) {
    if (USBFS_DevEnumStatus) {
      if (memcmp(KB_Data_Pack, PREV_KB_Data_Pack, sizeof(KB_Data_Pack)) != 0) {
        if (sending == 0) {
          int status = USBFS_Endp_DataUp(
              DEF_UEP1, KB_Data_Pack, sizeof(KB_Data_Pack), DEF_UEP_CPY_LOAD);
          sending = 1;
        } else if (USBFS_Endp_Busy[DEF_UEP1] == 0) {
          memcpy(PREV_KB_Data_Pack, KB_Data_Pack, sizeof(KB_Data_Pack));
          sending = 0;
        }
      }

      /* Handle keyboard lighting */
      KB_LED_Handle();
    }
  }
}
