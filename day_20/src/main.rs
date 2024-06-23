use std::collections::VecDeque;

use module::{get_modules, Signal};

mod module;

fn main() {
    let input = include_str!("../data/input.txt");
    let mut modules = get_modules(input);

    let mut signals = VecDeque::new();
    let mut signal_count = (0usize, 0usize);
    for _ in 0..1000 {
        // for button_presses in 1..=10000 {
        signals.push_back(("button", Signal::Low, "broadcaster"));

        while !signals.is_empty() {
            let (sender, signal, current_target) = signals.pop_front().unwrap();
            // if sender == "th" && signal == Signal::High {
            //     println!("{}", button_presses);
            // }
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
    // ========================

    // Part 2:
    // The "rx"-module is solely dependent on module "cn", which in turn is a conjunction with inputs "th", "sv", "gh" and "ch".
    // "cn" emits a low signal when all of its four inputs are high. The inputs become high after 3947, 4001, 3943 and 3917 button
    // presses, respectivally. The least common multiple (lcm) of these four numbers is 243902373381257, which is accepted as solution.
    // See commented code above how these four numbers has been found.
}
