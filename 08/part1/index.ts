import fs from "fs"
import path from "path"

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8")
const data = parse(input)

let total = 0

for (const { output } of data) {
  for (const digit of output) {
    if (hasEasyDigit(digit)) {
      console.log(digit)
      total += 1
    }
  }
}

console.log("Answer =", total)

function hasEasyDigit(digit: string) {
  if ([7, 4, 3, 2].includes(digit.length)) {
    return true
  }

  return false
}

function parse(input: string) {
  const lines = input.split("\n")
  return lines.map((line) => {
    const [left, right] = line.split(" | ")

    return {
      patterns: left.split(" ").map((x) => x.trim()),
      output: right.split(" ").map((x) => x.trim()),
    }
  })
}
