use std::io::Result;
fn main() -> Result<()> {
    let mut prost_config = prost_build::Config::new();
    prost_config.file_descriptor_set_path("test_protobuf.desc");
    prost_config.compile_protos(&["src/protos/test_protobuf.proto"], &["src/protos"])?;
    Ok(())
}
