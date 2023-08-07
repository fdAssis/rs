use std::cmp;

pub fn insertion_sort<T>(arr: &mut [T])
where
    T: cmp::PartialOrd + Copy,
{
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i - 1;

        while arr[j] > cur {
            arr.swap(j + 1, j);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

fn main() {
    let mut arr: Vec<u32> = vec![10, 2, 15, 5];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}
