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
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

impl From<f64> for Duration {
    fn from(s: f64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn orbital_duration() -> Duration;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::orbital_duration().seconds
    }
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
    fn orbital_duration() -> Duration {
        Duration::from(MERCURY_SECONDS)
    }
}
impl Planet for Venus {
    fn orbital_duration() -> Duration {
        Duration::from(VENUS_SECONDS)
    }
}
impl Planet for Earth {
    fn orbital_duration() -> Duration {
        Duration::from(EARTH_SECONDS)
    }
}
impl Planet for Mars {
    fn orbital_duration() -> Duration {
        Duration::from(MARS_SECONDS)
    }
}
impl Planet for Jupiter {
    fn orbital_duration() -> Duration {
        Duration::from(JUPITER_SECONDS)
    }
}
impl Planet for Saturn {
    fn orbital_duration() -> Duration {
        Duration::from(SATURN_SECONDS)
    }
}
impl Planet for Uranus {
    fn orbital_duration() -> Duration {
        Duration::from(URANUS_SECONDS)
    }
}
impl Planet for Neptune {
    fn orbital_duration() -> Duration {
        Duration::from(NEPTUNE_SECONDS)
    }
}
