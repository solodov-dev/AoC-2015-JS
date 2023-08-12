const solution = require("./1-1");
it("Test input, day 1 part 1", () => {
  expect(solution("(())") === solution("()()"));
  expect(solution("(((") === solution("(()(()("));
  expect(solution("))(((((") === 3);
  expect(solution("())") === solution("))("));
  expect(solution(")))") === solution(")())())"));
});
