use std::collections::HashMap;
use std::collections::HashSet;

mod murmur;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 127;
const ASCII_RANGE: AsciiIter = AsciiIter(ASCII_START);
const ASCII_HASH_MASK: u64 = 0b1111111110000000100000001000000010000000100000001000000010000000;

struct AsciiIter(u8);

impl AsciiIter {
    fn contains(c: u8) -> bool {
        c >= ASCII_START
            && c < ASCII_END
            && c != b'\\'
    }
}

impl Clone for AsciiIter {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Iterator for AsciiIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == b'\\' {
            self.0 += 1;
        }
        if self.0 < ASCII_END {
            let c = self.0;
            self.0 += 1;
            Some(c)
        } else {
            None
        }
    }
}

pub struct HashSlot(HashMap<u64, Vec<(u64, u64)>>, HashSet<u64>);

impl HashSlot {
    pub fn new() -> Self {
        Self(HashMap::new(), HashSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|(_, list)| list.len()).sum()
    }

    pub fn insert(&mut self, hash: u64) -> bool {
        if self.1.contains(&hash) {
            return false;
        }
        self.1.insert(hash);
        let trim = murmur::revhash_trim(hash);
        let slot = trim & ASCII_HASH_MASK;
        let list = self.0.entry(slot).or_insert(Vec::new());
        list.push((hash, trim));
        true
    }
}

pub struct Key([u8; 16]);

impl Key {
    fn new(prefix: [u8; 8], suffix: &[u8]) -> Self {
        assert!(suffix.len() < 8);
        let mut key = [0; 16];
        key[..8].copy_from_slice(&prefix);
        key[8..8 + suffix.len()].copy_from_slice(suffix);
        Self(key)
    }

    pub fn len(&self) -> usize {
        self.0.iter()
            .enumerate()
            .find_map(|(i, &b)| (b == 0).then_some(i))
            .unwrap()
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0[..self.len()]).unwrap()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..self.len()]
    }
}

pub fn bruteforce(
    mut hashes: HashSlot,
) -> Vec<(u64, Key)> {
    let len = hashes.len();
    let mut res = Vec::with_capacity(len);
    'outer_loop: for i0 in ASCII_RANGE {
        for i1 in ASCII_RANGE {
            for i2 in ASCII_RANGE {
                for i3 in ASCII_RANGE {
                    for i4 in ASCII_RANGE {
                        for i5 in ASCII_RANGE {
                            for i6 in ASCII_RANGE {
                                for i7 in ASCII_RANGE {
                                    let prefix = [
                                        i0,
                                        i1,
                                        i2,
                                        i3,
                                        i4,
                                        i5,
                                        i6,
                                        i7,
                                    ];
                                    let phash = murmur::prehash(&prefix, 15);

                                    let slot = phash & ASCII_HASH_MASK;
                                    if let Some(list) = hashes.0.get_mut(&slot) {
                                        list.retain(|(hash, trim)| {
                                            let check = u64::to_ne_bytes(phash ^ trim);
                                            assert!(check[7] == 0);

                                            let check = &check[..7];
                                            let valid = check.iter().all(|&b| AsciiIter::contains(b));
                                            if valid {
                                                res.push((*hash, Key::new(prefix, check)));
                                            }
                                            !valid
                                        });
                                        if list.is_empty() {
                                            hashes.0.remove(&slot);
                                            if hashes.0.is_empty() {
                                                break 'outer_loop;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

pub fn bruteforce_single(
    hash: u64,
) -> String {
    let mut hashes = HashSlot::new();
    hashes.insert(hash);
    let (check, key) = &bruteforce(hashes)[0];
    assert_eq!(*check, hash);
    key.as_str().to_string()
}

#[cfg(test)]
mod test {
    use std::hash::BuildHasher;
    use std::hash::Hasher;
    use super::*;

    #[test]
    fn hash_bruteforce() {
        let mut hashes = HashSlot::new();
        for _ in 0..50 {
            let random = std::hash::RandomState::new().build_hasher().finish();
            hashes.insert(random);
        }

        let res = bruteforce(hashes);
        for (hash, inverse_key) in res {
            assert_eq!(hash, murmur::hash(inverse_key.as_bytes()));
        }
    }
}
