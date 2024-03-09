const reportMissingFields = require("../checks/missingfields.js");

// Extract entries with citation name
const [entryPattern, fieldPattern] = require('../components/entrytypes.js');

const filePath = process.argv[2];

// Read the content of the file
fs.readFile(filePath, 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }
    fileContent = data;

});


let entries = [...fileContent.matchAll(entryPattern)].map(match => ({
    type: match[1],
    citationName: match[2].trim(), // Capture and trim the citation name
    content: match[3],
    position: match.index
}));

// Parse each entry's content into an object and validate required fields
entries.forEach(entry => {
    let fields = {};
    // Match key = {value} pairs, accounting for nested braces
    let fieldMatch;
    while ((fieldMatch = fieldPattern.exec(entry.content)) !== null) {
        fields[fieldMatch[1]] = fieldMatch[2] || fieldMatch[3];
    }

    reportMissingFields(entry);

});