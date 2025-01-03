add_library(smartkeymap INTERFACE)

target_include_directories(smartkeymap INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(smartkeymap INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/libsmart_keymap.a)
