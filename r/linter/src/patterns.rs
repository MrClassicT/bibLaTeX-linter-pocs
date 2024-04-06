pub mod patterns {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub static ENTRY_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?is)@(\w+)\{([^,]+),\s*(.*?)\},\s*\}").unwrap());
    pub static _FIELD_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))").unwrap());
}
