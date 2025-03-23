let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      split = {
        enabled
          | doc "Whether to enable split-keyboard functionality."
          | Bool
          | default
          = false,

        usart
          | C.UsartNumber
          | default
          = 2,
      },

      ..
    },
  },

  board
    | C.Board,

  gpio_pins
    | C.GpioPins
    =
      if board.split.enabled then
        {
          keyboard_split = m%"
          keyboard_split_init();
          "%,
        }
      else
        {},

  init_fragments =
    if board.split.enabled then
      {
        keyboard_split = m%"
          keyboard_split_init();
          "%,
      }
    else
      {},

  cmakelists =
    if board.split.enabled then
      {
        keyboard_split = m%"
          target_compile_definitions(keyboard_codegen INTERFACE KEYBOARD_SPLIT)
          target_sources(keyboard_codegen INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split.c)
          target_sources(keyboard_codegen INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split_rx_dma.c)
          target_sources(keyboard_codegen INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split_tx_dma.c)
    "%,
      }
    else
      {},

  includes =
    if board.split.enabled then
      {
        keyboard_split = m%"
          #pragma once

          #include "ch32x035_rcc.h"

          #include "smart_keymap.h"

          #define KEYBOARD_SPLIT_USART_RCC_APB_CLOCKCMD RCC_APB1PeriphClockCmd
          #define KEYBOARD_SPLIT_USART_RCC_APB_PERIPH RCC_APB1Periph_USART2
          #define KEYBOARD_SPLIT_RX_RCC_APB_PERIPH RCC_APB2Periph_GPIOA
          #define KEYBOARD_SPLIT_TX_RCC_APB_PERIPH RCC_APB2Periph_GPIOA

          #define KEYBOARD_SPLIT_RX_GPIO_PORT GPIO%{rx.port}
          #define KEYBOARD_SPLIT_RX_GPIO_PIN GPIO_Pin_%{rx.pin |> std.to_string}

          #define KEYBOARD_SPLIT_TX_GPIO_PORT GPIO%{tx.port}
          #define KEYBOARD_SPLIT_TX_GPIO_PIN GPIO_Pin_%{tx.pin |> std.to_string}

          #define KEYBOARD_SPLIT_USART USART%{usart_s}

          #define KEYBOARD_SPLIT_USART_IRQ_HANDLER USART%{usart |> std.to_string}_IRQHandler
          #define KEYBOARD_SPLIT_USART_IRQ_CHANNEL USART%{usart |> std.to_string}_IRQn

          #define KEYBOARD_SPLIT_RX_DMA DMA1_Channel%{rx_dma_channel_s}
          #define KEYBOARD_SPLIT_RX_DMA_IRQ_HANDLER DMA1_Channel%{rx_dma_channel_s}_IRQHandler
          #define KEYBOARD_SPLIT_RX_DMA_IRQ_CHANNEL DMA1_Channel%{rx_dma_channel_s}_IRQn
          #define KEYBOARD_SPLIT_RX_TC_FLAG DMA1_IT_TC%{rx_dma_channel_s}

          // DMA for USART2 TX: Channel 7
          #define KEYBOARD_SPLIT_TX_DMA DMA1_Channel7
          #define KEYBOARD_SPLIT_TX_DMA_IRQ_HANDLER DMA1_Channel7_IRQHandler
          #define KEYBOARD_SPLIT_TX_DMA_IRQ_CHANNEL DMA1_Channel7_IRQn
          #define KEYBOARD_SPLIT_TX_TC_FLAG DMA1_IT_TC7

          %{defines}

          void keyboard_split_init(void);

          int keyboard_split_write(KeymapInputEvent ev);
    "%,
      }
    else
      {},
}
