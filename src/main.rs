//Textzee
//This is a text-based version of Yahtzee

// For external flexibility, we will add the Rand crate. Please look up Rand on
// crates.io on how to place it in your Cargo.toml file, just right under
// [dependencies] .

// We will need to specify what parts of the crate to use.
mod dices;

fn main() {
    let number_of_dices = 5;
    let dices = dices::Dices::gen_range(number_of_dices);

    println!("Rolls: {}", dices);
    println!("Sum: {}", dices.sum());
}
