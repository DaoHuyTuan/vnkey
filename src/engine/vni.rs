use super::tone::{is_vowel_or_marked, strip_tone};
use super::Tone;

pub(crate) fn transform_from_raw(raw: &[char]) -> Vec<char> {
    let mut chars = raw.to_vec();
    apply_vni_shapes(&mut chars);
    let tone = parse_vni_tone(&mut chars);
    super::tone::apply_tone(&mut chars, tone);
    promote_uo_with_coda(&mut chars);
    chars
}

pub(crate) fn parse_vni_tone(chars: &mut Vec<char>) -> Tone {
    for idx in (0..chars.len()).rev() {
        let tone = match chars[idx] {
            '1' => Tone::Acute,
            '2' => Tone::Grave,
            '3' => Tone::Hook,
            '4' => Tone::Tilde,
            '5' => Tone::Dot,
            '0' => Tone::None,
            _ => continue,
        };
        if chars.iter().take(idx).any(|ch| is_vowel_or_marked(*ch)) {
            chars.remove(idx);
            return tone;
        }
    }
    Tone::None
}

pub(crate) fn apply_vni_shapes(chars: &mut Vec<char>) {
    replace_adjacent(chars, 'd', '9', 'đ');
    replace_adjacent(chars, 'D', '9', 'Đ');

    replace_last_marker(chars, 'a', '6', 'â');
    replace_last_marker(chars, 'A', '6', 'Â');
    replace_last_marker(chars, 'e', '6', 'ê');
    replace_last_marker(chars, 'E', '6', 'Ê');
    replace_last_marker(chars, 'o', '6', 'ô');
    replace_last_marker(chars, 'O', '6', 'Ô');

    replace_last_marker(chars, 'a', '8', 'ă');
    replace_last_marker(chars, 'A', '8', 'Ă');

    replace_last_marker(chars, 'o', '7', 'ơ');
    replace_last_marker(chars, 'O', '7', 'Ơ');
    replace_last_marker(chars, 'u', '7', 'ư');
    replace_last_marker(chars, 'U', '7', 'Ư');
}

fn replace_adjacent(chars: &mut Vec<char>, target: char, marker: char, replacement: char) {
    if chars.len() < 2 {
        return;
    }
    for i in (1..chars.len()).rev() {
        if chars[i] == marker && chars[i - 1] == target {
            chars[i - 1] = replacement;
            chars.remove(i);
            break;
        }
    }
}

fn replace_last_marker(chars: &mut Vec<char>, target: char, marker: char, replacement: char) {
    if chars.len() < 2 || chars.last().copied() != Some(marker) {
        return;
    }
    for i in (0..chars.len() - 1).rev() {
        if chars[i] == target {
            chars[i] = replacement;
            chars.pop();
            break;
        }
    }
}

fn promote_uo_with_coda(chars: &mut [char]) {
    if chars.len() < 3 {
        return;
    }
    for i in 0..(chars.len() - 2) {
        if is_u_or_uw(chars[i]) && is_o_horn_or_marked(chars[i + 1]) && is_coda_consonant(chars[i + 2]) {
            chars[i] = if chars[i].is_uppercase() { 'Ư' } else { 'ư' };
        }
    }
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
