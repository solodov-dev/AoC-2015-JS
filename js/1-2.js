#!/usr/bin/env node

const run = require("./utils/run");

const solution_part_2 = (input) => {
  let sum = 0;
  let i = 0;

  while (sum >= 0) {
    input[i] === "(" ? sum++ : sum--;
    i++;
  }

  return i;
};

run(solution_part_2);

module.exports = solution_part_2;
