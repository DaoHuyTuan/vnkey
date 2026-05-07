mod telex;
mod state;
mod tone;
mod vni;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    Telex,
    Vni,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    None,
    Acute,
    Grave,
    Hook,
    Tilde,
    Dot,
}

pub fn transform_word(raw: &str, method: InputMethod) -> String {
    state::transform_word_stateful(raw, method)
}

