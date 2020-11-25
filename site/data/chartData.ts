import { average } from './math'
import { Profile, Repo, getResults, getRepos } from './results'
import { getSystem } from './types';


export interface ChartData {
    repo: Repo,
    compile_times: ChartPoint[],
    sizes: ChartPoint[],
}

type ChartPoint = Record<string, string | number>;

export function getChartData(): Array<ChartData> {

    const repos = getRepos();
    const repo_names = repos.map(repo => repo.name);

    const results = getResults();
    const profiles: Array<[number, Record<string, Profile>]> = results.map(result => [result.system_info.num_cores, result.profiles]);
    const compile_times = combineCompileTimes(repo_names, profiles);
    const sizes = outputSizes(repo_names, profiles);

    return repos.map(repo => {
        return {
            "repo": repo,
            "compile_times": compile_times[repo.name],
            "sizes": sizes[repo.name],
        };
    })
}

function combineCompileTimes(repo_names: Array<string>, profiles: Array<[number, Record<string, Profile>]>): Record<string, ChartPoint[]> {
    return repo_names.reduce((map, repo_name) => {
        let output: { [version: string]: ChartPoint } = {};
        profiles.forEach(([cores, profile]) => {
            const system = getSystem(cores);
            const compile_times = profile[repo_name].compile_times;
            Object.entries(compile_times).forEach(([key, timings]) => {
                const [version, compiler_mode, profile_mode] = key.split(",");
                const new_key = compiler_mode + "," + profile_mode + "," + system;

                const average_timing = average(timings);
                if (!output[version]) {
                    output[version] = {}
                    output[version]['version'] = version;
                }
                output[version][new_key] = average_timing / 1000;
            });
        });
        const values = Object.entries(output).map(([_, value]) => value);

        map[repo_name] = values;
        return map;
    }, {});
}

function outputSizes(repo_names: Array<string>, profiles: Array<[number, Record<string, Profile>]>): Record<string, ChartPoint[]> {
    const [_, profile] = profiles.shift();
    return repo_names.reduce((map, repo_name) => {
        const sizes = profile[repo_name].output_sizes;
        const output: { [version: string]: ChartPoint } = {};
        Object.entries(sizes).forEach(([key, size]) => {
            const [version, compiler_mode] = key.split(',');
            if (!output[version]) {
                output[version] = {};
                output[version]['version'] = version;
            }
            const sizeMB = (size / (1024 * 1024)).toFixed(3);
            if (compiler_mode == 'Debug') {
                output[version]['Debug'] = sizeMB;
            }
            if (compiler_mode == 'Release') {
                output[version]['Release'] = sizeMB;
            }
        })
        const values: ChartPoint[] = Object.entries(output).map(([_, value]) => value);
        map[repo_name] = values;
        return map;
    }, {});
}
