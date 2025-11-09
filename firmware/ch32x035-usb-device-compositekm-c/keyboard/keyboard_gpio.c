#include "keyboard_gpio.h"

#include "ch32x035_gpio.h"

GPIO_TypeDef *const GPIO_PORTS[] = {GPIOA, GPIOB, GPIOC};

typedef struct {
  GPIO_TypeDef *port;
  uint32_t pin;
} ch32x_gpio_t;

ch32x_gpio_t to_ch32x_gpio(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t result = {
      .port = GPIO_PORTS[gpio_source.port],
      .pin = (1 << gpio_source.pin),
  };
  return result;
}

void keyboard_gpio_configure_ipu(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  GPIO_InitTypeDef gpio_init_value = {
      .GPIO_Pin = gpio.pin,
      .GPIO_Mode = GPIO_Mode_IPU,
      .GPIO_Speed = GPIO_Speed_50MHz,
  };
  GPIO_Init(gpio.port, &gpio_init_value);
}

void keyboard_gpio_configure_output(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  GPIO_InitTypeDef gpio_init_value = {
      .GPIO_Pin = gpio.pin,
      .GPIO_Mode = GPIO_Mode_Out_PP,
      .GPIO_Speed = GPIO_Speed_50MHz,
  };
  GPIO_Init(gpio.port, &gpio_init_value);
}

void keyboard_gpio_set(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  GPIO_SetBits(gpio.port, gpio.pin);
}

void keyboard_gpio_reset(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  GPIO_ResetBits(gpio.port, gpio.pin);
}

bool keyboard_gpio_is_set(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  return GPIO_ReadInputDataBit(gpio.port, gpio.pin) == (uint8_t)Bit_SET;
}

bool keyboard_gpio_is_reset(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  return GPIO_ReadInputDataBit(gpio.port, gpio.pin) == (uint8_t)Bit_RESET;
}
