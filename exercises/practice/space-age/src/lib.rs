// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const MERCURY_SECONDS: f64 = 7600543.81992;
const VENUS_SECONDS: f64 = 19414149.052176;
const EARTH_SECONDS: f64 = 31557600.0;
const MARS_SECONDS: f64 = 59354032.690079994;
const JUPITER_SECONDS: f64 = 374355659.124;
const SATURN_SECONDS: f64 = 929292362.8848;
const URANUS_SECONDS: f64 = 2651370019.3296;
const NEPTUNE_SECONDS: f64 = 5200418560.032001;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / MERCURY_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / VENUS_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / EARTH_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / MARS_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / JUPITER_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / SATURN_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / URANUS_SECONDS * 100.0).round()) / 100.0
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.seconds as f64 / NEPTUNE_SECONDS * 100.0).round()) / 100.0
    }
}
