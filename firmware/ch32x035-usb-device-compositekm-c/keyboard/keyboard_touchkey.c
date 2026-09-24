#include "keyboard_touchkey.h"

#include "ch32x035_adc.h"
#include "ch32x035_gpio.h"
#include "ch32x035_rcc.h"

#include "keyboard_gpio.h"

void keyboard_touchkey_configure_drive(keyboard_gpio_t gpio_source) {
  keyboard_gpio_configure_output(gpio_source);
  keyboard_gpio_reset(gpio_source);
}

void keyboard_touchkey_configure_sense(keyboard_gpio_t gpio_source) {
  ch32x_gpio_t gpio = to_ch32x_gpio(gpio_source);
  GPIO_InitTypeDef gpio_init_value = {
      .GPIO_Pin = gpio.pin,
      .GPIO_Mode = GPIO_Mode_AIN,
  };
  GPIO_Init(gpio.port, &gpio_init_value);
}

void keyboard_touchkey_init(void) {
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_ADC1, ENABLE);

  ADC_CLKConfig(ADC1, ADC_CLK_Div6);

  ADC_InitTypeDef adc_init_value = {
      .ADC_Mode = ADC_Mode_Independent,
      .ADC_ScanConvMode = DISABLE,
      .ADC_ContinuousConvMode = DISABLE,
      .ADC_ExternalTrigConv = ADC_ExternalTrigConv_None,
      .ADC_DataAlign = ADC_DataAlign_Right,
      .ADC_NbrOfChannel = 1,
  };
  ADC_Init(ADC1, &adc_init_value);

  ADC_Cmd(ADC1, ENABLE);

  TKey1->CTLR1 |= ADC_TKENABLE;
}

uint16_t keyboard_touchkey_read(uint8_t ch) {
  // TODO: allow configuration
  ADC_RegularChannelConfig(ADC1, ch, 1, ADC_SampleTime_11Cycles);
  TKey1->IDATAR1 = 0xFF; // Charging Time
  TKey1->RDATAR = 0xFF;  // Discharging Time; start read
  while (!ADC_GetFlagStatus(ADC1, ADC_FLAG_EOC))
    ;

  return (uint16_t)TKey1->RDATAR;
}
