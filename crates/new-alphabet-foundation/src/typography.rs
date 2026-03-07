use crate::DensityMode;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TypeToken {
    DisplayLarge,
    DisplayDefault,
    Heading1,
    Heading2,
    Heading3,
    BodyReading,
    BodyDefault,
    BodyCompact,
    AnnotationDefault,
    AnnotationStrong,
    DataLabel,
    DataCell,
    DataMetric,
}

pub const ALL_TYPE_TOKENS: [TypeToken; 13] = [
    TypeToken::DisplayLarge,
    TypeToken::DisplayDefault,
    TypeToken::Heading1,
    TypeToken::Heading2,
    TypeToken::Heading3,
    TypeToken::BodyReading,
    TypeToken::BodyDefault,
    TypeToken::BodyCompact,
    TypeToken::AnnotationDefault,
    TypeToken::AnnotationStrong,
    TypeToken::DataLabel,
    TypeToken::DataCell,
    TypeToken::DataMetric,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeStyle {
    pub size_px: u8,
    pub line_height_px: u8,
    pub weight: u16,
}

impl TypeToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::DisplayLarge => "type.display.large",
            Self::DisplayDefault => "type.display.default",
            Self::Heading1 => "type.heading.1",
            Self::Heading2 => "type.heading.2",
            Self::Heading3 => "type.heading.3",
            Self::BodyReading => "type.body.reading",
            Self::BodyDefault => "type.body.default",
            Self::BodyCompact => "type.body.compact",
            Self::AnnotationDefault => "type.annotation.default",
            Self::AnnotationStrong => "type.annotation.strong",
            Self::DataLabel => "type.data.label",
            Self::DataCell => "type.data.cell",
            Self::DataMetric => "type.data.metric",
        }
    }

    pub const fn style(self) -> TypeStyle {
        match self {
            Self::DisplayLarge => TypeStyle {
                size_px: 48,
                line_height_px: 56,
                weight: 600,
            },
            Self::DisplayDefault => TypeStyle {
                size_px: 36,
                line_height_px: 44,
                weight: 600,
            },
            Self::Heading1 => TypeStyle {
                size_px: 28,
                line_height_px: 34,
                weight: 600,
            },
            Self::Heading2 => TypeStyle {
                size_px: 22,
                line_height_px: 28,
                weight: 600,
            },
            Self::Heading3 => TypeStyle {
                size_px: 18,
                line_height_px: 24,
                weight: 600,
            },
            Self::BodyReading => TypeStyle {
                size_px: 18,
                line_height_px: 30,
                weight: 400,
            },
            Self::BodyDefault => TypeStyle {
                size_px: 16,
                line_height_px: 24,
                weight: 400,
            },
            Self::BodyCompact => TypeStyle {
                size_px: 14,
                line_height_px: 20,
                weight: 400,
            },
            Self::AnnotationDefault => TypeStyle {
                size_px: 12,
                line_height_px: 16,
                weight: 500,
            },
            Self::AnnotationStrong => TypeStyle {
                size_px: 12,
                line_height_px: 16,
                weight: 600,
            },
            Self::DataLabel => TypeStyle {
                size_px: 12,
                line_height_px: 16,
                weight: 600,
            },
            Self::DataCell => TypeStyle {
                size_px: 14,
                line_height_px: 18,
                weight: 500,
            },
            Self::DataMetric => TypeStyle {
                size_px: 20,
                line_height_px: 24,
                weight: 600,
            },
        }
    }
}

pub const fn body_for_density(density: DensityMode) -> TypeToken {
    match density {
        DensityMode::Calm => TypeToken::BodyReading,
        DensityMode::Regular => TypeToken::BodyDefault,
        DensityMode::Dense => TypeToken::BodyCompact,
    }
}

pub const fn table_text_for_density(density: DensityMode) -> TypeToken {
    match density {
        DensityMode::Calm => TypeToken::BodyDefault,
        DensityMode::Regular => TypeToken::DataCell,
        DensityMode::Dense => TypeToken::DataLabel,
    }
}
