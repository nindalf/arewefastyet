import * as fs from 'fs'
import * as path from 'path'

export interface Repo {
    name: string,
    sub_directory: string,
    url: string,
    touch_file: string,
    output: string,
    commit: string,
    min_version: string,
}

export interface Profile {
    compile_times: Record<string, Array<number>>,
    output_sizes: Record<string, number>,
}

export interface SystemInfo {
    num_cores: number,
    num_physical_cores: number,
    cpu_model: string,
}

export interface ResultFile {
    system_info: SystemInfo,
    profiles: Record<string, Profile>,
}

const resultsDirectory = path.join(process.cwd(), '../data/')

export function getResults(): Array<ResultFile> {
    const resultFiles = fs.readdirSync(resultsDirectory);
    return resultFiles
        .filter(fileName => fileName.startsWith('results'))
        .map(fileName => {
            const fullPath = path.join(resultsDirectory, fileName);
            const fileContents = fs.readFileSync(fullPath, 'utf-8');
            return JSON.parse(fileContents);
        });
}

export function getRepos(): Array<Repo> {
    return JSON.parse(fs.readFileSync(path.join(resultsDirectory, 'repos.json'), 'utf-8'));
}