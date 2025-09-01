use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut current_path = PathBuf::from(manifest_dir);

    // Traverse up from the macro's manifest directory to find the workspace root
    // which we identify by the presence of the `ncl` directory.
    while !current_path.join("ncl").is_dir() {
        if !current_path.pop() {
            panic!("Could not find workspace root containing 'ncl' directory");
        }
    }

    let ncl_path = current_path.join("ncl");
    println!(
        "cargo:rustc-env=SMART_KEYMAP_NCL_PATH={}",
        ncl_path.to_str().unwrap()
    );

    // Rerun build script if the environment variable it depends on changes.
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
}
