use crate::models::compressor_options::CompressorOptions;

pub struct Compressor {
    compressor_options: CompressorOptions,
}

impl Compressor {
    pub fn new(compressor_options: CompressorOptions) -> Self {
        Self { compressor_options }
    }

    pub fn compress_from_file(path: &str) -> Result<(), &dyn std::error::Error> {
        Ok(())
    }

    pub fn compress_from_dir(path: &str) -> Result<(), &dyn std::error::Error> {
        Ok(())
    }
}
