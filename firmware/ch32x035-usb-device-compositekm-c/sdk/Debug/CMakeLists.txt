target_include_directories(sdk INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
target_sources(sdk INTERFACE
    ${CMAKE_CURRENT_SOURCE_DIR}/debug.c
)
