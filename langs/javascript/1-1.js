const run = require("./utils/run");

module.exports = (input) =>
  input.split("").reduce((acc, cur) => (cur === "(" ? acc + 1 : acc - 1), 0);

run(module.exports);
