export class Vector {
  constructor(public x: number, public y: number) {}

  add(other: Vector) {
    return new Vector(this.x + other.x, this.y + other.y)
  }

  copy() {
    return new Vector(this.x, this.y)
  }
}
