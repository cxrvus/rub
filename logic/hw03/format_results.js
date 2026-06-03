const fs = require('fs');

// @ts-ignore
const path = process.argv[2];
const file_content = fs.readFileSync(path, 'utf-8').trim();

const groups = file_content.split('\n');

const formatted_groups = groups.map(group => {
	const x = group.slice(0, 2);
	const y = group.slice(2, 4);
	const z = group.slice(4, 6);

	return `A_{${x}, ${y}, ${z}}`
}).join(",\n");

const formatted_set = `L = \\set{${formatted_groups}`

console.log(formatted_set);
