const solution = require("./1-2");

it("Test input, day 1 part 2", () => {
  expect(solution(")")).toBe(1);
  expect(solution("()())")).toBe(5);
});
