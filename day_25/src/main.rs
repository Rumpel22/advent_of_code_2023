use std::mem::swap;

use nalgebra::{CsMatrix, SymmetricEigen};

// Fancy solution, I don't know why it works. But it works.
// Found idea here: https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/kgxsxbz

fn parse(input: &str) -> CsMatrix<f64> {
    let key_value_pairs = input.lines().flat_map(|line| {
        let key = &line[..3];
        let values = line[5..].split_ascii_whitespace();
        values.map(move |value| (key, value))
    });
    let mut nodes = vec![];

    let edges = key_value_pairs.map(|(a, b)| {
        let mut start = nodes
            .iter()
            .position(|node| *node == a)
            .or_else(|| {
                nodes.push(a.to_string());
                Some(nodes.len() - 1)
            })
            .unwrap();
        let mut end = nodes
            .iter()
            .position(|node| *node == b)
            .or_else(|| {
                nodes.push(b.to_string());
                Some(nodes.len() - 1)
            })
            .unwrap();
        if start < end {
            swap(&mut start, &mut end);
        }
        (start, end)
    });
    let (mut irows, mut icols, mut vals) = edges.fold(
        (vec![], vec![], vec![]),
        |(mut r, mut c, mut v), (start, end)| {
            r.push(start);
            c.push(end);
            v.push(-1.0);
            (r, c, v)
        },
    );

    for i in 0..nodes.len() {
        let count = irows
            .iter()
            .chain(icols.iter())
            .filter(|node| **node == i)
            .count();
        irows.push(i);
        icols.push(i);
        vals.push(count as f64);
    }
    CsMatrix::from_triplet(nodes.len(), nodes.len(), &irows, &icols, &vals)
}

fn main() {
    let input = include_str!("../data/input.txt");
    let matrix = parse(input);

    let eigen = SymmetricEigen::new(matrix.into());

    let l2 = eigen
        .eigenvalues
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let index = eigen
        .eigenvalues
        .iter()
        .position(|eigenvalue| eigenvalue == l2)
        .unwrap();

    let eigenvector = &eigen.eigenvectors.column(index);

    let (a, b): (Vec<f64>, Vec<f64>) = eigenvector.iter().partition(|value| **value < 0.0);

    println!(
        "a = {}, b = {}, a x b = {}",
        a.len(),
        b.len(),
        a.len() * b.len()
    )
}
