const { createHash } = require("crypto");
const run = require("./utils/run");

module.exports = (input) => {
  let c = -1;
  let res;

  do {
    c++;
    res = createHash("md5").update(`${input}${c.toString()}`).digest("hex");
  } while (!res.startsWith("000000"));

  return c;
};

run(module.exports);
