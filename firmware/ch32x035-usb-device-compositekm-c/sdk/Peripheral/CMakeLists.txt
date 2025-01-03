target_include_directories(sdk INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/inc)

target_sources(sdk INTERFACE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_adc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_dma.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_gpio.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_misc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_rcc.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_usart.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_awu.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_exti.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_i2c.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_opa.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_spi.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_wwdg.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_dbgmcu.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_flash.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_iwdg.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_pwr.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/ch32x035_tim.c
)
