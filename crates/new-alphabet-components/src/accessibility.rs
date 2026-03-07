#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessibilityRule {
    Keyboard,
    FocusVisible,
    SemanticName,
    ColorIndependentMeaning,
}

impl AccessibilityRule {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::FocusVisible => "focus_visible",
            Self::SemanticName => "semantic_name",
            Self::ColorIndependentMeaning => "color_independent_meaning",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessibilityCheck {
    pub component: &'static str,
    pub semantic_anchor: &'static str,
    pub rules: &'static [AccessibilityRule],
}

impl AccessibilityCheck {
    pub const fn new(
        component: &'static str,
        semantic_anchor: &'static str,
        rules: &'static [AccessibilityRule],
    ) -> Self {
        Self {
            component,
            semantic_anchor,
            rules,
        }
    }
}

const INTERACTIVE_RULES: &[AccessibilityRule] = &[
    AccessibilityRule::Keyboard,
    AccessibilityRule::FocusVisible,
    AccessibilityRule::SemanticName,
];

const STATUS_RULES: &[AccessibilityRule] = &[
    AccessibilityRule::SemanticName,
    AccessibilityRule::ColorIndependentMeaning,
];

const DENSE_RULES: &[AccessibilityRule] = &[
    AccessibilityRule::SemanticName,
    AccessibilityRule::ColorIndependentMeaning,
];

pub const COMPONENT_ACCESSIBILITY_CHECKS: &[AccessibilityCheck] = &[
    AccessibilityCheck::new("Button", "button", INTERACTIVE_RULES),
    AccessibilityCheck::new("LinkAction", "a", INTERACTIVE_RULES),
    AccessibilityCheck::new("TextField", "label + input", INTERACTIVE_RULES),
    AccessibilityCheck::new("Textarea", "label + textarea", INTERACTIVE_RULES),
    AccessibilityCheck::new("Select", "label + select", INTERACTIVE_RULES),
    AccessibilityCheck::new(
        "Checkbox",
        "label + input[type=checkbox]",
        INTERACTIVE_RULES,
    ),
    AccessibilityCheck::new("RadioGroup", "fieldset + legend", INTERACTIVE_RULES),
    AccessibilityCheck::new("Switch", "label + input[role=switch]", INTERACTIVE_RULES),
    AccessibilityCheck::new("Table", "table + caption", DENSE_RULES),
    AccessibilityCheck::new("MetricBlock", "section[aria-label]", DENSE_RULES),
    AccessibilityCheck::new(
        "Pagination",
        "nav[aria-label=Pagination]",
        INTERACTIVE_RULES,
    ),
    AccessibilityCheck::new("NavIndex", "nav[aria-label]", INTERACTIVE_RULES),
    AccessibilityCheck::new("CommandBar", "section[aria-label]", INTERACTIVE_RULES),
    AccessibilityCheck::new("FilterRail", "fieldset + legend", INTERACTIVE_RULES),
    AccessibilityCheck::new("DetailPane", "section[aria-label]", DENSE_RULES),
    AccessibilityCheck::new("StatusBadge", "text severity label", STATUS_RULES),
    AccessibilityCheck::new("InlineAlert", "role=status", STATUS_RULES),
    AccessibilityCheck::new("EmptyState", "role=status", STATUS_RULES),
];
