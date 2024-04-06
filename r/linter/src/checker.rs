pub mod checker {
    // use std::collections::hash_map::Entry;
    // use std::collections::HashMap;

    // use crate::patterns::patterns::{ENTRY_PATTERN, FIELD_PATTERN};
    //use crate::requirements::get_entry_requirements;

    // pub fn check_for_missing_fields<'a>(
    //     entry: &(&'a str, &'a str, &'a str, usize),
    // ) -> Vec<&'a str> {
    //     let mut fields: HashMap<&str, &str> = HashMap::new();
    //     let mut missing_fields: Vec<&str> = Vec::new();

    //     let field_pattern = FIELD_PATTERN;
    //     let entry_pattern = ENTRY_PATTERN;
    //     let entry_requirements = get_entry_requirements();

    //     for field_match in field_pattern.captures_iter(entry.0) {
    //         let field_name = field_match.get(1).unwrap().as_str();
    //         let field_value = field_match
    //             .get(2)
    //             .map_or_else(|| field_match.get(3).unwrap().as_str(), |m| m.as_str());
    //         fields.insert(field_name, field_value);
    //     }

    //     if let Some(required_fields) = entry_requirements.types.get(entry.0) {
    //         for field in required_fields.required.iter() {
    //             if !fields.contains_key(field) {
    //                 missing_fields.push(field);
    //             }
    //         }
    //     }

    //     missing_fields // Return the missing_fields vector instead of printing the error message.
    // }

    pub fn check_for_duplicates<'a>(
        entries: &[(&'a str, &'a str, &'a str, usize)],
    ) -> Vec<(&'a str, usize)> {
        let mut duplicates: Vec<(&str, usize)> = Vec::new();
        let mut seen: Vec<&str> = Vec::new();

        for entry in entries.iter() {
            let citation_name = entry.1;
            if seen.contains(&citation_name) {
                duplicates.push((citation_name, entry.3));
            } else {
                seen.push(citation_name);
            }
        }
        duplicates
    }
}
