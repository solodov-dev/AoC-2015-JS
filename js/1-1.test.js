const solution_part_1 = require("./1-1");
it("Test input, part 1", () => {
  expect(solution_part_1("(())") === solution_part_1("()()"));
  expect(solution_part_1("(((") === solution_part_1("(()(()("));
  expect(solution_part_1("))(((((") === 3);
  expect(solution_part_1("())") === solution_part_1("))("));
  expect(solution_part_1(")))") === solution_part_1(")())())"));
});
