use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    for (i, high) in highs.iter().enumerate() {
        println!("{}: {}", DAYS[i], high);
    }

    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

    println!();
    for (i, high) in highs.iter().enumerate() {
        println!("{}: {}", DAYS[i], high);
    }
}
