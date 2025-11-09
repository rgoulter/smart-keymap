#ifdef KEYBOARD_SPLIT
#include "keyboard_split.h"

#include <stdbool.h>
#include <stdint.h>

#include "ch32x035_dma.h"
#include "ch32x035_gpio.h"
#include "ch32x035_misc.h"
#include "ch32x035_rcc.h"
#include "ch32x035_usart.h"

#include "smart_keymap.h"

#define RX_BUFFER_SIZE 16

static uint8_t msg_buffer[MESSAGE_BUFFER_LEN];

static struct {
  volatile uint8_t current_buffer;
  uint8_t rx_buffer[2][RX_BUFFER_SIZE];
} rx_buffers = {
    .current_buffer = 0,
    .rx_buffer = {0},
};

void KEYBOARD_SPLIT_USART_IRQ_HANDLER(void) __attribute__((interrupt()));

void KEYBOARD_SPLIT_RX_DMA_IRQ_HANDLER(void) __attribute__((interrupt()));

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

void KEYBOARD_SPLIT_USART_IRQ_HANDLER(void) {
  if (USART_GetITStatus(KEYBOARD_SPLIT_USART, USART_IT_IDLE) != RESET) {
    uint16_t rx_len = (RX_BUFFER_SIZE - KEYBOARD_SPLIT_RX_DMA->CNTR);
    uint8_t old_buffer = rx_buffers.current_buffer;

    rx_buffers.current_buffer = !old_buffer;

    DMA_Cmd(KEYBOARD_SPLIT_RX_DMA, DISABLE);
    DMA_SetCurrDataCounter(KEYBOARD_SPLIT_RX_DMA, RX_BUFFER_SIZE);
    // Switch buffer
    KEYBOARD_SPLIT_RX_DMA->MADDR =
        (uint32_t)(rx_buffers.rx_buffer[rx_buffers.current_buffer]);
    DMA_Cmd(KEYBOARD_SPLIT_RX_DMA, ENABLE);

    USART_ReceiveData(KEYBOARD_SPLIT_USART); // clear IDLE flag

    // Process received data
    keymap_split_receive_bytes(rx_buffers.rx_buffer[old_buffer], rx_len);
  }
}

void KEYBOARD_SPLIT_RX_DMA_IRQ_HANDLER(void) {
  KeymapInputEvent ev;

  if (DMA_GetITStatus(KEYBOARD_SPLIT_RX_TC_FLAG)) {
    uint16_t rx_len = RX_BUFFER_SIZE;
    uint8_t old_buffer = rx_buffers.current_buffer;

    rx_buffers.current_buffer = !old_buffer;

    DMA_Cmd(KEYBOARD_SPLIT_RX_DMA, DISABLE);
    DMA_SetCurrDataCounter(KEYBOARD_SPLIT_RX_DMA, RX_BUFFER_SIZE);
    KEYBOARD_SPLIT_RX_DMA->MADDR =
        (uint32_t)(rx_buffers.rx_buffer[rx_buffers.current_buffer]);
    DMA_Cmd(KEYBOARD_SPLIT_RX_DMA, ENABLE);

    // Process received data
    keymap_split_receive_bytes(rx_buffers.rx_buffer[old_buffer], rx_len);

    DMA_ClearITPendingBit(KEYBOARD_SPLIT_RX_TC_FLAG);
  }
}

void keyboard_split_init_rx(void) {
  // Enable DMA1 clock
  RCC_AHBPeriphClockCmd(RCC_AHBPeriph_DMA1, ENABLE);

  // Configure DMA for USART RX
  DMA_DeInit(KEYBOARD_SPLIT_RX_DMA);

  DMA_InitTypeDef DMA_InitStructure;
  DMA_InitStructure.DMA_PeripheralInc = DMA_PeripheralInc_Disable;
  DMA_InitStructure.DMA_MemoryInc = DMA_MemoryInc_Enable;
  DMA_InitStructure.DMA_PeripheralDataSize = DMA_PeripheralDataSize_Byte;
  DMA_InitStructure.DMA_MemoryDataSize = DMA_MemoryDataSize_Byte;
  DMA_InitStructure.DMA_Priority = DMA_Priority_Medium;
  DMA_InitStructure.DMA_M2M = DMA_M2M_Disable;

  DMA_InitStructure.DMA_PeripheralBaseAddr =
      (uint32_t)(&KEYBOARD_SPLIT_USART->DATAR);
  DMA_InitStructure.DMA_MemoryBaseAddr = (uint32_t)rx_buffers.rx_buffer[0];
  DMA_InitStructure.DMA_DIR = DMA_DIR_PeripheralSRC;
  DMA_InitStructure.DMA_BufferSize = RX_BUFFER_SIZE;
  DMA_InitStructure.DMA_Mode = DMA_Mode_Normal;

  DMA_Init(KEYBOARD_SPLIT_RX_DMA, &DMA_InitStructure);

  // Enable DMA Channel Transfer Complete interrupt
  DMA_ITConfig(KEYBOARD_SPLIT_RX_DMA, DMA_IT_TC, ENABLE);

  // Configure NVIC for DMA Channel
  {
    NVIC_InitTypeDef NVIC_InitStructure = {0};
    NVIC_InitStructure.NVIC_IRQChannel = KEYBOARD_SPLIT_RX_DMA_IRQ_CHANNEL;
    NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelSubPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
    NVIC_Init(&NVIC_InitStructure);
  }

  USART_ITConfig(KEYBOARD_SPLIT_USART, USART_IT_IDLE, ENABLE);

  // Configure NVIC for USART
  {
    NVIC_InitTypeDef NVIC_InitStructure = {0};
    NVIC_InitStructure.NVIC_IRQChannel = KEYBOARD_SPLIT_USART_IRQ_CHANNEL;
    NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelSubPriority = 1;
    NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
    NVIC_Init(&NVIC_InitStructure);
  }

  // Enable USART DMA mode
  DMA_Cmd(KEYBOARD_SPLIT_RX_DMA, ENABLE);

  USART_DMACmd(KEYBOARD_SPLIT_USART, USART_DMAReq_Rx, ENABLE);
}
#endif
