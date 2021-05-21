use std::collections::HashMap;

fn main() {
    let range_of_values: Vec<f64> = vec![1.0, 1.0, 5.0, 5.0, 5.0, 8.0, 9.0];
    let length = range_of_values.len();
    let bound = length / 2;

    let median = {
        if length % 2 != 0 {
            range_of_values[bound as usize]
        } else {
            let lower_bound = bound - 1;
            let upper_bound = bound;

            let prev = range_of_values[lower_bound];
            let first = range_of_values[upper_bound];
            (first + prev) / 2.0
        }
    };

    let mut mode: HashMap<u64, usize> = HashMap::new();
    for values in &range_of_values {
        let key = values.to_bits();
        let count = mode.entry(key).or_insert(0);
        *count += 1;
    }

    let mode = mode.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(k, _)| f64::from_bits(k))
        .unwrap();

    let mean = range_of_values.iter().sum::<f64>() / length as f64;
    println!("mean: {:.2}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}

