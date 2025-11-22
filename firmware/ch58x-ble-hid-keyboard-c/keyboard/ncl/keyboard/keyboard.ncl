let C = import "keyboard/contracts.ncl" in

{
  contracts = {},

  sources.keyboard = m%"
  #include "keyboard_matrix.h"

  void keyboard_init(void) {
      keyboard_matrix_init();
  }

  "%
}
