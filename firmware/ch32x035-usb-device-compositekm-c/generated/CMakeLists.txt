add_library(keyboard_codegen INTERFACE)

target_include_directories(keyboard_codegen INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
target_sources(keyboard_codegen INTERFACE
    ${CMAKE_CURRENT_SOURCE_DIR}/keyboard.c
)

