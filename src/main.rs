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

    println!();
    println!("Average: {:.2}", average_temp(&highs));
    println!("Hottest day: {}", DAYS[hottest_day(&highs)]);
}

fn average_temp(log: &Vec<i32>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }
    let mut total: f64 = 0.0;
    for entry in log {
        total += *entry as f64;
    }
    total / log.len() as f64
}

fn hottest_day(log: &Vec<i32>) -> usize {
    if log.is_empty() {
        return 0;
    }
    let mut highest = log[0];
    let mut index = 0;
    for (i, entry) in log.iter().enumerate() {
        if *entry > highest {
            highest = *entry;
            index = i;
        }
    }
    index
}
