const solution = require("./3-2");

it("Test input, day 3 part 2", () => {
  expect(solution("^v")).toBe(3);
  expect(solution("^>v<")).toBe(3);
  expect(solution("^v^v^v^v^v")).toBe(11);
});
