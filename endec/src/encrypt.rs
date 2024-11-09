use aes::cipher::{generic_array::GenericArray, BlockCipher, BlockEncryptMut, KeyIvInit};
use cbc::Encryptor;

use crate::EncryptType;

use anyhow::Result;

type Aes128CbcEnc = Encryptor<aes::Aes128>;
type Aes256CbcEnc = Encryptor<aes::Aes256>;

pub struct Encrypter<'a> {
    key: &'a [u8; 32],
    iv: &'a [u8; 16],
    encrypt_type: EncryptType,
}

impl<'a> Encrypter<'a> {
    pub fn new(encrypt_type: EncryptType, key: &'a [u8; 32], iv: &'a [u8; 16]) -> Self {
        Self { encrypt_type, key, iv }
    }
}

trait EncryptorTrait {
    fn encrypt(&mut self, buffer: &mut [u8]);
}

impl<T: BlockEncryptMut + BlockCipher> EncryptorTrait for Encryptor<T> {
    fn encrypt(&mut self, buffer: &mut [u8]) {
        let generic_array = GenericArray::from_mut_slice(buffer);
        self.encrypt_block_mut(generic_array);
    }
}

impl<'a> Encrypter<'a> {
    fn get_encryptor(&self) -> Box<dyn EncryptorTrait> {
        match self.encrypt_type {
            EncryptType::Aes128Cbc => Box::new(Aes128CbcEnc::new(
                GenericArray::from_slice(&self.key[..16]),
                GenericArray::from_slice(self.iv),
            )),
            EncryptType::Aes256Cbc => Box::new(Aes256CbcEnc::new(
                GenericArray::from_slice(self.key),
                GenericArray::from_slice(self.iv),
            )),
        }
    }

    pub fn encrypt_data(&self, mut data: Vec<u8>) -> Result<Vec<u8>> {
        // Calculate padding needed to make data length a multiple of 16 bytes
        let padding_len = (16 - (data.len() % 16)) % 16;
        
        // Add PKCS7 padding
        data.extend(vec![padding_len as u8; padding_len]);

        let mut encryptor = self.get_encryptor();

        for chunk in data.chunks_mut(16) {
            encryptor.encrypt(chunk);
        }

        Ok(data)
    }
}
