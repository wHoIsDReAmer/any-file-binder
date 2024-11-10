use aes::cipher::{generic_array::GenericArray, BlockCipher, BlockDecryptMut, KeyIvInit};
use cbc::Decryptor;
use anyhow::Result;

use crate::EncryptType;

type Aes128CbcDec = Decryptor<aes::Aes128>;
type Aes256CbcDec = Decryptor<aes::Aes256>;

pub struct Decrypter<'a> {
    key: &'a [u8; 32],
    iv: &'a [u8; 16],
    encrypt_type: EncryptType,
}

impl<'a> Decrypter<'a> {
    pub fn new(encrypt_type: EncryptType, key: &'a [u8; 32], iv: &'a [u8; 16]) -> Self {
        Self { encrypt_type, key, iv }
    }
}

trait DecryptorTrait {
    fn decrypt(&mut self, buffer: &mut [u8]);
}

impl<T: BlockDecryptMut + BlockCipher> DecryptorTrait for Decryptor<T> {
    fn decrypt(&mut self, buffer: &mut [u8]) {
        let generic_array = GenericArray::from_mut_slice(buffer);
        self.decrypt_block_mut(generic_array);
    }
}

impl<'a> Decrypter<'a> {
    fn get_decryptor(&self) -> Box<dyn DecryptorTrait> {
        match self.encrypt_type {
            EncryptType::Aes128Cbc => Box::new(Aes128CbcDec::new(
                GenericArray::from_slice(&self.key[..16]),
                GenericArray::from_slice(self.iv),
            )),
            EncryptType::Aes256Cbc => Box::new(Aes256CbcDec::new(
                GenericArray::from_slice(self.key),
                GenericArray::from_slice(self.iv),
            )),
        }
    }

    pub fn decrypt_data(&self, mut data: Vec<u8>) -> Result<Vec<u8>> {
        let mut decryptor = self.get_decryptor();

        for chunk in data.chunks_mut(16) {
            decryptor.decrypt(chunk);
        }

        // Remove PKCS7 padding
        if let Some(&padding_len) = data.last() {
            if padding_len as usize <= 16 {
                data.truncate(data.len() - padding_len as usize);
            }
        }

        Ok(data)
    }
}