use std::time::{Duration, Instant};

use rand::Rng;
use tabled::{Table, Tabled};

/// Struct for displaying table for all runs
#[derive(Tabled)]
pub struct Run {
    pub count: u32,
    pub runs_in_microseconds: String,
    pub avg_time_per_10_000_elem: u128,
}

/// Oppgave 1-1
/// Lag og implementer en algoritme som finner
/// hvilket kjøps- og salgstidspunkt som lønner
/// seg best. Sjekk at algoritmen virker.
/// Run speed tests and output table
fn main() {
    // Number of runs per elems
    let runs: usize = 5;

    // Count of elements to run on for each test
    let elems = [10_000, 100_000, 1_000_000];

    // Store the durations for each run
    let mut durations = Vec::<Duration>::with_capacity(runs);

    // Store all runs for each [elems] in a table row for output
    let mut rows = Vec::<Run>::with_capacity(elems.len());

    println!("Running {:?} runs for each count ...\n", runs);

    for elem in elems {
        for run in 1..=runs {
            let stocks = create_rnd_rel_stocks(elem, -50, 50);

            let stop_watch = Instant::now();
            let result = find_sell_buy(stocks);

            println!("{:?} elems, run {:?}: {:?}", elem, run, result);
            durations.push(stop_watch.elapsed());
        }

        let run_durations = durations.iter().map(|d| d.as_micros());

        let (time_per_element, mut curr_runs): (u128, String) = run_durations
            .map(|rd| (rd * 10_000 / (elem as u128), rd.to_string() + ", "))
            .fold((0u128, String::new()), |acc, e| (acc.0 + e.0, acc.1 + &e.1));

        curr_runs.pop();
        curr_runs.pop();

        rows.push(Run {
            count: elem,
            runs_in_microseconds: curr_runs,
            avg_time_per_10_000_elem: time_per_element / runs as u128,
        });

        durations.clear();
    }

    println!("{}", Table::new(rows));
}

// Creates a random set of relative stocks
fn create_rnd_rel_stocks(count: u32, min: i32, max: i32) -> Vec<i32> {
    let mut random = rand::thread_rng();
    (0..count).map(|_| random.gen_range(min..max)).collect()
}

/// Finds the (buy_idx, sell_idx) that is best for the given stocks
fn find_sell_buy(relative_stocks: Vec<i32>) -> Option<(usize, usize)> {
    if relative_stocks.is_empty() {
        return None;
    }

    let mut curr_price = relative_stocks[0];
    let mut buy_price = relative_stocks[0];
    let mut buy_idx = 0;
    let mut sell_idx = 1;
    let mut profit = 0;
    let mut pot_buy_idx = 0;
    let mut pot_buy_price = relative_stocks[0];

    for (idx, rel_price) in relative_stocks.iter().skip(1).enumerate() {
        let idx = idx + 1;

        // Get absolute price instead of relative to previous day
        curr_price += rel_price;

        if curr_price > buy_price {
            let diff = curr_price - buy_price;
            let pot_diff = curr_price - pot_buy_price;
            if diff > profit {
                profit = diff;
                sell_idx = idx;
            }
            if pot_diff > profit {
                profit = pot_diff;
                buy_idx = pot_buy_idx;
                buy_price = pot_buy_price;
                sell_idx = idx;
            }
        } else {
            pot_buy_idx = idx;
            pot_buy_price = curr_price;
        }
    }

    if profit == 0 {
        return None;
    }

    Some((buy_idx, sell_idx))
}

// Testing functionality of algorithm
#[cfg(test)]
mod tests {
    use super::*;

    /// The algoritm should buy and sell at the best times
    #[test]
    fn case_1() {
        let stocks = vec![3, 5, -7, 3, 3, -2];
        // 3 8 1 4 7 5

        assert_eq!(find_sell_buy(stocks), Some((2, 4)));
    }

    /// The algoritm should buy and sell at the best times
    #[test]
    fn case_2() {
        let stocks = vec![0, 1, 2, -1, 2, 1, -3, 2, 1, -2, 5, -3, 2, -4, 5];

        assert_eq!(find_sell_buy(stocks), Some((0, 10)));
    }

    /// Finding price lower than the previous best should not result
    /// in selling before buying
    #[test]
    fn case_3() {
        let stocks = vec![2, 1, 1, 1, -4];

        assert_eq!(find_sell_buy(stocks), Some((0, 3)));
    }

    /// Stocks only going downwards should yield None
    #[test]
    fn case_4() {
        let stocks = vec![0, -1, -2, -1, -2];
        assert_eq!(find_sell_buy(stocks), None);
    }

    /// No stocks should return None
    #[test]
    fn case_5() {
        let stocks = vec![];

        assert_eq!(find_sell_buy(stocks), None)
    }
}
