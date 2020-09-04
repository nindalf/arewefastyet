import React from "react"
import Results from "../../static/results.json"
import LineChart from "../components/linechart";


export default function Home() {
  // console.log(Results);
  return <LineChart data={Results.ripgrep}/>;
}
