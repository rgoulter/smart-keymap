let C = import "codegen/contracts.ncl" in

{
  contracts = {},

  cmakelists.keyboard = m%"
    target_sources(keyboard INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard.c)
    "%,

  sources.keyboard = m%"
  #include "keyboard_matrix.h"

  void keyboard_init(void) {
      keyboard_matrix_init();
  }

  "%
}
