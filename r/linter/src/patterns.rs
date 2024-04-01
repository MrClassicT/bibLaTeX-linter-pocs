// use crate::Regex;

mod patterns {
    pub static entry_pattern: Regex = Regex::new(r"(?is)@(\w+)\{([^,]+),\s*(.*?)\},\s*\}").unwrap();
    pub static field_pattern: Regex = Regex::new(r"(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))").unwrap();
}
