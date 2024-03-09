const path = require('path');
const os = require('os');

function toFileUrl(filePath) {
    let absolutePath = path.resolve(filePath);

    // Encode as URI component to handle special characters
    absolutePath = encodeURI(absolutePath);

    if (os.platform() === 'win32') {
        // Replace backslashes with forward slashes and prepend with a slash
        absolutePath = '/' + absolutePath.replace(/\\/g, '/');
        return `file://${absolutePath}`;
    } else {
        return `file://${absolutePath}`;
    }
}

const relativePath = '../testData/test.bib';
const fileUrl = toFileUrl(relativePath);

console.log(fileUrl);

console.log('******'); console.log('******'); console.log('******'); console.log('******'); console.log('******'); console.log('******');
function printClickablePath(filePath) {
    const absolutePath = path.resolve(filePath);
    console.log(`\u001B]8;;file://${absolutePath}\u0007${absolutePath}\u001B]8;;\u0007`);
}


printClickablePath(relativePath);
