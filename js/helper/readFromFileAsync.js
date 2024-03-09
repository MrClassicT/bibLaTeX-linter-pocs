const fs = require('fs').promises;

// Function that reads content from the file and returns a promise
async function readFromFile(fileName) {
    try {
        const data = await fs.readFile(fileName, 'utf8');
        return data;
    } catch (err) {
        console.error(err);
        process.exit(1); // Exit the process in case of an error
    }
}

module.exports = { readFromFile };