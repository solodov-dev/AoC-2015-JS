#!/usr/bin/env node
const run = require("./utils/run");

const solution_part_1 = (input) => {
  return input
    .split("")
    .reduce((acc, cur) => (cur === "(" ? acc + 1 : acc - 1), 0);
};

run(solution_part_1);

module.exports = solution_part_1;
