const run = require("./utils/run");

module.exports = (input) => {
  const has_bads = /(ab|cd|pq|xy)/;
  const has_double = /(.)\1/;
  const three_vowels = /^(.*[aeuio].*){3,}$/;

  return (
    !has_bads.test(input) && has_double.test(input) && three_vowels.test(input)
  );
};

run(module.exports);
