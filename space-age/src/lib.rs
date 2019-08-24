// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub struct Duration(f64);

const EARTH_YEAR: f64 = 31_557_600_f64;

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration(seconds as f64)
    }
}

pub trait Planet {
    const CONVERSION_RATIO: f64;

    fn years_during(d: &Duration) -> f64 {
        (d.0 / EARTH_YEAR as f64) / Self::CONVERSION_RATIO
    }
}

macro_rules! planet {
    ($name: ident, $conversion_ratio: expr) => {
        pub struct $name;
        impl Planet for $name {
            const CONVERSION_RATIO: f64 = $conversion_ratio as f64;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

