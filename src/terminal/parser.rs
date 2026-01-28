pub(crate) enum ParserState {
    Ground,
    Escape,
    EscCharset(CharsetSlot),
    Csi(CsiParser),
    String(StringParser),
}

#[derive(Clone, Copy)]
pub(crate) enum CharsetSlot {
    G0,
    G1,
}

#[derive(Clone, Copy)]
pub(crate) enum Charset {
    Ascii,
    DecSpecial,
}

pub(crate) struct StringParser {
    pub(crate) kind: StringKind,
    pub(crate) buf: Vec<u8>,
    pub(crate) esc: bool,
}

impl StringParser {
    pub(crate) fn new(kind: StringKind) -> Self {
        Self {
            kind,
            buf: Vec::new(),
            esc: false,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum StringKind {
    Osc,
    Dcs,
    Sos,
    Pm,
    Apc,
}

pub(crate) struct CsiParser {
    pub(crate) params: Vec<u16>,
    pub(crate) current: Option<u16>,
    pub(crate) private: bool,
}

impl CsiParser {
    pub(crate) fn new() -> Self {
        Self {
            params: Vec::new(),
            current: None,
            private: false,
        }
    }

    pub(crate) fn push_current(&mut self) {
        let value = self.current.take().unwrap_or(0);
        self.params.push(value);
    }

    pub(crate) fn param(&self, idx: usize, default: u16) -> u16 {
        self.params.get(idx).copied().unwrap_or(default)
    }
}

pub(crate) fn map_dec_special(ch: char) -> char {
    match ch {
        'j' => '┘',
        'k' => '┐',
        'l' => '┌',
        'm' => '└',
        'n' => '┼',
        'q' => '─',
        't' => '├',
        'u' => '┤',
        'v' => '┴',
        'w' => '┬',
        'x' => '│',
        'y' => '≤',
        'z' => '≥',
        '{' => 'π',
        '|' => '≠',
        '}' => '£',
        '~' => '·',
        _ => ch,
    }
}
