use std::collections::HashMap;

fn main() {
    let range_of_values: Vec<f64> = vec![1.0, 1.0, 5.0, 5.0, 5.0, 8.0, 9.0];
    let length = range_of_values.len() as f64;
    let bound = length / 2.0;

    let median = {
        if length % 2.0 != 0.0 {
            range_of_values[bound as usize]
        } else {
            let lower_bound = (bound - 1.0) as usize;
            let upper_bound = bound as usize;

            let prev = range_of_values[lower_bound];
            let first = range_of_values[upper_bound];
            (first + prev) / 2.0
        }
    };

    let mut mode: HashMap<u64, usize> = HashMap::new();
    for values in &range_of_values {
        let key = values.to_bits();
        if mode.contains_key(&key) {
            let count = mode.get_mut(&key).unwrap();
            *count += 1;
            continue;
        }
        mode.insert(key, 1);
    }

    let mut mode_num = 0;
    let mut prev_freq = 0;
    for (number, cur_freq) in &mode {
        if cur_freq > &prev_freq {
            prev_freq = *cur_freq;
            mode_num = *number;
        } 
    }

    let mode = f64::from_bits(mode_num);

    let mean = range_of_values.iter().sum::<f64>() / length;
    println!("mean: {:.2}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}
