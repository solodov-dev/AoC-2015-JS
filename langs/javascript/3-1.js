const run = require("./utils/run");

module.exports = (input) => {
  const point = { x: 0, y: 0 };
  const presents = new Set().add("0_0");
  input.split("").forEach((dir) => {
    switch (dir) {
      case ">":
        point.y++;
        break;
      case "<":
        point.y--;
        break;
      case "^":
        point.x++;
        break;
      case "v":
        point.x--;
        break;
    }
    presents.add(`${point.x}_${point.y}`);
  });
  return presents.size;
};

run(module.exports);
