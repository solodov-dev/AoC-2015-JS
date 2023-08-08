const solution_part_1 = require("./part_1");
const solution_part_2 = require("./part_2");

describe("Day 1", () => {
  it("Test input, part 1", () => {
    expect(solution_part_1("(())") === solution_part_1("()()"));
    expect(solution_part_1("(((") === solution_part_1("(()(()("));
    expect(solution_part_1("))(((((") === 3);
    expect(solution_part_1("())") === solution_part_1("))("));
    expect(solution_part_1(")))") === solution_part_1(")())())"));
  });

  it("Test input, part 2", () => {
    expect(solution_part_2(")") === 1);
    expect(solution_part_2("()())") === 5);
  });
});
