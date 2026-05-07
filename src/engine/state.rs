use super::{telex, vni, InputMethod};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ComposeState {
    Idle,
    Composing,
}

pub(crate) fn transform_word_stateful(raw: &str, method: InputMethod) -> String {
    if raw.is_empty() {
        return String::new();
    }

    let mut composer = Composer::new(method);
    for ch in raw.chars() {
        composer.feed(ch);
    }
    composer.finish();
    composer.render().iter().collect()
}

struct Composer {
    method: InputMethod,
    state: ComposeState,
    raw_syllable: Vec<char>,
    rendered_syllable: Vec<char>,
    committed_output: Vec<char>,
}

impl Composer {
    fn new(method: InputMethod) -> Self {
        Self {
            method,
            state: ComposeState::Idle,
            raw_syllable: Vec::new(),
            rendered_syllable: Vec::new(),
            committed_output: Vec::new(),
        }
    }

    fn feed(&mut self, ch: char) {
        trace_step("feed", self.state, ch, &self.raw_syllable, &self.rendered_syllable);
        if is_word_char(ch) {
            self.state = ComposeState::Composing;
            self.raw_syllable.push(ch);
            self.recompose_current();
            return;
        }

        self.flush_syllable();
        self.committed_output.push(ch);
        self.state = ComposeState::Idle;
    }

    fn finish(&mut self) {
        self.flush_syllable();
    }

    fn flush_syllable(&mut self) {
        if self.state == ComposeState::Composing {
            self.recompose_current();
            trace_step(
                "flush",
                self.state,
                '\0',
                &self.raw_syllable,
                &self.rendered_syllable,
            );
            self.committed_output.extend(&self.rendered_syllable);
            self.raw_syllable.clear();
            self.rendered_syllable.clear();
        }
    }

    fn recompose_current(&mut self) {
        self.rendered_syllable = match self.method {
            InputMethod::Telex => telex::transform_from_raw(&self.raw_syllable),
            InputMethod::Vni => vni::transform_from_raw(&self.raw_syllable),
        };
        trace_step(
            "recompose",
            self.state,
            '\0',
            &self.raw_syllable,
            &self.rendered_syllable,
        );
    }

    fn render(&self) -> Vec<char> {
        let mut out = self.committed_output.clone();
        out.extend(&self.rendered_syllable);
        out
    }
}

fn is_word_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
}

fn trace_step(action: &str, state: ComposeState, ch: char, raw: &[char], rendered: &[char]) {
    if !trace_enabled() {
        return;
    }
    let key = if ch == '\0' { "-" } else { "" };
    if ch == '\0' {
        eprintln!(
            "[vnkey-trace] {action} state={state:?} raw={} rendered={}",
            raw.iter().collect::<String>(),
            rendered.iter().collect::<String>()
        );
    } else {
        eprintln!(
            "[vnkey-trace] {action} key={key}{ch} state={state:?} raw={} rendered={}",
            raw.iter().collect::<String>(),
            rendered.iter().collect::<String>()
        );
    }
}

fn trace_enabled() -> bool {
    static TRACE: OnceLock<bool> = OnceLock::new();
    *TRACE.get_or_init(|| match std::env::var("VNKEY_TRACE") {
        Ok(v) => matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => false,
    })
}

