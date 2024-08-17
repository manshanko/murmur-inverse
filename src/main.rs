use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;

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

struct HashSlot(HashMap<u64, Vec<(u64, u64)>>, HashSet<u64>);

impl HashSlot {
    fn new() -> Self {
        Self(HashMap::new(), HashSet::new())
    }

    fn len(&self) -> usize {
        self.0.iter().map(|(_, list)| list.len()).sum()
    }

    fn insert(&mut self, hash: u64) {
        if self.1.contains(&hash) {
            return;
        }
        self.1.insert(hash);
        let trim = murmur::revhash_trim(hash);
        let slot = trim & ASCII_HASH_MASK;
        let list = self.0.entry(slot).or_insert(Vec::new());
        list.push((hash, trim));
    }
}

struct Key([u8; 16]);

impl Key {
    fn new(prefix: [u8; 8], suffix: &[u8]) -> Self {
        assert!(suffix.len() < 8);
        let mut key = [0; 16];
        key[..8].copy_from_slice(&prefix);
        key[8..8 + suffix.len()].copy_from_slice(suffix);
        Self(key)
    }

    fn len(&self) -> usize {
        self.0.iter()
            .enumerate()
            .find_map(|(i, &b)| (b == 0).then_some(i))
            .unwrap()
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0[..self.len()]).unwrap()
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0[..self.len()]
    }
}

fn main() {
    let mut args = std::env::args();
    let _bin = args.next();
    let target = args.next().expect("expected 8 byte hex (16 characters)");

    let start = std::time::Instant::now();
    let mut targets = Vec::new();
    match u64::from_str_radix(&target, 16) {
        Ok(hash) => targets.push(hash),
        Err(_) => {
            let buffer = std::fs::read_to_string(&target)
                .expect("failed to parse argument as hash or path to file");
            for line in buffer.lines() {
                if let Ok(hash) = u64::from_str_radix(line, 16) {
                    targets.push(hash);
                }
            }
        }
    }

    let mut hashes = HashSlot::new();
    for hash in targets {
        hashes.insert(hash);
    }
    let num_hashes = hashes.len();

    let res = bruteforce(hashes);
    assert_eq!(res.len(), num_hashes);
    if cfg!(debug_assertions) {
        for (hash, inverse_key) in &res {
            assert_eq!(*hash, murmur::hash(inverse_key.as_bytes()));
        }
    }
    let elapsed = start.elapsed().as_millis();

    let mut out = String::new();
    for (hash, inverse_key) in &res {
        let inverse_key = inverse_key.as_str();
        writeln!(&mut out, "{hash:016x} = {inverse_key:?}").unwrap();
    }
    print!("{out}");
    if num_hashes > 1 {
        eprintln!("generated keys for {} of {} hashes in {}ms", res.len(), num_hashes, elapsed);
    }
}

fn bruteforce(
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
