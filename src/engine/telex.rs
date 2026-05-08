use super::tone::{detect_tone, is_vowel_or_marked, strip_tone, tone_mark};
use super::Tone;

const TELEX_TONE_KEYS: [char; 6] = ['s', 'f', 'r', 'x', 'j', 'z'];

pub(crate) fn transform_from_raw(raw: &[char]) -> Vec<char> {
    let mut chars = raw.to_vec();
    let tone = parse_telex_tone(&mut chars);
    apply_telex_shapes(&mut chars);
    super::tone::apply_tone(&mut chars, tone);
    promote_uo_with_coda(&mut chars);
    chars
}

pub(crate) fn parse_telex_tone(chars: &mut Vec<char>) -> Tone {
    for idx in (0..chars.len()).rev() {
        let tone = match chars[idx].to_ascii_lowercase() {
            's' => Tone::Acute,
            'f' => Tone::Grave,
            'r' => Tone::Hook,
            'x' => Tone::Tilde,
            'j' => Tone::Dot,
            _ => continue,
        };
        let suffix = &chars[idx + 1..];
        let suffix_is_shape_tail = suffix.iter().all(|ch| matches!(ch.to_ascii_lowercase(), 'w' | 'a' | 'e' | 'o' | 'd'));
        if !suffix.is_empty() && !suffix_is_shape_tail {
            continue;
        }
        if suffix.is_empty() && vowel_cluster_count(&chars[..idx]) != 1 {
            continue;
        }
        if chars.iter().take(idx).any(|ch| is_vowel_or_marked(*ch)) {
            chars.remove(idx);
            return tone;
        }
    }
    Tone::None
}

pub(crate) fn apply_telex_shapes(chars: &mut Vec<char>) {
    loop {
        let mut changed = false;
        for idx in (0..chars.len()).rev() {
            if apply_telex_marker_at(chars, idx) {
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }
}

pub(crate) fn promote_uo_with_coda(chars: &mut [char]) {
    if chars.len() < 3 {
        return;
    }
    for i in 0..(chars.len() - 2) {
        if is_u_or_uw(chars[i]) && is_o_horn_or_marked(chars[i + 1]) && is_coda_consonant(chars[i + 2]) {
            chars[i] = if chars[i].is_uppercase() { 'Ư' } else { 'ư' };
        }
    }
}

#[allow(dead_code)]
pub(crate) fn is_telex_tone_key(ch: char) -> bool {
    TELEX_TONE_KEYS.contains(&ch.to_ascii_lowercase())
}

fn apply_telex_marker_at(chars: &mut Vec<char>, idx: usize) -> bool {
    match chars[idx].to_ascii_lowercase() {
        'd' => {
            if idx > 0 && matches!(chars[idx - 1], 'd' | 'D') {
                chars[idx - 1] = if chars[idx - 1].is_uppercase() { 'Đ' } else { 'đ' };
                chars.remove(idx);
                return true;
            }
            false
        }
        'a' => replace_shape_before(chars, idx, 'a', 'â', 'Â'),
        'e' => replace_shape_before(chars, idx, 'e', 'ê', 'Ê'),
        'o' => replace_shape_before(chars, idx, 'o', 'ô', 'Ô'),
        'w' => apply_telex_w_marker_at(chars, idx),
        _ => false,
    }
}

fn replace_shape_before(
    chars: &mut Vec<char>,
    marker_idx: usize,
    target_lower: char,
    replacement_lower: char,
    replacement_upper: char,
) -> bool {
    for i in (0..marker_idx).rev() {
        if strip_tone(chars[i]).to_ascii_lowercase() == target_lower {
            let tone = detect_tone(chars[i]);
            let replacement_base = if chars[i].is_uppercase() {
                replacement_upper
            } else {
                replacement_lower
            };
            chars[i] = tone_mark(replacement_base, tone);
            chars.remove(marker_idx);
            return true;
        }
    }
    false
}

fn apply_telex_w_marker_at(chars: &mut Vec<char>, marker_idx: usize) -> bool {
    if marker_idx == 0 {
        return false;
    }

    for i in (1..marker_idx).rev() {
        let pair = (strip_tone(chars[i - 1]).to_ascii_lowercase(), strip_tone(chars[i]).to_ascii_lowercase());
        if pair == ('u', 'o') {
            let tone = detect_tone(chars[i]);
            let base = if chars[i].is_uppercase() { 'Ơ' } else { 'ơ' };
            chars[i] = tone_mark(base, tone);
            chars.remove(marker_idx);
            return true;
        }
    }

    for i in (0..marker_idx).rev() {
        if is_u_or_uw(chars[i]) {
            let tone = detect_tone(chars[i]);
            let base = if chars[i].is_uppercase() { 'Ư' } else { 'ư' };
            chars[i] = tone_mark(base, tone);
            chars.remove(marker_idx);
            return true;
        }
    }

    for i in (0..marker_idx).rev() {
        if strip_tone(chars[i]).to_ascii_lowercase() == 'o' {
            let tone = detect_tone(chars[i]);
            let base = if chars[i].is_uppercase() { 'Ơ' } else { 'ơ' };
            chars[i] = tone_mark(base, tone);
            chars.remove(marker_idx);
            return true;
        }
    }

    for i in (0..marker_idx).rev() {
        if strip_tone(chars[i]).to_ascii_lowercase() == 'a' {
            let tone = detect_tone(chars[i]);
            let base = if chars[i].is_uppercase() { 'Ă' } else { 'ă' };
            chars[i] = tone_mark(base, tone);
            chars.remove(marker_idx);
            return true;
        }
    }

    false
}

fn is_u_or_uw(ch: char) -> bool {
    matches!(strip_tone(ch), 'u' | 'ư' | 'U' | 'Ư')
}

fn is_o_horn_or_marked(ch: char) -> bool {
    matches!(strip_tone(ch), 'ơ' | 'Ơ')
}

fn is_coda_consonant(ch: char) -> bool {
    ch.is_alphabetic() && !is_vowel_or_marked(ch)
}

fn vowel_cluster_count(chars: &[char]) -> usize {
    let mut clusters = 0usize;
    let mut in_cluster = false;
    for &ch in chars {
        if is_vowel_or_marked(ch) {
            if !in_cluster {
                clusters += 1;
                in_cluster = true;
            }
        } else {
            in_cluster = false;
        }
    }
    clusters
}
