// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds : u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds : s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

const SECONDS_IN_YEAR : u64 = 31557600;

macro_rules! planet_impl {
    ($($planet_type:ty, $orbital_period:literal),+) => {
        $(
            impl $planet_type {
                const ORBITAL_PERIOD : f64 = $orbital_period;
            }
            impl Planet for $planet_type {
                fn years_during(d: &Duration) -> f64 {
                    d.seconds as f64 / (SECONDS_IN_YEAR as f64 * Self::ORBITAL_PERIOD)
                }
            }
        )*
    };
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

planet_impl!(
    Mercury, 0.2408467,
    Venus, 0.61519726,
    Earth, 1.00000000000,
    Mars, 1.8808158,
    Jupiter, 11.862615,
    Saturn, 29.447498,
    Uranus, 84.016846,
    Neptune, 164.79132
);