const ciphertext = 'HWHWZB';
const key = 'BSASRP';

const alphabet = [...'abcdefghijklmnopqrstuvwxyz'];

const getValues = chars => chars
	.filter(char => alphabet.includes(char))
	.map(char => alphabet.indexOf(char))
;

const cipherValues = getValues([...ciphertext.toLowerCase()]);
const keyValues = getValues([...key.toLowerCase()]);

const plaintext = cipherValues
	.map((c,i) => c ^ keyValues[i])
	.map(p => alphabet[p])
	.join('')
;

console.log(plaintext);
