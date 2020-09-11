import * as d3 from "d3"
import styles from '../styles/Home.module.css'
import React, { Component } from "react"
import { ChartData, Point } from '../data/profiling'

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
        svgCanvas.append("g")
            .attr("transform", "translate(0," + height + ")")
            .call(d3.axisBottom(x));

        let ys = Object.keys(this.props.profiles).map(profileType => {
            const profile = this.props.profiles[profileType]
            const ys = profile.map(point => point.y)
            return Math.max(...ys)
        })
        const max_y = Math.max(...ys)
        const y = d3.scaleLinear()
            .domain([0, max_y])
            .range([height, 0]);
        svgCanvas.append("g")
            .call(d3.axisLeft(y));
        
        // Add X axis label:
        svgCanvas.append("text")
            .attr("text-anchor", "end")
            .attr("x", width)
            .attr("y", height + margin.top + 20)
            .text("Compiler");

        // Y axis label:
        svgCanvas.append("text")
            .attr("text-anchor", "end")
            .attr("transform", "rotate(-90)")
            .attr("y", -margin.left+20)
            .attr("x", -margin.top)
            .text("Time (seconds)")

        Object.keys(this.props.profiles).forEach(profileType => {
            let profile = this.props.profiles[profileType];
            svgCanvas
                .append("path")
                .datum(profile)
                .attr("fill", "#cce5df")
                .attr("stroke", "none")
                .attr("d", d3.area()
                    .x(function (d) { return x(d.x) })
                    .y0(function (d) { return y(d.y0) })
                    .y1(function (d) { return y(d.y1) })
                )
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
        return <div className={styles.card} key={this.props.name}>
            <h3><a href={this.props.url}>{this.props.name}</a></h3>
            <div ref={this.state.myRef} />
        </div>
    }
}

export default LineChart