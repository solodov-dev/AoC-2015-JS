const solution = require("./5-2");

test("Day 5 part 2", () => {
  expect(solution("qjhvhtzxzqqjkmpb")).toBe(true);
  expect(solution("xxyxx")).toBe(true);
  expect(solution("uurcxstgmygtbstg")).toBe(false);
  expect(solution("ieodomkazucvgmuy")).toBe(false);
});
