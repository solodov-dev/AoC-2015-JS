const { run, lines } = require("./utils");

module.exports = (input) => {
  const grid = new Array(1000).fill(null).map(() => new Array(1000).fill(false));
  for (const line of lines(input)) {
    const [command, from, to] = line.replace("turn ", "").replace("through ", "").split(" ");
    const [fromX, fromY, toX, toY] = [from, to].join(",").split(",").map(Number);
    for (let i = fromX; i <= toX; i++) {
      for (let j = fromY; j <= toY; j++) {
        switch (command) {
          case "on": grid[i][j] = true; break;
          case "off": grid[i][j] = false; break;
          case "toggle": grid[i][j] = !grid[i][j];
        }
      }
    }
  }
  return grid.flat().filter(Boolean).length;
};

run(module.exports);
