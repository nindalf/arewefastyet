import styles from '../styles/Home.module.css'
import React, { Component } from "react"
import { ChartData } from '../data/chartData'
import { CompilerMode, ProfileMode, System } from '../data/types'
import { LineChartX, LineChartXProps } from '../components/linechart'
import ToggleButtonGroup from 'react-bootstrap/ToggleButtonGroup'
import ToggleButton from 'react-bootstrap/ToggleButton'

interface AppConfig {
    compiler_modes: Array<CompilerMode>,
    profile_modes: Array<ProfileMode>,
    systems: Array<System>,
    show_size_chart: boolean,
}

export class AppContainer extends Component<Array<ChartData>, AppConfig> {
    constructor(props) {
        super(props);
        this.state = {
            profile_modes: [ProfileMode.Clean],
            compiler_modes: [CompilerMode.Debug, CompilerMode.Release],
            systems: [System.FourCores, System.EightCores],
            show_size_chart: false,
        };
        this.onCMChanged = this.onCMChanged.bind(this);
        this.onPMChanged = this.onPMChanged.bind(this);
        this.onSystemChanged = this.onSystemChanged.bind(this);
        this.onSizeChartClicked = this.onSizeChartClicked.bind(this);
    }

    onCMChanged(x) {
        this.setState({
            compiler_modes: x
        })
    }

    onPMChanged(x) {
        this.setState({
            profile_modes: x
        })
    }

    onSystemChanged(x) {
        this.setState({
            systems: x
        })

    }

    onSizeChartClicked(x) {
        this.setState({
            show_size_chart: !this.state.show_size_chart,
        })
    }

    render() {
        return <div className={styles.grid}>
            <div className={styles.card}>
                <ToggleButtonGroup name='Compiler Mode' type="checkbox" defaultValue={[CompilerMode.Debug, CompilerMode.Release]} className="mb-2" onChange={this.onCMChanged}>
                    <ToggleButton value={CompilerMode.Check}>Check</ToggleButton>
                    <ToggleButton value={CompilerMode.Debug}>Debug</ToggleButton>
                    <ToggleButton value={CompilerMode.Release}>Release</ToggleButton>
                </ToggleButtonGroup>

                <ToggleButtonGroup name='Profile Mode' type="checkbox" defaultValue={[ProfileMode.Clean]} className="mb-2" onChange={this.onPMChanged}>
                    <ToggleButton value={ProfileMode.Clean}>Clean</ToggleButton>
                    <ToggleButton value={ProfileMode.Incremental}>Incremental</ToggleButton>
                </ToggleButtonGroup>

                <ToggleButtonGroup name='Number of Cores' type="checkbox" defaultValue={[System.FourCores, System.EightCores]} className="mb-2" onChange={this.onSystemChanged}>
                    <ToggleButton value={System.TwoCores}>2 cores</ToggleButton>
                    <ToggleButton value={System.FourCores}>4 cores</ToggleButton>
                    <ToggleButton value={System.EightCores}>8 cores</ToggleButton>
                    <ToggleButton value={System.SixteenCores}>16 cores</ToggleButton>
                </ToggleButtonGroup>
                
                <ToggleButtonGroup name='Number of Cores' type="checkbox" defaultValue={[]} className="mb-2" onChange={this.onSizeChartClicked}>
                    <ToggleButton value={1}>Show binary size chart</ToggleButton>
                </ToggleButtonGroup>
            </div>
            {
                Object.keys(this.props).map(repo => {
                    const props: LineChartXProps = {
                        chartData: this.props[repo],
                        profile_modes: this.state.profile_modes,
                        compiler_modes: this.state.compiler_modes,
                        systems: this.state.systems,
                        show_size_chart: this.state.show_size_chart,
                    };
                    return <LineChartX {...props} key={repo} />
                })
            }
        </div >
    }
}