use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let n = std::env::args()
        .nth(1)
        .map(|x| x.parse::<usize>().unwrap_or(8))
        .unwrap_or(8);
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect();
    println!("{}", s);
}
