// pre-defined

let K = 2 ** 128; 				// possible keys
let ASIC_speed = 7 * (10 ** 8); // keys per second
let budget = 1000000;			// euros
let ASIC_price = 80; 			// 40 * 2, due to overhead
let universe_age = 10 ** 10;	// years

// obtained

let ASIC_count = budget / ASIC_price;								// ASICs

function search_duration_sec(speed) {
	return K / (ASIC_count * speed);
}

let search_duration = search_duration_sec(ASIC_speed) / (60 * 60 * 24 * 365);	// years

let relative_duration = search_duration / universe_age;

console.log(`- ${ASIC_count} ASICs\n- ${search_duration} years (${relative_duration} times the age of the universe)`);

let months_passed = 0;

// Moore's Law
for (let i = 0; i < 100; i++) {
	let secs = search_duration_sec(ASIC_speed);
	let hrs = secs / (60 * 60);

	if (hrs <= 24) {
		break;
	}

	months_passed += 18;
	ASIC_speed *= 2;
}

console.log(`- solvable in <24h after ${months_passed / 12} years`);
