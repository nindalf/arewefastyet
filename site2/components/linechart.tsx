import * as d3 from "d3"
import styles from '../styles/Home.module.css'
import React, { Component } from "react"

// const data = function(){
//     // massage our data into this
//     const data = Results["helloworld"]["check"];
//     const x_axis = ["V1_34", "V1_35", "V1_36"]
//     return {
//         y: "Compile time",
//         series: [{
//             name: "Ripgrep check",
//             values: x_axis.map(x => data[x])
//         }],
//         dates: x_axis
//     };
// }


class LineChart extends Component {
    constructor(props) {
        super(props);
        this.state = {
            myRef: React.createRef(),
            name: this.props.perfData['repo']['name'],
            url: this.props.perfData['repo']['url']
        }
    }

    componentDidMount() {
        // this.setState();
        
        const data = [2, 4, 2, 6, 8]
        this.drawBarChart(data)
    }

    drawBarChart(data) {
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
        d3.scaleLinear().domain()

    }
    render() {
        return <a href={this.state.url} className={styles.card} key={this.state.name}>
            <h3>{this.state.name}</h3>
            <p>Find in-depth information about this repo</p>
        </a>
    }
}

export default LineChart