use space_age_2::planet;

#[planet(rate=3600)]
struct Earth;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}


fn main() {
    let duration = Duration::from(1_000_000_000);
    let d = Earth::years_during(&duration);
    println!("d={}", d);
}
