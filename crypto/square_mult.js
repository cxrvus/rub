let n = process.argv[2] * 1;
let e = process.argv[3] * 1;
let m = process.argv[4] * 1;

let x_i = n;
let e_i = 1;

let ops = 0;
let anchor = false;

for (let i = 31; i >= 0; i--) {
	let mult = (e & (1 << i)) != 0;

	if (!anchor) {
		anchor = mult;
		continue;
	}

	let factor = mult ? n : 1;

	x_i = (x_i * x_i * factor) % m;
	e_i = e_i * 2 + mult * 1;

	let op = mult ? 'SQ + MUL' : 'SQ'
	console.log(`${ops}:\t${n} ^ ${e_i} = \t${x_i}\t${op}`);
	//console.log({mult, factor, x_i, e_i})

	if (e_i >= e) break;

	ops++;
}
