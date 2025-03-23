add_library(keyboard_codegen INTERFACE)

target_include_directories(keyboard_codegen INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

include(debug.cmake)
include(keyboard.cmake)
include(keyboard_matrix.cmake)
include(keyboard_led.cmake OPTIONAL)
include(keyboard_split.cmake OPTIONAL)
