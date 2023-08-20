const run = require("./utils/run");

module.exports = (input) => {
  let has_pair = /.*(\w\w).*\1/;
  let has_middle = /.*(\w).\1/;

  return has_pair.test(input) && has_middle.test(input);
};

run(module.exports);
