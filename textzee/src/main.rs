//Textzee
//This is a text-based version of Yahtzee

// For external flexibility, we will add the Rand crate. Please look up Rand on 
// crates.io on how to place it in your Cargo.toml file, just right under 
// [dependencies] .
extern crate rand;

	//We will need to specify what parts of the crate to use. 
	use rand::Rng;

fn main() {
	//  die_n
	let die_1 = rand::thread_rng().gen_range(1, 7);//-------- These are your dice. 
	let die_2 = rand::thread_rng().gen_range(1, 7);//	| Each one will 
	let die_3 = rand::thread_rng().gen_range(1, 7);//	| generate a random 
	let die_4 = rand::thread_rng().gen_range(1, 7);//	| number. The actual 
	let die_5 = rand::thread_rng().gen_range(1, 7);//-------- range is 1 to 6. 

		// See the text that says "let die_n = rand::thread_rng().gen_range(0, 7);"?
		// "die_n" represent the generated number(s) for each printed line as shown
		// below. The five five sets brackets are linked to "die_n".
		println!("Your dice...");
			println!("{} + {} + {} + {} + {}", die_1, die_2, die_3, die_4, die_5);

	// "total" defines the sum of the dice. This value is NOT randmomly generated, 
	// but rather added. 
	let total = die_1 + die_2 + die_3 + die_4 + die_5;
		println!("Your total...");
			println!("{}", total);
}
