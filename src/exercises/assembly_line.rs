/**
 * In this exercise you'll be writing code to analyze the production
 * of an assembly line in a car factory. The assembly line's speed can range
 * from 0 (off) to 10 (maximum).
 * At its lowest speed (1), 221 cars are produced each hour.
 * The production increases linearly with the speed. So with the speed set to 4,
 * it should produce 4 * 221 = 884 cars per hour. However, higher speeds increase
 * the likelihood that faulty cars are produced, which then have to be discarded.
 * The following table shows how speed influences the success rate:
 *  1 to 4: 100% success rate.
 *  5 to 8: 90% success rate.
 *  9 and 10: 77% success rate.
 */

const PRODUCTION_PER_HOUR: f64 = 221.0;
const MINUTES_PER_HOUR: u32 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1..=4 => speed as f64 * 1.0 * PRODUCTION_PER_HOUR,
        5..=8 => speed as f64 * 0.9 * PRODUCTION_PER_HOUR,
        9..=10 => speed as f64 * 0.77 * PRODUCTION_PER_HOUR,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / MINUTES_PER_HOUR
}

fn main() {
    println!("{}", production_rate_per_hour(6));
    println!("{}", working_items_per_minute(6));
}
