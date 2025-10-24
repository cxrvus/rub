const letter_pairs = [
	['X','E'], ['Q','N'], ['I','R'], ['D','S'], ['T','I'], ['J','T'],
	['A','A'], ['C','D'], ['L','H'], ['G','U'], ['U','L'], ['N','C'],
	['W','G'], ['H','O'], ['E','M'], ['Y','B'], ['Z','W'], ['B','K'],
	['K','F'], ['S','P'], ['V','Z'], ['F','V'], ['M','J'], ['O','Y'],
	['P','Q'], ['R','X']
];

const path = process.argv[2];

if (!path) { console.log('please provide a path to the ciphertext'); return; }

const fs = require('fs');

let ciphertext = fs.readFileSync(path, 'utf-8');
let chars = [...ciphertext].map(char => ({ value: char, subst: false }));

letter_pairs.forEach(([cipher, plain]) => {
	chars.forEach(char => {
		if (!char.subst && char.value == cipher) {
			char.value = plain;
			char.subst = true;
		}
	})
});

const plaintext = chars.map(char => char.value).join('');

console.log(plaintext);
