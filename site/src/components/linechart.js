import * as d3 from "d3"
import parse from 'parse-duration'
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
        console.log(props);
        this.myRef = React.createRef();
        const x_axis = ["V1_34", "V1_35", "V1_36"];
        let one = props.data["release"]["V1_34"][0];
        console.log(one);
        console.log(parse(props.data["check"]["V1_34"][0]))
    }

    componentDidMount() {
        const data = [ 2, 4, 2, 6, 8 ]
        this.drawBarChart(data)
    }

    drawBarChart(data)  {
        var margin = {top: 10, right: 30, bottom: 30, left: 60},
        width = 460 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;
        const svgCanvas = d3.select(this.myRef.current)
            .append("svg")
            .attr("width", width + margin.left + margin.right)
            .attr("height", height + margin.top + margin.bottom)
            .append("g")
            .attr("transform",
              "translate(" + margin.left + "," + margin.top + ")");
        d3.scaleLinear().domain()
        
        console.log(this.props.data);
    }
    render() { return <div ref={this.myRef}/> }
}

export default LineChart