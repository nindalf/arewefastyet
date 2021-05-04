import styles from '../styles/Home.module.css'
import React, { Component } from "react"
import { ChartData } from '../data/chartData'
import { CompilerMode, ProfileMode, System } from '../data/types'
import {
    LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, Label, ResponsiveContainer, Text
} from 'recharts';

export interface LineChartXProps {
    chartData: ChartData,
    compiler_modes: Array<CompilerMode>,
    profile_modes: Array<ProfileMode>,
    systems: Array<System>,
    show_size_chart: boolean,
}

const lineColours: Record<string, string> = {
    'Check, 2 cores': '#ff94a8',
    'Check, 4 cores': '#fa738c',
    'Check, 8 cores': '#fc3d60',
    'Check, 16 cores': '#f71640',

    'Debug, 2 cores': '#9f95fc',
    'Debug, 4 cores': '#8377f7',
    'Debug, 8 cores': '#6556f5',
    'Debug, 16 cores': '#4a38fc',

    'Release, 2 cores': '#6cb871',
    'Release, 4 cores': '#4db353',
    'Release, 8 cores': '#2bb534',
    'Release, 16 cores': '#05b511',
};

const strokeWidthMap: Record<string, number> = {
    '2 cores': 1,
    '4 cores': 1.33,
    '8 cores': 1.66,
    '16 cores': 2,
};

export class LineChartX extends Component<LineChartXProps> {
    constructor(props) {
        super(props);
    }

    render() {
        const name = this.props.chartData.repo.name;
        const base_url = this.props.chartData.repo.url;
        const release = this.props.chartData.repo.commit;
        const release_url = base_url + '/releases/tag/' + release;
        return <div className={styles.card}>
            <h3 className={styles.title}>
                <a href={base_url}>{name}</a> (<a href={release_url}>{release}</a>)
            </h3>
            {this.compileTimeCharts()}
            {this.sizeChart()}
        </div>
    }

    compileTimeDataKeys() {
        return this.props.compiler_modes.map(compile_mode => {
            return this.props.profile_modes.map(profile_mode => {
                return this.props.systems.map(system => {
                    return [compile_mode, profile_mode, system]
                })
            })
        })
            .flatMap(x => x)
            .flatMap(x => x)
            .sort((a, b) => parseInt(a[2].substr(0, 2)) - parseInt(b[2].substr(0, 2)));
    }


    perfDelta(key: string, currentVersion: string) {
        let compile_times = this.props.chartData.compile_times;
        if (currentVersion === compile_times[0]["version"]) {
            return 0;
        }
        let i = 0;
        let prev = compile_times[i];
        let current = compile_times[i+1];
        while (currentVersion !== current["version"]) {
            i+=1;
            prev = compile_times[i];
            current = compile_times[i+1];
        }
        let currentVal = current[key] as number;
        const previousVal = prev[key] as number;
        return (previousVal - currentVal)/previousVal * 100;
    }


    compileTimeCharts() {
        return <ResponsiveContainer width="99%" height={300}>
            <LineChart
                data={this.props.chartData.compile_times}
                margin={{
                    top: 5, right: 30, left: 20, bottom: 5,
                }}
            >
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="version"></XAxis>
                <YAxis><Label value="Time (seconds)" position='left' angle={-90} /></YAxis>
                <Tooltip
                   labelFormatter={e => `v${e}`}
                   separator={': '}
                   formatter={(value, name, props) => {
                     const currentVersion = props.payload.version;
                     const delta = this.perfDelta(name, currentVersion);
                     return `${value.toFixed(2)}s (${delta.toFixed(2)}%)`;
                   }}
                />
                <Legend align='right' />
                {this.compileTimeDataKeys().map(([cm, pm, system]) => {
                    const key = `${cm}, ${pm}, ${system}`;
                    return <Line type="monotone" dataKey={key} stroke={lineColours[cm + ', ' + system]} strokeWidth={strokeWidthMap[system]} key={key} />;
                })
                }

            </LineChart>
        </ResponsiveContainer>
    }

    sizeChart() {
        if (!this.props.show_size_chart) {
            return <div />;
        }

        return <ResponsiveContainer width="99%" height={300}>
            <LineChart
                data={this.props.chartData.sizes}
                margin={{
                    top: 30, right: 30, left: 20, bottom: 5,
                }}
            >
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="version" ></XAxis>
                <YAxis><Label value="Size (MB)" position='left' angle={-90} /> </YAxis>
                <Tooltip />
                <Legend align='right' />
                <Line type="monotone" dataKey="Debug" stroke="#8884d8" />
                <Line type="monotone" dataKey="Release" stroke="#82ca9d" />

            </LineChart>
        </ResponsiveContainer>
    }
}
