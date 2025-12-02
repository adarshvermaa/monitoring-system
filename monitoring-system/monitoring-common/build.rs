fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "grpc")]
    {
        prost_build::compile_protos(&["proto/monitoring.proto"], &["proto/"])?;
    }
    Ok(())
}
