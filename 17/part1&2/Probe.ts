import { Box } from "./types"
import { Vector } from "./Vector"

type Vec = {
  x: number
  y: number
}

export class Probe {
  pos = new Vector(0, 0)

  constructor(public target: Box, public vel: Vector) {}

  /**
   * returns max height if hit, else undefined
   */
  fire(): number | undefined {
    let highPoint = 0

    while (true) {
      if (this.isMiss()) {
        return
      }

      if (this.isHit()) {
        return highPoint
      }

      this.pos = this.pos.add(this.vel)

      // drag
      if (this.vel.x !== 0) {
        if (this.vel.x > 0) {
          this.vel.x -= 1
        } else {
          this.vel.x += 1
        }
      }

      // gravity
      this.vel.y -= 1

      if (this.pos.y > highPoint) {
        highPoint = this.pos.y
      }
    }
  }

  private isMiss(): boolean {
    if (this.pos.x > this.target.right) {
      return true
    }

    if (this.pos.y < this.target.bottom) {
      return true
    }

    return false
  }

  private isHit(): boolean {
    const isInLeftBound = this.pos.x >= this.target.left
    const isInRightBound = this.pos.x <= this.target.right
    const isInTopBound = this.pos.y <= this.target.top
    const isInBottomBound = this.pos.y >= this.target.bottom

    return isInLeftBound && isInRightBound && isInTopBound && isInBottomBound
  }
}
