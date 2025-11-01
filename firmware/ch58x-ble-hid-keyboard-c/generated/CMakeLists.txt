add_library(keyboard INTERFACE)

target_include_directories(keyboard INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

include(keyboard.cmake)
include(keyboard_matrix.cmake)
