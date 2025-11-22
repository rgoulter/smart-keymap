target_sources(HID_Keyboard PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/battservice.c
    ${CMAKE_CURRENT_SOURCE_DIR}/devinfoservice.c
    ${CMAKE_CURRENT_SOURCE_DIR}/hiddev.c
    ${CMAKE_CURRENT_SOURCE_DIR}/hidkbdservice.c
    ${CMAKE_CURRENT_SOURCE_DIR}/scanparamservice.c
)

target_include_directories(HID_Keyboard
    PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)
