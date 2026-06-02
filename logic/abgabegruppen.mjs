import { readFileSync } from 'fs'

const file_name = process.argv[2]
const file_content = readFileSync(file_name, 'utf-8').trim()

let output = parse_graph(file_content)
console.log(output)

/**
 * @param {string} string
 */
function parse_graph(string) {
	let lines = string.split('\n')
	const first_line = lines.shift();

	// @ts-ignore
	const [_p, _edge, node_count, edge_count] = first_line.split(' ') 
	
	const edges = lines.map(line => {
		const [_, from, to] = line.split(' ').map(s => parseInt(s))
		return [from, to]
	})

	return { node_count, edge_count, edges }
}

/**
 * @param {number} x
 * @param {number} y
 * @param {number} z
 */
function group(x, y, z) {
	return [x, y, z]
}

function clause(disjunctions) {
	return { disjunctions }
}

function formula(conjunctions) {
	return { conjunctions }
}
