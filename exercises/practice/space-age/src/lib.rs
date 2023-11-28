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

macro_rules! impl_trait_for {
    ($Planet:ident $($t:ident($v:expr)),+)=> {
        $(impl $Planet for $t{
            const ORBITAL_PERIOD: f64 = $v * EARTH_SECONDS;
        })+
        
    }
}
impl_trait_for!(Planet Mercury(0.2408467), Venus(0.61519726), Earth(1.0), Mars(1.8808158), Jupiter(11.862615), Saturn(29.447498), Uranus(84.016846), Neptune(164.79132));
