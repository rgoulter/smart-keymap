/********************************** (C) COPYRIGHT
 ******************************** File Name          : usbd_composite_km.h
 * Author             : WCH
 * Version            : V1.0.0
 * Date               : 2023/04/06
 * Description        : USB keyboard and mouse processing.
 *********************************************************************************
 * Copyright (c) 2021 Nanjing Qinheng Microelectronics Co., Ltd.
 * Attention: This software (modified or not) and binary are used for
 * microcontroller manufactured by Nanjing Qinheng Microelectronics.
 *******************************************************************************/

#ifndef __USBD_COMPOSITE_KM_H
#define __USBD_COMPOSITE_KM_H

/*******************************************************************************/
/* Header Files */

#include <stdint.h>

/*******************************************************************************/
/* Keyboard Key Value Macro Definition */
#define DEF_KEY_CHAR_W 0x1A /* "W" */
#define DEF_KEY_CHAR_A 0x04 /* "A" */
#define DEF_KEY_CHAR_S 0x16 /* "S" */
#define DEF_KEY_CHAR_D 0x07 /* "D" */

/*******************************************************************************/
/* Global Variable Declaration */
extern volatile uint8_t KB_LED_Last_Status;
extern volatile uint8_t KB_LED_Cur_Status;

/*******************************************************************************/
/* Function Declaration */
extern void TIM3_Init(uint16_t arr, uint16_t psc);
extern void KB_Scan_Init(void);
extern void KB_Sleep_Wakeup_Cfg(void);
extern void KB_Scan(void);
extern void KB_Scan_Handle(void);
extern void KB_LED_Handle(void);
extern void USB_Sleep_Wakeup_CFG(void);
extern void MCU_Sleep_Wakeup_Operate(void);

#endif
