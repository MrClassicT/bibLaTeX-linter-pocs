use crate::patterns::entry_pattern;
use crate::patterns::field_pattern;
// use crate::requirements::get_entry_requirements;
// use std::collections::HashMap;

mod checker {

    pub fn check_for_missing_fields(entry: &str) -> Vec<&str> {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        let mut missing_fields: Vec<&str> = Vec::new();

        let field_pattern = patterns::field_pattern;
        let required_fields = get_entry_requirements();

        for field_match in field_pattern.captures_iter(entry) {
            let field_name = field_match.get(1).unwrap().as_str();
            let field_value = field_match
                .get(2)
                .map_or_else(|| field_match.get(3).unwrap().as_str(), |m| m.as_str());
            fields.insert(field_name, field_value);
        }

        if let Some(required_fields) = entry_requirements.get(entry.r#type) {
            for field in required_fields {
                if !fields.contains_key(field) {
                    missing_fields.push(field);
                }
            }
        }

        if !missing_fields.is_empty() {
            let plural = missing_fields.len() > 1;
            eprintln!(
                "Caution: {}missing field{} {} ha{} been found!",
                if plural { "" } else { "A " },
                if plural { "s" } else { "" },
                missing_fields.join(", "),
                if plural { "ve" } else { "s" }
            );
        }
    }

    pub fn check_for_duplicates(entries: &[Entry]) {
        let citation_names: Vec<&str> = entries.iter().map(|entry| entry.citation_name).collect();
        let duplicates: Vec<&str> = citation_names
            .iter()
            .filter(|&citation_name| {
                citation_names
                    .iter()
                    .position(|&name| name == citation_name)
                    != citation_names
                        .iter()
                        .rposition(|&name| name == citation_name)
            })
            .cloned()
            .collect();

        if !duplicates.is_empty() {
            let plural = duplicates.len() > 1;
            eprintln!(
                "Caution: {}duplicate key{} {} ha{} been found!",
                if plural { "" } else { "A " },
                if plural { "s" } else { "" },
                duplicates.join(", "),
                if plural { "ve" } else { "s" }
            );
        }
    }
}
