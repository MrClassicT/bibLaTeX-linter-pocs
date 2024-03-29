entry_requirements = {
    "Article": {
        "required": ["author", "title", "journaltitle", "date"],
        "recommended": ["doi", "volume", "number", "pages"],
    },
    "Book": {
        "required": ["author", "title", "date", "publisher"],
        "recommended": ["isbn"],
    },
    "Booklet": {
        "required": ["title", "date"],
        "recommended": ["author", "howpublished", "address"],
    },
    "Conference": {  # Alias for Inproceedings
        "required": ["author", "title", "booktitle", "date"],
        "recommended": ["editor", "volume", "number", "series", "pages", "address", "month", "organization", "publisher", "note"],
    },
    "Inproceedings": {  # Given Conference is an alias for Inproceedings, they should have the same requirements
        "required": ["author", "title", "booktitle", "date"],
        "recommended": ["editor", "volume", "number", "series", "pages", "address", "month", "organization", "publisher", "note"],
    },
    "Dataset": {
        "required": ["author", "title", "date", "url", "urldate"],
        "recommended": [],
    },
    "InBook": {
        "required": ["author", "title", "booktitle", "date", "publisher"],
        "recommended": ["volume", "series", "type", "address", "edition", "month", "note"],
    },
    "Manual": {
        "required": ["title"],
        "recommended": ["author", "organization", "address", "edition", "month", "year", "note"],
    },
    "Misc": {
        "required": [],
        "recommended": ["author", "title", "howpublished", "month", "year", "note"],
    },
    "Online": {
        "required": ["author", "title", "date", "url", "urldate"],
        "recommended": [],
    },
    "Report": {
        "required": ["author", "title", "institution", "year"],
        "recommended": ["type", "number", "address", "month", "note"],
    },
    "Thesis": {
        "required": ["author", "title", "school", "year"],
        "recommended": ["type", "address", "month", "note"],
    },
}

# Assigning aliases
entry_requirements["Electronic"] = entry_requirements["Online"]
entry_requirements["Www"] = entry_requirements["Online"]
entry_requirements["Mastersthesis"] = entry_requirements["Thesis"]
entry_requirements["Phdthesis"] = entry_requirements["Thesis"]
entry_requirements["Techreport"] = entry_requirements["Report"]
entry_requirements["Software"] = entry_requirements["Misc"]


def get_requirements():
    return entry_requirements

if __name__ == "__main__":
    # This code will not be executed when imported as a module.
    print("This is the entry_requirements module.")