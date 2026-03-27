#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ColorToken {
    CanvasBase,
    CanvasElevated,
    CanvasInset,
    TextPrimary,
    TextSecondary,
    TextInverse,
    AccentPrimary,
    AccentMuted,
    BorderDefault,
    BorderStrong,
    BorderFocus,
    StatusInfo,
    StatusSuccess,
    StatusWarning,
    StatusError,
    EmphasisSubtle,
    EmphasisStrong,
}

pub const ALL_COLOR_TOKENS: [ColorToken; 17] = [
    ColorToken::CanvasBase,
    ColorToken::CanvasElevated,
    ColorToken::CanvasInset,
    ColorToken::TextPrimary,
    ColorToken::TextSecondary,
    ColorToken::TextInverse,
    ColorToken::AccentPrimary,
    ColorToken::AccentMuted,
    ColorToken::BorderDefault,
    ColorToken::BorderStrong,
    ColorToken::BorderFocus,
    ColorToken::StatusInfo,
    ColorToken::StatusSuccess,
    ColorToken::StatusWarning,
    ColorToken::StatusError,
    ColorToken::EmphasisSubtle,
    ColorToken::EmphasisStrong,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorValue {
    pub hex: &'static str,
}

impl ColorToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::CanvasBase => "color.canvas.base",
            Self::CanvasElevated => "color.canvas.elevated",
            Self::CanvasInset => "color.canvas.inset",
            Self::TextPrimary => "color.text.primary",
            Self::TextSecondary => "color.text.secondary",
            Self::TextInverse => "color.text.inverse",
            Self::AccentPrimary => "color.accent.primary",
            Self::AccentMuted => "color.accent.muted",
            Self::BorderDefault => "color.border.default",
            Self::BorderStrong => "color.border.strong",
            Self::BorderFocus => "color.border.focus",
            Self::StatusInfo => "color.status.info",
            Self::StatusSuccess => "color.status.success",
            Self::StatusWarning => "color.status.warning",
            Self::StatusError => "color.status.error",
            Self::EmphasisSubtle => "color.emphasis.subtle",
            Self::EmphasisStrong => "color.emphasis.strong",
        }
    }

    pub const fn value(self) -> ColorValue {
        match self {
            Self::CanvasBase => ColorValue { hex: "#F7F4EC" },
            Self::CanvasElevated => ColorValue { hex: "#FFFFFF" },
            Self::CanvasInset => ColorValue { hex: "#ECE7DA" },
            Self::TextPrimary => ColorValue { hex: "#111111" },
            Self::TextSecondary => ColorValue { hex: "#4C4C47" },
            Self::TextInverse => ColorValue { hex: "#FAF8F2" },
            Self::AccentPrimary => ColorValue { hex: "#D9371E" },
            Self::AccentMuted => ColorValue { hex: "#A04B3A" },
            Self::BorderDefault => ColorValue { hex: "#C9C1B1" },
            Self::BorderStrong => ColorValue { hex: "#7E7463" },
            Self::BorderFocus => ColorValue { hex: "#111111" },
            Self::StatusInfo => ColorValue { hex: "#165A8A" },
            Self::StatusSuccess => ColorValue { hex: "#17603A" },
            Self::StatusWarning => ColorValue { hex: "#8A5A00" },
            Self::StatusError => ColorValue { hex: "#A2211F" },
            Self::EmphasisSubtle => ColorValue { hex: "#D9D2C4" },
            Self::EmphasisStrong => ColorValue { hex: "#2A2620" },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BorderToken {
    RegionDefault,
    PanelDefault,
    PanelStrong,
    ControlDefault,
    ControlFocus,
}

pub const ALL_BORDER_TOKENS: [BorderToken; 5] = [
    BorderToken::RegionDefault,
    BorderToken::PanelDefault,
    BorderToken::PanelStrong,
    BorderToken::ControlDefault,
    BorderToken::ControlFocus,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BorderSpec {
    pub width_px: u8,
    pub color: ColorToken,
}

impl BorderToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::RegionDefault => "border.region.default",
            Self::PanelDefault => "border.panel.default",
            Self::PanelStrong => "border.panel.strong",
            Self::ControlDefault => "border.control.default",
            Self::ControlFocus => "border.control.focus",
        }
    }

    pub const fn spec(self) -> BorderSpec {
        match self {
            Self::RegionDefault => BorderSpec {
                width_px: 1,
                color: ColorToken::BorderDefault,
            },
            Self::PanelDefault => BorderSpec {
                width_px: 1,
                color: ColorToken::BorderDefault,
            },
            Self::PanelStrong => BorderSpec {
                width_px: 2,
                color: ColorToken::BorderStrong,
            },
            Self::ControlDefault => BorderSpec {
                width_px: 1,
                color: ColorToken::BorderDefault,
            },
            Self::ControlFocus => BorderSpec {
                width_px: 2,
                color: ColorToken::BorderFocus,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RadiusToken {
    CornerSubtle,
}

pub const ALL_RADIUS_TOKENS: [RadiusToken; 1] = [RadiusToken::CornerSubtle];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadiusSpec {
    pub value_px: u8,
}

impl RadiusToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::CornerSubtle => "radius.corner.subtle",
        }
    }

    pub const fn spec(self) -> RadiusSpec {
        match self {
            Self::CornerSubtle => RadiusSpec { value_px: 5 },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MotionToken {
    TransitionFast,
    TransitionDefault,
    RegionEnter,
    RegionExit,
    ReducedNone,
}

pub const ALL_MOTION_TOKENS: [MotionToken; 5] = [
    MotionToken::TransitionFast,
    MotionToken::TransitionDefault,
    MotionToken::RegionEnter,
    MotionToken::RegionExit,
    MotionToken::ReducedNone,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MotionSpec {
    pub duration_ms: u16,
    pub easing: &'static str,
    pub reduced: bool,
}

impl MotionToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::TransitionFast => "motion.transition.fast",
            Self::TransitionDefault => "motion.transition.default",
            Self::RegionEnter => "motion.region.enter",
            Self::RegionExit => "motion.region.exit",
            Self::ReducedNone => "motion.reduced.none",
        }
    }

    pub const fn spec(self) -> MotionSpec {
        match self {
            Self::TransitionFast => MotionSpec {
                duration_ms: 120,
                easing: "ease-out",
                reduced: false,
            },
            Self::TransitionDefault => MotionSpec {
                duration_ms: 180,
                easing: "ease-out",
                reduced: false,
            },
            Self::RegionEnter => MotionSpec {
                duration_ms: 180,
                easing: "ease-out",
                reduced: false,
            },
            Self::RegionExit => MotionSpec {
                duration_ms: 140,
                easing: "ease-in",
                reduced: false,
            },
            Self::ReducedNone => MotionSpec {
                duration_ms: 0,
                easing: "linear",
                reduced: true,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StateToken {
    FocusRing,
    LoadingMuted,
    SuccessEmphasis,
    ErrorEmphasis,
    DisabledAlpha,
}

pub const ALL_STATE_TOKENS: [StateToken; 5] = [
    StateToken::FocusRing,
    StateToken::LoadingMuted,
    StateToken::SuccessEmphasis,
    StateToken::ErrorEmphasis,
    StateToken::DisabledAlpha,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateSpec {
    pub border: Option<BorderToken>,
    pub color: Option<ColorToken>,
    pub opacity_percent: u8,
}

impl StateToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::FocusRing => "state.focus.ring",
            Self::LoadingMuted => "state.loading.muted",
            Self::SuccessEmphasis => "state.success.emphasis",
            Self::ErrorEmphasis => "state.error.emphasis",
            Self::DisabledAlpha => "state.disabled.alpha",
        }
    }

    pub const fn spec(self) -> StateSpec {
        match self {
            Self::FocusRing => StateSpec {
                border: Some(BorderToken::ControlFocus),
                color: Some(ColorToken::BorderFocus),
                opacity_percent: 100,
            },
            Self::LoadingMuted => StateSpec {
                border: None,
                color: Some(ColorToken::EmphasisSubtle),
                opacity_percent: 72,
            },
            Self::SuccessEmphasis => StateSpec {
                border: Some(BorderToken::PanelStrong),
                color: Some(ColorToken::StatusSuccess),
                opacity_percent: 100,
            },
            Self::ErrorEmphasis => StateSpec {
                border: Some(BorderToken::PanelStrong),
                color: Some(ColorToken::StatusError),
                opacity_percent: 100,
            },
            Self::DisabledAlpha => StateSpec {
                border: None,
                color: None,
                opacity_percent: 48,
            },
        }
    }
}
