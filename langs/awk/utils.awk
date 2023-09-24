function sum(arr) {
  acc = 0
  for (i in arr)
    acc += arr[i]
  return acc
}

function product(arr) {
  acc = 1
  for (i in arr)
    acc *= arr[i]
  return acc
}

function min(arr) {
  result = arr[0]
  for (i in arr) {
    if (arr[i] < result)
      result = arr[i]
  }
  return result
}

function max(arr) {
  result = arr[0]
  for (i in arr) {
    if (arr[i] > result)
      result = arr[i]
  }
  return result
}

