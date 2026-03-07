#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DensityMode {
    Calm,
    Regular,
    Dense,
}

pub const ALL_DENSITY_MODES: [DensityMode; 3] =
    [DensityMode::Calm, DensityMode::Regular, DensityMode::Dense];

impl DensityMode {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Calm => "density.calm",
            Self::Regular => "density.regular",
            Self::Dense => "density.dense",
        }
    }
}
