#!/usr/bin/env sh

# Continuously attempts to deploy
#  the ELF file at the given path to the UF2 bootloader.

which elf2uf2-rs >/dev/null 2>&1 || {
    echo "ERROR: 'elf2uf2-rs' not found on PATH."
    exit 1
}

elf_file="$1"

if [ ! "$(elf2uf2-rs --deploy "${elf_file}" 2>&1 | grep "Unable to find mounted pico")" ]; then
    echo "Problem with input:"
    elf2uf2-rs --deploy "${elf_file}"
    exit 1
fi

attempt_deploy () {
    elf2uf2-rs --deploy "${elf_file}" 2>/dev/null
    return $?
}

if ! attempt_deploy; then
    echo "Waiting for UF2 volume..."
    while ! attempt_deploy
    do
        sleep 1
    done
fi
