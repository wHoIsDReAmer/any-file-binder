use std::path::Path;
use std::fs::File;
use std::io::Read;

use anyhow::Result;

use endec::constatants::{IV, KEY, SYMBOL_ARRAY};
use endec::encrypt::Encrypter;

#[derive(Debug, Default)]
pub struct Binder {
    pub encryption_type: endec::EncryptType,
    pub stub: Vec<u8>,
    pub input_files: Vec<String>,
}

impl Binder {
    pub fn bind(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.stub.len());

        // Write stub to buffer
        buffer.extend_from_slice(&self.stub);

        // Write symbol array to buffer
        buffer.extend_from_slice(SYMBOL_ARRAY);

        // Write encryption type to buffer
        buffer.push(self.encryption_type as u8);

        for file_name in &self.input_files {
            let mut file = File::open(file_name)?;

            // write file extension size
            let blank_string = String::new();
            let extension = Path::new(file_name).extension().unwrap_or(blank_string.as_ref());

            let extension_size = extension.len() as u8;
            buffer.extend_from_slice(&extension_size.to_le_bytes());

            // write file extension
            buffer.extend_from_slice(extension.to_str().unwrap_or_default().as_bytes());

            let mut buf = Vec::with_capacity(file.metadata()?.len() as usize);
            file.read_to_end(&mut buf)?;

            let encrypter = Encrypter::new(self.encryption_type, KEY, IV);
            let encrypted_file_data = encrypter.encrypt_data(buf)?;

            // write encrypted file data length
            let len = encrypted_file_data.len() as u64;
            buffer.extend_from_slice(&len.to_le_bytes());

            // write encrypted file data
            buffer.extend_from_slice(&encrypted_file_data);
        }

        Ok(buffer)
    }
}
