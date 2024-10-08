use anyhow::Result;
use aes::cipher::{consts::U16, generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockDecryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};

const SYMBOL_ARRAY: &[u8] = &[0x00, 0x00, 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00];

// 특정 symbol array를 스캔해서 그 symbol array 뒤에 있는 데이터를 파싱합니다.
pub fn parse_stub(binary: &[u8]) -> Option<Vec<u8>> {
    let symbol_array_index = binary.windows(SYMBOL_ARRAY.len()).position(|window| window == SYMBOL_ARRAY);

    if let Some(index) = symbol_array_index {
        let data = &binary[index + SYMBOL_ARRAY.len()..];
        return Some(data.to_vec());
    }

    None
}

#[derive(Debug, Clone)]
pub struct EncryptedFileData {
    file_name: String,
    data: Vec<u8>,
}

type Aes128CbcEnc = Encryptor<aes::Aes128>;
type Aes128CbcDec = Decryptor<aes::Aes128>;

type Aes256CbcEnc = Encryptor<aes::Aes256>;
type Aes256CbcDec = Decryptor<aes::Aes256>;

trait DecryptorTrait {
    fn decrypt(&mut self, buffer: &mut [u8]);
}

impl<T: BlockDecryptMut + BlockCipher> DecryptorTrait for Decryptor<T> {
    fn decrypt(&mut self, buffer: &mut [u8]) {
        if buffer.len() != 16 {
            panic!("Buffer length must be 16");
        }

        let mut block = GenericArray::from_mut_slice(buffer);
        self.decrypt_block_mut(&mut block);
    }
}

pub struct DecryptedStub<'a> {
    bytes: &'a [u8],
    key: &'a [u8; 32],
    iv: &'a [u8; 16],
}

impl<'a> DecryptedStub<'a> {
    pub fn new(stub: &'a [u8], key: &'a [u8; 32], iv: &'a [u8; 16]) -> Self {
        Self { bytes: stub, key, iv }
    }

    // fn get_encrypt_type(&self) -> u8 {
    //     self.bytes[0]
    // }

    fn get_decrypter(&self) -> Box<dyn DecryptorTrait> {
        match self.bytes[0] {
            1 => Box::new(Aes128CbcDec::new(
                GenericArray::from_slice(&self.key[..16]),  // AES-128: 16바이트 키
                GenericArray::from_slice(self.iv)
            )),
            2 => Box::new(Aes256CbcDec::new(
                GenericArray::from_slice(self.key),  // AES-256: 32바이트 키
                GenericArray::from_slice(self.iv)
            )),
            _ => panic!("Invalid encrypt type"),
        }
    }

    fn get_file_length(&self) -> u64 {
        u64::from_le_bytes(self.bytes[1..9].try_into().expect("Failed to convert file length"))
    }

    fn get_encrypted_binded_files(&self) -> Vec<EncryptedFileData> {
        let mut current_index = 9;
        let mut encrypted_binded_files = Vec::new();

        for _ in 0..self.get_file_length() {
            let file_size = u64::from_le_bytes(self.bytes[current_index..current_index + 8]
                .try_into()
                .expect("Failed to convert file size"));
            current_index += 8;

            let file_name_size = u64::from_le_bytes(
                self.bytes[current_index..current_index + 8]
                    .try_into()
                    .expect("Failed to convert file name size"),
            );
            current_index += 8;

            let file_name = String::from_utf8(self.bytes[current_index..current_index + file_name_size as usize].to_vec())
                .expect("Failed to convert file name");
            current_index += file_name_size as usize;

            let file_data = self.bytes[current_index..current_index + file_size as usize].to_vec();
            current_index += file_size as usize;

            let encrypted_file_data = EncryptedFileData {
                file_name,
                data: file_data,
            };

            encrypted_binded_files.push(encrypted_file_data);
        }

        encrypted_binded_files
    }

    pub fn decrypt(&self) -> Result<Vec<BindedFile>> {
        // let mut decrypter = Aes256CbcDec::new(self.key.into(), self.iv.into());
        let mut decrypter = self.get_decrypter();
        let encrypted_binde_files = self.get_encrypted_binded_files();

        let mut decrypted_binded_files: Vec<BindedFile> = Vec::new();

        for mut binded_file in encrypted_binde_files {
            let mut decrypted_data = Vec::new();
            for encrypted_chunk in binded_file.data.chunks_mut(16) {
                decrypter.decrypt(encrypted_chunk);
                decrypted_data.extend_from_slice(encrypted_chunk);
            }

            decrypted_binded_files.push(BindedFile {
                file_name: binded_file.file_name,
                data: decrypted_data,
            });
        }

        Ok(decrypted_binded_files)
    }
}

#[derive(Debug, Clone)]
pub struct BindedFile {
    file_name: String,
    data: Vec<u8>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stub() {
        let binary = &[0x00, 0x00, 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x01, 0x02, 0x03];
        let parsed_data = parse_stub(binary).expect("Failed to parse stub");
        assert_eq!(parsed_data, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_decrypt() {
        let key = [0u8; 32];
        let iv = [0u8; 16];
        let encrypted_data = vec![
            0x02, // encrypt type
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // file length
            0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // file size
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // file name size
            b't', b'e', b's', b't', // file name
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34, // encrypted data
        ];

        let decrypted_stub = DecryptedStub::new(&encrypted_data, &key, &iv);
        let decrypted_files = decrypted_stub.decrypt().expect("Failed to decrypt");

        // dbg!(&decrypted_files);
        assert_eq!(decrypted_files.len(), 1);

        assert_eq!(decrypted_files[0].file_name, "test");

        assert_eq!(decrypted_files[0].data.len(), 16);
        assert_eq!(decrypted_files[0].data, vec![210, 113, 35, 3, 70, 181, 43, 125, 180, 194, 149, 171, 226, 219, 227, 116]);
    }
}