use std::fmt::Write;

mod murmur;

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

    let mut hashes = murmur_inverse::HashSlot::new();
    for hash in targets {
        hashes.insert(hash);
    }
    let num_hashes = hashes.len();

    let res = murmur_inverse::bruteforce(hashes);
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
        eprintln!("generated keys for {} of {} hashes in {}.{:03} seconds",
            res.len(),
            num_hashes,
            elapsed / 1000,
            elapsed % 1000);
    }
}
