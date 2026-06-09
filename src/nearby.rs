use crate::MAP;

pub fn find_nearby_postcode(code: &str) -> Option<&'static (f64, f64)> {
    let bytes = code.as_bytes();
    let len = bytes.len();

    if len < 2 || !bytes[len - 1].is_ascii_alphabetic() || !bytes[len - 2].is_ascii_alphabetic() {
        return None;
    }

    let prefix = &code[..len - 2];
    let mut unit = [bytes[len - 2], bytes[len - 1]];

    for _ in 0..(26 * 26) {
        unit[1] = next_letter(unit[1]);
        if unit[1] == b'A' {
            unit[0] = next_letter(unit[0]);
            if unit[0] == b'A' {
                break;
            }
        }

        let candidate = format!("{}{}{}", prefix, unit[0] as char, unit[1] as char);
        if let Some(s) = MAP.get(candidate.as_str()) {
            return Some(s);
        }
    }

    None
}

fn next_letter(b: u8) -> u8 {
    match b {
        b'A'..=b'Y' => b + 1,
        _ => b'A', // wraps Z -> A
    }
}

