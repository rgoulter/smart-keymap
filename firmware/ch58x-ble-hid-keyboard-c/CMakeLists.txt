cmake_minimum_required(VERSION 3.29)

project(ble-hid-keyboard)
enable_language(C ASM)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT DEFINED BOARD)
  set(BOARD "ncl/boards/wabble-60.ncl" CACHE STRING "Board to build for")
endif()

add_executable(HID_Keyboard)
add_subdirectory(APP)
add_subdirectory(Profile)

target_compile_definitions(HID_Keyboard PUBLIC HAL_SLEEP=TRUE)

add_subdirectory(keyboard)
target_link_libraries(HID_Keyboard keyboard)

add_subdirectory(sdk sdk)

target_link_libraries(HID_Keyboard sdk)

add_subdirectory(libsmartkeymap smartkeymap)
target_link_libraries(HID_Keyboard smartkeymap)

target_compile_features(HID_Keyboard PUBLIC c_std_99)

target_compile_options(HID_Keyboard PUBLIC -MMD -MP)

add_custom_command(TARGET HID_Keyboard POST_BUILD
    COMMAND ${CROSS_OBJCOPY} "-Oihex" HID_Keyboard HID_Keyboard.hex
    COMMENT "Generating hex file")
