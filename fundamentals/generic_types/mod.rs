pub fn higher<T>(list: &[T]) -> T
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut higher = list[0];

    for &item in list.iter() {
        if item > higher {
            higher = item;
        }
    }
    higher
}

#[derive(Debug)]
struct Pointer<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pointer<T, U> {
    fn cordinates(x: T, y: U) -> (T, U) {
        let pointers = Self { x, y };

        (pointers.x, pointers.y)
    }
}

impl Pointer<f32, f32> {
    fn distance(&self, other: &Pointer<f32, f32>) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

enum EnumPointer<T, U> {
    X(T),
    Y(U),
}
