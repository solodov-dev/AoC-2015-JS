const solution_part_2 = require("./1-2");

it("Test input, part 2", () => {
  expect(solution_part_2(")") === 1);
  expect(solution_part_2("()())") === 5);
});
