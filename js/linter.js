"use strict";

// Import submodules
const { exit } = require('process');
const { readFromFile } = require('./helper/readFromFileAsync.js');
const { checkForMissingFields } = require('./checks/missingfields.js');
const { entryPattern } = require('./components/regex.js');
const { toFileUrl } = require('./helper/getFileUrl.js')

// Get the filename from the command line arguments (also checks if one is actually present.)
const filePath = process.argv[2];

if (!filePath || filePath.trim() === "") {
    console.error('Error: Please provide a .bib filepath.');
    exit(1);
}


async function main() {
    let fileContent;
    try {
        fileContent = await readFromFile(filePath);
    } catch (err) {
        console.error('Error:', err);
    }

    // Extract all entries
    const entries = [...fileContent.matchAll(entryPattern)].map(match => ({
        type: match[1],
        citationName: match[2].trim(),
        content: match[3],
        position: match.index
    }));

    // Check for missing fields:
    entries.forEach(entry => {
        const missingFields = checkForMissingFields(entry);

        if (missingFields.length > 0) {
            const lineNumber = fileContent.substring(0, entry.position).split('\n').length;
            const fileUrl = toFileUrl(filePath);
            console.log(`Anomaly detected in ${entry.type} entry "${entry.citationName}" at position ${entry.position}: Missing fields - ${missingFields.join(', ')}.\nAt -> ${fileUrl}:${lineNumber}`);

        }
    }
    );

}

main();
