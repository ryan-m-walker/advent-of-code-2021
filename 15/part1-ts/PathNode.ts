import { Coord } from "./types"

export class PathNode {
  constructor(
    public value: number,
    public coords: Coord,
    public total: number = 0,
    public pervious?: PathNode
  ) {}

  static rewind(path: PathNode) {
    let total = path.value
    let node = path

    while (true) {
      console.log(node)
      const prev = node.pervious

      if (!prev) {
        return total
      }

      total += prev.value
      node = prev
    }
  }
}
