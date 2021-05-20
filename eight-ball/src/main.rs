use rand::Rng;
use std::io;

fn main() {
    let responses = [
        "the stars say yes.",
        "yes.",
        "Of course!",
        "Absolutely.",
        "Affirmative.",
        "The best choice you'll ever make.",
        "no.",
        "ask me again.",
        "the stones says no.",
        "ask me the same question in a moment, I require time to process this."
    ];

    let mut rng = rand::thread_rng();

    loop {
        println!("Ask me anything.");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let upper_bound = responses.len() - 1;
        let index: usize = rng.gen_range(1..upper_bound);
        let response = responses[index];

        println!("You asked me {}\nI say {}", user_input, response);
    }
}
