use crate::{
    BorderToken, ColorToken, DensityMode, MotionToken, SpacingToken, TypeToken, body_for_density,
};

const MEDIUM_BREAKPOINT_REM: u8 = 48;
const WIDE_BREAKPOINT_REM: u8 = 78;

pub fn render_stylesheet() -> String {
    let canvas_base = ColorToken::CanvasBase.value().hex;
    let canvas_elevated = ColorToken::CanvasElevated.value().hex;
    let canvas_inset = ColorToken::CanvasInset.value().hex;
    let text_primary = ColorToken::TextPrimary.value().hex;
    let text_secondary = ColorToken::TextSecondary.value().hex;
    let text_inverse = ColorToken::TextInverse.value().hex;
    let accent_primary = ColorToken::AccentPrimary.value().hex;
    let accent_muted = ColorToken::AccentMuted.value().hex;
    let border_default = ColorToken::BorderDefault.value().hex;
    let border_strong = ColorToken::BorderStrong.value().hex;
    let border_focus = ColorToken::BorderFocus.value().hex;
    let status_info = ColorToken::StatusInfo.value().hex;
    let status_success = ColorToken::StatusSuccess.value().hex;
    let status_warning = ColorToken::StatusWarning.value().hex;
    let status_error = ColorToken::StatusError.value().hex;
    let emphasis_subtle = ColorToken::EmphasisSubtle.value().hex;
    let emphasis_strong = ColorToken::EmphasisStrong.value().hex;

    let stack_tight = px(SpacingToken::StackTight.points(DensityMode::Calm));
    let stack_default = px(SpacingToken::StackDefault.points(DensityMode::Calm));
    let stack_loose = px(SpacingToken::StackLoose.points(DensityMode::Calm));
    let region_default = px(SpacingToken::RegionDefault.points(DensityMode::Calm));
    let region_spacious = px(SpacingToken::RegionSpacious.points(DensityMode::Calm));
    let panel_content = px(SpacingToken::PanelContent.points(DensityMode::Calm));
    let control_inset = px(SpacingToken::ControlInset.points(DensityMode::Calm));
    let table_row = px(SpacingToken::TableRow.points(DensityMode::Regular));

    let display_large = type_rule(TypeToken::DisplayLarge);
    let heading_one = type_rule(TypeToken::Heading1);
    let heading_two = type_rule(TypeToken::Heading2);
    let heading_three = type_rule(TypeToken::Heading3);
    let body_reading = type_rule(body_for_density(DensityMode::Calm));
    let body_default = type_rule(TypeToken::BodyDefault);
    let body_compact = type_rule(TypeToken::BodyCompact);
    let annotation = type_rule(TypeToken::AnnotationStrong);
    let data_label = type_rule(TypeToken::DataLabel);
    let data_cell = type_rule(TypeToken::DataCell);
    let data_metric = type_rule(TypeToken::DataMetric);

    let border_panel_default = BorderToken::PanelDefault.spec().width_px;
    let border_panel_strong = BorderToken::PanelStrong.spec().width_px;
    let border_control_default = BorderToken::ControlDefault.spec().width_px;
    let border_control_focus = BorderToken::ControlFocus.spec().width_px;
    let transition_fast = MotionToken::TransitionFast.spec().duration_ms;
    let transition_default = MotionToken::TransitionDefault.spec().duration_ms;

    format!(
        r#":root {{
  --na-canvas-base: {canvas_base};
  --na-canvas-elevated: {canvas_elevated};
  --na-canvas-inset: {canvas_inset};
  --na-text-primary: {text_primary};
  --na-text-secondary: {text_secondary};
  --na-text-inverse: {text_inverse};
  --na-accent-primary: {accent_primary};
  --na-accent-muted: {accent_muted};
  --na-border-default: {border_default};
  --na-border-strong: {border_strong};
  --na-border-focus: {border_focus};
  --na-status-info: {status_info};
  --na-status-success: {status_success};
  --na-status-warning: {status_warning};
  --na-status-error: {status_error};
  --na-emphasis-subtle: {emphasis_subtle};
  --na-emphasis-strong: {emphasis_strong};
  --na-space-stack-tight: {stack_tight};
  --na-space-stack-default: {stack_default};
  --na-space-stack-loose: {stack_loose};
  --na-space-region-default: {region_default};
  --na-space-region-spacious: {region_spacious};
  --na-space-panel-content: {panel_content};
  --na-space-control-inset: {control_inset};
  --na-space-table-row: {table_row};
  --na-border-panel-default: {border_panel_default}px;
  --na-border-panel-strong: {border_panel_strong}px;
  --na-border-control-default: {border_control_default}px;
  --na-border-control-focus: {border_control_focus}px;
  --na-motion-fast: {transition_fast}ms;
  --na-motion-default: {transition_default}ms;
  --na-shadow-panel: 0 1px 0 rgba(17, 17, 17, 0.06);
  --na-max-width: 88rem;
  --na-font-sans: "Helvetica Neue", "Univers", "Arial Narrow", Arial, sans-serif;
  --na-font-mono: "SF Mono", "IBM Plex Mono", Menlo, monospace;
}}

html {{
  color-scheme: light;
  background: var(--na-canvas-base);
}}

*,
*::before,
*::after {{
  box-sizing: border-box;
}}

body {{
  margin: 0;
  min-height: 100vh;
  background: linear-gradient(180deg, var(--na-canvas-base) 0%, #f1ede3 100%);
  color: var(--na-text-primary);
  font-family: var(--na-font-sans);
  {body_default}
  letter-spacing: 0.01em;
}}

a {{
  color: inherit;
  text-decoration-color: var(--na-accent-primary);
  text-decoration-thickness: 0.12em;
  text-underline-offset: 0.18em;
}}

a:hover {{
  color: var(--na-accent-primary);
}}

a:focus-visible,
button:focus-visible,
input:focus-visible,
select:focus-visible,
textarea:focus-visible {{
  outline: var(--na-border-control-focus) solid var(--na-border-focus);
  outline-offset: 2px;
}}

h1,
h2,
h3,
h4,
h5,
h6,
p,
dl,
ul,
ol,
fieldset,
table {{
  margin: 0;
}}

img {{
  max-width: 100%;
  display: block;
}}

.na-app-shell {{
  max-width: var(--na-max-width);
  margin: 0 auto;
  padding: var(--na-space-region-default);
}}

.na-app-shell[data-density="density.dense"] {{
  {body_compact}
}}

.na-app-shell[data-density="density.calm"] {{
  {body_reading}
}}

.na-band {{
  margin-bottom: var(--na-space-region-default);
  padding-bottom: var(--na-space-stack-default);
  border-bottom: var(--na-border-panel-strong) solid var(--na-border-strong);
}}

.na-page-grid {{
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  gap: var(--na-space-region-default);
  align-items: start;
}}

.na-page-grid > * {{
  min-width: 0;
}}

.na-page-grid > .na-region,
.na-page-grid > .na-rail {{
  width: 100%;
}}

@media (min-width: {medium_breakpoint}rem) {{
  .na-page-grid:has(> .na-region--main):has(> .na-region--support) {{
    grid-template-columns: minmax(0, 3fr) minmax(16rem, 1fr);
  }}

  .na-page-grid:has(> .na-rail):has(> .na-region--main):not(:has(> .na-region--detail)) {{
    grid-template-columns: minmax(14rem, 1fr) minmax(0, 3fr);
  }}
}}

@media (min-width: {wide_breakpoint}rem) {{
  .na-page-grid:has(> .na-rail):has(> .na-region--main):has(> .na-region--detail) {{
    grid-template-columns: minmax(15rem, 0.9fr) minmax(0, 2fr) minmax(18rem, 1.1fr);
  }}

  .na-page-grid:has(> .na-region--main):has(> .na-region--detail):not(:has(> .na-rail)) {{
    grid-template-columns: minmax(0, 2fr) minmax(18rem, 1fr);
  }}

  .na-page-grid:has(> .na-rail):has(> .na-region--main):not(:has(> .na-region--detail)) {{
    grid-template-columns: minmax(15rem, 1fr) minmax(0, 2.6fr);
  }}
}}

.na-panel,
.na-detail-pane,
.na-table,
.na-metric-block,
.na-command-bar,
.na-inline-alert {{
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(8px);
}}

.na-panel,
.na-detail-pane,
.na-metric-block {{
  padding: var(--na-space-panel-content);
  border: var(--na-border-panel-default) solid var(--na-border-default);
  box-shadow: var(--na-shadow-panel);
}}

.na-panel[data-border-token="border.panel.strong"] {{
  border-width: var(--na-border-panel-strong);
  border-color: var(--na-border-strong);
}}

.na-stack {{
  display: flex;
  flex-direction: column;
}}

.na-stack[data-gap-token="spacing.stack.tight"] {{
  gap: var(--na-space-stack-tight);
}}

.na-stack[data-gap-token="spacing.stack.default"] {{
  gap: var(--na-space-stack-default);
}}

.na-stack[data-gap-token="spacing.stack.loose"] {{
  gap: var(--na-space-stack-loose);
}}

.na-section-header {{
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--na-space-stack-default);
  margin-bottom: var(--na-space-stack-default);
}}

.na-section-header__body {{
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  min-width: 0;
}}

.na-section-header__body h2 {{
  {heading_two}
  text-transform: uppercase;
  letter-spacing: 0.06em;
}}

.na-band .na-section-header__body h2 {{
  {display_large}
  letter-spacing: 0.04em;
}}

.na-section-header__subtitle {{
  color: var(--na-text-secondary);
  max-width: 42rem;
  {body_default}
}}

.na-section-header__annotation {{
  color: var(--na-accent-primary);
  white-space: nowrap;
  {annotation}
  text-transform: uppercase;
  letter-spacing: 0.12em;
}}

.na-nav-index ul,
.na-filter-rail ul {{
  list-style: none;
  padding: 0;
}}

.na-nav-index ul {{
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}}

.na-nav-index li {{
  border-bottom: 1px solid var(--na-emphasis-subtle);
}}

.na-nav-index a {{
  display: block;
  padding: 0.55rem 0;
  text-transform: uppercase;
  {annotation}
  letter-spacing: 0.08em;
}}

.na-nav-index li[data-state="current"] a {{
  color: var(--na-accent-primary);
}}

.na-blog-index__entry {{
  padding-top: var(--na-space-stack-default);
  border-top: 1px solid var(--na-emphasis-subtle);
}}

.na-blog-index__entry:first-child {{
  padding-top: 0;
  border-top: 0;
}}

.na-blog-index__meta,
.na-article-shell__meta dt,
.na-workspace-context dt,
.na-detail-pane__field dt,
.na-metric-block__label {{
  color: var(--na-text-secondary);
  {annotation}
  text-transform: uppercase;
  letter-spacing: 0.08em;
}}

.na-blog-index__entry h2,
.na-article-shell__section h2,
.na-detail-pane__header h3,
.na-metric-block__value {{
  {heading_one}
  margin-top: 0.35rem;
}}

.na-blog-index__summary,
.na-article-shell p,
.na-detail-pane__summary,
.na-inline-alert__message,
.na-field-help,
.na-field-message,
.na-workspace-context dd,
.na-detail-pane__field dd {{
  color: var(--na-text-secondary);
  {body_default}
}}

.na-article-shell {{
  display: flex;
  flex-direction: column;
  gap: var(--na-space-stack-loose);
}}

.na-article-shell__section {{
  padding-top: var(--na-space-stack-default);
  border-top: 1px solid var(--na-emphasis-subtle);
}}

.na-article-shell__section:first-child {{
  padding-top: 0;
  border-top: 0;
}}

.na-article-shell__section > h2 {{
  margin-bottom: var(--na-space-stack-default);
  {heading_two}
  text-transform: uppercase;
  letter-spacing: 0.06em;
}}

.na-article-shell__meta,
.na-workspace-context,
.na-detail-pane__fields {{
  display: grid;
  gap: var(--na-space-stack-tight);
}}

.na-article-shell__meta-item,
.na-workspace-context__item,
.na-detail-pane__field {{
  display: grid;
  gap: 0.2rem;
  padding-top: 0.65rem;
  border-top: 1px solid var(--na-emphasis-subtle);
}}

.na-article-shell__meta-item:first-child,
.na-workspace-context__item:first-child,
.na-detail-pane__field:first-child {{
  padding-top: 0;
  border-top: 0;
}}

.na-link-action,
.na-button {{
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.45rem;
  min-height: 2.75rem;
  padding: 0.8rem 1rem 0.72rem;
  border: var(--na-border-control-default) solid var(--na-text-primary);
  background: transparent;
  color: var(--na-text-primary);
  {annotation}
  letter-spacing: 0.08em;
  text-transform: uppercase;
  transition:
    background-color var(--na-motion-fast) ease-out,
    color var(--na-motion-fast) ease-out,
    border-color var(--na-motion-fast) ease-out,
    opacity var(--na-motion-fast) ease-out;
}}

.na-link-action--primary,
.na-button--primary {{
  background: var(--na-text-primary);
  color: var(--na-text-inverse);
  border-color: var(--na-text-primary);
}}

.na-link-action:hover,
.na-button:hover {{
  background: var(--na-accent-primary);
  color: var(--na-text-inverse);
  border-color: var(--na-accent-primary);
}}

.na-link-action[aria-disabled="true"],
.na-button:disabled {{
  opacity: 0.45;
  pointer-events: none;
}}

.na-command-bar {{
  display: flex;
  flex-direction: column;
  gap: var(--na-space-stack-tight);
  padding: var(--na-space-panel-content);
  border: var(--na-border-panel-default) solid var(--na-border-default);
}}

.na-command-bar__secondary {{
  display: flex;
  flex-wrap: wrap;
  gap: 0.65rem;
}}

.na-inline-alert {{
  padding: 1rem 1rem 1rem 1.15rem;
  border: 1px solid var(--na-border-default);
  border-left: 4px solid var(--na-status-info);
}}

.na-inline-alert[data-severity="success"] {{
  border-left-color: var(--na-status-success);
}}

.na-inline-alert[data-severity="warning"] {{
  border-left-color: var(--na-status-warning);
}}

.na-inline-alert[data-severity="error"] {{
  border-left-color: var(--na-status-error);
}}

.na-inline-alert__title {{
  {annotation}
  letter-spacing: 0.08em;
  text-transform: uppercase;
  margin-bottom: 0.35rem;
}}

.na-text-field,
.na-select,
.na-textarea,
.na-checkbox,
.na-switch {{
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}}

.na-text-field label,
.na-select label,
.na-textarea label,
.na-checkbox span,
.na-switch span,
.na-filter-rail legend {{
  {annotation}
  letter-spacing: 0.08em;
  text-transform: uppercase;
}}

.na-text-field input,
.na-select select,
.na-textarea textarea {{
  width: 100%;
  padding: 0.85rem var(--na-space-control-inset);
  border: var(--na-border-control-default) solid var(--na-border-default);
  background: var(--na-canvas-elevated);
  color: var(--na-text-primary);
  font: inherit;
  border-radius: 0;
}}

.na-checkbox label,
.na-switch label {{
  display: flex;
  align-items: center;
  gap: 0.75rem;
}}

.na-checkbox input,
.na-switch input {{
  width: 1rem;
  height: 1rem;
  accent-color: var(--na-accent-primary);
}}

.na-filter-rail {{
  display: flex;
  flex-direction: column;
  gap: var(--na-space-stack-tight);
}}

.na-filter-rail h3 {{
  {heading_three}
  text-transform: uppercase;
  letter-spacing: 0.06em;
}}

.na-filter-rail__group {{
  padding: var(--na-space-stack-tight);
  border: 1px solid var(--na-emphasis-subtle);
  background: rgba(236, 231, 218, 0.5);
}}

.na-filter-rail__group label {{
  display: grid;
  grid-template-columns: auto 1fr auto;
  align-items: center;
  gap: 0.65rem;
  padding: 0.4rem 0;
}}

.na-filter-rail__label {{
  {body_default}
}}

.na-filter-rail__count {{
  color: var(--na-text-secondary);
  {annotation}
}}

.na-table {{
  overflow-x: auto;
  padding: var(--na-space-panel-content);
  border: var(--na-border-panel-default) solid var(--na-border-default);
}}

.na-table table {{
  width: 100%;
  border-collapse: collapse;
}}

.na-table caption {{
  margin-bottom: var(--na-space-stack-tight);
  text-align: left;
  color: var(--na-text-secondary);
  {annotation}
  text-transform: uppercase;
  letter-spacing: 0.08em;
}}

.na-table th,
.na-table td {{
  padding: 0.8rem 0;
  border-bottom: 1px solid var(--na-emphasis-subtle);
  text-align: left;
  vertical-align: top;
}}

.na-table th {{
  color: var(--na-text-secondary);
  {data_label}
  text-transform: uppercase;
  letter-spacing: 0.08em;
}}

.na-table td {{
  {data_cell}
}}

.na-table td[data-cell-mode="truncate"] {{
  max-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}}

.na-table__status {{
  padding: 1rem 0;
  color: var(--na-text-secondary);
}}

.na-detail-pane__header {{
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  margin-bottom: var(--na-space-stack-default);
}}

.na-pagination {{
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}}

.na-pagination__status {{
  color: var(--na-text-secondary);
  {annotation}
  text-transform: uppercase;
}}

.na-pagination__control {{
  {annotation}
  text-transform: uppercase;
  letter-spacing: 0.08em;
}}

.na-metric-block {{
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}}

.na-metric-block__value {{
  {data_metric}
}}

.na-metric-block__context,
.na-metric-block__note {{
  color: var(--na-text-secondary);
  {body_default}
}}

@media (max-width: {medium_breakpoint}rem) {{
  .na-app-shell {{
    padding: var(--na-space-stack-loose);
  }}

  .na-section-header,
  .na-command-bar,
  .na-pagination {{
    flex-direction: column;
    align-items: flex-start;
  }}
}}

@media (prefers-reduced-motion: reduce) {{
  *,
  *::before,
  *::after {{
    animation-duration: 0ms !important;
    transition-duration: 0ms !important;
    scroll-behavior: auto !important;
  }}
}}
"#,
        medium_breakpoint = MEDIUM_BREAKPOINT_REM,
        wide_breakpoint = WIDE_BREAKPOINT_REM,
    )
}

fn px(value: u8) -> String {
    format!("{value}px")
}

fn type_rule(token: TypeToken) -> String {
    let style = token.style();
    format!(
        "font-size: {}px;\n  line-height: {}px;\n  font-weight: {};",
        style.size_px, style.line_height_px, style.weight
    )
}

#[cfg(test)]
mod tests {
    use super::render_stylesheet;

    #[test]
    fn stylesheet_contains_foundation_variables_and_core_classes() {
        let stylesheet = render_stylesheet();

        assert!(stylesheet.contains("--na-canvas-base"));
        assert!(stylesheet.contains(".na-page-grid"));
        assert!(stylesheet.contains(".na-panel"));
        assert!(stylesheet.contains(".na-link-action"));
        assert!(stylesheet.contains("@media (prefers-reduced-motion: reduce)"));
    }
}
