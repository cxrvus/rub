// pre-defined

let K = 2 ** 128; 				// possible keys
let ASIC_speed = 7 * 10 ** 8; 	// keys per second
let budget = 1000000;			// euros
let ASIC_price = 80; 			// 40 * 2, due to overhead
let universe_age = 10 ** 10;	// years

// obtained

let ASIC_count = budget / ASIC_price;								// ASICs
let search_duration_sec = K / (ASIC_count * ASIC_speed);			// seconds
let search_duration = search_duration_sec / (60 * 60 * 24 * 365);	// years

let relative_duration = search_duration / universe_age;

console.log(`- ${ASIC_count} ASICs\n- ${search_duration} years (${relative_duration} times the age of the universe)`);
