extern crate prost_build;
use std::env;
use std::io::Result;

fn main() -> Result<()> {
    env::set_var("OUT_DIR", "src");
    prost_build::compile_protos(
        &[
            "../../proto/message.proto",
            "../../proto/actions.proto",
            "../../proto/passport.proto",
        ],
        &["../../proto/"],
    )
    .unwrap();
    Ok(())
}
// use std::{
//     env,
//     fs::OpenOptions,
//     io::{Read, Write},
//     path::Path,
// };
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // override the build location, in order to check in the changes to proto files
//     env::set_var("OUT_DIR", "src");
//
//     // The current working directory can vary depending on how the project is being
//     // built or released so we build an absolute path to the proto file
//     let path = Path::new("../../proto/message.proto");
//     if path.exists() {
//         // avoid rerunning build if the file has not changed
//         // println!("cargo:rerun-if-changed=../../protos/message.proto");
//         tonic_build::configure().build_server(false).compile(
//             &["../../proto/message.proto"],
//             &["../../proto/"],
//         )?;
//         tonic_build::compile_protos(path.to_str().unwrap())?;
//         // read file contents to string
//         let mut file = OpenOptions::new()
//             .read(true)
//             .open("src/arrow.flight.protocol.rs")?;
//         let mut buffer = String::new();
//         file.read_to_string(&mut buffer)?;
//         // append warning that file was auto-generate
//         let mut file = OpenOptions::new()
//             .write(true)
//             .truncate(true)
//             .open("src/arrow.flight.protocol.rs")?;
//         file.write_all("// This file was automatically generated through the build.rs script, and should not be edited.\n\n".as_bytes())?;
//         file.write_all(buffer.as_bytes())?;
//     }
//
//     // As the proto file is checked in, the build should not fail if the file is not found
//     Ok(())
// }
