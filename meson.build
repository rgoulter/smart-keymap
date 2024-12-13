project('smart_keymap', ['c'],
  version : '0.1.0',
  default_options : ['warning_level=3'])

# Find Cargo
cargo = find_program('cargo')

build_type = get_option('buildtype')
cargo_build_type = build_type == 'debug' ? 'debug' : 'release'

# Include directory
smart_keymap_inc = include_directories('include')
cc = meson.get_compiler('c')
lib_dir = join_paths(meson.source_root(), 'target', cargo_build_type)
lib_path = join_paths('target', cargo_build_type, 'libsmart_keymap.a')

# Build Rust library as a custom target
rust_lib = cc.find_library('smart_keymap', dirs: lib_dir)

# Unity test framework
unity_inc = include_directories('tests/c_tests/unity')
unity_src = ['tests/c_tests/unity/unity.c']

# Test executable
test_exe = executable('c_smart_keymap_test', 
  ['tests/c_tests/test_keymap.c', unity_src],
  include_directories : [
    smart_keymap_inc, 
    unity_inc
  ],
  dependencies : rust_lib
)

test('Keymap Library Tests', test_exe)
