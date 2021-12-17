import { getTarget } from "./getTarget"
import { Probe } from "./Probe"
import { Box } from "./types"
import { Vector } from "./Vector"

const target = getTarget()

let hits = 0
let maxHeight = 0
let bestVel

for (let i = 0; i < target.right + 1; i++) {
  for (let j = target.bottom; j < Math.abs(target.bottom); j++) {
    const vel = new Vector(i, j)
    const copy = vel.copy()
    const probe = new Probe(target, vel)
    const height = probe.fire()

    if (height !== undefined) {
      hits += 1
    }

    if (height && height > maxHeight) {
      maxHeight = height
      bestVel = copy
    }
  }
}

console.log("Best Vel =", bestVel)

// Part 1
console.log("Highest Point =", maxHeight)

// Part 2
console.log("hits =", hits)
