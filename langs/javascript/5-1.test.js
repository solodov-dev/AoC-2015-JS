const solution = require("./5-1");

test("Day 5 part 1", () => {
  expect(solution("ugknbfddgicrmopn")).toBe(true);
  expect(solution("aaa")).toBe(true);
  expect(solution("jchzalrnumimnmhp")).toBe(false);
  expect(solution("haegwjzuvuyypxyu")).toBe(false);
  expect(solution("dvszwmarrgswjxmb")).toBe(false);
});
