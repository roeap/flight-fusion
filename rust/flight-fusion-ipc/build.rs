extern crate prost_build;

use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    env::set_var("OUT_DIR", "src");

    prost_build::compile_protos(
        &[
            "../../proto/message.proto",
            "../../proto/actions.proto",
            "../../proto/passport.proto",
            "../../proto/signals.proto",
        ],
        &["../../proto/"],
    )
    .unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .open("src/flight_fusion.ipc.v1alpha1.rs")?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    // append warning that file was auto-generate
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/flight_fusion.ipc.v1alpha1.rs")?;
    file.write_all("// This file was automatically generated through the build.rs script, and should not be edited.\n\n".as_bytes())?;
    file.write_all(buffer.as_bytes())?;

    Ok(())
}
