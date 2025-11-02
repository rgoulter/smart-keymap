add_library(keyboard INTERFACE)

target_sources(keyboard PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split.c
    ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split_rx_dma.c
    ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_split_tx_dma.c
)

target_include_directories(keyboard INTERFACE ${CMAKE_CURRENT_BINARY_DIR})
target_include_directories(keyboard INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

set(CODEGEN_DIR ${CMAKE_CURRENT_BINARY_DIR})
set(NCL_DIR ${CMAKE_CURRENT_SOURCE_DIR}/ncl)
set(NCL_CODEGEN_DIR ${NCL_DIR}/keyboard)

find_program(NICKEL_EXECUTABLE nickel)
if(NOT NICKEL_EXECUTABLE)
    message(FATAL_ERROR "nickel executable not found")
endif()

# List of Nickel files for code generation
set(
    CODEGEN_DEPS_LIST
    "${CMAKE_SOURCE_DIR}/${BOARD}"
    "${NCL_CODEGEN_DIR}/debug.ncl"
    "${NCL_CODEGEN_DIR}/gpio.ncl"
    "${NCL_CODEGEN_DIR}/keyboard.ncl"
    "${NCL_CODEGEN_DIR}/keyboard_led.ncl"
    "${NCL_CODEGEN_DIR}/keyboard_matrix.ncl"
    "${NCL_CODEGEN_DIR}/keyboard_split.ncl"
)

# Set CODEGEN_CMAKELISTS_IDS from the codegen.ncl.
# The `cmakelists_filenames` field is a list of nickel modules which output .cmake files.
# e.g. "debug;keyboard;keyboard_matrix".
execute_process(
    COMMAND
        ${NICKEL_EXECUTABLE} export
            --field=cmakelists_filenames
            --format=raw
            --import-path=${NCL_DIR}/
            ${CODEGEN_DEPS_LIST}
            ${NCL_DIR}/codegen.ncl
            --
            separator="\;"
    OUTPUT_VARIABLE CODEGEN_CMAKELISTS_IDS
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

# Set CODEGEN_INCLUDES_IDS from the codegen.ncl.
# The `includes_filenames` field is a list of nickel modules which output .h files.
# e.g. "debug;keyboard;keyboard_matrix".
execute_process(
    COMMAND
        ${NICKEL_EXECUTABLE} export
            --field=includes_filenames
            --format=raw
            --import-path=${NCL_DIR}/
            ${CODEGEN_DEPS_LIST}
            ${NCL_DIR}/codegen.ncl
            --
            separator="\;"
    OUTPUT_VARIABLE CODEGEN_INCLUDES_IDS
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

# Set CODEGEN_SOURCE_IDS from the codegen.ncl.
# The `source_filenames` field is a list of nickel modules which output .c files.
# e.g. "debug;keyboard;keyboard_matrix".
execute_process(
    COMMAND
        ${NICKEL_EXECUTABLE} export
            --field=source_filenames
            --format=raw
            --import-path=${NCL_DIR}/
            ${CODEGEN_DEPS_LIST}
            ${NCL_DIR}/codegen.ncl
            --
            separator="\;"
    OUTPUT_VARIABLE CODEGEN_SOURCE_IDS
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

# Generate .cmake files at configure time
foreach(id IN LISTS CODEGEN_CMAKELISTS_IDS)
    message(STATUS "Generating ${id}.cmake from Nickel")
    execute_process(
        COMMAND
            ${NICKEL_EXECUTABLE} export
                --field=cmakelists.${id}
                --format=text
                --import-path=${NCL_DIR}/
                --output=${CODEGEN_DIR}/${id}.cmake
                ${CODEGEN_DEPS_LIST}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
endforeach()

# Custom commands to generate .h and .c files at build time
set(GENERATED_FILES "")

foreach(id IN LISTS CODEGEN_INCLUDES_IDS)
    set(header_file ${CODEGEN_DIR}/${id}.h)
    message(STATUS "Scheduling generation of ${id}.h from Nickel")
    list(APPEND GENERATED_FILES ${header_file})
    add_custom_command(
        OUTPUT ${header_file}
        COMMAND
            ${NICKEL_EXECUTABLE} export
                --field=includes.${id}
                --format=text
                --import-path=${NCL_DIR}/
                --output="${header_file}"
                ${CODEGEN_DEPS_LIST}
        COMMENT "Generating ${id}.h from Nickel"
        DEPENDS ${CODEGEN_DEPS_LIST}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
endforeach()

foreach(id IN LISTS CODEGEN_SOURCE_IDS)
    set(source_file ${CODEGEN_DIR}/${id}.c)
    message(STATUS "Scheduling generation of ${id}.c from Nickel")
    list(APPEND GENERATED_FILES ${source_file})
    add_custom_command(
        OUTPUT ${source_file}
        COMMAND
            ${NICKEL_EXECUTABLE} export
                --field=sources.${id}
                --format=text
                --import-path=${NCL_DIR}/
                --output="${source_file}"
                ${CODEGEN_DEPS_LIST}
        COMMENT "Generating ${id}.c from Nickel"
        DEPENDS ${CODEGEN_DEPS_LIST}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
endforeach()

set_source_files_properties(${GENERATED_FILES} PROPERTIES GENERATED TRUE)

target_sources(keyboard PUBLIC ${GENERATED_FILES})

foreach(id IN LISTS CODEGEN_CMAKELISTS_IDS)
    include(${CODEGEN_DIR}/${id}.cmake OPTIONAL)
endforeach()

