#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    pub angle: AngleMeasure,
}

/// Angle measurement unit: degrees or radians.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AngleMeasure {
    Degrees,
    Radians,
}

impl AngleMeasure {
    pub fn to_rad(&self, value: f64) -> f64 {
        match self {
            AngleMeasure::Degrees => value.to_radians(),
            AngleMeasure::Radians => value,
        }
    }

    pub fn from_rad(&self, value: f64) -> f64 {
        match self {
            AngleMeasure::Degrees => value.to_degrees(),
            AngleMeasure::Radians => value,
        }
    }
}

impl Default for AngleMeasure {
    fn default() -> Self {
        AngleMeasure::Radians
    }
}
