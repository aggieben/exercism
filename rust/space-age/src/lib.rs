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

const SECONDS_IN_YEAR : f64 = 31536000.0;

pub struct Mercury;
impl Mercury {
    const ORBITAL_PERIOD : f64 = 0.2408467;
}

pub struct Venus;
impl Venus {
    const ORBITAL_PERIOD : f64 = 0.061519726;
}

pub struct Earth;
impl Earth {
    const ORBITAL_PERIOD : f64 = 1.0;
}

pub struct Mars;
impl Mars {
    const ORBITAL_PERIOD : f64 = 1.8808158;
}

pub struct Jupiter;
impl Jupiter {
    const ORBITAL_PERIOD : f64 = 11.862615;
}

pub struct Saturn;
impl Saturn {
    const ORBITAL_PERIOD : f64 = 29.447498;
}

pub struct Uranus;
impl Uranus {
    const ORBITAL_PERIOD : f64 = 84.016846;
}

pub struct Neptune;
impl Neptune {
    const ORBITAL_PERIOD : f64 = 164.79132;
}


impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / SECONDS_IN_YEAR * Self::ORBITAL_PERIOD
    }
}
