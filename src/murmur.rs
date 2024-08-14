const MAGIC: u64 = 0xc6a4a7935bd1e995;
const ROLL: u8 = 47;
const MAGIC_INVERSE: u64 = 0x5f7a0ea7e59b19bd;

pub fn hash<T: AsRef<[u8]>>(key: T) -> u64 {
    hash_(key.as_ref())
}

fn hash_(mut key: &[u8]) -> u64 {
    let seed = 0;
    let mut hash = seed ^ (key.len() as u64).wrapping_mul(MAGIC);

    while key.len() > 7 {
        let split = key.split_at(8);
        let chunk = split.0;
        key = split.1;
        let mut k = u64::from_le_bytes(<[u8; 8]>::try_from(chunk).unwrap());
        k = k.wrapping_mul(MAGIC);
        k ^= k >> ROLL;
        k = k.wrapping_mul(MAGIC);

        hash ^= k;
        hash = hash.wrapping_mul(MAGIC);
    }

    if !key.is_empty() {
        let mut xor = [0_u8; 8];
        let rem = key.len();
        if rem >= 4 {
            xor[0] = key[0];
            xor[1] = key[1];
            xor[2] = key[2];
            xor[3] = key[3];
            if rem >= 6 {
                xor[4] = key[4];
                xor[5] = key[5];
                if rem == 7 {
                    xor[6] = key[6];
                }
            } else if rem == 5 {
                xor[4] = key[4];
            }
        } else if rem >= 2 {
            xor[0] = key[0];
            xor[1] = key[1];
            if rem == 3 {
                xor[2] = key[2];
            }
        } else if rem == 1 {
            xor[0] = key[0];
        }

        hash ^= u64::from_le_bytes(xor);
        hash = hash.wrapping_mul(MAGIC);
    }

    hash ^= hash >> ROLL;
    hash = hash.wrapping_mul(MAGIC);
    hash ^= hash >> ROLL;
    hash
}

pub fn prehash<T: AsRef<[u8]>>(key: T, len: usize) -> u64 {
    prehash_(key.as_ref(), len)
}

fn prehash_(mut key: &[u8], len: usize) -> u64 {
    assert_eq!(key.len() % 8, 0);
    assert!(key.len() <= len);

    let seed = 0;
    let mut hash = seed ^ (len as u64).wrapping_mul(MAGIC);

    while key.len() > 7 {
        let split = key.split_at(8);
        let chunk = split.0;
        key = split.1;
        let mut k = u64::from_le_bytes(<[u8; 8]>::try_from(chunk).unwrap());
        k = k.wrapping_mul(MAGIC);
        k ^= k >> ROLL;
        k = k.wrapping_mul(MAGIC);

        hash ^= k;
        hash = hash.wrapping_mul(MAGIC);
    }

    hash
}

pub fn revhash_trim(mut hash: u64) -> u64 {
    hash ^= hash >> ROLL;
    hash = hash.wrapping_mul(MAGIC_INVERSE);
    hash ^= hash >> ROLL;
    hash = hash.wrapping_mul(MAGIC_INVERSE);
    hash
}
