macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };

    ($a:expr) => {
        $a
    };
}

macro_rules! add_as {
    ($first_number:expr, $second_number:expr, $type:ty) => {
        $first_number as $type + $second_number as $type
    };
}

macro_rules! add_as_two{
  ($($a:expr),*)=>{{
      0
      $(+$a)*
  }}
}

fn main() {
    let add_two_numbers = add!(1, 2);
    let add_one_number = add!(1);

    println!("a + b = {add_two_numbers}");
    println!("a = {add_one_number}");
}
