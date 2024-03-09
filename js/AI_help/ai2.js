const fs = require('fs');

const filePath = process.argv[2];

let fileContent;

// Read the content of the file
fs.readFile(filePath, 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    fileContent = data;
    let entries = [...fileContent.matchAll(entryPattern)].map(match => ({
        type: match[1],
        citationName: match[2].trim(),
        content: match[3],
        position: match.index
    }));

    console.log(entries);
    checkForMissingFields(entries);
});

const { requirements } = require("../components/entrytypes.js")

const entryPattern = /@(\w+)\{([^,]+),\s*(.*?)\}\n\n/sg;

function checkForMissingFields(entries) {
    entries.forEach(entry => {
        let fields = {};
        let fieldPattern = /(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))/sg;
        let fieldMatch;
        while ((fieldMatch = fieldPattern.exec(entry.content)) !== null) {
            fields[fieldMatch[1]] = fieldMatch[2] || fieldMatch[3];
            console.log(fieldMatch);
        }

        let missingFields = [];
        if (requirements[entry.type]) {
            requirements[entry.type].forEach(field => {
                if (!fields[field]) {
                    missingFields.push(field);
                }
            });
        }

        // Report/Log the anomalies.
        if (missingFields.length > 0) {
            // Calculate the line number for the anomaly position
            const lineNumber = fileContent.substring(0, entry.position).split('\n').length;
            console.log(`Anomaly detected in ${entry.type} entry "${entry.citationName}" at position ${entry.position}: Missing fields - ${missingFields.join(', ')}.\nAt -> ${filePath}:${lineNumber}`);
        }
    });
}
