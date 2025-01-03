/********************************** (C) COPYRIGHT *******************************
 * File Name          : main.c
 * Author             : WCH
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
 * This example uses PB0, PB1, PB3, PB11 to simulate keyboard key pressing, active low.
 */

#include <ch32x035_usbfs_device.h>
#include "debug.h"
#include "usbd_composite_km.h"


/*********************************************************************
 * @fn      main
 *
 * @brief   Main program.
 *
 * @return  none
 */
int main(void)
{

    NVIC_PriorityGroupConfig(NVIC_PriorityGroup_1);
    SystemCoreClockUpdate();
    Delay_Init();
    USART_Printf_Init(115200);
    printf("SystemClk:%d\r\n", SystemCoreClock);
    printf("ChipID:%08x\r\n", DBGMCU_GetCHIPID() );

    /* Initialize GPIO for keyboard scan */
    KB_Scan_Init( );
    KB_Sleep_Wakeup_Cfg( );
    printf( "KB Scan Init OK!\r\n" );

    /* Initialize timer for Keyboard and mouse scan timing */
    TIM3_Init( 47999, 0 );
    printf( "TIM3 Init OK!\r\n" );


    /* Usb Init */
    USBFS_RCC_Init( );
    USBFS_Device_Init( ENABLE , PWR_VDD_SupplyVoltage());
    USB_Sleep_Wakeup_CFG( );
    while(1)
    {
    }
}
