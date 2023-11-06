BEGIN {FS=" |( -> )"}
NF == 2 {action[$2] = "ID"; op1[$2] = $1}
NF == 3 {action[$3] = $1; op1[$3] = $2}
NF == 4 {action[$4] = $2; op1[$4] = $1; op2[$4] = $3 }
END { print get("a") }

function get(label) {
  if (label ~ /[0-9]/) return label
  if (action[label] !~ /[0-9]/) {
    wire = action[label]
    if (wire == "ID")
      return action[label] = get(op1[label])
    if (wire == "NOT")
      return action[label] = compl(get(op1[label]))
    if (wire == "AND")
      return action[label] = and(get(op1[label]), get(op2[label]))
    if (wire == "OR")
      return action[label] = or(get(op1[label]), get(op2[label]))
    if (wire == "RSHIFT")
      return action[label] = rshift(get(op1[label]), get(op2[label]))
    if (wire == "LSHIFT")
      return action[label] = lshift(get(op1[label]), get(op2[label]))
  }
  return action[label]
}
