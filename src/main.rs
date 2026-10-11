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
    let mut log: Vec<Reading> = Vec::new();
    for (i, high) in [72, 68, 75, 81, 79].iter().enumerate() {
        log.push(Reading {
            day: DAYS[i].to_string(),
            high: *high,
        });
    }

    for reading in &log {
        println!("{}: {}", reading.day, reading.high);
    }

    let mut rng = rand::rng();
    log.push(Reading {
        day: DAYS[log.len()].to_string(),
        high: rng.random_range(60..=100),
    });
    log.push(Reading {
        day: DAYS[log.len()].to_string(),
        high: rng.random_range(60..=100),
    });

    println!();
    for reading in &log {
        println!("{}: {}", reading.day, reading.high);
    }

    println!();
    let summary = summarize(&log, 75);
    println!("Average: {:.2}", summary.average);
    println!("Hottest day: {}", summary.hottest_day);
    println!("Days above 75: {}", summary.days_above);
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

fn hottest_day(log: &Vec<Reading>) -> usize {
    if log.is_empty() {
        return 0;
    }
    let mut highest = log[0].high;
    let mut index = 0;
    for (i, entry) in log.iter().enumerate() {
        if entry.high > highest {
            highest = entry.high;
            index = i;
        }
    }
    index
}

fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    let mut count = 0;
    for entry in log {
        if entry.high > threshold {
            count += 1;
        }
    }
    count
}

fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    Summary {
        average: average_temp(log),
        hottest_day: log[hottest_day(log)].day.clone(),
        days_above: count_above(log, threshold),
    }
}
