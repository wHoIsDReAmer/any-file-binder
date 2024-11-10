#[cfg(test)]
mod integration {
    use crate::{decrypt::Decrypter, encrypt::Encrypter, EncryptType};

    const KEY: [u8; 32] = [0u8; 32];
    const IV: [u8; 16] = [0u8; 16];

    #[test]
    fn test_encrypt_decrypt_aes128cbc() {
        const DATA: &[u8] = b"Hello world!";
        println!("{:?}", DATA);

        let encryption_type = EncryptType::Aes128Cbc;
        // Encrypting
        let encrypter = Encrypter::new(encryption_type, &KEY, &IV);
        let encrypted_data = encrypter.encrypt_data(DATA.to_vec()).expect("Failed to encrypt");
        println!("{:?}", encrypted_data);
        println!("Padding Size: {}", encrypted_data.len() - DATA.len());

        // Decrypting
        let decrypter = Decrypter::new(encryption_type, &KEY, &IV);
        let decrypted_data = decrypter.decrypt_data(encrypted_data).expect("Failed to decrypt");

        println!("{:?}", decrypted_data);
    
        assert_eq!(DATA, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_aes256cbc() {
        const DATA: &[u8] = b"Hello world!";
        println!("{:?}", DATA);

        let encryption_type = EncryptType::Aes256Cbc;
        // Encrypting
        let encrypter = Encrypter::new(encryption_type, &KEY, &IV);
        let encrypted_data = encrypter.encrypt_data(DATA.to_vec()).expect("Failed to encrypt");
        println!("{:?}", encrypted_data);
        println!("Padding Size: {}", encrypted_data.len() - DATA.len());

        // Decrypting
        let decrypter = Decrypter::new(encryption_type, &KEY, &IV);
        let decrypted_data = decrypter.decrypt_data(encrypted_data).expect("Failed to decrypt");

        println!("{:?}", decrypted_data);
    
        assert_eq!(DATA, decrypted_data);
    }
}