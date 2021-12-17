import * as fs from "fs"
import { PacketReader } from "./PacketReader"
import { parseHex } from "./parseHex"

const input = fs.readFileSync("./input.txt", "utf-8")
const binary = parseHex(input)

fs.writeFileSync("binary", binary, "utf-8")

const reader = new PacketReader(binary)
const { versionCount } = reader.readPacket()
console.log("->", versionCount)
