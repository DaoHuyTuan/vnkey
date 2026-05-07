use super::Tone;

pub(crate) fn apply_tone(chars: &mut [char], tone: Tone) {
    if tone == Tone::None {
        return;
    }
    if let Some(idx) = find_tone_target(chars) {
        chars[idx] = tone_mark(strip_tone(chars[idx]), tone);
    }
}

pub(crate) fn is_vowel_or_marked(ch: char) -> bool {
    is_vowel(strip_tone(ch))
}

pub(crate) fn strip_tone(ch: char) -> char {
    match ch {
        'á' | 'à' | 'ả' | 'ã' | 'ạ' => 'a',
        'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' => 'ă',
        'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'â',
        'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' => 'e',
        'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 'ê',
        'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
        'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' => 'o',
        'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' => 'ô',
        'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'ơ',
        'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' => 'u',
        'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'ư',
        'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
        'Á' | 'À' | 'Ả' | 'Ã' | 'Ạ' => 'A',
        'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' => 'Ă',
        'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' => 'Â',
        'É' | 'È' | 'Ẻ' | 'Ẽ' | 'Ẹ' => 'E',
        'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' => 'Ê',
        'Í' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Ị' => 'I',
        'Ó' | 'Ò' | 'Ỏ' | 'Õ' | 'Ọ' => 'O',
        'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' => 'Ô',
        'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' => 'Ơ',
        'Ú' | 'Ù' | 'Ủ' | 'Ũ' | 'Ụ' => 'U',
        'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => 'Ư',
        'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
        _ => ch,
    }
}

pub(crate) fn detect_tone(ch: char) -> Tone {
    let base = strip_tone(ch);
    for tone in [Tone::Acute, Tone::Grave, Tone::Hook, Tone::Tilde, Tone::Dot] {
        if tone_mark(base, tone) == ch {
            return tone;
        }
    }
    Tone::None
}

pub(crate) fn tone_mark(ch: char, tone: Tone) -> char {
    let idx = tone_index(tone);
    match ch {
        'a' => ['a', 'á', 'à', 'ả', 'ã', 'ạ'][idx],
        'ă' => ['ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ'][idx],
        'â' => ['â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ'][idx],
        'e' => ['e', 'é', 'è', 'ẻ', 'ẽ', 'ẹ'][idx],
        'ê' => ['ê', 'ế', 'ề', 'ể', 'ễ', 'ệ'][idx],
        'i' => ['i', 'í', 'ì', 'ỉ', 'ĩ', 'ị'][idx],
        'o' => ['o', 'ó', 'ò', 'ỏ', 'õ', 'ọ'][idx],
        'ô' => ['ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ'][idx],
        'ơ' => ['ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ'][idx],
        'u' => ['u', 'ú', 'ù', 'ủ', 'ũ', 'ụ'][idx],
        'ư' => ['ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự'][idx],
        'y' => ['y', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ'][idx],
        'A' => ['A', 'Á', 'À', 'Ả', 'Ã', 'Ạ'][idx],
        'Ă' => ['Ă', 'Ắ', 'Ằ', 'Ẳ', 'Ẵ', 'Ặ'][idx],
        'Â' => ['Â', 'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ'][idx],
        'E' => ['E', 'É', 'È', 'Ẻ', 'Ẽ', 'Ẹ'][idx],
        'Ê' => ['Ê', 'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ'][idx],
        'I' => ['I', 'Í', 'Ì', 'Ỉ', 'Ĩ', 'Ị'][idx],
        'O' => ['O', 'Ó', 'Ò', 'Ỏ', 'Õ', 'Ọ'][idx],
        'Ô' => ['Ô', 'Ố', 'Ồ', 'Ổ', 'Ỗ', 'Ộ'][idx],
        'Ơ' => ['Ơ', 'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ'][idx],
        'U' => ['U', 'Ú', 'Ù', 'Ủ', 'Ũ', 'Ụ'][idx],
        'Ư' => ['Ư', 'Ứ', 'Ừ', 'Ử', 'Ữ', 'Ự'][idx],
        'Y' => ['Y', 'Ý', 'Ỳ', 'Ỷ', 'Ỹ', 'Ỵ'][idx],
        _ => ch,
    }
}

fn find_tone_target(chars: &[char]) -> Option<usize> {
    let vowel_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| is_vowel_or_marked(*ch).then_some(idx))
        .collect();
    if vowel_indices.is_empty() {
        return None;
    }
    let effective = effective_vowel_indices(chars, &vowel_indices);

    for &idx in &effective {
        if is_priority_vowel(chars[idx]) {
            return Some(idx);
        }
    }

    if effective.len() >= 3 {
        return Some(effective[1]);
    }

    if effective.len() == 2 {
        let first = effective[0];
        let second = effective[1];
        let second_is_last = second + 1 == chars.len();
        let first_char = strip_tone(chars[first]).to_ascii_lowercase();
        let starts_with_qu_or_gi =
            chars.len() >= 2 && matches!((chars[0], chars[1]), ('q' | 'Q', 'u' | 'U') | ('g' | 'G', 'i' | 'I'));

        if second_is_last && first != 0 && !starts_with_qu_or_gi && first_char != 'i' {
            return Some(first);
        }
        return Some(second);
    }

    effective.first().copied()
}

fn effective_vowel_indices(chars: &[char], vowel_indices: &[usize]) -> Vec<usize> {
    if vowel_indices.len() < 2 || vowel_indices[0] != 1 || chars.len() < 2 {
        return vowel_indices.to_vec();
    }
    let starts_with_qu_or_gi =
        matches!((chars[0], chars[1]), ('q' | 'Q', 'u' | 'U') | ('g' | 'G', 'i' | 'I'));
    if starts_with_qu_or_gi {
        vowel_indices[1..].to_vec()
    } else {
        vowel_indices.to_vec()
    }
}

fn is_vowel(ch: char) -> bool {
    matches!(
        ch,
        'a'
            | 'ă'
            | 'â'
            | 'e'
            | 'ê'
            | 'i'
            | 'o'
            | 'ô'
            | 'ơ'
            | 'u'
            | 'ư'
            | 'y'
            | 'A'
            | 'Ă'
            | 'Â'
            | 'E'
            | 'Ê'
            | 'I'
            | 'O'
            | 'Ô'
            | 'Ơ'
            | 'U'
            | 'Ư'
            | 'Y'
    )
}

fn is_priority_vowel(ch: char) -> bool {
    matches!(
        strip_tone(ch),
        'ă' | 'â' | 'ê' | 'ô' | 'ơ' | 'ư' | 'Ă' | 'Â' | 'Ê' | 'Ô' | 'Ơ' | 'Ư'
    )
}

fn tone_index(tone: Tone) -> usize {
    match tone {
        Tone::None => 0,
        Tone::Acute => 1,
        Tone::Grave => 2,
        Tone::Hook => 3,
        Tone::Tilde => 4,
        Tone::Dot => 5,
    }
}
