const gpu_rate = 16.2 * 10**7;
const fpga_rate = 10.5 * 10**8;

const key_sizes = [56, 128, 256];

const get_all_tries = size => 2 ** size;
const get_avg_tries = size => 2 ** (size-1);

const all_tries = key_sizes.map(size => get_all_tries(size));
const avg_tries = key_sizes.map(size => get_avg_tries(size));

const secs_to_years = secs => secs / (60 * 60 * 24 * 365);
const get_search_dur = (rate, tries) => secs_to_years(tries / rate);
const print_search_dur = (target, rate, tries, bits) => {
	console.log(`${target} | ${bits} bits | ${get_search_dur(rate, tries)}`);
}

const test_hardware = try_counts => 
	try_counts.forEach((try_count, i) => {
		print_search_dur('GPU ', gpu_rate, try_count, key_sizes[i]);
		print_search_dur('FPGA', fpga_rate, try_count, key_sizes[i]);
	})
;

console.log('Worst Case:');
test_hardware(all_tries)

console.log();
console.log('Average Case:');
test_hardware(avg_tries);
