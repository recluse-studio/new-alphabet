use crate::DensityMode;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpacingToken {
    StackTight,
    StackDefault,
    StackLoose,
    RegionDefault,
    RegionSpacious,
    PanelContent,
    ControlInset,
    TableRow,
}

pub const ALL_SPACING_TOKENS: [SpacingToken; 8] = [
    SpacingToken::StackTight,
    SpacingToken::StackDefault,
    SpacingToken::StackLoose,
    SpacingToken::RegionDefault,
    SpacingToken::RegionSpacious,
    SpacingToken::PanelContent,
    SpacingToken::ControlInset,
    SpacingToken::TableRow,
];

impl SpacingToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::StackTight => "spacing.stack.tight",
            Self::StackDefault => "spacing.stack.default",
            Self::StackLoose => "spacing.stack.loose",
            Self::RegionDefault => "spacing.region.default",
            Self::RegionSpacious => "spacing.region.spacious",
            Self::PanelContent => "spacing.panel.content",
            Self::ControlInset => "spacing.control.inset",
            Self::TableRow => "spacing.table.row",
        }
    }

    pub const fn points(self, density: DensityMode) -> u8 {
        match (self, density) {
            (Self::StackTight, DensityMode::Calm) => 12,
            (Self::StackTight, DensityMode::Regular) => 8,
            (Self::StackTight, DensityMode::Dense) => 6,
            (Self::StackDefault, DensityMode::Calm) => 16,
            (Self::StackDefault, DensityMode::Regular) => 12,
            (Self::StackDefault, DensityMode::Dense) => 8,
            (Self::StackLoose, DensityMode::Calm) => 24,
            (Self::StackLoose, DensityMode::Regular) => 16,
            (Self::StackLoose, DensityMode::Dense) => 12,
            (Self::RegionDefault, DensityMode::Calm) => 32,
            (Self::RegionDefault, DensityMode::Regular) => 24,
            (Self::RegionDefault, DensityMode::Dense) => 16,
            (Self::RegionSpacious, DensityMode::Calm) => 48,
            (Self::RegionSpacious, DensityMode::Regular) => 32,
            (Self::RegionSpacious, DensityMode::Dense) => 24,
            (Self::PanelContent, DensityMode::Calm) => 24,
            (Self::PanelContent, DensityMode::Regular) => 16,
            (Self::PanelContent, DensityMode::Dense) => 12,
            (Self::ControlInset, DensityMode::Calm) => 12,
            (Self::ControlInset, DensityMode::Regular) => 10,
            (Self::ControlInset, DensityMode::Dense) => 8,
            (Self::TableRow, DensityMode::Calm) => 16,
            (Self::TableRow, DensityMode::Regular) => 12,
            (Self::TableRow, DensityMode::Dense) => 8,
        }
    }
}
