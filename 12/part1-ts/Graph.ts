enum NodeType {
  Start = "start",
  End = "end",
  Large = "large",
  Small = "small",
}

type TreeNode = {
  value: string
  children: TreeNode[]
  visited: string[]
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

  private walk(node: string, prevVisited: string[] = []): TreeNode {
    const visited = [...prevVisited]

    if (!startsWithCapital(node)) {
      visited.push(node)
    }

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
        .filter((o) => !prevVisited.includes(o))
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
