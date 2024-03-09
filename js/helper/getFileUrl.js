"use strict";

const path = require('path');
const os = require('os');

function toFileUrl(filePath) {
    let absolutePath = path.resolve(filePath);

    // Encode as URI component to handle special characters
    absolutePath = encodeURI(absolutePath);

    if (os.platform() === 'win32') {
        // Replace backslashes with forward slashes and prepend with a slash
        absolutePath = '/' + absolutePath.replace(/\\/g, '/');
        return `file://${absolutePath}`; // TODO - check this on a Windows machine.
    } else {
        return `${absolutePath}`;
    }
}

module.exports = { toFileUrl };

