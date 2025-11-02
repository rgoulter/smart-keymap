target_sources(usb-device-compositekm PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/ch32x035_it.c
  ${CMAKE_CURRENT_SOURCE_DIR}/ch32x035_usbfs_device.c
  ${CMAKE_CURRENT_SOURCE_DIR}/main.c
  ${CMAKE_CURRENT_SOURCE_DIR}/system_ch32x035.c
  ${CMAKE_CURRENT_SOURCE_DIR}/usb_desc.c
  ${CMAKE_CURRENT_SOURCE_DIR}/usbd_composite_km.c
)

target_include_directories(usb-device-compositekm PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
