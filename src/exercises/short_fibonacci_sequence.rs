/**
 * Instructions
 *  You are going to initialize empty buffers and list the first five numbers,
 *  or elements, of the Fibonacci sequence.
 *  
 *  The Fibonacci sequence is a set of numbers where the next
 *  element is the sum of the prior two. We start the sequence at one.
 *  So the first two elements are 1 and 1.
 *
 *  Exercise link: https://exercism.org/tracks/rust/exercises/short-fibonacci
 */

/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::<u8>::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fib: Vec<u8> = Vec::new();
    for i in 0..5 {
        match i {
            0 => fib.push(1),
            1 => fib.push(1),
            _ => fib.push(fib[i - 2] + fib[i - 1]),
        }
    }

    fib
}

pub fn main() {
    println!("{:?}", fibonacci());
}
