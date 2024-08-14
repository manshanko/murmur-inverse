use std::process::ExitCode;

mod murmur;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 127;
const ASCII_RANGE: AsciiIter = AsciiIter(ASCII_START);

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

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _bin = args.next();
    let target_hash = args.next().expect("expected 8 byte hex (16 characters)");
    assert_eq!(target_hash.len(), 16);
    let target_hash = u64::from_str_radix(&target_hash, 16).unwrap();

    let trim = murmur::revhash_trim(target_hash);

    if let Some(res) = bruteforce(trim) {
        let hash = murmur::hash(&res);
        assert_eq!(hash, target_hash);
        println!("{:?}", res);
        ExitCode::SUCCESS
    } else {
        eprintln!("failed to find ascii inverse in range");
        ExitCode::FAILURE
    }
}

fn bruteforce(
    trim_hash: u64
) -> Option<String> {
    let mut res = None;
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

                                    let check = u64::to_ne_bytes(phash ^ trim_hash);
                                    if i7 == ASCII_START && !AsciiIter::contains(check[0]) {
                                        break;
                                    }
                                    if check[7] != 0 {
                                        continue;
                                    }

                                    let check = &check[..7];
                                    let valid = check.iter().all(|&b| AsciiIter::contains(b));
                                    if valid {
                                        let mut buf = prefix.to_vec();
                                        buf.extend(check);
                                        res = Some(String::from_utf8(buf).unwrap());
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
    res
}
