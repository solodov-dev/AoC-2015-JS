const solution = require("./3-1");

test("Day 3 part 1", () => {
  expect(solution(">")).toBe(2);
  expect(solution("^>v<")).toBe(4);
  expect(solution("^v^v^v^v^v")).toBe(2);
});