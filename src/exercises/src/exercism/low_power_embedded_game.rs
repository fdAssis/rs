/**
* You are working on a game targeting a low-power embedded system and need
* to write several convenience functions which will be used by
* other parts of the game.
*
* Exercise link: https://exercism.org/tracks/rust/exercises/low-power-embedded-game
*/

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    std::iter::empty()
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0 + self.1
    }
}
