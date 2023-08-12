const run = require("./utils/run");

module.exports = (input) => {
  let sum = 0;
  let i = 0;

  while (sum >= 0) {
    input[i] === "(" ? sum++ : sum--;
    i++;
  }

  return i;
};

run(module.exports);