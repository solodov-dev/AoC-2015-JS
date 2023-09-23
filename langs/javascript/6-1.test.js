const solution = require("./6-1");

test("Day 6 part 1", () => {
  expect(solution("turn on 0,0 through 999,999")).toBe(1000*1000);
  expect(solution("turn on 499,499 through 500,500")).toBe(4);
  expect(solution("toggle 0,0 through 999,0")).toBe(1000);
});
