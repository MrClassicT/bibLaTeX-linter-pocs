pub struct EntryType {
    pub required: Vec<&'static str>,
    pub recommended: Vec<&'static str>,
}

pub struct EntryRequirements {
    pub article: EntryType,
    pub book: EntryType,
    pub booklet: EntryType,
    pub conference: EntryType,
    pub dataset: EntryType,
    pub electronic: Option<EntryType>,
    pub in_book: EntryType,
    pub inproceedings: Option<EntryType>,
    pub manual: EntryType,
    pub mastersthesis: Option<EntryType>,
    pub misc: EntryType,
    pub online: EntryType,
    pub phdthesis: Option<EntryType>,
    pub report: EntryType,
    pub software: Option<EntryType>,
    pub techreport: EntryType,
    pub thesis: EntryType,
    pub www: Option<EntryType>,
}

pub fn get_entry_requirements() -> EntryRequirements {
    let mut entry_requirements = EntryRequirements {
        article: EntryType {
            required: vec!["author", "title", "journaltitle", "date"],
            recommended: vec!["doi", "volume", "number", "pages"],
        },
        book: EntryType {
            required: vec!["author", "title", "date", "publisher"],
            recommended: vec!["isbn"],
        },
        booklet: EntryType {
            required: vec!["author", "title", "date", "publisher"],
            recommended: vec!["isbn", "doi", "url", "howpublished"],
        },
        conference: EntryType {
            required: vec!["author", "title", "booktitle", "date"],
            recommended: vec!["editor", "eventtitle", "isbn", "doi", "url"],
        },
        dataset: EntryType {
            required: vec!["author", "title", "date", "url", "urldate"],
            recommended: vec![],
        },
        electronic: None,
        in_book: EntryType {
            required: vec!["author", "title", "booktitle", "date", "publisher"],
            recommended: vec!["isbn", "doi", "pages"],
        },
        inproceedings: None,
        manual: EntryType {
            required: vec!["author", "title", "date"],
            recommended: vec!["organization", "publisher", "isbn", "doi", "url"],
        },
        mastersthesis: None,
        misc: EntryType {
            required: vec!["author", "title", "date"],
            recommended: vec![],
        },
        online: EntryType {
            required: vec!["author", "date", "title", "url", "urldate"],
            recommended: vec![],
        },
        phdthesis: None,
        report: EntryType {
            required: vec!["author", "title", "date", "type", "institution"],
            recommended: vec!["doi", "url"],
        },
        software: None,
        techreport: EntryType {
            required: vec!["author", "title", "institution", "date"],
            recommended: vec!["type", "number", "doi", "url"],
        },
        thesis: EntryType {
            required: vec!["author", "title", "date", "type", "institution"],
            recommended: vec!["url"],
        },
        www: None,
    };

    entry_requirements.electronic = Some(entry_requirements.online);
    entry_requirements.www = Some(entry_requirements.online);
    entry_requirements.mastersthesis = Some(entry_requirements.thesis);
    entry_requirements.phdthesis = Some(entry_requirements.thesis);
    entry_requirements.software = Some(entry_requirements.misc);
    entry_requirements.inproceedings = Some(entry_requirements.conference);

    entry_requirements
}
