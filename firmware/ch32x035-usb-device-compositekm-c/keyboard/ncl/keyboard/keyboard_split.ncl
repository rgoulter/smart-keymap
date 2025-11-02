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
    | C.GpioPins,

  cmakelists =
    if board.split.enabled then
      {
        keyboard_split = m%"
          target_compile_definitions(keyboard INTERFACE KEYBOARD_SPLIT)
          target_sources(keyboard INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split_rx_dma.c)
          target_sources(keyboard INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split_tx_dma.c)
    "%,
      }
    else
      {},

  includes =
    if board.split.enabled then
      let { usart, .. } = board.split in
      let usart_s = usart |> std.to_string in
      let usart_apb =
        usart
        |> match {
          2 => "APB1",
          _ => std.fail_with "not implemented",
        }
      in
      let { rx, tx } =
        usart
        |> match {
          2 =>
            {
              rx = gpio_pins.A3,
              tx = gpio_pins.A2,
            },
          _ => std.fail_with "not implemented",
        }
      in
      let { rx_dma_channel, tx_dma_channel } =
        usart
        |> match {
          2 =>
            {
              rx_dma_channel = 6,
              tx_dma_channel = 7,
            },
          _ => std.fail_with "not implemented",
        }
      in
      let rx_dma_channel_s = rx_dma_channel |> std.to_string in
      let tx_dma_channel_s = tx_dma_channel |> std.to_string in
      let defines = m%"
          #define KEYBOARD_SPLIT_USART_RCC_APB_CLOCKCMD RCC_%{usart_apb}PeriphClockCmd
          #define KEYBOARD_SPLIT_USART_RCC_APB_PERIPH RCC_%{usart_apb}Periph_USART%{usart_s}
          #define KEYBOARD_SPLIT_RX_RCC_APB_PERIPH RCC_APB2Periph_GPIO%{rx.port}
          #define KEYBOARD_SPLIT_TX_RCC_APB_PERIPH RCC_APB2Periph_GPIO%{tx.port}

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

          #define KEYBOARD_SPLIT_TX_DMA DMA1_Channel%{tx_dma_channel_s}
          #define KEYBOARD_SPLIT_TX_DMA_IRQ_HANDLER DMA1_Channel%{tx_dma_channel_s}_IRQHandler
          #define KEYBOARD_SPLIT_TX_DMA_IRQ_CHANNEL DMA1_Channel%{tx_dma_channel_s}_IRQn
          #define KEYBOARD_SPLIT_TX_TC_FLAG DMA1_IT_TC%{tx_dma_channel_s}
          "%
      in
      {
        keyboard_split = m%"
          #pragma once

          #include "ch32x035_rcc.h"

          %{defines}
    "%,
      }
    else
      {},
}
