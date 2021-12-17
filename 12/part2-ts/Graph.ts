enum NodeType {
  Start = "start",
  End = "end",
  Large = "large",
  Small = "small",
}

type TreeNode = {
  value: string
  children: TreeNode[]
  visited: Visited
}

type Visited = {
  values: string[]
  hasVisitedSmall: boolean
}

export class Graph {
  nodes: Record<string, NodeType> = {}
  edges: string[][] = []
  pathCount = 0

  constructor(input: string) {
    const lines = input.trim().split("\n")

    for (const line of lines) {
      const [left, right] = line.split("-")
      this.nodes[left] = getNodeType(left)
      this.nodes[right] = getNodeType(right)
      this.edges.push([left, right])
    }
  }

  findAllPaths() {
    return this.walk("start")
  }

  private walk(
    node: string,
    prevVisited: Visited = { hasVisitedSmall: false, values: [] }
  ): TreeNode {
    const visited = pushVisited(prevVisited, node)

    if (node === "end") {
      this.pathCount += 1
      return {
        value: node,
        children: [],
        visited,
      }
    }

    return {
      value: node,
      children: this.findConnections(node)
        .filter((o) => {
          if (o === "start") {
            return !visited.values.includes("start")
          }

          if (o === "end") {
            return !visited.values.includes("end")
          }

          if (visited.values.includes(o)) {
            if (visited.hasVisitedSmall) {
              return false
            } else {
              return true
            }
          }

          return true
        })
        .map((o) => this.walk(o, visited)),
      visited,
    }
  }

  private findConnections(value: string) {
    const edges = this.findEdges(value)
    return edges.map(([left, right]) => (left === value ? right : left))
  }

  private findEdges(value: string) {
    return this.edges.filter(
      ([left, right]) => left === value || right === value
    )
  }
}

function getNodeType(value: string) {
  if (value === "start") {
    return NodeType.Start
  }

  if (value === "end") {
    return NodeType.End
  }

  if (startsWithCapital(value)) {
    return NodeType.Large
  }

  return NodeType.Small
}

function startsWithCapital(word: string) {
  return word.charAt(0) === word.charAt(0).toUpperCase()
}

function pushVisited(visitedData: Visited, value: string): Visited {
  const visited = [...visitedData.values]

  if (startsWithCapital(value)) {
    return {
      ...visitedData,
    }
  }

  if (value === "start") {
    if (visited.includes("start")) {
      return {
        ...visitedData,
      }
    }

    return {
      ...visitedData,
      values: [...visited, "start"],
    }
  }

  if (value === "end") {
    if (visited.includes("end")) {
      return { ...visitedData }
    }

    return {
      ...visitedData,
      values: [...visited, "end"],
    }
  }

  if (!visited.includes(value)) {
    return {
      ...visitedData,
      values: [...visited, value],
    }
  } else {
    if (visitedData.hasVisitedSmall) {
      return {
        ...visitedData,
      }
    } else {
      return {
        ...visitedData,
        hasVisitedSmall: true,
      }
    }
  }
}

function hasDuplicates(array: string[]) {
  return new Set(array).size !== array.length
}
