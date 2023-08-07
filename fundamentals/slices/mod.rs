use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn print_phrase(phrase: &str) {
    let jump: usize = 2;
    let mut stdout = stdout();

    loop {
        for i in 0..phrase.len() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            if i + jump == phrase.len() {
                print!("\t----[{}]----", &phrase[i..]);
                break;
            }
            print!("\t----[{}]----", &phrase[i..i + jump]);
            stdout.flush().unwrap();
            sleep(Duration::from_millis(1000));
        }
    }
}
