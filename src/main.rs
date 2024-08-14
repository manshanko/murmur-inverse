use std::process::ExitCode;

mod murmur;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 127;
const ASCII_RANGE: std::ops::Range<u8> = ASCII_START..ASCII_END;

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _bin = args.next();
    let target_hash = args.next().expect("expected 8 byte hex (16 characters)");
    assert_eq!(target_hash.len(), 16);
    let target_hash = u64::from_str_radix(&target_hash, 16).unwrap();

    let trim = murmur::revhash_trim(target_hash);

    let mut res = None;
    'outer_loop: for i0 in ASCII_RANGE {
        for i1 in ASCII_RANGE {
            for i2 in ASCII_RANGE {
                for i3 in ASCII_RANGE {
                    for i4 in ASCII_RANGE {
                        for i5 in ASCII_RANGE {
                            for i6 in ASCII_RANGE {
                                for (i, i7) in ASCII_RANGE.enumerate() {
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

                                    let check = u64::to_ne_bytes(phash ^ trim);
                                    if i == 0 && !(check[0] >= ASCII_START && check[0] < ASCII_END) {
                                        break;
                                    }
                                    if check[7] != 0 {
                                        continue;
                                    }

                                    let check = &check[..7];
                                    let valid = check.iter().all(|&b| b >= ASCII_START && b < ASCII_END);
                                    if valid {
                                        let mut buf = prefix.to_vec();
                                        buf.extend(check);
                                        res = Some(buf);
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

    if let Some(res) = res {
        let hash = murmur::hash(&res);
        assert_eq!(hash, target_hash);
        println!("{:?}", String::from_utf8_lossy(&res));
        ExitCode::SUCCESS
    } else {
        eprintln!("failed to find ascii inverse in range");
        ExitCode::FAILURE
    }
}
