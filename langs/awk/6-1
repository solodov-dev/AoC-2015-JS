BEGIN { FS = " |," }
{
  gsub(/turn /, "")
  for (x = $2; x <= $5; x++) {
    for (y = $3; y <= $6; y++) {
      key = x ("-"y)
      switch ($1) {
        case "on": grid[key] = true ; break
        case "off": delete grid[key] ; break
        case "toggle": if (key in grid) { delete grid[key] } else { grid[key] = true }
      }
    }
  }
}
END {print length(grid)}


