add_library(keyboard INTERFACE)

target_include_directories(keyboard INTERFACE ${CMAKE_CURRENT_BINARY_DIR})
target_include_directories(keyboard INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

target_sources(keyboard PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/keyboard.c
  ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_gpio.c
  ${CMAKE_CURRENT_SOURCE_DIR}/keyboard_matrix.c
  ${CMAKE_CURRENT_SOURCE_DIR}/matrix/col_to_row.c
)

set(CODEGEN_DIR ${CMAKE_CURRENT_BINARY_DIR})
set(NCL_DIR ${CMAKE_CURRENT_SOURCE_DIR}/ncl)
set(NCL_CODEGEN_DIR ${NCL_DIR}/keyboard)

find_program(NICKEL_EXECUTABLE nickel)
if(NOT NICKEL_EXECUTABLE)
    message(FATAL_ERROR "nickel executable not found")
endif()

if(NOT DEFINED BOARD_FULL_PATH)
    message(FATAL_ERROR "BOARD_FULL_PATH is not set (parent CMakeLists should absolute-path BOARD)")
endif()
if(NOT EXISTS "${BOARD_FULL_PATH}")
    message(FATAL_ERROR "Board NCL not found: ${BOARD_FULL_PATH}")
endif()

# Inputs passed to `nickel export` (merged records).
set(
    CODEGEN_NICKEL_INPUTS
    "${BOARD_FULL_PATH}"
    "${NCL_CODEGEN_DIR}/gpio.ncl"
    "${NCL_CODEGEN_DIR}/keyboard_matrix.ncl"
    "${NCL_DIR}/codegen.ncl"
)

# Rebuild deps: export inputs plus modules only reached via import.
set(
    CODEGEN_DEPS_LIST
    ${CODEGEN_NICKEL_INPUTS}
    "${NCL_CODEGEN_DIR}/contracts.ncl"
    "${NCL_CODEGEN_DIR}/keyboard_gpio.ncl"
)

# Fail configure if Nickel exits non-zero (was ignored → empty codegen ids).
# Keep separator= here; passing it via ARGN re-splits on `;`.
function(nickel_export_filename_list out_var field)
    execute_process(
        COMMAND
            ${NICKEL_EXECUTABLE} export
                --field=${field}
                --format=raw
                --import-path=${NCL_DIR}/
                ${CODEGEN_NICKEL_INPUTS}
                --
                separator="\;"
        RESULT_VARIABLE _nickel_rc
        OUTPUT_VARIABLE _nickel_out
        ERROR_VARIABLE _nickel_err
        OUTPUT_STRIP_TRAILING_WHITESPACE
        ERROR_STRIP_TRAILING_WHITESPACE
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
    if(NOT _nickel_rc EQUAL 0)
        message(FATAL_ERROR
            "Nickel export failed (exit ${_nickel_rc}) while setting ${out_var} (${field}).\n"
            "Board: ${BOARD_FULL_PATH}\n"
            "${_nickel_err}"
        )
    endif()
    set(${out_var} "${_nickel_out}" PARENT_SCOPE)
endfunction()

# e.g. "debug;keyboard;keyboard_matrix"
nickel_export_filename_list(CODEGEN_CMAKELISTS_IDS cmakelists_filenames)
nickel_export_filename_list(CODEGEN_INCLUDES_IDS includes_filenames)
nickel_export_filename_list(CODEGEN_SOURCE_IDS source_filenames)

if(CODEGEN_CMAKELISTS_IDS STREQUAL "")
    message(FATAL_ERROR
        "Nickel returned empty cmakelists_filenames for board ${BOARD_FULL_PATH}.\n"
        "Expected at least keyboard_matrix (matrix compile definitions)."
    )
endif()

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
                ${CODEGEN_NICKEL_INPUTS}
        RESULT_VARIABLE _nickel_rc
        ERROR_VARIABLE _nickel_err
        ERROR_STRIP_TRAILING_WHITESPACE
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
    if(NOT _nickel_rc EQUAL 0)
        message(FATAL_ERROR
            "Nickel export failed (exit ${_nickel_rc}) while generating ${id}.cmake.\n"
            "Board: ${BOARD_FULL_PATH}\n"
            "${_nickel_err}"
        )
    endif()
endforeach()

# Custom commands to generate .h and .c files at build time
set(GENERATED_FILES "")

foreach(id IN LISTS CODEGEN_INCLUDES_IDS)
    set(header_file ${CODEGEN_DIR}/generated/${id}.h)
    file(MAKE_DIRECTORY ${CODEGEN_DIR}/generated)
    message(STATUS "Scheduling generation of ${id}.h from Nickel")
    list(APPEND GENERATED_FILES ${header_file})
    add_custom_command(
        OUTPUT ${header_file}
        COMMAND
            ${NICKEL_EXECUTABLE} export
                --field=includes.${id}
                --format=text
                --import-path=${NCL_DIR}/
                --output=${header_file}
                ${CODEGEN_NICKEL_INPUTS}
        COMMENT "Generating ${id}.h from Nickel"
        DEPENDS ${CODEGEN_DEPS_LIST}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        VERBATIM
    )
endforeach()

foreach(id IN LISTS CODEGEN_SOURCE_IDS)
    set(source_file ${CODEGEN_DIR}/generated/${id}.c)
    file(MAKE_DIRECTORY ${CODEGEN_DIR}/generated)
    message(STATUS "Scheduling generation of ${id}.c from Nickel")
    list(APPEND GENERATED_FILES ${source_file})
    add_custom_command(
        OUTPUT ${source_file}
        COMMAND
            ${NICKEL_EXECUTABLE} export
                --field=sources.${id}
                --format=text
                --import-path=${NCL_DIR}/
                --output=${source_file}
                ${CODEGEN_NICKEL_INPUTS}
        COMMENT "Generating ${id}.c from Nickel"
        DEPENDS ${CODEGEN_DEPS_LIST}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        VERBATIM
    )
endforeach()

set_source_files_properties(${GENERATED_FILES} PROPERTIES GENERATED TRUE)

target_sources(keyboard PUBLIC ${GENERATED_FILES})

foreach(id IN LISTS CODEGEN_CMAKELISTS_IDS)
    include(${CODEGEN_DIR}/${id}.cmake)
endforeach()
