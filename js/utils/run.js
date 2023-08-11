async function run(callback) {
  let data = "";
  for await (const chunk of process.stdin) data += chunk;
  const solution = callback(data);
  process.stdout.write(solution.toString());
}

module.exports = run;
