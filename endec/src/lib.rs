pub mod constatants;
pub mod encrypt;
pub mod decrypt;

mod tests;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum EncryptType {
    #[default]
    Aes128Cbc = 1,
    Aes256Cbc = 2,
}

impl From<u8> for EncryptType {
    fn from(value: u8) -> Self {
        match value {
            1 => EncryptType::Aes128Cbc,
            2 => EncryptType::Aes256Cbc,
            _ => EncryptType::Aes128Cbc,
        }
    }
}