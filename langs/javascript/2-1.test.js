const solution = require("./2-1");

test("Day 2 part 1", () => {
  expect(solution("2x3x4")).toBe(58);
  expect(solution("1x1x10")).toBe(43);
});