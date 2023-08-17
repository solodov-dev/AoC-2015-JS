const solution = require("./2-2");

test("Day 2 part 2", () => {
  expect(solution("2x3x4")).toBe(34);
  expect(solution("1x1x10")).toBe(14);
});
