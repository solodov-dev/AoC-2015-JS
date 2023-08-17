const solution = require("./1-2");

test("Day 1 part 2", () => {
  expect(solution(")")).toBe(1);
  expect(solution("()())")).toBe(5);
});
