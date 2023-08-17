const solution = require("./1-1");
test("Day 1 part 1", () => {
  expect(solution("(())")).toBe(solution("()()"));
  expect(solution("(((")).toBe(solution("(()(()("));
  expect(solution("))(((((")).toBe(3);
  expect(solution("())")).toBe(solution("))("));
  expect(solution(")))")).toBe(solution(")())())"));
});
