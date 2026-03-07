#![forbid(unsafe_code)]

mod density;
mod grid;
mod spacing;
mod surface;
mod typography;

pub use density::{ALL_DENSITY_MODES, DensityMode};
pub use grid::{
    ALL_BREAKPOINTS, ALL_RAIL_WIDTH_TOKENS, ALL_REGION_CLASSES, Breakpoint, RailWidthToken,
    RegionClass, ResponsiveSpan, SpanBehavior, allows_parallel_secondary,
    allows_three_region_layout,
};
pub use spacing::{ALL_SPACING_TOKENS, SpacingToken};
pub use surface::{
    ALL_BORDER_TOKENS, ALL_COLOR_TOKENS, ALL_MOTION_TOKENS, ALL_STATE_TOKENS, BorderSpec,
    BorderToken, ColorToken, ColorValue, MotionSpec, MotionToken, StateSpec, StateToken,
};
pub use typography::{
    ALL_TYPE_TOKENS, TypeStyle, TypeToken, body_for_density, table_text_for_density,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FoundationSpec {
    pub version: &'static str,
    pub ssr_safe: bool,
    pub hydration_ready: bool,
}

pub const FOUNDATION_SPEC: FoundationSpec = FoundationSpec {
    version: "0.1.0",
    ssr_safe: true,
    hydration_ready: true,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breakpoints_are_fixed() {
        assert_eq!(Breakpoint::Compact.columns(), 4);
        assert_eq!(Breakpoint::Medium.columns(), 8);
        assert_eq!(Breakpoint::Wide.columns(), 12);
    }

    #[test]
    fn region_collapse_is_explicit() {
        assert_eq!(RegionClass::Rail.spans().compact, SpanBehavior::Stacked);
        assert_eq!(RegionClass::Detail.spans().compact, SpanBehavior::Stacked);
        assert_eq!(RegionClass::Main.spans().wide, SpanBehavior::Columns(8));
    }

    #[test]
    fn density_changes_spacing_and_type() {
        assert_eq!(SpacingToken::TableRow.points(DensityMode::Calm), 16);
        assert_eq!(SpacingToken::TableRow.points(DensityMode::Regular), 12);
        assert_eq!(SpacingToken::TableRow.points(DensityMode::Dense), 8);
        assert_eq!(body_for_density(DensityMode::Calm), TypeToken::BodyReading);
        assert_eq!(
            body_for_density(DensityMode::Regular),
            TypeToken::BodyDefault
        );
        assert_eq!(body_for_density(DensityMode::Dense), TypeToken::BodyCompact);
    }

    #[test]
    fn state_and_motion_rules_are_explicit() {
        assert_eq!(MotionToken::ReducedNone.spec().duration_ms, 0);
        assert_eq!(
            StateToken::FocusRing.spec().border,
            Some(BorderToken::ControlFocus)
        );
        assert_eq!(
            StateToken::ErrorEmphasis.spec().color,
            Some(ColorToken::StatusError)
        );
    }

    #[test]
    fn foundation_is_ssr_safe() {
        assert!(FOUNDATION_SPEC.ssr_safe);
        assert!(FOUNDATION_SPEC.hydration_ready);
    }
}
