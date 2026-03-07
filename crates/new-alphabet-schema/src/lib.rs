#![forbid(unsafe_code)]

mod catalog;
mod validate;

pub use catalog::{
    bundle_format_version, contract_bundle, example_project_manifest, serialize_bundle_pretty,
    serialize_manifest_pretty,
};
pub use validate::validate_manifest;

#[cfg(test)]
mod tests {
    use new_alphabet_core::{
        AccessibilitySnapshot, ComponentInstance, IntentKind, ProjectManifest, SurfaceManifest,
        SurfaceRegion, ValidationCategory,
    };

    use super::*;

    #[test]
    fn contract_bundle_contains_required_sections_and_examples() {
        let bundle = contract_bundle();

        assert_eq!(bundle.bundle_format_version, "0.1.0");
        assert!(!bundle.foundations.is_empty());
        assert!(!bundle.primitives.is_empty());
        assert!(!bundle.components.is_empty());
        assert!(!bundle.recipes.is_empty());
        assert!(
            bundle
                .prompt_intents
                .iter()
                .any(|prompt| prompt.recommended_recipe == "BlogIndex")
        );
        assert!(
            bundle
                .validation_rules
                .iter()
                .any(|rule| rule.category == ValidationCategory::AntiPatternUsage)
        );
    }

    #[test]
    fn exported_bundle_is_deterministic() {
        let first = serialize_bundle_pretty().expect("bundle to serialize");
        let second = serialize_bundle_pretty().expect("bundle to serialize");

        assert_eq!(first, second);
    }

    #[test]
    fn example_manifest_passes_validation() {
        let report = validate_manifest(&example_project_manifest());

        assert!(report.valid);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn validator_reports_meaningful_drift_and_repairs() {
        let manifest = ProjectManifest {
            schema_version: bundle_format_version().to_owned(),
            project_name: "vibe dashboard".to_owned(),
            surfaces: vec![SurfaceManifest {
                id: "sparkle-admin".to_owned(),
                name: "Sparkle Magic Dashboard".to_owned(),
                intent: IntentKind::Workspace,
                recipe: "ReviewQueue".to_owned(),
                regions: vec![SurfaceRegion {
                    kind: "rail".to_owned(),
                    placement: "rail_start".to_owned(),
                }],
                primitives: vec!["AppShell".to_owned(), "PageGrid".to_owned()],
                components: vec![ComponentInstance {
                    id: "Table".to_owned(),
                    states: vec!["default".to_owned()],
                    accessible_name: false,
                    keyboard_support: false,
                    text_equivalent: false,
                    custom_variant_props: true,
                }],
                spacing_tokens: vec!["spacing.stack.improvised".to_owned()],
                custom_spacing_values: vec![13],
                accessibility: AccessibilitySnapshot {
                    semantic_html: false,
                    focus_visible: false,
                    color_independent_meaning: false,
                },
                style_escape_hatch: true,
                invented_layout: true,
            }],
        };

        let report = validate_manifest(&manifest);

        assert!(!report.valid);
        assert!(
            report
                .messages
                .iter()
                .any(|message| message.category == ValidationCategory::Composition)
        );
        assert!(
            report
                .messages
                .iter()
                .any(|message| message.category == ValidationCategory::Spacing)
        );
        assert!(
            report
                .messages
                .iter()
                .any(|message| message.category == ValidationCategory::StateCoverage)
        );
        assert!(
            report
                .messages
                .iter()
                .any(|message| message.category == ValidationCategory::Accessibility)
        );
        assert!(
            report
                .messages
                .iter()
                .any(|message| message.category == ValidationCategory::Naming)
        );
        assert!(
            report
                .messages
                .iter()
                .any(|message| message.category == ValidationCategory::AntiPatternUsage)
        );
        assert!(
            report
                .messages
                .iter()
                .all(|message| message.repair.is_some())
        );
    }
}
