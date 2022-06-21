pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    let mut sorted = false;
    let mut n = array.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

pub fn main() {
    let mut vec1 = vec![5, 3, 8, 4, 6];

    println!("{:?}", vec1);
    bubble_sort(&mut vec1);
    println!("{:?}", vec1);
}
