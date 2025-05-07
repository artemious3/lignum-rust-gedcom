project('lignum-rust-gedcom')

cargo = find_program('cargo')
cbindgen = find_program('cbindgen')
cp = find_program('cp')

# I spent here 3 hours and I didn't find the way to 
# integrate Cargo into Meson better than this
# Please dm me if you did

cargo_cmd = [cargo, 'build', '--release', '--lib']

build = run_command(cargo_cmd, check: true)

target_path = join_paths( meson.current_source_dir(), 'target', 'release')

gedcom_lib_target = custom_target( 
    input : join_paths(target_path, 'libgedcom.a'),
    output : ['libgedcom.a'],
    command : [cp, '@INPUT@', '@OUTDIR@'],
)

gedcom_inc = custom_target('Gedcom.h',
    input : ['src/ffi.rs', 'cbindgen.toml'],
    output : ['Gedcom.h'],
    command : [cbindgen, '@INPUT0@', '--config', '@INPUT1@', '--crate', 'gedcom', '--output', '@OUTPUT@'],
)

gedcom_dep = declare_dependency(
    link_with : gedcom_lib_target,
    sources :gedcom_inc,
)


