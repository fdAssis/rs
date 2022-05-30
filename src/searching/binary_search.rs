use std::cmp::Ordering;

fn iterative_binary_search(v: &Vec<u32>, item: u32) {
    let mut first = 0;
    let mut last = v.len() - 1;

    loop {
        let middle = (first + last) / 2;
        match v[middle].cmp(&item) {
            Ordering::Equal => {
                println!("{}", middle);
                break;
            }
            Ordering::Less => first = middle + 1,

            Ordering::Greater => last = middle,
        }
    }
}

fn recursive_binary_search(v: &Vec<u32>, item: u32, first: usize, last: usize) {
    let middle = (first + last) / 2;

    if v[middle] == item {
        println!("{}", middle);
    } else if v[middle] > item {
        recursive_binary_search(v, item, first, middle);
    } else {
        recursive_binary_search(v, item, middle + 1, last)
    }
}

fn main() {
    let test_vec: Vec<u32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    iterative_binary_search(&test_vec, 30);
    recursive_binary_search(&test_vec, 30, 0, test_vec.len() - 1);
}
