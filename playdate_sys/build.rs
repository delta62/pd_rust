use std::{env, io, path::PathBuf, process};

fn main() -> io::Result<()> {
    let header_path = "wrapper.h";
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_enum_style(bindgen::EnumVariation::Consts)
        .allowlist_file(".*playdate-sdk.*")
        .use_core()
        .generate_cstr(true)
        .generate()
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    bindings.write_to_file(out_path.join("bindings.rs"))
}
