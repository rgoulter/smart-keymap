cmake_minimum_required(VERSION 3.29)

project(ble-hid-keyboard)
enable_language(C ASM)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(HID_Keyboard
    APP/hidkbd.c
    APP/hidkbd_main.c
    generated/keyboard.c
    Profile/battservice.c
    Profile/devinfoservice.c
    Profile/hiddev.c
    Profile/hidkbdservice.c
    Profile/scanparamservice.c
)

target_include_directories(HID_Keyboard
    PUBLIC
    APP/include
    Profile/include
)

add_subdirectory(sdk sdk)

target_link_libraries(HID_Keyboard sdk)

add_subdirectory(libsmartkeymap smartkeymap)
target_link_libraries(HID_Keyboard smartkeymap)

target_compile_features(HID_Keyboard PUBLIC c_std_99)

target_compile_options(HID_Keyboard PUBLIC -MMD -MP)

add_custom_command(TARGET HID_Keyboard POST_BUILD
    COMMAND ${CROSS_OBJCOPY} "-Oihex" HID_Keyboard HID_Keyboard.hex
    COMMENT "Generating hex file")
