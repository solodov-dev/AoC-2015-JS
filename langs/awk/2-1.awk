BEGIN { FS = "x" }
{ 
  s[0] = $1 * $2
  s[1] = $2 * $3
  s[2] = $1 * $3
  len += 2 * sum(s) + min(s)
}
END { print len }
