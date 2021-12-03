import fs from "fs"

type Element = "oxygen" | "c02"

const input = fs.readFileSync("input.txt", "utf-8").split("\n")
const numLength = input[0].length

let oxygenRating = input
let c02 = input

for (let i = 0; i < numLength; i++) {
  if (oxygenRating.length > 1) {
    oxygenRating = filter(oxygenRating, i, "oxygen")
  }

  if (c02.length > 1) {
    c02 = filter(c02, i, "c02")
  }
}

const parsedOxygen = parseInt(Number(oxygenRating[0]) + "", 2)
const parsedC02 = parseInt(Number(c02[0]) + "", 2)

console.log("Answer =", parsedOxygen * parsedC02)

function filter(numbers: string[], index: number, element: Element) {
  const hasZero = []
  const hasOne = []

  for (const num of numbers) {
    const bit = num[index]
    if (bit === "0") {
      hasZero.push(num)
    } else {
      hasOne.push(num)
    }
  }

  if (element === "oxygen") {
    return hasZero.length > hasOne.length ? hasZero : hasOne
  } else {
    const res = hasZero.length > hasOne.length ? hasOne : hasZero
    return res
  }
}
