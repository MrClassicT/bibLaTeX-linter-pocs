pub mod checker {
    // use std::collections::HashMap;

    // use regex::Regex;

    //use crate::patterns::patterns::FIELD_PATTERN;
    //use crate::requirements::get_entry_requirements;

    // pub fn check_for_missing_fields(entry: (String, String, String, usize)) {
    //     let mut fields: HashMap<&str, &str> = HashMap::new();
    //     let mut missing_fields: Vec<&str> = Vec::new();

    //     let field_pattern = Regex::new(r"(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))").unwrap();
    //     let entry_requirements = get_entry_requirements();

    //     // Get fields from entry
    //     for field_match in field_pattern.captures_iter(&entry.2) {
    //         let field_name = field_match.get(1).unwrap().as_str();
    //         // println!("{:?}", field_name);
    //         let field_value = field_match
    //             .get(2)
    //             .map_or_else(|| field_match.get(3).unwrap().as_str(), |m| m.as_str());
    //         fields.insert(field_name, field_value);
    //     }

    //     // Check if all required fields are present
    //     let entry = &entry.0[..]; // Convert entry to &str
    //     if let Some(required_fields) = entry_requirements.types.get(entry) {
    //         for reqfield in required_fields.required.iter() {
    //             if !fields.contains_key(reqfield) {
    //                 missing_fields.push(reqfield);
    //                 println!("Missing field: {}", reqfield);
    //             }
    //         }
    //     }

    //     // Print the missing fields
    //     if !missing_fields.is_empty() {
    //         println!("Missing fields:");
    //         for field in missing_fields {
    //             println!("{}", field);
    //         }
    //     } else {
    //         println!("Empty!");
    //     }
    // }

    pub fn check_for_duplicates<'a>(
        entries: &[(&'a str, &'a str, &'a str, usize)],
        file_path: &str,
    ) {
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

        for duplicate in duplicates {
            println!("All entry names should be unique.");
            let (entry_name, zerobased_line_number) = duplicate;
            let line_number = zerobased_line_number + 1;
            println!(
                "Duplicate field found in entry '{}' at line {}",
                entry_name, line_number
            );

            println!("External file path: {}:{}", file_path, line_number);
        }
    }
}
