// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    age_in_seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            age_in_seconds: s, 
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.age_in_seconds as f64 / Self::ORBITAL_PERIOD
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

const EARTH_SECONDS: f64 = 31557600.0;

impl Planet for Mercury {const ORBITAL_PERIOD: f64 = 0.2408467*EARTH_SECONDS;}
impl Planet for Venus {const ORBITAL_PERIOD: f64 =  0.61519726*EARTH_SECONDS;}
impl Planet for Earth {const ORBITAL_PERIOD: f64 = 1.0*EARTH_SECONDS;}
impl Planet for Mars {const ORBITAL_PERIOD: f64 = 1.8808158*EARTH_SECONDS;}
impl Planet for Jupiter {const ORBITAL_PERIOD: f64 = 11.862615*EARTH_SECONDS;}
impl Planet for Saturn {const ORBITAL_PERIOD: f64 =  29.447498 *EARTH_SECONDS;}
impl Planet for Uranus {const ORBITAL_PERIOD: f64 =  84.016846*EARTH_SECONDS;}
impl Planet for Neptune {const ORBITAL_PERIOD: f64 = 164.79132*EARTH_SECONDS;}