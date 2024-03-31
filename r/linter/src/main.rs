mod checker;
mod patterns;
mod requirements;
// use regex::Regex;
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

    // //println!("{}", file_content); // In orde, we kunnen de data goed inlezen.

    // Regex patterns
    // let entry_pattern = Regex::new(r"@(\w+)\{([^,]+),\s*(.*?)\}\n").unwrap();// <- This one was faulty, causing the app not to recognize any matches!

    // Patterns have been moved to seperate file:
    // let entry_pattern = Regex::new(r"(?is)@(\w+)\{([^,]+),\s*(.*?)\}\n").unwrap();
    // let field_pattern = Regex::new(r"(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))").unwrap();

    let entries: Vec<_> = entry_pattern
        .captures_iter(&file_content)
        .map(|cap| {
            let entry_type = &cap[1];
            let citation_name = cap[2].trim();
            let content = &cap[3];
            let position = cap.get(0).unwrap().start();
            (entry_type, citation_name, content, position)
        })
        .collect();

    checker::check_for_duplicates(&entries);
    checker::check_for_missing_fields(&entries);

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
