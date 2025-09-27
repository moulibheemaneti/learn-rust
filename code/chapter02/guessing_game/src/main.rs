/*
 * use: brings items from a crate or module into scope, so you can refer to them without their full path.
 * - Like `import` in other languages
 * std: standard library
 * io: input/output library from standard (std) library
 */
use std::cmp::Ordering;
use std::io;
/*
 * Rng: random number generator
 * rand::Rng: random number generator from rand crate
 */
use rand::Rng;

/*
 * The main function is the entry point for a executable rust program.
 * println!: a macro that prints text to the console.
 */
fn main() {
    println!("Guess the number!");

    /*
     * In the first line, we call the rand::thread_rng function that gives us
     * the particular random number generator we’re going to use:
     * one that is local to the current thread of execution and is seeded by the operating system
     *
     * Format: rand::rng().random_range(<min_value>..=<max_value>)
     */
    let secret_number = rand::rng().random_range(1..=100);

    let mut lives = 5;

    loop {
        if lives == 0 {
            println!("You lose! The secret number was {secret_number}.");
            break;
        } else {
            println!("Please input your guess. You have {lives} lives left.");
        }

        /*
         * Create a mutable, empty String to store user input.
         * String::new() is an associated function.
         */
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Ordering is an enum with three variants: Less, Greater, and Equal
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        lives -= 1;
    }
}
