use sails_idl_gen::program;
use std::{env, fs::File, path::PathBuf};
use app::PingProgram;

fn main() {
    gwasm_builder::build();

    let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let idl_file_path = manifest_dir_path.join("ping.idl");

    let idl_file = File::create(idl_file_path).unwrap();

    program::generate_idl::<PingProgram>(idl_file).unwrap();
}