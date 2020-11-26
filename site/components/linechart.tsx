import styles from '../styles/Home.module.css'
import React, { Component } from "react"
import { ChartData } from '../data/chartData'
import { CompilerMode, ProfileMode, System } from '../data/types'
import {
    LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, Label, Text
} from 'recharts';

export interface LineChartXProps {
    chartData: ChartData,
    compiler_modes: Array<CompilerMode>,
    profile_modes: Array<ProfileMode>,
    systems: Array<System>,
    show_size_chart: boolean,
}

const lineColours: Record<string, string> = {
    'Check,2 cores': '#ff94a8',
    'Check,4 cores': '#fa738c',
    'Check,8 cores': '#fc3d60',
    'Check,16 cores': '#f71640',

    'Debug,2 cores': '#9f95fc',
    'Debug,4 cores': '#8377f7',
    'Debug,8 cores': '#6556f5',
    'Debug,16 cores': '#4a38fc',

    'Release,2 cores': '#6cb871',
    'Release,4 cores': '#4db353',
    'Release,8 cores': '#2bb534',
    'Release,16 cores': '#05b511',
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
        }).flatMap(x => x).flatMap(x => x);
    }

    compileTimeCharts() {
        return <LineChart
            width={700}
            height={300}
            data={this.props.chartData.compile_times}
            margin={{
                top: 5, right: 30, left: 20, bottom: 5,
            }}
        >
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="version"></XAxis>
            <YAxis><Label value="Time (seconds)" position='insideLeft' /> </YAxis>
            <Tooltip />
            <Legend align='right' />
            {this.compileTimeDataKeys().map(([cm, pm, system]) => {
                return <Line type="monotone" dataKey={cm + ',' + pm + ',' + system} stroke={lineColours[cm + ',' + system]} />;
            })
            }

        </LineChart>
    }

    sizeChart() {
        if (!this.props.show_size_chart) {
            return <div/>;
        }
        
        return <LineChart
            width={700}
            height={300}
            data={this.props.chartData.sizes}
            margin={{
                top: 30, right: 30, left: 20, bottom: 5,
            }}
        >
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="version" ></XAxis>
            <YAxis><Label value="Size (MB)" position='insideLeft' /> </YAxis>
            <Tooltip />
            <Legend align='right' />
            <Line type="monotone" dataKey="Debug" stroke="#8884d8" />
            <Line type="monotone" dataKey="Release" stroke="#82ca9d" />

        </LineChart>
    }
}
