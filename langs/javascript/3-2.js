const run = require("./utils/run");

module.exports = (input) => {
  const santa = { x: 0, y: 0 };
  const robot = { x: 0, y: 0 };
  const presentsSanta = new Set().add("0_0");
  const presentsRobot = new Set().add("0_0");
  for (let i = 0; i < input.length; i += 2) {
    move(input[i], santa);
    move(input[i + 1], robot);
    presentsSanta.add(`${santa.x}_${santa.y}`);
    presentsRobot.add(`${robot.x}_${robot.y}`);
  }
  return new Set([...presentsSanta, ...presentsRobot]).size;
};

const move = (dir, point) => {
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
};

run(module.exports);
