const ciphertext = 'LFK NDP, VDK XQG VLHJWH.';

const alphabet = [...'abcdefghijklmnopqrstuvwxyz'];

const cipher_values = [...ciphertext.toLowerCase()]
	.filter(char => alphabet.includes(char))
	.map(char => alphabet.indexOf(char))
;

const m = alphabet.length;

for (let k = 0; k < m; k++) {
	const plaintext = cipher_values
		.map(x => Math.abs(x - k % m))
		.map(x => alphabet[x])
		.join('')
	;

	console.log(`${k}:  ${plaintext}`);
}
