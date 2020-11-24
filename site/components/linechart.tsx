import styles from '../styles/Home.module.css'
import React, { Component } from "react"
import { ChartData, CompilerMode, ProfileMode, System } from '../data/chartData'
// import { ChartData } from '../data/chartData'
import {
    LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, Label, Text
} from 'recharts';

export interface LineChartXProps {
    chartData: ChartData,
    compiler_modes: Array<CompilerMode>,
    profile_modes: Array<ProfileMode>,
    systems: Array<System>,
}

export class LineChartX extends Component<LineChartXProps> {
    constructor(props) {
        super(props);
    }

    render() {
        return <div className={styles.card} key={this.props.chartData.repo.name}>
            <h3><a href={this.props.chartData.repo.url}>{this.props.chartData.repo.name}</a></h3>
            {this.compileTimeCharts()}
            {this.sizeChart()}
        </div>
    }

    compileTimeDataKeys() {
        return this.props.compiler_modes.map(compile_mode => {
            return this.props.profile_modes.map(profile_mode => {
                return this.props.systems.map(system => {
                    return compile_mode + "," + profile_mode + "," + system
                })
            })
        })
    }

    compileTimeCharts() {

        return <LineChart
            width={800}
            height={300}
            data={this.props.chartData.compile_times}
            margin={{
                top: 5, right: 30, left: 20, bottom: 5,
            }}
        >
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="version" ><Label value="Rust version" position='bottom' /></XAxis>
            <YAxis><Label value="Time (seconds)" angle="-90" position='insideLeft' /> </YAxis>
            <Tooltip />
            <Legend align='right' />
            {/* {this.compileTimeDataKeys().map(key => <Line type="monotone" dataKey={key} stroke="#8884d8" />)} */}
            <Line type="monotone" dataKey="Debug,Clean,2 cores" stroke="#8884d8" />
            <Line type="monotone" dataKey="Debug,Clean,4 cores" stroke="#82ca9d" />

        </LineChart>
    }

    sizeChart() {
        return <LineChart
            width={800}
            height={300}
            data={this.props.chartData.sizes}
            margin={{
                top: 5, right: 30, left: 20, bottom: 5,
            }}
        >
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="version" ><Label value="Rust version" position='bottom' /></XAxis>
            <YAxis><Label value="Size (MB)" angle="-90" position='insideLeft' /> </YAxis>
            <Tooltip />
            <Legend align='right' />
            <Line type="monotone" dataKey="Debug" stroke="#8884d8" />
            <Line type="monotone" dataKey="Release" stroke="#82ca9d" />

        </LineChart>
    }
}
