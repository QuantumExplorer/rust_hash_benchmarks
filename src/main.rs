use crate::hash::{Hash, HashType};

mod hash;

fn main() {
    for size in 3..256 {
        let sha3 = Hash {
            hash_type: HashType::SHA3_256,
            size
        };
        let sha3time = sha3.random_sample(1000);
        let blake3 = Hash {
            hash_type: HashType::BLAKE3_256,
            size
        };
        let blake3time = blake3.random_sample(1000);
        println!("for size {} sha3 took {} and blake3 took {}", size, sha3time, blake3time)
    }

}
