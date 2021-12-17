const TypeId = {
  LITERAL_VALUE: "100",
}

type PacketReadResult = {
  data: any
  rest: string
  versionCount: number
  bitsRead: number
}

export class PacketReader {
  data: string
  pointer = 0

  constructor(input: string) {
    this.data = input
  }

  readPacket(): PacketReadResult {
    const { typeId, version } = this.readHeader()

    switch (typeId) {
      case TypeId.LITERAL_VALUE: {
        const data = this.readLiteralValue()
        return {
          data: parseInt(data, 2),
          rest: this.data.slice(this.pointer),
          versionCount: version,
          bitsRead: this.pointer,
        }
      }
      // all operator packets
      default: {
        const op = this.getOperator(typeId)
        const { data, versionCount } = this.readOperator()
        return {
          data: op(data),
          rest: this.data.slice(this.pointer),
          versionCount: versionCount + version,
          bitsRead: this.pointer,
        }
      }
    }
  }

  readHeader() {
    const version = parseInt(this.readBits(3), 2)
    const typeId = this.readBits(3)

    return { version, typeId }
  }

  readLiteralValue() {
    const data = []

    while (true) {
      const groupPrefix = this.readBit()
      const group = this.readBits(4)
      data.push(group)

      if (groupPrefix === "0") {
        break
      }
    }

    return data.join("")
  }

  readOperator() {
    const lengthTypeId = this.readBit()

    if (lengthTypeId === "0") {
      return this.readOperatorByBitLength()
    } else {
      return this.readOperatorByPacketCount()
    }
  }

  readOperatorByBitLength() {
    const bitLength = parseInt(this.readBits(15), 2)
    let packets = this.readBits(bitLength)

    const finalData = []
    let totalVersionCount = 0

    while (packets.length) {
      const reader = new PacketReader(packets)
      const { data, rest, versionCount, bitsRead } = reader.readPacket()
      finalData.push(data)

      totalVersionCount += versionCount
      packets = rest
    }

    return { data: finalData, versionCount: totalVersionCount }
  }

  readOperatorByPacketCount() {
    const packetCount = parseInt(this.readBits(11), 2)
    let packets = this.data.slice(this.pointer)
    const finalData = []
    let totalVersionCount = 0

    for (let i = 0; i < packetCount; i++) {
      const reader = new PacketReader(packets)
      const { data, rest, versionCount, bitsRead } = reader.readPacket()

      finalData.push(data)
      totalVersionCount += versionCount
      packets = rest
      this.pointer += bitsRead
    }

    return { data: finalData, versionCount: totalVersionCount, rest: "" }
  }

  readBits(count: number) {
    const bits = this.data.slice(this.pointer, this.pointer + count)
    this.pointer += count

    const padding = count - bits.length

    if (bits.length !== count) {
      throw new Error("unexpected end of input")
    }

    return bits
  }

  readBit() {
    const bit = this.data[this.pointer]
    this.pointer += 1

    if (!bit) {
      throw new Error("Unexpected end of input")
    }

    return bit
  }

  getOperator(id: string) {
    const idNum = parseInt(id, 2)

    switch (idNum) {
      case 0:
        return (data: number[]) => data.reduce((sum, n) => sum + n)
      case 1:
        return (data: number[]) => data.reduce((prod, n) => prod * n)
      case 2:
        return (data: number[]) => Math.min(...data)
      case 3:
        return (data: number[]) => Math.max(...data)
      case 5:
        return ([a, b]: number[]) => (a > b ? 1 : 0)
      case 6:
        return ([a, b]: number[]) => (a < b ? 1 : 0)
      case 7:
        return ([a, b]: number[]) => (a === b ? 1 : 0)
    }
  }
}
