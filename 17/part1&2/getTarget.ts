import { Box } from "./types"

export function getTarget(useTestDate = false): Box {
  if (useTestDate) {
    return {
      left: 20,
      right: 30,
      top: -5,
      bottom: -10,
    }
  }

  return {
    left: 155,
    right: 182,
    top: -67,
    bottom: -117,
  }
}
