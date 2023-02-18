use std::collections::HashMap;

pub fn find_median(vec: &Vec<i32>) -> i32 {
    let mid = vec.len() / 2;

    if vec.len() % 2 != 0 {
        return vec[mid];
    }

    return (vec[mid - 1] + vec[mid]) / 2;
}

// FIXME: seems wrong, fix it
pub fn find_mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for n in list {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max = 0;

    for (_, value) in &map {
        if value > &max {
            max = *value;
        }
    }

    let mut mode = 0;

    for (key, value) in &map {
        if value == &max {
            mode = **key;
        }
    }

    return mode;
}
