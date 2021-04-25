use sechsundsechzig::cards::Card;

fn main() {
    for card in Card::deck() {
        println!("{}", card);
    }
}
