use std::env;
use std::fs;
use std::time::Instant;

mod checker;
mod patterns;
mod requirements;

fn main() {
    // Measure execution time
    let start_time = Instant::now();

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

    checker::checker::check_for_duplicates(
        &entries
            .iter()
            .map(|(a, b, c, d)| (a.as_str(), b.as_str(), c.as_str(), *d))
            .collect::<Vec<_>>(),
        &file_path,
    );

    // for entry in entries {
    //     checker::checker::check_for_missing_fields(entry);
    // }

    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    println!("Execution time: {:?}", execution_time);
}
