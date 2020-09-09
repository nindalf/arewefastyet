import fs from 'fs'
import path from 'path'

const resultsDirectory = path.join(process.cwd(), 'results')

export function getAllPerfData() {
    const fileNames = fs.readdirSync(resultsDirectory);

    const perfData = fileNames.map(fileName => {
        // Read file as string
        const fullPath = path.join(resultsDirectory, fileName);
        const fileContents = fs.readFileSync(fullPath, 'utf8');

        // Use gray-matter to parse the post metadata section
        const profiles = processProfiles(fileContents);

        // Combine the data with the id
        return profiles;
    })
    return perfData;
}

function processProfiles(results) {
    return JSON.parse(results);
}