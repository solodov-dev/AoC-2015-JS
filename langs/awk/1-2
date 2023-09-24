BEGIN { FS="" }
{
  i = 1
  sum = 0
  while (sum >= 0) {
    sum += $i == "(" ? 1 : -1
    i++
  }
  print i
}
