const solution = require("./7-1");

const testInput = `123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> a
NOT x -> h
NOT y -> i`;

const testOutput = 114;

test.only("Day 7 part 1", () => {
  expect(solution(testInput)).toBe(testOutput);
});
