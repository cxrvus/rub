const letter_pairs = [
	['X','E'], ['Q','N'], ['I','I'], ['D','S'], ['T','R'], ['J','A'],
	['A','T'], ['C','D'], ['L','H'], ['G','U'], ['U','L'], ['N','C'],
	['W','G'], ['H','M'], ['E','O'], ['Y','B'], ['Z','W'], ['B','F'],
	['K','K'], ['S','Z'], ['V','P'], ['F','V'], ['M','J'], ['O','Y'],
	['P','X'], ['R','Q']
];

const path = process.argv[2];

if (!path) { console.log('please provide a path to the ciphertext'); return; }

const fs = require('fs');

let ciphertext = fs.readFileSync(path, 'utf-8');

letter_pairs.forEach(([cipher, plain]) =>
	ciphertext = ciphertext.replaceAll(cipher, plain)
);

console.log(ciphertext);
