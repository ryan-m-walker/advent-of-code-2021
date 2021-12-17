import { PathNode } from "./PathNode"
import { Coord } from "./types"

function doCoordsEqual([x1, y1]: Coord, [x2, y2]: Coord) {
  return x1 == x2 && y1 == y2
}

export class RiskMap {
  data: number[][]
  completedPaths: PathNode[] = []

  constructor(input: string) {
    this.data = input
      .trim()
      .split("\n")
      .map((line) => line.split("").map(Number))
  }

  getFinishingCoord(): Coord {
    const rows = this.data.length - 1
    const cols = this.data[0].length - 1

    return [cols, rows]
  }

  get([x, y]: Coord): number {
    return this.data[y]?.[x]
  }

  getNeighbors([x, y]: Coord, pathNode?: PathNode) {
    const neighbors: PathNode[] = []

    const rightCoord: Coord = [x + 1, y]
    const downCoord: Coord = [x, y + 1]

    const finishCoord = this.getFinishingCoord()
    const finishValue = this.get(finishCoord)
    const finishTotal = (pathNode?.total ?? 0) + finishValue

    if (doCoordsEqual(rightCoord, finishCoord)) {
      this.completedPaths.push(
        new PathNode(finishValue, finishCoord, finishTotal, pathNode)
      )
    } else {
      const right = this.get(rightCoord)
      if (right) {
        const rightTotal = (pathNode?.total ?? 0) + right
        neighbors.push(new PathNode(right, rightCoord, rightTotal, pathNode))
      }
    }

    if (doCoordsEqual(downCoord, finishCoord)) {
      this.completedPaths.push(
        new PathNode(finishValue, finishCoord, finishTotal, pathNode)
      )
    } else {
      const down = this.get(downCoord)
      if (down) {
        const downTotal = (pathNode?.total ?? 0) + down
        neighbors.push(new PathNode(down, downCoord, downTotal, pathNode))
      }
    }

    return neighbors
  }
}
