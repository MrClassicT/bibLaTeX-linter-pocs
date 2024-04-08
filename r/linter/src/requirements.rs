use std::collections::HashMap;

pub struct _EntryType {
    pub required: Vec<&'static str>,
    pub recommended: Vec<&'static str>,
}

pub struct _EntryRequirements {
    pub types: HashMap<&'static str, _EntryType>,
}

pub fn _get_entry_requirements() -> _EntryRequirements {
    let mut entry_requirements = _EntryRequirements {
        types: HashMap::new(),
    };

    // Add your entry types here
    entry_requirements.types.insert(
        "article",
        _EntryType {
            required: vec!["author", "title", "journaltitle", "date"],
            recommended: vec!["doi", "volume", "number", "pages"],
        },
    );

    entry_requirements.types.insert(
        "book",
        _EntryType {
            required: vec!["author", "title", "date", "publisher"],
            recommended: vec!["isbn"],
        },
    );

    entry_requirements.types.insert(
        "booklet",
        _EntryType {
            required: vec!["author", "title", "date", "publisher"],
            recommended: vec!["isbn", "doi", "url", "howpublished"],
        },
    );

    entry_requirements.types.insert(
        "conference",
        _EntryType {
            required: vec!["author", "title", "booktitle", "date"],
            recommended: vec!["editor", "eventtitle", "isbn", "doi", "url"],
        },
    );

    entry_requirements.types.insert(
        "dataset",
        _EntryType {
            required: vec!["author", "title", "date", "url", "urldate"],
            recommended: vec![],
        },
    );

    entry_requirements.types.insert(
        "in_book",
        _EntryType {
            required: vec!["author", "title", "booktitle", "date", "publisher"],
            recommended: vec!["isbn", "doi", "pages"],
        },
    );

    entry_requirements.types.insert(
        "inproceedings",
        _EntryType {
            required: vec!["author", "title", "booktitle", "date"],
            recommended: vec!["editor", "eventtitle", "isbn", "doi", "url"],
        },
    );

    entry_requirements.types.insert(
        "manual",
        _EntryType {
            required: vec!["author", "title", "date"],
            recommended: vec!["organization", "publisher", "isbn", "doi", "url"],
        },
    );

    entry_requirements.types.insert(
        "mastersthesis",
        _EntryType {
            required: vec!["author", "title", "date", "type", "institution"],
            recommended: vec!["url"],
        },
    );

    entry_requirements.types.insert(
        "misc",
        _EntryType {
            required: vec!["author", "title", "date"],
            recommended: vec![],
        },
    );

    entry_requirements.types.insert(
        "online",
        _EntryType {
            required: vec!["author", "date", "title", "url", "urldate"],
            recommended: vec![],
        },
    );

    entry_requirements.types.insert(
        "phdthesis",
        _EntryType {
            required: vec!["author", "title", "date", "type", "institution"],
            recommended: vec!["url"],
        },
    );

    entry_requirements.types.insert(
        "report",
        _EntryType {
            required: vec!["author", "title", "date", "type", "institution"],
            recommended: vec!["doi", "url"],
        },
    );

    entry_requirements.types.insert(
        "software",
        _EntryType {
            required: vec!["author", "title", "date"],
            recommended: vec![],
        },
    );

    entry_requirements.types.insert(
        "techreport",
        _EntryType {
            required: vec!["author", "title", "institution", "date"],
            recommended: vec!["type", "number", "doi", "url"],
        },
    );

    entry_requirements.types.insert(
        "thesis",
        _EntryType {
            required: vec!["author", "title", "date", "type", "institution"],
            recommended: vec!["url"],
        },
    );

    entry_requirements.types.insert(
        "www",
        _EntryType {
            required: vec!["author", "date", "title", "url", "urldate"],
            recommended: vec![],
        },
    );

    entry_requirements.types.insert(
        "electronic",
        _EntryType {
            required: entry_requirements
                .types
                .get("online")
                .unwrap()
                .required
                .clone(),
            recommended: entry_requirements
                .types
                .get("online")
                .unwrap()
                .recommended
                .clone(),
        },
    );

    entry_requirements.types.insert(
        "software",
        _EntryType {
            required: entry_requirements
                .types
                .get("misc")
                .unwrap()
                .required
                .clone(),
            recommended: entry_requirements
                .types
                .get("misc")
                .unwrap()
                .recommended
                .clone(),
        },
    );

    entry_requirements.types.insert(
        "inproceedings",
        _EntryType {
            required: entry_requirements
                .types
                .get("conference")
                .unwrap()
                .required
                .clone(),
            recommended: entry_requirements
                .types
                .get("conference")
                .unwrap()
                .recommended
                .clone(),
        },
    );

    entry_requirements.types.insert(
        "mastersthesis",
        _EntryType {
            required: entry_requirements
                .types
                .get("thesis")
                .unwrap()
                .required
                .clone(),
            recommended: entry_requirements
                .types
                .get("thesis")
                .unwrap()
                .recommended
                .clone(),
        },
    );

    entry_requirements.types.insert(
        "phdthesis",
        _EntryType {
            required: entry_requirements
                .types
                .get("thesis")
                .unwrap()
                .required
                .clone(),
            recommended: entry_requirements
                .types
                .get("thesis")
                .unwrap()
                .recommended
                .clone(),
        },
    );

    entry_requirements
}

// fn main() {
//     let requirements = get_entry_requirements();

//     // Retrieve the entry type dynamically
//     let entry_type_name = "article";
//     if let Some(entry_type) = requirements.types.get(entry_type_name) {
//         println!("Required fields: {:?}", entry_type.required);
//         println!("Recommended fields: {:?}", entry_type.recommended);
//     } else {
//         println!("Entry type not found");
//     }
// }
