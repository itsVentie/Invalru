fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["../../idl/protobuf/analysis.proto"],
            &["../../idl/protobuf"],
        )?;
    Ok(())
}