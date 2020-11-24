export function average(a: Array<number>): number {
    const sum = a.reduce((a, b) => a+b,0)
    return sum/a.length
}

function standardDeviation(a: Array<number>): number {
    const mean = average(a)
    let temp = a.map(x => Math.pow((x - mean), 2)).reduce((a, b) => a+b,0)
    return Math.pow(temp/a.length, 0.5)
}

function confidenceIntervals(a: Array<number>): [number, number] {
    let Z = 1.96
    let x = average(a)
    let s = standardDeviation(a)
    let step = Z * s / Math.pow(a.length, 0.5)
    return [x - step, x + step]
}
