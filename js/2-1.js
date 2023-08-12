const {run, lines} = require('./utils');

module.exports = (input) => {
	const [l, w, h] = input.split('x').map(Number);
	const [s1, s2, s3] = [l*w, w*h, h*l];
	return 2 * (s1 + s2 + s3) + Math.min(s1, s2, s3);
}

run((input) => lines(input).reduce((acc, cur) => module.exports(cur) + acc, 0));