const {run, lines} = require('./utils');

module.exports = (input) => {
	const [l, w, h] = input.split('x').map(Number);
	const perimeters = [l+w, w+h, h+l].map(p => p*2);
	return Math.min(...perimeters) + l * w * h;
}

run((input) => lines(input).reduce((acc, cur) => module.exports(cur) + acc, 0));