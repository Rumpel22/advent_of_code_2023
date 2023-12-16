fn differences(data: &[i32]) -> Vec<i32> {
    data.iter()
        .skip(1)
        .scan(data[0], |current, actual| {
            let diff = actual - *current;
            *current = *actual;
            Some(diff)
        })
        .collect()
}

fn extrapolate(data: &[i32]) -> i32 {
    let diffs = differences(data);
    if diffs.iter().all(|diff| diff == &0) {
        data[0]
    } else {
        data.last().unwrap() + extrapolate(&diffs)
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let data_histories = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let extrapolated_data_sum = data_histories
        .iter()
        .map(|data| extrapolate(data))
        .sum::<i32>();

    println!("The sum of the interpolated values is {extrapolated_data_sum}.");
}
