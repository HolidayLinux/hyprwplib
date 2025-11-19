use std::{error::Error, fs::exists, io::Write};

use crate::{
    engines::checkers,
    models::{compressor_options::CompressorOptions, errors::NotCorrectPathError, types::Mode},
};

pub struct Compressor {
    compressor_options: CompressorOptions,
}

impl Compressor {
    pub fn new(compressor_options: CompressorOptions) -> Self {
        Self { compressor_options }
    }

    pub fn compress_file(&self, mode: Mode::File) -> Result<Vec<u8>, &dyn std::error::Error> {
        if let Mode::File(path_to_file) = mode {
            Ok(self.compress_work(path_to_file)?)
        }

        Err(NotCorrectPathError)
    }

    pub fn compress_dir(&self, mode: Mode::Directory) -> Result<(), &dyn std::error::Error> {
        if let Mode::Directory(file_paths) = mode {
            for file in file_paths {
                if self.compressor_options.save_temp_file {
                    save_file(
                        &self.compressor_options.temp_files_directory,
                        &self.compress_work(file)?,
                    )
                }
            }
            return Ok(());
        }
        Err(NotCorrectPathError)
    }

    fn compress_work(&self, path_to_file: &Path) -> Result<Vec<u8>, &dyn std::error::Error> {
        let image = image::open(path_to_file)?;
        let resized = image.resize(
            self.compressor_options.compressed_size.width,
            self.compressor_options.compressed_size.height,
            image::imageops::FilterType::Triangle,
        );
        resized.into_bytes()?
    }
}

fn save_file(path_to_save: &str, file_bytes: &Vec<u8>) -> Result<(), &dyn std::error::Error> {
    if path_to_save.exists()? {
        let mut file = std::fs::File::create_new(path_to_save)?;
        file.write(&file_bytes)?;
    }
    Ok(())
}
