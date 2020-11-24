import * as fs from 'fs'
import * as path from 'path'
import { reduce, SeriesPoint } from './math'


const MAX_COMPILER_VERSION = 46

const ALL_PROFILE_TYPES = ['check', 'check_incremental', 'release', 'release_incremental', 'debug', 'debug_incremental'] as const
type ProfileTypeTuple = typeof ALL_PROFILE_TYPES
type ProfileType = ProfileTypeTuple[number]

export interface ChartData {
    repo: Repo,
    x_axis: Array<string>,
    compile_times: Array<SeriesData>,
    debug_sizes: Array<string | number>,
    release_sizes: Array<string | number>,
}

interface Repo {
    name: string,
    sub_directory: string,
    url: string,
    touch_file: string,
    output: string,
    commit: string,
    min_version: string,
}

interface Profile {
    compile_times: Record<string, Array<number>>,
    output_sizes: Record<string, number>,
}

interface SystemInfo {
    num_cores: number,
    num_physical_cores: number,
    cpu_model: string,
}

interface ResultFile {
    system_info: SystemInfo,
    profiles: Record<string, Profile>,
}

type SeriesData = Array<string | SeriesPoint>;

const resultsDirectory = path.join(process.cwd(), '../data/')

export function getChartData(): Array<ChartData> {

    const repos: Array<Repo> = JSON.parse(fs.readFileSync(path.join(resultsDirectory, 'repos.json'), 'utf-8'));
    const repo_names = repos.map(repo => repo.name);

    const resultFiles = fs.readdirSync(resultsDirectory);
    const results: Array<ResultFile> = resultFiles
        .filter(fileName => fileName.startsWith('results'))
        .map(fileName => {
            const fullPath = path.join(resultsDirectory, fileName);
            const fileContents = fs.readFileSync(fullPath, 'utf-8');
            return JSON.parse(fileContents);
        });

    const system_infos = results.map(result => result.system_info);
    const profiles: Array<[number, Record<string, Profile>]> = results.map(result => [result.system_info.num_cores, result.profiles]);
    const compile_times = combineCompileTimes(repo_names, profiles);
    const sizes = output_sizes(repo_names, profiles);
    const axes = x_axes(repo_names, profiles);

    return repos.map(repo => {
        const [debug_size, release_size] = sizes[repo.name];
        return {
            "repo": repo,
            "x_axis": axes[repo.name],
            "compile_times": compile_times[repo.name],
            "debug_sizes": debug_size,
            "release_sizes": release_size,
        };
    })
}

function combineCompileTimes(repo_names: Array<string>, profiles: Array<[number, Record<string, Profile>]>): Record<string, Array<SeriesData>> {
    return repo_names.reduce((map, repo_name) => {
        let z: Record<string, Array<SeriesPoint>> = {};
        profiles.forEach(profile => {
            const processor = profile[0].toString() + " cores";
            const compile_times = profile[1][repo_name].compile_times;
            Object.entries(compile_times).forEach(value => {
                const existing_key = value[0].split(",");
                const version = existing_key[0];
                const compiler_mode = existing_key[1];
                const profile_mode = existing_key[2];
                const new_key = compiler_mode + "," + profile_mode + "," + processor;

                const series_point = reduce(value[1], version);
                if (z[new_key]) {
                    z[new_key].push(series_point);
                } else {
                    z[new_key] = [series_point];
                }

            });
        });
        // prepend string to the value, making it SeriesData. sort it
        let all_series: Array<SeriesData> = Object.entries(z).map(key_with_points => {
            const key = key_with_points[0];
            const points = key_with_points[1];
            const series_data: SeriesData = points.sort((a, b) => a.version.localeCompare(b.version));
            series_data.unshift(key);
            return series_data;
        });
        map[repo_name] = all_series;
        return map;
    }, {});
}

function x_axes(repo_names: Array<string>, profiles: Array<[number, Record<string, Profile>]>): Record<string, Array<string>> {
    const profile = profiles.shift()[1];
    return repo_names.reduce((map, repo_name) => {
        const versions = ['x'];
        const sizes = profile[repo_name].output_sizes;
        Object.entries(sizes).forEach(size => {
            if (size[0].indexOf('Debug') > 0) {
                const existing_key = size[0].split(",");
                const version = existing_key[0];
                versions.push(version);
            }
        });
        map[repo_name] = versions;
        return map;
    }, {});
}

function output_sizes(repo_names: Array<string>, profiles: Array<[number, Record<string, Profile>]>): Record<string, [Array<string | number>, Array<string | number>]> {
    const profile = profiles.shift()[1];
    return repo_names.reduce((map, repo_name) => {
        const sizes = profile[repo_name].output_sizes;
        const debug_sizes: Array<string | number> = ['Debug'];
        const release_sizes: Array<string | number> = ['Release'];
        Object.entries(sizes).forEach(size => {
            if (size[0].indexOf('Debug') > 0) {
                debug_sizes.push(size[1]);
            }
            if (size[0].indexOf('Release') > 0) {
                release_sizes.push(size[1]);
            }
        })
        map[repo_name] = [debug_sizes, release_sizes];
        return map;
    }, {});
}
