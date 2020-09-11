import * as d3 from "d3"
import styles from '../styles/Home.module.css'
import React, { Component } from "react"
import ChartData from '../data/profiling'

class LineChart extends Component<ChartData> {
    constructor(props) {
        super(props);
        this.state = {
            myRef: React.createRef(),
        }
    }

    componentDidMount() {
        this.drawBarChart()
    }

    drawBarChart() {
        var margin = { top: 10, right: 30, bottom: 30, left: 60 },
            width = 460 - margin.left - margin.right,
            height = 400 - margin.top - margin.bottom;
        const svgCanvas = d3.select(this.state.myRef.current)
            .append("svg")
            .attr("width", width + margin.left + margin.right)
            .attr("height", height + margin.top + margin.bottom)
            .append("g")
            .attr("transform",
                "translate(" + margin.left + "," + margin.top + ")");

        const x = d3.scaleOrdinal()
            .domain(this.props.compiler_versions)
            .range(Array.from({ length: this.props.compiler_versions.length }, (_, k) => k * width / this.props.compiler_versions.length))
        // .range([0, width]);
        svgCanvas.append("g")
            .attr("transform", "translate(0," + height + ")")
            .call(d3.axisBottom(x));
        const y = d3.scaleLinear()
            .domain([0, 200])
            .range([height, 0]);
        svgCanvas.append("g")
            .call(d3.axisLeft(y));
        Object.keys(this.props.profiles).forEach(profileType => {
            let profile = this.props.profiles[profileType];
            svgCanvas
                .append("path")
                .datum(profile)
                .attr("fill", "none")
                .attr("stroke", "steelblue")
                .attr("stroke-width", 1.5)
                .attr("d", d3.line()
                    .x(function (d) { return x(d.x) })
                    .y(function (d) { return y(d.y) })
                )
        })
    }

    render() {
        return <a href={this.props.url} className={styles.card} key={this.props.name}>
            <h3>{this.props.name}</h3>
            <div ref={this.state.myRef} />
        </a>
    }
}

export default LineChart