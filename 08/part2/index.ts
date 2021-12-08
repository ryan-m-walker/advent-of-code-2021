import fs from "fs"
import path from "path"

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8")
const data = parse(input)

let total = 0

const confirmed = {}

for (const { patterns, output } of data) {
  const one = patterns.find((p) => p.length === 2)
  const four = patterns.find((p) => p.length === 4)
  const seven = patterns.find((p) => p.length === 3)
  const eight = patterns.find((p) => p.length === 7)

  confirmed["a"] = getUnique(one, seven)
  const nine = patterns.find((p) => match(p, four + confirmed["a"]))

  confirmed["g"] = getUnique(nine, four + confirmed["a"])

  const three = patterns.find((p) =>
    match(p, one + confirmed["a"] + confirmed["g"])
  )

  confirmed["d"] = getUnique(three, one + confirmed["a"] + confirmed["g"])

  const five = patterns.find((p) => {
    if (p.length !== nine.length - 1) return false
    if (p === three) return false

    const unique = getUnique(p, nine)
    if (unique.length > 1) return false

    return true
  })

  confirmed["c"] = getUnique(five, nine)

  const two = patterns.find((p) => {
    if (p.length !== 5) return false
    if (p === three) return false
    if (p === five) return false
    return true
  })

  confirmed["f"] = getUnique(two, three)
  confirmed["e"] = getUnique(three, two)

  const six = patterns.find((p) =>
    match(
      p,
      confirmed["a"] +
        confirmed["d"] +
        confirmed["g"] +
        confirmed["e"] +
        confirmed["f"]
    )
  )

  const zero = patterns.find((p) => {
    if (p.length !== 6) return false
    if (p === nine) return false
    if (p === six) return false

    return true
  })

  confirmed["b"] = getUnique(eight, Object.values(confirmed).join(""))

  const numbers = {
    [sort(zero)]: 0,
    [sort(one)]: 1,
    [sort(two)]: 2,
    [sort(three)]: 3,
    [sort(four)]: 4,
    [sort(five)]: 5,
    [sort(six)]: 6,
    [sort(seven)]: 7,
    [sort(eight)]: 8,
    [sort(nine)]: 9,
  }

  let outputSignal = ""

  for (const item of output) {
    const number = numbers[sort(item)]
    outputSignal += String(number)
  }

  total += Number(outputSignal)
}

console.log("Answer =", total)

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

function getUnique(a: string, b: string) {
  const aArray = a.split("")
  const bArray = b.split("")

  const baseArray = aArray.length > bArray.length ? aArray : bArray
  const otherArray = aArray.length > bArray.length ? bArray : aArray

  return baseArray.filter((item) => !otherArray.includes(item)).join("")
}

function match(pattern: string, toMatch: string) {
  if (pattern.length !== toMatch.length + 1) {
    return false
  }

  for (const char of toMatch) {
    if (!pattern.includes(char)) {
      return false
    }
  }

  return true
}

function getUniqueByCount(a: string, b: string, n: number) {
  if (a === b) {
    return false
  }

  if (a.length !== b.length) {
    return false
  }

  return getUnique(a, b).length === n
}

function findMatch(numbers: { code: number }, toMatch: string) {
  const sorted = toMatch
}

function sort(input: string) {
  return input.split("").sort().join("")
}
