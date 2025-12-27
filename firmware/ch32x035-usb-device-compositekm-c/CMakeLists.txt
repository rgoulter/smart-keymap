cmake_minimum_required(VERSION 3.29)

project(usb-device-compositekm)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT DEFINED BOARD)
  set(BOARD "ncl/boards/weact-ch32x-core-board.ncl" CACHE STRING "Board to build for")
endif()

cmake_path(ABSOLUTE_PATH BOARD
  BASE_DIRECTORY "${CMAKE_SOURCE_DIR}"
  OUTPUT_VARIABLE BOARD_FULL_PATH
)

enable_language(C ASM)

add_executable(usb-device-compositekm)
add_subdirectory(app)

add_subdirectory(keyboard)
target_link_libraries(usb-device-compositekm keyboard)

add_subdirectory(sdk sdk)
target_link_libraries(usb-device-compositekm sdk)

add_subdirectory(libsmartkeymap smartkeymap)
target_link_libraries(usb-device-compositekm smartkeymap)

target_compile_features(usb-device-compositekm PUBLIC c_std_99)

target_compile_options(usb-device-compositekm PUBLIC -MMD -MP)

add_custom_command(TARGET usb-device-compositekm POST_BUILD
    COMMAND ${CROSS_OBJCOPY} "-Oihex" usb-device-compositekm usb-device-compositekm.hex
    COMMENT "Generating hex file")

