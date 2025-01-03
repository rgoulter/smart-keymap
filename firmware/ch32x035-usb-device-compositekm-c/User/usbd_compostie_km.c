/********************************** (C) COPYRIGHT *******************************
 * File Name          : usbd_composite_km.c
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
#include <ch32x035_usbfs_device.h>
#include "usbd_composite_km.h"

#include "smart_keymap.h"

/*******************************************************************************/
/* Global Variable Definition */

/* Keyboard */
volatile uint8_t  KB_Scan_Done = 0x00;                                          // Keyboard Keys Scan Done
volatile uint16_t KB_Scan_Result = (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11);                                      // Keyboard Keys Current Scan Result
volatile uint16_t KB_Scan_Last_Result = (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11);                                 // Keyboard Keys Last Scan Result
uint8_t  KB_Data_Pack[ 8 ] = { 0x00 };                                          // Keyboard IN Data Packet
volatile uint8_t  KB_LED_Last_Status = 0x00;                                    // Keyboard LED Last Result
volatile uint8_t  KB_LED_Cur_Status = 0x00;                                     // Keyboard LED Current Result

/*******************************************************************************/
/* Interrupt Function Declaration */
void TIM3_IRQHandler(void) __attribute__((interrupt("WCH-Interrupt-fast")));

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
void TIM3_Init( uint16_t arr, uint16_t psc )
{
    TIM_TimeBaseInitTypeDef TIM_TimeBaseStructure = { 0 };
    NVIC_InitTypeDef NVIC_InitStructure = { 0 };

    /* Enable Timer3 Clock */
    RCC_APB1PeriphClockCmd( RCC_APB1Periph_TIM3, ENABLE );

    /* Initialize Timer3 */
    TIM_TimeBaseStructure.TIM_Period = arr;
    TIM_TimeBaseStructure.TIM_Prescaler = psc;
    TIM_TimeBaseStructure.TIM_ClockDivision = TIM_CKD_DIV1;
    TIM_TimeBaseStructure.TIM_CounterMode = TIM_CounterMode_Up;
    TIM_TimeBaseInit( TIM3, &TIM_TimeBaseStructure );

    TIM_ITConfig( TIM3, TIM_IT_Update, ENABLE );

    NVIC_InitStructure.NVIC_IRQChannel = TIM3_IRQn;
    NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelSubPriority = 2;
    NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
    NVIC_Init( &NVIC_InitStructure );

    /* Enable Timer3 */
    TIM_Cmd( TIM3, ENABLE );
}

/*********************************************************************
 * @fn      TIM3_IRQHandler
 *
 * @brief   This function handles TIM3 global interrupt request.
 *
 * @return  none
 */
void TIM3_IRQHandler( void )
{
    if( TIM_GetITStatus( TIM3, TIM_IT_Update ) != RESET )
    {
        /* Clear interrupt flag */
        TIM_ClearITPendingBit( TIM3, TIM_IT_Update );

        /* Handle keyboard scan */
        KB_Scan( );

        /* Handle keyboard scan data */
        KB_Scan_Handle(  );

        keymap_tick(KB_Data_Pack);

        /* Determine if enumeration is complete, perform data transfer if completed */
        if(USBFS_DevEnumStatus)
        {
            if( USBFS_DevEnumStatus )
            {
                USBFS_Endp_DataUp( DEF_UEP1, KB_Data_Pack, sizeof( KB_Data_Pack ), DEF_UEP_CPY_LOAD );

                /* Handle keyboard lighting */
                KB_LED_Handle( );
            }
        }
    }
}

/*********************************************************************
 * @fn      KB_Scan_Init
 *
 * @brief   Initialize IO for keyboard scan.
 *
 * @return  none
 */
void KB_Scan_Init( void )
{
    GPIO_InitTypeDef GPIO_InitStructure = { 0 };

    /* Enable GPIOB clock */
    RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOB, ENABLE );

    /* Initialize GPIOB (Pin4-Pin7) for the keyboard scan */
    GPIO_InitStructure.GPIO_Pin = GPIO_Pin_0 | GPIO_Pin_1 | GPIO_Pin_3 | GPIO_Pin_11;
    GPIO_InitStructure.GPIO_Mode = GPIO_Mode_IPU;
    GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
    GPIO_Init( GPIOB, &GPIO_InitStructure );

    keymap_init();
}

/*********************************************************************
 * @fn      KB_Sleep_Wakeup_Cfg
 *
 * @brief   Configure keyboard wake up mode.
 *
 * @return  none
 */
void KB_Sleep_Wakeup_Cfg( void )
{
    EXTI_InitTypeDef EXTI_InitStructure = { 0 };

    /* Enable GPIOB clock */
    RCC_APB2PeriphClockCmd( RCC_APB2Periph_AFIO, ENABLE );

    GPIO_EXTILineConfig( GPIO_PortSourceGPIOB, GPIO_PinSource0 );
    EXTI_InitStructure.EXTI_Line = EXTI_Line0;
    EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
    EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
    EXTI_InitStructure.EXTI_LineCmd = ENABLE;
    EXTI_Init( &EXTI_InitStructure );

    GPIO_EXTILineConfig( GPIO_PortSourceGPIOB, GPIO_PinSource1 );
    EXTI_InitStructure.EXTI_Line = EXTI_Line1;
    EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
    EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
    EXTI_InitStructure.EXTI_LineCmd = ENABLE;
    EXTI_Init( &EXTI_InitStructure );

    GPIO_EXTILineConfig( GPIO_PortSourceGPIOB, GPIO_PinSource3 );
    EXTI_InitStructure.EXTI_Line = EXTI_Line3;
    EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
    EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
    EXTI_InitStructure.EXTI_LineCmd = ENABLE;
    EXTI_Init( &EXTI_InitStructure );

    GPIO_EXTILineConfig( GPIO_PortSourceGPIOB, GPIO_PinSource11 );
    EXTI_InitStructure.EXTI_Line = EXTI_Line11;
    EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
    EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Falling;
    EXTI_InitStructure.EXTI_LineCmd = ENABLE;
    EXTI_Init( &EXTI_InitStructure );

    EXTI->INTENR |= EXTI_INTENR_MR0 | EXTI_INTENR_MR1 | EXTI_INTENR_MR3 | EXTI_INTENR_MR11;
}

/*********************************************************************
 * @fn      KB_Scan
 *
 * @brief   Perform keyboard scan.
 *
 * @return  none
 */
void KB_Scan( void )
{
    static uint16_t scan_cnt = 0;
    static uint16_t scan_result = 0;

    scan_cnt++;
    if( ( scan_cnt % 10 ) == 0 )
    {
        scan_cnt = 0;

        /* Determine whether the two scan results are consistent */
        if( scan_result == ( GPIO_ReadInputData( GPIOB ) & (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11) ) )
        {
            KB_Scan_Done = 1;
            KB_Scan_Result = scan_result;
        }
    }
    else if( ( scan_cnt % 5 ) == 0 )
    {
        /* Save the first scan result */
        scan_result = ( GPIO_ReadInputData( GPIOB ) & (1 << 0 | 1 << 1 | 1 << 3 | 1 << 11) );
    }
}

/*********************************************************************
 * @fn      KB_Scan_Handle
 *
 * @brief   Handle keyboard scan data.
 *
 * @return  none
 */
void KB_Scan_Handle( void )
{
    uint8_t i, j;
    uint8_t status;
    static uint8_t flag = 0x00;

    if( KB_Scan_Done )
    {
        KB_Scan_Done = 0;

        if( KB_Scan_Result != KB_Scan_Last_Result )
        {
            for( i = 0; i < 16; i++ )
            {
                // ignore all except 0, 1, 3, 11:
                switch (i) {
                    case 2:
                    case 4:
                    case 5:
                    case 6:
                    case 7:
                    case 8:
                    case 9:
                    case 10:
                    case 12:
                    case 13:
                    case 14:
                    case 15:
                        continue;
                }

                /* Determine that there is at least one key is pressed or released */
                if( ( KB_Scan_Result & ( 1 << i ) ) != ( KB_Scan_Last_Result & ( 1 << i ) ) )
                {
                    if( ( KB_Scan_Result & ( 1 << i ) ) )           // Key press
                    {
                        if( i == 0 )
                        {
                            keymap_register_input_keyrelease(0);
                        }
                        else if( i == 1 )
                        {
                            keymap_register_input_keyrelease(1);
                        }
                        else if( i == 3 )
                        {
                            keymap_register_input_keyrelease(2);
                        }
                        else if( i == 11 )
                        {
                            keymap_register_input_keyrelease(3);
                        }
                    }
                    else                                            // Key release
                    {
                        if( i == 0 )
                        {
                            keymap_register_input_keypress(0);
                        }
                        else if( i == 1 )
                        {
                            keymap_register_input_keypress(1);
                        }
                        else if( i == 3 )
                        {
                            keymap_register_input_keypress(2);
                        }
                        else if( i == 11 )
                        {
                            keymap_register_input_keypress(3);
                        }
                    }
                }
            }

            /* Copy the keyboard data to the buffer of endpoint 1 and set the data uploading flag */
            KB_Scan_Last_Result = KB_Scan_Result;
            flag = 1;
        }
    }
}

/*********************************************************************
 * @fn      KB_LED_Handle
 *
 * @brief   Handle keyboard lighting.
 *
 * @return  none
 */
void KB_LED_Handle( void )
{
    if( KB_LED_Cur_Status != KB_LED_Last_Status )
    {
        if( ( KB_LED_Cur_Status & 0x01 ) != ( KB_LED_Last_Status & 0x01 ) )
        {
            if( KB_LED_Cur_Status & 0x01 )
            {
                printf("Turn on the NUM LED\r\n");
            }
            else
            {
                printf("Turn off the NUM LED\r\n");
            }
        }
        if( ( KB_LED_Cur_Status & 0x02 ) != ( KB_LED_Last_Status & 0x02 ) )
        {
            if( KB_LED_Cur_Status & 0x02 )
            {
                printf("Turn on the CAPS LED\r\n");
            }
            else
            {
                printf("Turn off the CAPS LED\r\n");
            }
        }
        if( ( KB_LED_Cur_Status & 0x04 ) != ( KB_LED_Last_Status & 0x04 ) )
        {
            if( KB_LED_Cur_Status & 0x04 )
            {
                printf("Turn on the SCROLL LED\r\n");
            }
            else
            {
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
void USB_Sleep_Wakeup_CFG( void )
{
    EXTI_InitTypeDef EXTI_InitStructure = { 0 };

    EXTI_InitStructure.EXTI_Line = EXTI_Line28;
    EXTI_InitStructure.EXTI_Mode = EXTI_Mode_Event;
    EXTI_InitStructure.EXTI_Trigger = EXTI_Trigger_Rising;
    EXTI_InitStructure.EXTI_LineCmd = ENABLE;
    EXTI_Init( &EXTI_InitStructure );
}

/*********************************************************************
 * @fn      MCU_Sleep_Wakeup_Operate
 *
 * @brief   Perform sleep operation
 *
 * @return  none
 */
void MCU_Sleep_Wakeup_Operate( void )
{
    printf( "Sleep\r\n" );
    __disable_irq();
    EXTI_ClearFlag( EXTI_Line0 | EXTI_Line1 | EXTI_Line3 | EXTI_Line11 );
    EXTI_ClearFlag( EXTI_Line4 | EXTI_Line5 | EXTI_Line6 | EXTI_Line7 );

    PWR_EnterSTOPMode(PWR_STOPEntry_WFE);
    SystemInit();
    SystemCoreClockUpdate();
    USBFS_RCC_Init();

    if( EXTI_GetFlagStatus( EXTI_Line0 | EXTI_Line1 | EXTI_Line3 | EXTI_Line11 ) != RESET  )
    {
        EXTI_ClearFlag( EXTI_Line0 | EXTI_Line1 | EXTI_Line3 | EXTI_Line11 );
        USBFS_Send_Resume( );
    }
    else if( EXTI_GetFlagStatus( EXTI_Line4 | EXTI_Line5 | EXTI_Line6 | EXTI_Line7 ) != RESET )
    {
        EXTI_ClearFlag( EXTI_Line4 | EXTI_Line5 | EXTI_Line6 | EXTI_Line7 );
        USBFS_Send_Resume( );
    }
    __enable_irq();
    printf( "Wake\r\n" );
}
