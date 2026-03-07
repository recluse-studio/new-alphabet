use std::collections::BTreeSet;

use new_alphabet_components::COMPONENT_ACCESSIBILITY_CHECKS;
use new_alphabet_core::{
    ProjectManifest, Severity, SurfaceManifest, ValidationCategory, ValidationMessage,
    ValidationReport,
};

use crate::bundle_format_version;

pub fn validate_manifest(manifest: &ProjectManifest) -> ValidationReport {
    let mut report = ValidationReport::new();

    if manifest.schema_version != bundle_format_version() {
        report.push(message(
            "V-000",
            Severity::Error,
            ValidationCategory::Composition,
            manifest.project_name.as_str(),
            format!(
                "Manifest schema_version {} does not match the current bundle format {}.",
                manifest.schema_version,
                bundle_format_version()
            ),
            "Regenerate the manifest from the current contract bundle before validating again.",
        ));
    }

    for surface in &manifest.surfaces {
        validate_surface(surface, &mut report);
    }

    report
}

fn validate_surface(surface: &SurfaceManifest, report: &mut ValidationReport) {
    validate_regions(surface, report);
    validate_spacing(surface, report);
    validate_naming(surface, report);
    validate_state_coverage(surface, report);
    validate_accessibility(surface, report);
    validate_anti_patterns(surface, report);
}

fn validate_regions(surface: &SurfaceManifest, report: &mut ValidationReport) {
    let region_kinds: BTreeSet<&str> = surface
        .regions
        .iter()
        .map(|region| region.kind.as_str())
        .collect();

    if !region_kinds.contains("main") {
        report.push(message(
            "V-001",
            Severity::Error,
            ValidationCategory::Composition,
            surface.id.as_str(),
            format!(
                "{} is missing Region(main), so the page grid no longer has a named primary field.",
                surface.recipe
            ),
            "Add Region(main) before adjusting rails, detail panes, or local composition.",
        ));
    }

    let required_regions = required_regions(surface.recipe.as_str());
    for region in required_regions {
        if !region_kinds.contains(region) {
            report.push(message(
                "V-001",
                Severity::Error,
                ValidationCategory::Composition,
                surface.id.as_str(),
                format!(
                    "{} requires the {} region, but it is missing from the manifest.",
                    surface.recipe, region
                ),
                "Restore the missing required region before refining local layout.",
            ));
        }
    }

    let allowed_regions = allowed_regions(surface.recipe.as_str());
    for region in &region_kinds {
        if !allowed_regions.contains(region) {
            report.push(message(
                "V-002",
                Severity::Error,
                ValidationCategory::Composition,
                surface.id.as_str(),
                format!(
                    "{} is not an allowed region for {}.",
                    region, surface.recipe
                ),
                "Delete the invalid region or switch to the recipe that owns it.",
            ));
        }
    }

    let start_rails = surface
        .regions
        .iter()
        .filter(|region| region.kind == "rail" && region.placement == "rail_start")
        .count();
    if start_rails > 1 {
        report.push(message(
            "P-102",
            Severity::Error,
            ValidationCategory::Composition,
            surface.id.as_str(),
            "Multiple start-side rails were declared, which breaks the bounded side-structure rule."
                .to_owned(),
            "Keep one rail on each side at most and move extra structure into the main field.",
        ));
    }
}

fn validate_spacing(surface: &SurfaceManifest, report: &mut ValidationReport) {
    for token in &surface.spacing_tokens {
        if !allowed_spacing_tokens()
            .iter()
            .any(|allowed| *allowed == token.as_str())
        {
            report.push(message(
                "V-003",
                Severity::Error,
                ValidationCategory::Spacing,
                surface.id.as_str(),
                format!("{token} is not a canonical New Alphabet spacing token."),
                "Replace the unknown spacing token with a foundation token or a Stack or Row option.",
            ));
        }
    }

    if !surface.custom_spacing_values.is_empty() {
        report.push(message(
            "P-104",
            Severity::Error,
            ValidationCategory::Spacing,
            surface.id.as_str(),
            format!(
                "{} custom spacing values were declared, which means rhythm is being repaired locally.",
                surface.custom_spacing_values.len()
            ),
            "Delete arbitrary spacing values and move the rhythm back into Stack, Row, or panel content tokens.",
        ));
    }
}

fn validate_naming(surface: &SurfaceManifest, report: &mut ValidationReport) {
    if !surface.id.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    }) {
        report.push(message(
            "V-004",
            Severity::Warning,
            ValidationCategory::Naming,
            surface.id.as_str(),
            format!(
                "{} is not a plain lowercase role id, so the name is drifting away from the naming law.",
                surface.id
            ),
            "Rename the surface with a lowercase role id such as review-queue or docs-shell.",
        ));
    }

    let lowered = format!("{} {}", surface.id, surface.name).to_ascii_lowercase();
    if decorative_terms().iter().any(|term| lowered.contains(term)) {
        report.push(message(
            "V-004",
            Severity::Warning,
            ValidationCategory::Naming,
            surface.id.as_str(),
            format!(
                "{} uses decorative language instead of role-based naming.",
                surface.name
            ),
            "Rename the surface by structure or task rather than mood, magic, or ornament.",
        ));
    }
}

fn validate_state_coverage(surface: &SurfaceManifest, report: &mut ValidationReport) {
    for component in &surface.components {
        let required_states = component_required_states(component.id.as_str());
        if required_states.is_empty() {
            continue;
        }

        let actual_states: BTreeSet<&str> = component.states.iter().map(String::as_str).collect();
        let missing: Vec<&str> = required_states
            .iter()
            .copied()
            .filter(|state| !actual_states.contains(state))
            .collect();

        if !missing.is_empty() {
            report.push(message(
                "V-005",
                Severity::Error,
                ValidationCategory::StateCoverage,
                surface.id.as_str(),
                format!(
                    "{} is missing required states: {}.",
                    component.id,
                    missing.join(", ")
                ),
                "Add the missing state coverage or narrow the component usage until the contract is true.",
            ));
        }
    }
}

fn validate_accessibility(surface: &SurfaceManifest, report: &mut ValidationReport) {
    if !surface.accessibility.semantic_html {
        report.push(message(
            "V-006",
            Severity::Error,
            ValidationCategory::Accessibility,
            surface.id.as_str(),
            "The surface declares semantic_html = false, which violates the accessibility law."
                .to_owned(),
            "Restore semantic HTML before refining the surface design.",
        ));
    }

    if !surface.accessibility.focus_visible {
        report.push(message(
            "V-006",
            Severity::Error,
            ValidationCategory::Accessibility,
            surface.id.as_str(),
            "The surface does not guarantee focus-visible behavior.".to_owned(),
            "Restore visible focus treatment and keyboard coverage before validating again.",
        ));
    }

    if !surface.accessibility.color_independent_meaning {
        report.push(message(
            "V-007",
            Severity::Error,
            ValidationCategory::Accessibility,
            surface.id.as_str(),
            "The surface relies on color alone for meaning.".to_owned(),
            "Add explicit text and semantic status markup so meaning survives without color.",
        ));
    }

    for component in &surface.components {
        let Some(check) = COMPONENT_ACCESSIBILITY_CHECKS
            .iter()
            .find(|check| check.component == component.id)
        else {
            continue;
        };

        let requires_keyboard = check.rules.iter().any(|rule| rule.id() == "keyboard");
        let requires_name = check.rules.iter().any(|rule| rule.id() == "semantic_name");
        let requires_text = check
            .rules
            .iter()
            .any(|rule| rule.id() == "color_independent_meaning");

        if requires_name && !component.accessible_name {
            report.push(message(
                "V-006",
                Severity::Error,
                ValidationCategory::Accessibility,
                surface.id.as_str(),
                format!("{} is missing an accessible name.", component.id),
                "Bind the component to a label, aria-label, legend, or semantic text anchor.",
            ));
        }

        if requires_keyboard && !component.keyboard_support {
            report.push(message(
                "V-006",
                Severity::Error,
                ValidationCategory::Accessibility,
                surface.id.as_str(),
                format!("{} is missing keyboard support.", component.id),
                "Use the native semantic control or restore keyboard navigation behavior.",
            ));
        }

        if requires_text && !component.text_equivalent {
            report.push(message(
                "V-007",
                Severity::Error,
                ValidationCategory::Accessibility,
                surface.id.as_str(),
                format!(
                    "{} is missing a text equivalent for status meaning.",
                    component.id
                ),
                "Add explicit text so the state is legible without color or decoration.",
            ));
        }
    }
}

fn validate_anti_patterns(surface: &SurfaceManifest, report: &mut ValidationReport) {
    if surface.style_escape_hatch {
        report.push(message(
            "V-008",
            Severity::Error,
            ValidationCategory::AntiPatternUsage,
            surface.id.as_str(),
            "The manifest declares a style escape hatch, which bypasses the finite system."
                .to_owned(),
            "Delete the escape hatch and use approved primitives, components, or recipes instead.",
        ));
    }

    if surface.invented_layout {
        report.push(message(
            "AP-006",
            Severity::Error,
            ValidationCategory::AntiPatternUsage,
            surface.id.as_str(),
            "The manifest declares invented_layout = true, which means layout was improvised outside the grammar."
                .to_owned(),
            "Choose a valid recipe or primitive combination before generating more code.",
        ));
    }

    for component in &surface.components {
        if component.custom_variant_props {
            report.push(message(
                "AP-005",
                Severity::Error,
                ValidationCategory::AntiPatternUsage,
                surface.id.as_str(),
                format!(
                    "{} declares custom variant props, which reintroduces open-ended API sprawl.",
                    component.id
                ),
                "Replace the freeform variant with a finite semantic option or split the component by role.",
            ));
        }
    }
}

fn required_regions(recipe: &str) -> &'static [&'static str] {
    match recipe {
        "BlogIndex" => &["main"],
        "ArticleShell" => &["lead", "main"],
        "DocsShell" => &["rail", "main"],
        "SearchResultsWorkspace" => &["rail", "main"],
        "ReviewQueue" => &["action_band", "main", "detail"],
        "SettingsWorkspace" => &["rail", "main"],
        "DashboardShell" => &["main"],
        _ => &["main"],
    }
}

fn allowed_regions(recipe: &str) -> BTreeSet<&'static str> {
    required_regions(recipe)
        .iter()
        .copied()
        .chain(optional_regions(recipe).iter().copied())
        .collect()
}

fn optional_regions(recipe: &str) -> &'static [&'static str] {
    match recipe {
        "BlogIndex" => &["support"],
        "ArticleShell" => &["support"],
        "DocsShell" => &["detail"],
        "SearchResultsWorkspace" => &["action_band", "detail"],
        "ReviewQueue" => &["rail"],
        "SettingsWorkspace" => &["action_band", "detail"],
        "DashboardShell" => &["action_band", "detail"],
        _ => &[],
    }
}

fn allowed_spacing_tokens() -> &'static [&'static str] {
    &[
        "spacing.stack.tight",
        "spacing.stack.default",
        "spacing.stack.loose",
        "spacing.region.default",
        "spacing.region.spacious",
        "spacing.panel.content",
        "spacing.control.inset",
        "spacing.table.row",
    ]
}

fn component_required_states(component: &str) -> &'static [&'static str] {
    match component {
        "Button" | "LinkAction" => &[
            "default",
            "loading",
            "disabled",
            "hover",
            "active",
            "focus_visible",
        ],
        "TextField" | "Textarea" | "Select" | "Checkbox" | "RadioGroup" | "Switch" => {
            &["default", "error", "success", "disabled", "focus_visible"]
        }
        "Table" => &["default", "loading", "empty", "error"],
        "FilterRail" => &["default", "zero_result"],
        "DetailPane" => &["default", "loading", "unavailable"],
        _ => &[],
    }
}

fn decorative_terms() -> &'static [&'static str] {
    &[
        "vibe", "magic", "sparkle", "cute", "ornament", "mood", "delight",
    ]
}

fn message(
    rule_id: &str,
    severity: Severity,
    category: ValidationCategory,
    target: &str,
    message: String,
    repair: &str,
) -> ValidationMessage {
    ValidationMessage {
        rule_id: rule_id.to_owned(),
        severity,
        category,
        target: target.to_owned(),
        message,
        repair: Some(repair.to_owned()),
    }
}
