use std::time::{SystemTime, UNIX_EPOCH};

fn hex_format(out: &mut [u8], bin: &[u8]) {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
    let mut i = 0;
    let mut j = 0;
    while i < bin.len() {
        let b = bin[i];
        out[j] = HEX_CHARS[(b >> 4) as usize];
        out[j + 1] = HEX_CHARS[(b & 0x0f) as usize];
        i += 1;
        j += 2;
    }
}

/// Return a UUIDv7 string.
pub fn create() -> String {
    let start = SystemTime::now();
    let ts = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    let mut buf = [0u8; 16];
    println!("{}", ts);
    buf[0..8].copy_from_slice(&(ts << 16).to_be_bytes());

    let mut rnd = [0u8; 10];
    getrandom::getrandom(&mut rnd).unwrap();

    buf[6..].copy_from_slice(&rnd);
    buf[6] &= 0x0f;
    buf[6] |= 0x07;
    buf[8] &= 0xc0;
    buf[8] |= 0x02;

    let mut out = [0u8; 4 + 32];
    out[8] = b'-';
    out[13] = b'-';
    out[18] = b'-';
    out[23] = b'-';

    hex_format(&mut out[0..], &buf[0..4]);
    hex_format(&mut out[9..], &buf[4..6]);
    hex_format(&mut out[14..], &buf[6..8]);
    hex_format(&mut out[19..], &buf[8..10]);
    hex_format(&mut out[24..], &buf[10..]);

    String::from_utf8_lossy(&out).into_owned()
}
