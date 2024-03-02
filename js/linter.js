"use strict"
const fs = require('fs'); // File system, this is needed to import external files into the program.

// Get the filename from the command line arguments
const fileName = process.argv[2];

let fileContent;

// Read the content of the file
fs.readFile(fileName, 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    fileContent = data;

    extractEntry(fileContent);

});



function extractEntry(data) {
    // Split the data into lines
    const lines = data.split('\n');


    // Iterate through the lines and extract the entry
    let entry = '';
    let inEntry = false;

    for (const line of lines) {
        if (line.startsWith('@Comment')) { /* Do nothing */ }
        else if (line.startsWith('@')) {
            entry += line + '\n';
            inEntry = true;
        } else if (inEntry) {
            entry += line + '\n';
        }
    }

    console.log(entry);
}


