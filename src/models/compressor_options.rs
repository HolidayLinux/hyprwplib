/// Struct for complete compressed size.
/// Width, Height.
#[derive(Clone, Debug)]
pub struct CompressedSize {
    pub width: f32,
    pub height: f32,
}

/// Options for compressor.
#[derive(Clone, Debug)]
pub struct CompressorOptions {
    pub compressed_size: CompressedSize,
    pub temp_files_directory: String,
    pub save_temp_files: bool,
}

impl CompressorOptions {
    pub fn new(
        compressed_size: CompressedSize,
        temp_files_directory: &str,
        save_temp_files: bool,
    ) -> Self {
        Self {
            compressed_size: compressed_size,
            temp_files_directory: temp_files_directory.to_owned(),
            save_temp_files,
        }
    }
}
