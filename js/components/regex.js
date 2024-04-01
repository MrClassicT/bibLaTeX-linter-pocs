// List of all the regex being used inside the linter.
//
// Extract each entry from the .bib file.
const entryPattern = /@(\w+)\{([^,]+),\s*(.*?)\},\s*\}/sg;
// Extract each field from the entry.
const fieldPattern = /(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))/sg;

module.exports = { entryPattern, fieldPattern };
