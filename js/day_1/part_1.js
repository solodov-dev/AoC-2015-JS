module.exports = (input) => {
  return input
    .split("")
    .reduce((acc, cur) => (cur === "(" ? acc + 1 : acc - 1), 0);
};
