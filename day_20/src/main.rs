use std::collections::VecDeque;

use module::{get_modules, Signal};

mod module;

fn main() {
    let input = include_str!("../data/input.txt");
    let mut modules = get_modules(input);

    let mut signals = VecDeque::new();
    let mut signal_count = (0usize, 0usize);
    for _ in 0..1000 {
        signals.push_back(("button", Signal::Low, "broadcaster"));

        while !signals.is_empty() {
            let (sender, signal, current_target) = signals.pop_front().unwrap();
            match signal {
                Signal::High => signal_count.1 += 1,
                Signal::Low => signal_count.0 += 1,
            };

            if let Some(module) = modules.get_mut(current_target) {
                let new_signals = module.recv_signal(&signal, sender);

                for (target, new_signal) in new_signals {
                    signals.push_back((current_target, new_signal, target));
                }
            }
        }
    }

    println!(
        "There are {} high signals and {} low signals",
        signal_count.1, signal_count.0
    );
    println!(
        "Multiplied, the result is {}.",
        signal_count.0 * signal_count.1
    );
}
