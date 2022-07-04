use std::time::SystemTime;
use sha3::{Digest, Sha3_256};
use blake3::Hash as Blake3Hash;
use rand::Rng;

pub enum HashType {
    SHA3_256,
    BLAKE3_256
}

impl HashType {
    fn hash(&self, bytes: Vec<u8>) -> Vec<u8> {
        match self {
            HashType::SHA3_256 => {
                let mut hasher = Sha3_256::default();
                hasher.update(bytes);
                hasher.finalize().to_vec()
            }
            HashType::BLAKE3_256 => {
                blake3::hash(bytes.as_slice()).as_bytes().to_vec()
            }
        }
    }
}

pub struct Hash {
    pub(crate) hash_type : HashType,
    pub(crate) size: u16,
}

impl Hash {
    pub fn random_sample(&self, count: usize) -> f64 {
        let mut vec = vec![];
        for _i in 0..count {
            let random_bytes: Vec<u8> = (0..self.size).map(|_| { rand::random::<u8>() }).collect();
            vec.push(random_bytes);
        }
        let start = SystemTime::now();
        for bytes in vec.into_iter() {
            let _result = self.hash_type.hash(bytes);
        }
        let end_time = SystemTime::now();
        end_time.duration_since(start).unwrap().as_secs_f64()
    }
}