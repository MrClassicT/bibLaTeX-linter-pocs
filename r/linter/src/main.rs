mod checker;
mod patterns;
mod requirements;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please provide a .bib filepath.");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");

    let entry_pattern = &patterns::patterns::ENTRY_PATTERN;

    let entries: Vec<_> = entry_pattern
        .captures_iter(&file_content)
        .map(|cap| {
            let entry_type = cap[1].to_owned();
            let citation_name = cap[2].trim().to_owned();
            let content = cap[3].to_owned();
            let line_number = file_content[..cap.get(0).unwrap().start()].lines().count();
            (entry_type, citation_name, content, line_number)
        })
        .collect();

    let duplicates = checker::checker::check_for_duplicates(
        &entries
            .iter()
            .map(|(a, b, c, d)| (a.as_str(), b.as_str(), c.as_str(), *d))
            .collect::<Vec<_>>(),
    );

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

    // println!(
    //     "Entry type: {}, Citation name: {}, \nContent: {}",
    //     entry_type, citation_name, content
    // );
    // println!("-------");

    // for field_cap in field_pattern.captures_iter(content) {
    //     let field = &field_cap[1];
    //     let value = field_cap
    //         .get(2)
    //         .or(field_cap.get(3))
    //         .map_or("No value captured", |m| m.as_str().trim());

    //     println!("Field: {}, Value: {}", field, value);
    // }

    // ------------ Test code -----------
    // Regex example with dates.
    // let text = "Here are some dates: 2023-03-28, 2024-01-01, and 2025-12-31.";
    // let date_pattern = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    // for date_match in date_pattern.find_iter(text) {
    //     println!("Found date: {}", &date_match.as_str());
    // }

    // let res = entry_pattern.find(&file_content);
    // if let Some(matched) = res {
    //     println!("{}", &matched.as_str()); // `as_str` returns the matched string
    // } else {
    //     println!("No match found");
    // }

    // for entry_match in entry_pattern.find_iter(&file_content) {
    //     println!("match found!");
    // }
}
