// Define entry types, their required and recommended fields (with all the fields HOGENT prefers! Differs from actual BibLaTeX manual.)
const entryRequirements = {
    Article: {
        required: ['author', 'title', 'journaltitle', 'date'],
        recommended: ['doi', 'volume', 'number', 'pages'],
    },
    Book: {
        required: ['author', 'title', 'date', 'publisher'],
        recommended: ['isbn'],
    },
    Booklet: {
        required: ['author', 'title', 'date', 'publisher'],
        recommended: ['isbn', 'doi', 'url', 'howpublished'],
    },
    Conference: { // Alias for Inproceedings
        required: ['author', 'title', 'booktitle', 'date'],
        recommended: ['editor', 'eventtitle', 'isbn', 'doi', 'url'],
    },
    Dataset: {
        required: ['author', 'title', 'date', 'url', 'urldate'],
        recommended: [],
    },
    InBook: {
        required: ['author', 'title', 'booktitle', 'date', 'publisher'],
        recommended: ['isbn', 'doi', 'pages'],
    },
    Manual: {
        required: ['author', 'title', 'date'],
        recommended: ['organization', 'publisher', 'isbn', 'doi', 'url'],
    },
    Misc: { // Alias for Software
        required: ['author', 'title', 'date'],
        recommended: [],
    },
    Online: { // Alias: Electronic, Www
        required: ['author', 'date', 'title', 'url', 'urldate'],
        recommended: [],
    },
    Report: { // Alias: Techreport
        required: ['author', 'title', 'date', 'type', 'institution'],
        recommended: ['doi', 'url'],
    },
    Thesis: { // Alias: Mastersthesis, Phdthesis
        required: ['author', 'title', 'date', 'type', 'institution'],
        recommended: ['url'],
    },
};

// Assigning aliases
entryRequirements.Electronic = entryRequirements.Online;
entryRequirements.Www = entryRequirements.Online;
entryRequirements.Mastersthesis = entryRequirements.Thesis;
entryRequirements.Phdthesis = entryRequirements.Thesis;
entryRequirements.Techreport = entryRequirements.Report;
entryRequirements.Software = entryRequirements.Misc;
entryRequirements.Inproceedings = entryRequirements.Conference;

// Export the entry requirements object
module.exports = { entryRequirements };
