#include "keyboard_split.h"

#include "ch32x035.h"

#include "ch32x035_gpio.h"
#include "ch32x035_rcc.h"
#include "ch32x035_usart.h"

void keyboard_split_init_tx(void);

void keyboard_split_init_rx(void);

void keyboard_split_init(void) {
  GPIO_InitTypeDef GPIO_InitStructure = {0};

  KEYBOARD_SPLIT_USART_RCC_APB_CLOCKCMD(KEYBOARD_SPLIT_USART_RCC_APB_PERIPH,
                                        ENABLE);
  RCC_APB2PeriphClockCmd(KEYBOARD_SPLIT_RX_RCC_APB_PERIPH, ENABLE);
  RCC_APB2PeriphClockCmd(KEYBOARD_SPLIT_TX_RCC_APB_PERIPH, ENABLE);

  GPIO_InitStructure.GPIO_Pin = KEYBOARD_SPLIT_TX_GPIO_PIN;
  GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
  GPIO_InitStructure.GPIO_Mode = GPIO_Mode_AF_PP;
  GPIO_Init(KEYBOARD_SPLIT_TX_GPIO_PORT, &GPIO_InitStructure);

  GPIO_InitStructure.GPIO_Pin = KEYBOARD_SPLIT_RX_GPIO_PIN;
  GPIO_InitStructure.GPIO_Mode = GPIO_Mode_IN_FLOATING;
  GPIO_Init(KEYBOARD_SPLIT_RX_GPIO_PORT, &GPIO_InitStructure);

  USART_InitTypeDef USART_InitStructure = {0};
  USART_InitStructure.USART_BaudRate = 115200;
  USART_InitStructure.USART_WordLength = USART_WordLength_8b;
  USART_InitStructure.USART_StopBits = USART_StopBits_1;
  USART_InitStructure.USART_Parity = USART_Parity_No;
  USART_InitStructure.USART_HardwareFlowControl =
      USART_HardwareFlowControl_None;
  USART_InitStructure.USART_Mode = USART_Mode_Tx | USART_Mode_Rx;

  USART_Init(KEYBOARD_SPLIT_USART, &USART_InitStructure);

  USART_Cmd(KEYBOARD_SPLIT_USART, ENABLE);

  keyboard_split_init_rx();
  keyboard_split_init_tx();
}
