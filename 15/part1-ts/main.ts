import fs from "fs"
import { RiskMap } from "./RiskMap"
import { PathNode } from "./PathNode"

const input = fs.readFileSync("./input.txt", "utf-8")

const map = new RiskMap(input)

let paths = map.getNeighbors([0, 0])

while (paths.length) {
  let bestPath = Infinity

  for (const path of paths) {
    const neighbors = map.getNeighbors(path.coords, path)

    for (const neighbor of neighbors) {
      if (neighbor.value < bestPath) {
        bestPath = neighbor.value
      }
    }
  }

  const newPaths = []

  for (const path of paths) {
    const neighbors = map.getNeighbors(path.coords, path)

    for (const neighbor of neighbors) {
      if (neighbor.value === bestPath) {
        newPaths.push(neighbor)
      }
    }
  }

  console.log({ bestPath })

  paths = newPaths
}

// while (paths.length) {
// console.log
// paths = getShortestPaths(paths)

// const path = paths.pop()

// const neighbors = map.getNeighbors(path!.coords, path)

// paths = [...paths, ...getShortestPaths(neighbors)]
// }

console.log(map.completedPaths)

// let lowestTotalRisk = Infinity

// for (const path of map.completedPaths) {
//   if (path.total < lowestTotalRisk) {
//     lowestTotalRisk = path.total
//   }
// }

// console.log("Answer =", lowestTotalRisk)

// function getShortestPaths(paths: PathNode[]): PathNode[] {
//   let shortest = Infinity

//   for (const path of paths) {
//     if (path.total < shortest) {
//       shortest = path.total
//     }
//   }

//   return paths.filter((p) => {
//     return p.total === shortest
//   })
// }

// // PathNode.rewind(map.completedPaths[0])
