function checkForDuplicates(entries) {
    const citationNames = entries.map(entry => entry.citationName);
    const duplicates = citationNames.filter((citationName, index, self) => self.indexOf(citationName) !== index && self.lastIndexOf(citationName) === index
    );

    if (duplicates.length > 0) {
        const plural = duplicates.length > 1;
        console.error(`Caution: ${plural ? "" : "A "}duplicate key${plural ? "s" : ""} ${duplicates} ha${plural ? "ve" : "s"} been found!`);
        exit(1);
    }
}

module.exports = { checkForDuplicates };