import React from "react"
import logo from "./logo.svg"
import "./App.css"
import { input } from "./input"
import { Tooltip } from "@chakra-ui/react"

function getColor(value: number) {
  switch (value) {
    case 9:
      return "#999"
    case 8:
      return "#888"
    case 7:
      return "#777"
    case 6:
      return "#666"
    case 5:
      return "#555"
    case 4:
      return "#444"
    case 3:
      return "#333"
    case 2:
      return "#222"
    case 1:
      return "#111"
    case 0:
      return "#000"
    default:
      return "#000000"
  }
}

function App() {
  const rows = input.split("\n")

  return (
    <div className="App">
      <div className="wrapper">
        {rows.map((row, rowIndex) => {
          return (
            <div className="row" key={"row-" + rowIndex}>
              {row.split("").map((col, colIndex) => {
                return (
                  <div
                    data-coord={`(${colIndex}, ${rowIndex})`}
                    title={`(${colIndex}, ${rowIndex})`}
                    style={{
                      backgroundColor: getColor(Number(col)),
                      color: Number(col) < 5 ? "#fff" : "#000",
                    }}
                    className="col"
                    key={`col-${rowIndex}-${colIndex}`}
                  >
                    {col}
                  </div>
                )
              })}
            </div>
          )
        })}
      </div>
    </div>
  )
}

export default App
