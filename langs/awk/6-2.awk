BEGIN { FS = " |," }
{
  gsub(/turn /, "")
  for (x = $2; x <= $5; x++) {
    for (y = $3; y <= $6; y++) {
      key = x ("-"y)
      switch ($1) {
        case "on": grid[key]++ ; break
        case "off": if (grid[key] > 0) grid[key]-- ; break
        case "toggle": grid[key] += 2 
      }
    }
  }
}
END {print sum(grid)}
