const fs = require("fs");

const args = process.argv.slice(2);
if (args.length < 2)
  throw new Error("Not enough arguments. Usage: yarn start [day] [part]");

const [day, part] = [...args];

async function run() {
  console.log(`Solution for day ${day} - part ${part}`);
  const input = fs.readFileSync(__dirname + `/../input/day_${day}`, "utf-8");
  const runner = await import(`./day_${day}/part_${part}.js`);
  console.log(runner.default(input));
}

run();
