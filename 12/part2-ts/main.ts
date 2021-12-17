import * as fs from "fs"
import { Graph } from "./Graph"

const input = fs.readFileSync("input.txt", "utf-8")
const graph = new Graph(input)
const tree = graph.findAllPaths()

// console.log(JSON.stringify(tree, null, 2))
console.log(graph.pathCount)
