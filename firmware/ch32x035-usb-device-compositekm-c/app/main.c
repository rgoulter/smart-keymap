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
#include "keyboard_led.h"
#include "smart_keymap.h"
#include "system_ch32x035.h"
#include "usbd_composite_km.h"

extern void keymap_set_panic_hook(void (*hook)(void));

static void DebugProbe_OnPanic(void) { DebugProbe_Fault('P'); }

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

#ifdef KEYBOARD_LED_ENABLED
  /* TIM3 also toggles this pin. A blink from here means main is looping,
   * including after the timer handler has stopped returning.
   */
  keyboard_led_claim();
#endif
  keymap_set_panic_hook(DebugProbe_OnPanic);
  printf("probe: blink=main alive, solid=fault; UART .=main, phase T/k/t, "
         "H/P\r\n");

  /* Initialize timer for Keyboard and mouse scan timing */
  TIM3_Init(47999, 0);
  printf("TIM3 Init OK!\r\n");

  /* Usb Init */
  USBFS_RCC_Init();
  USBFS_Device_Init(ENABLE, PWR_VDD_SupplyVoltage());
  USB_Sleep_Wakeup_CFG();
  while (1) {
    USB_ReportQueue_Tick();
    DebugProbe_MainHeartbeat();

    if (USBFS_DevEnumStatus) {
      KB_LED_Handle();
    }
  }
}
