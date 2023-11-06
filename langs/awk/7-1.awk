BEGIN {FS=" |( -> )"}
NF == 2 {action[$2] = "ID"; op1[$2] = $1}
NF == 3 {action[$3] = $1; op1[$3] = $2}
NF == 4 {action[$4] = $2; op1[$4] = $1; op2[$4] = $3 }
END { print get("a") }

function get(label) {
  if (isNumber(label)) return label
  if (!isNumber(action[label])) {
    wire = action[label]
    if (wire == "ID")
        res = get(op1[label])
    if (wire == "NOT")
      res = and(compl(get(op1[label])), 65535)
    if (wire == "AND")
      res = and(get(op1[label]), get(op2[label]))
    if (wire == "OR")
      res = or(get(op1[label]), get(op2[label]))
    if (wire == "RSHIFT")
      res = rshift(get(op1[label]), get(op2[label]))
    if (wire == "LSHIFT")
      res = lshift(get(op1[label]), get(op2[label]))
    action[label] = res
  }
  return action[label]
}


function isNumber(value) {
 return (value==value+0)
}

