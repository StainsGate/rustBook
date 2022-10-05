use crossbeam::thread;
use crossbeam::thread::ScopedJoinHandle;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result_map: HashMap<char, usize> = HashMap::new();

    let balance = input.len() % worker_count;
    let edge = input.len() / worker_count;

    if balance == 0 {
        for i in 0..worker_count {
            thread::scope(|scope| {
                let mut handlers: Vec<ScopedJoinHandle<HashMap<char, usize>>> = vec![];

                let handler = scope.spawn(move |_| {
                    let mut local_store: HashMap<char, usize> = HashMap::new();

                    for k in input[i * edge..(i + 1) * edge].iter() {
                        for v in k
                            .to_ascii_lowercase()
                            .chars()
                            .filter(|c| c != &',' && !c.is_digit(10))
                        {
                            let count = local_store.entry(v).or_insert(0);
                            *count += 1;
                        }
                    }

                    return local_store;
                });

                handlers.push(handler);

                for handler in handlers {
                    let local_store = handler.join().unwrap();

                    for (c, count) in local_store {
                        *result_map.entry(c).or_insert(0) += count;
                    }
                }
            })
            .unwrap();
        }
    } else {
        for i in 0..worker_count {
            thread::scope(|scope| {
                let mut handlers: Vec<ScopedJoinHandle<HashMap<char, usize>>> = vec![];
                let handler = scope.spawn(move |_| {
                    let mut local_store: HashMap<char, usize> = HashMap::new();

                    if i == worker_count - 1 {
                        for k in input[i * edge..input.len()].iter() {
                            for v in k
                                .to_ascii_lowercase()
                                .chars()
                                .filter(|c| c != &',' && !c.is_digit(10))
                            {
                                let count = local_store.entry(v).or_insert(0);
                                *count += 1;
                            }
                        }
                    } else {
                        for k in input[i * edge..(i + 1) * edge].iter() {
                            for v in k
                                .to_ascii_lowercase()
                                .chars()
                                .filter(|c| c != &',' && !c.is_digit(10))
                            {
                                let count = local_store.entry(v).or_insert(0);
                                *count += 1;
                            }
                        }
                    }
                    return local_store;
                });

                handlers.push(handler);

                for handler in handlers {
                    let local_store = handler.join().unwrap();

                    for (c, count) in local_store {
                        *result_map.entry(c).or_insert(0) += count;
                    }
                }
            })
            .unwrap();
        }
    }

    return result_map;
}

// pub fn get_feq(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let n_elements = input.len() / worker_count;

//     let handles: Vec<_> = (0..worker_count)
//         .map(|i| {
//             thread::spawn(move || {
//                 let start = i * n_elements;
//                 let end = if i < worker_count - 1 {
//                     (i + 1) * n_elements
//                 } else {
//                     input.len()
//                 };
//                 let local_input = &input[start..end];
//                 let mut local_hashmap = HashMap::new();

//                 for s in local_input {
//                     for c in s.chars() {
//                         *local_hashmap.entry(c).or_insert(0) += 1;
//                     }
//                 }

//                 local_hashmap
//             })
//         })
//         .collect();

//     let mut freq_map = HashMap::new();

//     for handle in handles {
//         let local_hashmap = handle.join().unwrap();
//         for (c, count) in local_hashmap {
//             *freq_map.entry(c).or_insert(0) += count;
//         }
//     }

//     println!("{:#?}", freq_map);

//     return freq_map;
// }
