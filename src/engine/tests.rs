use super::{transform_word, InputMethod};

#[test]
fn vietnamese_paragraph_words_stay_intact() {
    let text = "Những người bạn thân thiết ở thành phố Hồ Chí Minh thường tụ họp vào buổi chiều tà, nhâm nhi tách cà phê đắng, trò chuyện về những kỷ niệm đẹp đẽ thuở thiếu thời. Bà Nguyễn Thị Hường kể lại rằng hồi nhỏ, bà hay chạy nhảy trên những cánh đồng lúa chín vàng ươm dưới ánh nắng hè, nghe tiếng chim hót líu lo, ngửi mùi hương đồng nội ngào ngạt. Thế mà giờ đây, phố xá ồn ào tấp nập, xe cộ ngược xuôi bất tận — cuộc sống đổi thay khiến lòng người bỗng chốc bồi hồi, xao xuyến khôn nguôi";

    for word in text.split(|ch: char| !ch.is_alphabetic()) {
        if word.is_empty() {
            continue;
        }
        assert_eq!(
            transform_word(word, InputMethod::Telex),
            word,
            "word changed unexpectedly: `{word}`"
        );
        assert_eq!(
            transform_word(word, InputMethod::Vni),
            word,
            "word changed unexpectedly: `{word}`"
        );
    }
}

#[test]
fn telex_w_handles_context_cases() {
    assert_eq!(simulate_telex_word("thuowr"), "thuở");
    assert_eq!(simulate_telex_word("thuowwng"), "thương");
    assert_eq!(simulate_telex_word("thuowng"), "thương");
    assert_eq!(simulate_telex_word("luoon"), "luôn");
    assert_eq!(simulate_telex_word("duawj"), "dựa");
    assert_eq!(simulate_telex_word("nhungxw"), "những");
    assert_eq!(simulate_telex_word("nhuwngx"), "những");
    assert_eq!(simulate_telex_word("nhungwx"), "những");
}

#[test]
fn telex_regression_real_world_words() {
    let cases = [
        ("giar", "giả"),
        ("gias", "giá"),
        ("giaf", "già"),
        ("quas", "quá"),
        ("quaf", "quà"),
        ("quyeen", "quyên"),
        ("quyeets", "quyết"),
        ("nghieenj", "nghiện"),
        ("chuyeenj", "chuyện"),
        ("thuyeenf", "thuyền"),
        ("thuowng", "thương"),
        ("thuowr", "thuở"),
        ("luoon", "luôn"),
        ("duawj", "dựa"),
        ("chieecs", "chiếc"),
    ];

    for (raw, expected) in cases {
        assert_eq!(transform_word(raw, InputMethod::Telex), expected, "raw={raw}");
    }

    // Vietlish safety: keep common English words untouched.
    assert_eq!(transform_word("test", InputMethod::Telex), "test");
    assert_eq!(transform_word("case", InputMethod::Telex), "case");
    assert_eq!(transform_word("cursor", InputMethod::Telex), "cursor");
    assert_eq!(transform_word("buzz", InputMethod::Telex), "buzz");
}

#[test]
fn vni_regression_basic_words() {
    let cases = [
        ("gia2", "già"),
        ("qua1", "quá"),
        ("thuong7", "thương"),
    ];

    for (raw, expected) in cases {
        assert_eq!(transform_word(raw, InputMethod::Vni), expected, "raw={raw}");
    }
}

fn simulate_telex_word(keys: &str) -> String {
    let mut raw = String::new();
    for key in keys.chars() {
        raw.push(key);
    }
    transform_word(&raw, InputMethod::Telex)
}
