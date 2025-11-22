target_sources(HID_Keyboard PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/hidkbd.c
    ${CMAKE_CURRENT_SOURCE_DIR}/hidkbd_main.c
)

target_include_directories(HID_Keyboard
    PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)
