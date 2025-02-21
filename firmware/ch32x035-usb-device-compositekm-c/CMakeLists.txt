cmake_minimum_required(VERSION 3.29)

project(usb-device-compositekm)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(usb-device-compositekm
  generated/matrix.c
  User/ch32x035_it.c
  User/ch32x035_usbfs_device.c
  User/main.c
  User/system_ch32x035.c
  User/usb_desc.c
  User/usbd_composite_km.c
)
target_include_directories(usb-device-compositekm PUBLIC User)



enable_language(C ASM)

add_subdirectory(sdk sdk)

target_link_libraries(usb-device-compositekm sdk)

add_subdirectory(libsmartkeymap smartkeymap)
target_link_libraries(usb-device-compositekm smartkeymap)

target_compile_definitions(usb-device-compositekm PUBLIC DEBUG=2)

target_compile_features(usb-device-compositekm PUBLIC c_std_99)

target_compile_options(usb-device-compositekm PUBLIC -MMD -MP)

add_custom_command(TARGET usb-device-compositekm POST_BUILD
    COMMAND ${CROSS_OBJCOPY} "-Oihex" usb-device-compositekm usb-device-compositekm.hex
    COMMENT "Generating hex file")

