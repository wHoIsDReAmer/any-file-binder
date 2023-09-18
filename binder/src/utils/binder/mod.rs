use crate::utils::crypter;

use rand::Rng;
use base64::encode;
use crypter::encrypt;

pub struct Binder<'a> {
    pub output_file_name: String,
    pub seperator: String,

    pub stub: Vec<u8>,

    files: Vec<(String, Vec<u8>)>,
    key_hint: &'a [u8;32],
    fix_sep: String
}

impl Default for Binder<'_> {
    fn default() -> Self {
        Self {
            key_hint: &[239, 39, 152, 9, 150, 12, 250, 189, 213, 50, 40, 58, 108, 102, 200, 138, 222, 213, 47, 3, 107, 4, 47, 80, 169, 41, 212, 121, 139, 175, 214, 29], // if you change it you gotta change stub too
            fix_sep: "♠●☆♠♠●☆♠●☆●♠♠●☆♠●☆●☆☆".into(), // if you change it you gotta change stub too

            seperator: "EIOGHWEIOGHWEFOIWEHG".into(),
            output_file_name: "output".into(),

            stub: Vec::new(),
            files: Vec::new(),
        }
    }
}

// impl<T: AsRef<str>> From<T> for Binder {
//     fn from(value: T) -> Self {
//         Binder {
//             encrypt_key: value,
//                 ..Default::default()
//         }
//     }
// }

impl Binder<'_> {
    pub fn add_file(&mut self, name: String, file: Vec<u8>) {
        self.files.push((name, file));
    }

    /*
        encode like this
        <BASE64> `cuz of read string from stub
            <AES256 key=some>
                data
            </>
        </>
     */
    pub fn encode(&self, content: Vec<u8>, key: &[u8], iv: &[u8]) -> Vec<u8> {
        encode(encrypt(content.as_slice(), &key, &iv).unwrap()).into_bytes()
    }

    /*
        Structure for stub

        <seperator>name<seperator>file

        at last
        <KeyHint>
     */
    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stub = self.stub.clone();

        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);

        let iv: [u8; 16] = [136, 165, 117, 219, 181, 24, 7, 110, 151, 155, 142, 28, 142, 88, 64, 9];
        let key_hint = encode(encrypt(&key, self.key_hint, &iv).unwrap());
        let sep_hint = encode(encrypt(&self.seperator.clone().as_bytes(), self.key_hint, &iv).unwrap());
        let fix_sep = encode(encrypt(&self.fix_sep.clone().as_bytes(), self.key_hint, &iv).unwrap());

        fix_sep.as_bytes().iter().for_each(|l| stub.push(*l));
        key_hint.as_bytes().iter().for_each(|l| stub.push(*l));
        fix_sep.as_bytes().iter().for_each(|l| stub.push(*l));
        sep_hint.as_bytes().iter().for_each(|l| stub.push(*l));
        fix_sep.as_bytes().iter().for_each(|l| stub.push(*l));

        let seperator = self.encode(self.seperator.clone().into(), &key, &iv);
        for f in &self.files {
            let name_ext = self.encode(f.0.clone().into(), &key, &iv);
            let bind = self.encode(f.1.clone().into(), &key, &iv);

            seperator.iter().for_each(|l| stub.push(*l));
            name_ext.iter().for_each(|l| stub.push(*l));
            seperator.iter().for_each(|l| stub.push(*l));
            bind.iter().for_each(|l| stub.push(*l));
        }

        Ok(std::fs::write(&self.output_file_name, stub)?)
    }
}