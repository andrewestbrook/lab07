use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    let mut log: Vec<Reading> = Vec::new();
    for (i, high) in highs.iter().enumerate() {
        log.push(Reading {
            day: DAYS[i].to_string(),
            high: *high,
        });
    }

    for reading in &log {
        println!("{}: {}", reading.day, reading.high);
    }

    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));
    for i in log.len()..highs.len() {
        log.push(Reading {
            day: DAYS[i].to_string(),
            high: highs[i],
        });
    }

    println!();
    for reading in &log {
        println!("{}: {}", reading.day, reading.high);
    }

    println!();
    println!("Average: {:.2}", average_temp(&log));
    println!("Hottest day: {}", DAYS[hottest_day(&highs)]);
    println!("Days above 75: {}", count_above(&highs, 75));
}

fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }
    let mut total: f64 = 0.0;
    for entry in log {
        total += entry.high as f64;
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

fn count_above(log: &Vec<i32>, threshold: i32) -> usize {
    let mut count = 0;
    for entry in log {
        if *entry > threshold {
            count += 1;
        }
    }
    count
}
