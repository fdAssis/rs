pub fn swap(slice: &Vec<u32>) -> Vec<u32> {
    vec![slice[1], slice[0]]
}

pub fn bubble_sort(vec: &mut Vec<u32>) {
    let mut temp: Vec<u32> = Vec::new();
    for i in 0..vec.len() - 1 {
        if vec[i] > vec[i + 1] {
            temp.append(&mut swap(&Vec::<u32>::from(&vec[i..=i + 1])));
        }
    }

    println!("{:#?}", temp);
}

pub fn main() {
    let mut vec: Vec<u32> = vec![5, 3, 8, 4, 6];
    bubble_sort(&mut vec);
}
