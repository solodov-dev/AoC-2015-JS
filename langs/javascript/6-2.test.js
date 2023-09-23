const solution = require("./6-2");

test("Day 6 part 2", () => {
  expect(solution("turn on 0,0 through 0,0")).toBe(1);
  expect(solution("toggle 0,0 through 999,999")).toBe(2000000);
});
