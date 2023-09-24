BEGIN {FS = ""}
{
  x = 0
  y = 0

  for (i = 1; i <= NF; i++) {
    houses[x ("-"y)] = true
    switch ($i) {
      case ">": 
        x++
        break
      case "<": 
        x--
        break
      case "^": 
        y++
        break
      case "v": 
        y--
    }
  }

  print length(houses)
}
