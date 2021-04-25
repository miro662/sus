use sechsundsechzig::cards::Card;
use rand::prelude::*;

fn main() {
    println!("unshuffled:");
    for (id, card) in Card::deck().enumerate() {
        println!("{}: {}", id, card);
    }

    println!("shuffled:");
    for (id, card) in Card::shuffled_deck(&mut thread_rng()).enumerate() {
        println!("{}: {}", id, card);
    }
}
