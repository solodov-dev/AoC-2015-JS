BEGIN {FS = ""}
{
  x1 = 0
  y1 = 0

  x2 = 0
  y2 = 0

  for (i = 1; i <= NF; i+=2) {
    houses[x1 ("-" y1)] = true
    switch ($i) {
      case ">": 
        x1++
        break
      case "<": 
        x1--
        break
      case "^": 
        y1++
        break
      case "v": 
        y1--
    }

    houses[x2 ("-" y2)] = true
    switch ($(i+1)) {
      case ">": 
        x2++
        break
      case "<": 
        x2--
        break
      case "^": 
        y2++
        break
      case "v": 
        y2--
    }
  }

  print length(houses)
}
