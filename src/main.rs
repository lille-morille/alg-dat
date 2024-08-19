use std::time::{Duration, Instant};

use rand::Rng;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Run {
    count: u32,
    runs_in_microseconds: String,
    avg_time_per_10_000_elem: u128,
}

// Run speed tests and output table
fn main() {
    let runs: u8 = 5;
    let elems = [1_000, 10_000, 100_000, 1_000_000];
    let mut durations = Vec::<Duration>::with_capacity(elems.len());

    let mut rows = Vec::<Run>::with_capacity(elems.len());

    println!("Running {:?} runs for each count ...\n", runs);

    for elem in elems {
        for _ in 1..=runs {
            let stocks = create_rnd_rel_stocks(elem, -50, 50);

            let stop_watch = Instant::now();
            find_sell_buy(stocks);
            durations.push(stop_watch.elapsed());
        }

        let run_durations = durations.iter().map(|d| d.as_micros());

        let curr_runs = run_durations
            .clone()
            .map(|x| x.to_string() + ",")
            .collect::<String>();

        let time_per_element = run_durations
            .map(|rd| rd * 10_000 / (elem as u128))
            .sum::<u128>();

        rows.push(Run {
            count: elem,
            runs_in_microseconds: curr_runs,
            avg_time_per_10_000_elem: time_per_element / runs as u128,
        });

        durations.clear();
    }

    println!("{}", Table::new(rows).to_string());
}

// Oppgave 1-1
// Lag og implementer en algoritme som finner
// hvilket kjøps- og salgstidspunkt som lønner
// seg best. Sjekk at algoritmen virker.

// Creates a random set of relative stocks
fn create_rnd_rel_stocks(count: u32, min: i32, max: i32) -> Vec<i32> {
    let mut random = rand::thread_rng();
    (0..count).map(|_| random.gen_range(min..max)).collect()
}

/// Finds the (buy_idx, sell_idx) that is best for the given stocks
fn find_sell_buy(relative_stocks: Vec<i32>) -> (usize, usize) {
    let mut curr_price = relative_stocks[0];
    let mut buy_price = relative_stocks[0];
    let mut buy_idx = 0;
    let mut sell_idx = 1;
    let mut profit = 0;

    for (idx, price) in relative_stocks.iter().enumerate() {
        // Skip first iteration
        if idx == 0 {
            continue;
        };

        // Get absolute price instead of relative to previous day
        curr_price += price;

        if curr_price > buy_price {
            let diff = curr_price - buy_price;
            if diff > profit {
                profit = diff;
                sell_idx = idx;
            }
        } else {
            buy_idx = idx;
            buy_price = curr_price;
        }
    }

    (buy_idx, sell_idx)
}

// Testing functionality of algorithm
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let stocks = vec![3, 5, -7, 3, 3, -2];

        assert_eq!(find_sell_buy(stocks), (2, 4));
        // assert_eq!(find_sell_buy(stocks),);
    }

    #[test]
    fn case_2() {
        let stocks = vec![0, 1, 2, -1, 2, 1, -3, 2, 1, -2, 5, -3, 2, -4, 5];

        assert_eq!(find_sell_buy(stocks), (0, 10));
    }
}
