import fs from 'fs'
import path from 'path'
import {average, confidenceIntervals} from './math'


const MAX_COMPILER_VERSION = 46

const ALL_PROFILE_TYPES = ['check', 'check_incremental', 'release', 'release_incremental', 'debug', 'debug_incremental'] as const
type ProfileTypeTuple = typeof ALL_PROFILE_TYPES
type ProfileType = ProfileTypeTuple[number]

export interface ChartData {
    name: string,
    url: string,
    commit: string,
    compiler_versions: Array<string>,
    profiles: Record<ProfileType, Array<Point>>,
}   

export interface Point {
    x: string,
    y: number,
    y0: number,
    y1: number,
}

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

function processProfiles(results): Array<ChartData> {
    const parsedResults = JSON.parse(results)
    return Object.keys(parsedResults).map(repo => {
        const name = parsedResults[repo]['repo']['name']
        const url = parsedResults[repo]['repo']['url']
        const commit = parsedResults[repo]['repo']['commit_hash']
        const compiler_versions = get_compiler_versions(parsedResults[repo]['repo']['min_version'])
        const profiles: Record<string, Array<Point>> = {};
        ALL_PROFILE_TYPES.forEach((profile_type) => {
            profiles[profile_type] = processProfile(parsedResults[repo][profile_type])
        })
        return {
            name,
            url,
            commit,
            compiler_versions,
            profiles
        }
    })
}

function get_compiler_versions(min_version: string): Array<string> {
    const start = parseInt(min_version.split("_")[1])
    const end = MAX_COMPILER_VERSION
    return Array.from({length: (end - start + 1)}, (v, k) => k + start).map(x => "1." + x)
}

function processProfile(profile): Array<Point> {
    const result: Array<Point> = []
    Object.keys(profile).sort().map(compiler_version => {
        const x = "1." + parseInt(compiler_version.split("_")[1]);
        const y = average(profile[compiler_version])/1000
        const [y1, y0] = confidenceIntervals(profile[compiler_version])
        result.push({
            x, y, y0, y1
        })
    })
    return result
}
