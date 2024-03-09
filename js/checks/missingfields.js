// Required fields for each entry type
const { entryRequirements } = require('../components/entrytypes.js');
const { fieldPattern } = require('../components/regex.js')

function checkForMissingFields(entry) {

    let fields = {};

    while ((fieldMatch = fieldPattern.exec(entry.content)) !== null) {
        fields[fieldMatch[1]] = fieldMatch[2] || fieldMatch[3];
        // console.debug(fieldMatch[1]);
        // console.debug(fieldMatch[2]);
        // console.debug(fieldMatch[3]);
    }

    let missingFields = [];
    if (entryRequirements[entry.type]) {
        entryRequirements[entry.type].required.forEach(field => {
            if (!fields[field]) {
                missingFields.push(field);
            }
        });
    }


    return missingFields;
}

module.exports = { checkForMissingFields };
