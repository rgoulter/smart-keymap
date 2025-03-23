#include "keyboard_split.h"

#include <stdbool.h>
#include <stdint.h>

#include "debug.h"

#include "ch32x035_dma.h"
#include "ch32x035_gpio.h"
#include "ch32x035_misc.h"
#include "ch32x035_rcc.h"
#include "ch32x035_usart.h"

#include "smart_keymap.h"

#define TX_QUEUE_SIZE 16 // Must be a power of 2 (e.g., 8, 16, 32)
#if (TX_QUEUE_SIZE & (TX_QUEUE_SIZE - 1)) != 0
#error TX_QUEUE_SIZE must be a power of 2
#endif

// Ring buffer structure to hold outgoing events
static volatile KeymapInputEvent tx_event_queue[TX_QUEUE_SIZE];
static volatile uint8_t tx_queue_head = 0; // Index to write next event
static volatile uint8_t tx_queue_tail = 0; // Index to read next event for TX

static uint32_t transmit_dma_buffer[MESSAGE_BUFFER_LEN / 4];

static volatile bool uart_tx_busy = false;

void KEYBOARD_SPLIT_TX_DMA_IRQ_HANDLER(void)
    __attribute__((interrupt("WCH-Interrupt-fast")));
void KEYBOARD_SPLIT_TX_DMA_IRQ_HANDLER(void) {
  if (DMA_GetITStatus(KEYBOARD_SPLIT_TX_TC_FLAG)) {
    DMA_ClearITPendingBit(KEYBOARD_SPLIT_TX_TC_FLAG);

    // Check if there are more events in the queue
    if (tx_queue_head != tx_queue_tail) {
      // Dequeue the next event
      KeymapInputEvent event_to_send = tx_event_queue[tx_queue_tail];
      tx_queue_tail = (tx_queue_tail + 1) & (TX_QUEUE_SIZE - 1);

      keymap_serialize_event((uint8_t *)transmit_dma_buffer, event_to_send);

      DMA_Cmd(KEYBOARD_SPLIT_TX_DMA, DISABLE);
      DMA_SetCurrDataCounter(KEYBOARD_SPLIT_TX_DMA, MESSAGE_BUFFER_LEN);
      DMA_Cmd(KEYBOARD_SPLIT_TX_DMA, ENABLE);

      uart_tx_busy = true;
    } else {
      uart_tx_busy = false;
    }
  }
}

void keyboard_split_init_tx(void) {
  // Enable DMA1 clock
  RCC_AHBPeriphClockCmd(RCC_AHBPeriph_DMA1, ENABLE);

  // Configure DMA for USART TX
  DMA_DeInit(KEYBOARD_SPLIT_TX_DMA);
  DMA_InitTypeDef DMA_InitStructure;
  DMA_InitStructure.DMA_PeripheralBaseAddr =
      (uint32_t)(&KEYBOARD_SPLIT_USART->DATAR);
  DMA_InitStructure.DMA_MemoryBaseAddr = (uint32_t)transmit_dma_buffer;
  DMA_InitStructure.DMA_DIR = DMA_DIR_PeripheralDST;
  DMA_InitStructure.DMA_BufferSize = MESSAGE_BUFFER_LEN;
  DMA_InitStructure.DMA_PeripheralInc = DMA_PeripheralInc_Disable;
  DMA_InitStructure.DMA_MemoryInc = DMA_MemoryInc_Enable;
  DMA_InitStructure.DMA_PeripheralDataSize = DMA_PeripheralDataSize_Byte;
  DMA_InitStructure.DMA_MemoryDataSize = DMA_MemoryDataSize_Byte;
  DMA_InitStructure.DMA_Mode = DMA_Mode_Normal;
  DMA_InitStructure.DMA_Priority = DMA_Priority_Medium;
  DMA_InitStructure.DMA_M2M = DMA_M2M_Disable;
  DMA_Init(KEYBOARD_SPLIT_TX_DMA, &DMA_InitStructure);

  // --- Enable DMA TX Transfer Complete Interrupt ---
  DMA_ITConfig(KEYBOARD_SPLIT_TX_DMA, DMA_IT_TC, ENABLE);

  // Configure NVIC for DMA Channel
  NVIC_InitTypeDef NVIC_InitStructure = {0};
  NVIC_InitStructure.NVIC_IRQChannel = KEYBOARD_SPLIT_TX_DMA_IRQ_CHANNEL;
  NVIC_InitStructure.NVIC_IRQChannelPreemptionPriority = 1;
  NVIC_InitStructure.NVIC_IRQChannelSubPriority = 1;
  NVIC_InitStructure.NVIC_IRQChannelCmd = ENABLE;
  NVIC_Init(&NVIC_InitStructure);

  USART_DMACmd(KEYBOARD_SPLIT_USART, USART_DMAReq_Tx, ENABLE);
}

/*********************************************************************
 * @fn      keyboard_split_queue_event
 *
 * @brief   Adds a keyboard event to the TX queue for asynchronous sending.
 *          If the UART TX is idle, it starts the first DMA transfer.
 *
 * @param   ev - The KeymapInputEvent to queue.
 *
 * @return  0 on success, -1 if queue is full.
 */
int keyboard_split_write(KeymapInputEvent ev) {
  uint8_t next_head = (tx_queue_head + 1) & (TX_QUEUE_SIZE - 1);

  // Check if the queue is full
  if (next_head == tx_queue_tail) {
    // Queue is full, drop the event (or handle error differently)
    printf("WARN: TX Queue Full! Dropping event {t=%d, v=%d}\r\n",
           ev.event_type, ev.value);
    return -1;
  }

  // Add the event to the queue
  tx_event_queue[tx_queue_head] = ev;
  tx_queue_head = next_head; // Move head index

  // --- Critical Section Start ---
  __disable_irq();

  // If UART TX was idle, start the transmission process
  if (!uart_tx_busy) {
    if (tx_queue_head != tx_queue_tail) {
      uart_tx_busy = true; // Mark as busy *before* starting DMA

      // Dequeue the next event
      KeymapInputEvent event_to_send = tx_event_queue[tx_queue_tail];
      tx_queue_tail = (tx_queue_tail + 1) & (TX_QUEUE_SIZE - 1);

      // Serialize the event into the DMA buffer
      keymap_serialize_event((uint8_t *)transmit_dma_buffer, event_to_send);

      // Configure and start DMA
      DMA_Cmd(KEYBOARD_SPLIT_TX_DMA, DISABLE);
      DMA_SetCurrDataCounter(KEYBOARD_SPLIT_TX_DMA, MESSAGE_BUFFER_LEN);
      KEYBOARD_SPLIT_TX_DMA->MADDR = (uint32_t)transmit_dma_buffer;
      DMA_Cmd(KEYBOARD_SPLIT_TX_DMA, ENABLE);
    }
  }

  // --- Critical Section End ---
  __enable_irq();

  return 0;
}
