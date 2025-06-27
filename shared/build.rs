use std::io::Result;
extern crate prost_build;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    let _ = config.compile_protos(&["src/proto/protocol-1.proto"], &["src/"]).unwrap();

    Ok(())
}