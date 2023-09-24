BEGIN { FS = "x" }
{ 
  p[0] = 2 * ($1 + $2)
  p[1] = 2 * ($2 + $3)
  p[2] = 2 * ($1 + $3)

  for (i = 1; i <= NF; i++) {
    sides[i] = $i
  }
  
  len += min(p) + product(sides)
}
END { print len }
