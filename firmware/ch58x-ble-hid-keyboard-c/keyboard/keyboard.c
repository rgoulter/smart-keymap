#include "keyboard.h"

#include "CH58x_common.h"

#include "CH58xBLE_LIB.h"

#include "hidkbd.h"
#include "keyboard_matrix.h"

void keyboard_init(void) {
  keyboard_await_interrupt(); // Start in low-power interrupt mode
}

void keyboard_start_scanning(void) {
  PFIC_DisableIRQ(GPIO_A_IRQn);
  PFIC_DisableIRQ(GPIO_B_IRQn);
  keyboard_matrix_configure_for_scanning();
}

void keyboard_await_interrupt(void) {
  keyboard_matrix_configure_for_interrupt();

  // Clear any pending GPIO interrupts before re-enabling
  GPIOA_ClearITFlagBit(0xFFFF);
  GPIOB_ClearITFlagBit(0xFFFF);

  PFIC_EnableIRQ(GPIO_A_IRQn);
  PFIC_EnableIRQ(GPIO_B_IRQn);
}

__INTERRUPT
__HIGH_CODE
void GPIOA_IRQHandler(void) {
  // signal main loop to start scanning
  HidEmu_Wakeup();
  GPIOA_ClearITFlagBit(0xFFFF);
}

__INTERRUPT
__HIGH_CODE
void GPIOB_IRQHandler(void) {
  // signal main loop to start scanning
  HidEmu_Wakeup();
  GPIOB_ClearITFlagBit(0xFFFF);
}
