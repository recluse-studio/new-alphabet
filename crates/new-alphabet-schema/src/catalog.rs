use new_alphabet_components::COMPONENT_ACCESSIBILITY_CHECKS;
use new_alphabet_core::{
    AntiPattern, ComponentContract, CompositionRule, CompositionRuleKind, ContractBundle,
    ContractOptionGroup, DoctrineSummary, FlavorContract, FoundationFamily, FoundationToken,
    IntentKind, LayerKind, NamedValue, PrimitiveContract, ProjectManifest, PromptIntent,
    PromptLevel, RecipeContract, ReferenceExample, SchemaDocument, Severity, StateContract,
    SurfaceManifest, SurfaceRegion, ValidationCategory, ValidationRule,
};
use new_alphabet_foundation::{
    ALL_BORDER_TOKENS, ALL_BREAKPOINTS, ALL_COLOR_TOKENS, ALL_DENSITY_MODES, ALL_MOTION_TOKENS,
    ALL_RAIL_WIDTH_TOKENS, ALL_RADIUS_TOKENS, ALL_REGION_CLASSES, ALL_SPACING_TOKENS,
    ALL_STATE_TOKENS, ALL_TYPE_TOKENS, FOUNDATION_SPEC,
};
use serde_json::{Error, Value, json};

pub fn bundle_format_version() -> &'static str {
    "0.1.0"
}

pub fn contract_bundle() -> ContractBundle {
    ContractBundle {
        bundle_format_version: bundle_format_version().to_owned(),
        doctrine: doctrine_summary(),
        foundations: foundation_families(),
        flavors: flavor_contracts(),
        primitives: primitive_contracts(),
        components: component_contracts(),
        recipes: recipe_contracts(),
        composition_rules: composition_rules(),
        state_contracts: state_contracts(),
        anti_patterns: anti_patterns(),
        validation_rules: validation_rules(),
        schemas: schema_documents(),
        examples: reference_examples(),
        prompt_intents: prompt_intents(),
        generation_sequence: vec![
            "Interpret user intent.".to_owned(),
            "Choose an explicit runtime flavor when the task names a host stack or platform boundary.".to_owned(),
            "Choose a recipe or primitive composition before writing UI code.".to_owned(),
            "Select semantic components that satisfy the required states.".to_owned(),
            "Generate only valid structures and finite variants.".to_owned(),
            "Run validation and explain the result in framework terms.".to_owned(),
        ],
        repair_sequence: vec![
            "Inspect the current structure.".to_owned(),
            "Confirm that the runtime flavor still matches the named host stack and platform.".to_owned(),
            "Identify which constitutional rule is being violated.".to_owned(),
            "Prefer narrowing or deleting invalid structure before adding new abstractions."
                .to_owned(),
            "Re-validate after repair.".to_owned(),
        ],
    }
}

pub fn serialize_bundle_pretty() -> Result<String, Error> {
    serde_json::to_string_pretty(&contract_bundle())
}

pub fn serialize_manifest_pretty(manifest: &ProjectManifest) -> Result<String, Error> {
    serde_json::to_string_pretty(manifest)
}

pub fn example_project_manifest() -> ProjectManifest {
    ProjectManifest {
        schema_version: bundle_format_version().to_owned(),
        project_name: "new-alphabet-proof".to_owned(),
        surfaces: vec![SurfaceManifest {
            id: "review-queue".to_owned(),
            name: "Review queue".to_owned(),
            intent: IntentKind::Workspace,
            recipe: "ReviewQueue".to_owned(),
            regions: vec![
                SurfaceRegion {
                    kind: "action_band".to_owned(),
                    placement: "action_band".to_owned(),
                },
                SurfaceRegion {
                    kind: "rail".to_owned(),
                    placement: "rail_start".to_owned(),
                },
                SurfaceRegion {
                    kind: "main".to_owned(),
                    placement: "main".to_owned(),
                },
                SurfaceRegion {
                    kind: "detail".to_owned(),
                    placement: "detail".to_owned(),
                },
            ],
            primitives: vec![
                "AppShell".to_owned(),
                "PageGrid".to_owned(),
                "Rail".to_owned(),
                "Region".to_owned(),
                "Panel".to_owned(),
                "Stack".to_owned(),
                "Band".to_owned(),
            ],
            components: vec![
                component_instance("Table", &["default", "loading", "empty", "error"]),
                component_instance("FilterRail", &["default", "zero_result"]),
                component_instance("DetailPane", &["default", "loading", "unavailable"]),
                component_instance("CommandBar", &[]),
                component_instance("NavIndex", &[]),
                component_instance("InlineAlert", &[]),
            ],
            spacing_tokens: vec![
                "spacing.stack.tight".to_owned(),
                "spacing.stack.default".to_owned(),
                "spacing.panel.content".to_owned(),
            ],
            custom_spacing_values: Vec::new(),
            accessibility: new_alphabet_core::AccessibilitySnapshot {
                semantic_html: true,
                focus_visible: true,
                color_independent_meaning: true,
            },
            style_escape_hatch: false,
            invented_layout: false,
        }],
    }
}

fn component_instance(id: &str, states: &[&str]) -> new_alphabet_core::ComponentInstance {
    new_alphabet_core::ComponentInstance {
        id: id.to_owned(),
        states: text_list(states),
        accessible_name: true,
        keyboard_support: true,
        text_equivalent: true,
        custom_variant_props: false,
    }
}

fn doctrine_summary() -> DoctrineSummary {
    DoctrineSummary {
        thesis: "New Alphabet is a strict, grid-governed, typographic design constitution for Rust and Leptos.".to_owned(),
        posture: text_list(&[
            "order before ornament",
            "grid as operating law",
            "information over personality",
            "constraint over styling freedom",
            "systems over pages",
            "agents need rules not vibes",
        ]),
        required_session_context: text_list(&[
            "foundations",
            "flavors",
            "primitives",
            "components",
            "recipes",
            "composition_rules",
            "state_contracts",
            "anti_patterns",
            "examples",
        ]),
        hard_rules: text_list(&[
            "Do not invent layout outside the defined primitive and recipe model unless the PRD is expanded.",
            "Do not weaken the doctrine to make generation easier.",
            "Do not hide uncertainty behind decorative output.",
        ]),
    }
}

fn foundation_families() -> Vec<FoundationFamily> {
    vec![
        FoundationFamily {
            id: "foundation.runtime".to_owned(),
            title: "Runtime".to_owned(),
            tokens: vec![FoundationToken {
                id: "foundation.spec".to_owned(),
                description: "The runtime foundation is SSR-safe and hydration-ready.".to_owned(),
                values: vec![
                    named_value("version", FOUNDATION_SPEC.version),
                    named_value("ssr_safe", FOUNDATION_SPEC.ssr_safe.to_string()),
                    named_value(
                        "hydration_ready",
                        FOUNDATION_SPEC.hydration_ready.to_string(),
                    ),
                ],
            }],
        },
        FoundationFamily {
            id: "foundation.grid".to_owned(),
            title: "Grid and breakpoints".to_owned(),
            tokens: ALL_BREAKPOINTS
                .into_iter()
                .map(|breakpoint| FoundationToken {
                    id: breakpoint.id().to_owned(),
                    description: "Named breakpoint columns are fixed by law.".to_owned(),
                    values: vec![named_value("columns", breakpoint.columns().to_string())],
                })
                .chain(
                    ALL_REGION_CLASSES
                        .into_iter()
                        .map(|region| FoundationToken {
                            id: format!("region.{}", region.id()),
                            description: "Region spans stay explicit across breakpoints."
                                .to_owned(),
                            values: vec![
                                named_value("compact", span_value(region.spans().compact)),
                                named_value("medium", span_value(region.spans().medium)),
                                named_value("wide", span_value(region.spans().wide)),
                            ],
                        }),
                )
                .chain(
                    ALL_RAIL_WIDTH_TOKENS
                        .into_iter()
                        .map(|width| FoundationToken {
                            id: width.id().to_owned(),
                            description:
                                "Rail widths collapse on compact and remain finite elsewhere."
                                    .to_owned(),
                            values: vec![
                                named_value(
                                    "medium_columns",
                                    width
                                        .columns(new_alphabet_foundation::Breakpoint::Medium)
                                        .map(|columns| columns.to_string())
                                        .unwrap_or_else(|| "stacked".to_owned()),
                                ),
                                named_value(
                                    "wide_columns",
                                    width
                                        .columns(new_alphabet_foundation::Breakpoint::Wide)
                                        .map(|columns| columns.to_string())
                                        .unwrap_or_else(|| "stacked".to_owned()),
                                ),
                            ],
                        }),
                )
                .collect(),
        },
        FoundationFamily {
            id: "foundation.spacing".to_owned(),
            title: "Spacing".to_owned(),
            tokens: ALL_SPACING_TOKENS
                .into_iter()
                .map(|token| FoundationToken {
                    id: token.id().to_owned(),
                    description: "Spacing is finite and density-aware.".to_owned(),
                    values: ALL_DENSITY_MODES
                        .into_iter()
                        .map(|density| named_value(density.id(), token.points(density).to_string()))
                        .collect(),
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.type".to_owned(),
            title: "Type".to_owned(),
            tokens: ALL_TYPE_TOKENS
                .into_iter()
                .map(|token| {
                    let style = token.style();
                    FoundationToken {
                        id: token.id().to_owned(),
                        description:
                            "Type roles are shared across editorial and workflow surfaces."
                                .to_owned(),
                        values: vec![
                            named_value("size_px", style.size_px.to_string()),
                            named_value("line_height_px", style.line_height_px.to_string()),
                            named_value("weight", style.weight.to_string()),
                        ],
                    }
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.density".to_owned(),
            title: "Density".to_owned(),
            tokens: ALL_DENSITY_MODES
                .into_iter()
                .map(|density| FoundationToken {
                    id: density.id().to_owned(),
                    description: "Density alters spacing and typographic rhythm together."
                        .to_owned(),
                    values: Vec::new(),
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.color".to_owned(),
            title: "Color".to_owned(),
            tokens: ALL_COLOR_TOKENS
                .into_iter()
                .map(|token| FoundationToken {
                    id: token.id().to_owned(),
                    description: "Color roles stay semantic and finite.".to_owned(),
                    values: vec![named_value("hex", token.value().hex)],
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.border".to_owned(),
            title: "Border".to_owned(),
            tokens: ALL_BORDER_TOKENS
                .into_iter()
                .map(|token| {
                    let spec = token.spec();
                    FoundationToken {
                        id: token.id().to_owned(),
                        description:
                            "Borders carry structural emphasis instead of decorative styling."
                                .to_owned(),
                        values: vec![
                            named_value("width_px", spec.width_px.to_string()),
                            named_value("color", spec.color.id()),
                        ],
                    }
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.radius".to_owned(),
            title: "Radius".to_owned(),
            tokens: ALL_RADIUS_TOKENS
                .into_iter()
                .map(|token| {
                    let spec = token.spec();
                    FoundationToken {
                        id: token.id().to_owned(),
                        description:
                            "Corner radius stays subtle and fixed so surfaces remain severe."
                                .to_owned(),
                        values: vec![named_value("value_px", spec.value_px.to_string())],
                    }
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.motion".to_owned(),
            title: "Motion".to_owned(),
            tokens: ALL_MOTION_TOKENS
                .into_iter()
                .map(|token| {
                    let spec = token.spec();
                    FoundationToken {
                        id: token.id().to_owned(),
                        description: "Motion remains short, purposeful, and reduced-motion aware."
                            .to_owned(),
                        values: vec![
                            named_value("duration_ms", spec.duration_ms.to_string()),
                            named_value("easing", spec.easing),
                            named_value("reduced", spec.reduced.to_string()),
                        ],
                    }
                })
                .collect(),
        },
        FoundationFamily {
            id: "foundation.state".to_owned(),
            title: "State".to_owned(),
            tokens: ALL_STATE_TOKENS
                .into_iter()
                .map(|token| {
                    let spec = token.spec();
                    FoundationToken {
                        id: token.id().to_owned(),
                        description: "Global state tokens stay explicit and reusable.".to_owned(),
                        values: vec![
                            named_value(
                                "border",
                                spec.border
                                    .map(|border| border.id().to_owned())
                                    .unwrap_or_else(|| "none".to_owned()),
                            ),
                            named_value(
                                "color",
                                spec.color
                                    .map(|color| color.id().to_owned())
                                    .unwrap_or_else(|| "none".to_owned()),
                            ),
                            named_value("opacity_percent", spec.opacity_percent.to_string()),
                        ],
                    }
                })
                .collect(),
        },
    ]
}

fn flavor_contracts() -> Vec<FlavorContract> {
    vec![
        FlavorContract {
            id: "LeptosSsr".to_owned(),
            runtime: "Leptos SSR and hydration".to_owned(),
            purpose:
                "Canonical New Alphabet web runtime for editorial and workspace surfaces."
                    .to_owned(),
            stack: text_list(&["Leptos", "semantic HTML", "SSR", "hydration"]),
            required_bindings: text_list(&[
                "Leptos owns routing, shell composition, and hydration-safe state.",
                "New Alphabet foundations emit the stylesheet and token law.",
                "Recipes remain the first choice for surface geometry.",
            ]),
            laws: text_list(&[
                "The constitution stays primary and hydration must not change layout meaning.",
                "Semantic HTML stays visible rather than disappearing behind opaque wrappers.",
                "First builds keep the canonical New Alphabet palette strict.",
                "Corners use the subtle radius token instead of improvised rounding.",
            ]),
            anti_patterns: text_list(&[
                "Hydration-only layout shifts.",
                "Router chrome that displaces the primary work surface.",
                "Runtime palette improvisation or theme-family sprawl at V0.",
            ]),
        },
        FlavorContract {
            id: "DioxusDesktopWorkbench".to_owned(),
            runtime: "Dioxus desktop".to_owned(),
            purpose: "Dense desktop analytics binding for table-first work surfaces with native Rust services.".to_owned(),
            stack: text_list(&["Dioxus Desktop", "Polars", "Charton", "SVG output"]),
            required_bindings: text_list(&[
                "Dioxus owns the shell, pane layout, and local interaction state.",
                "Polars owns filtering, shaping, grouping, aggregation, sorting, and export slices.",
                "Charton owns chart grammar, scales, legends, and SVG rendering output.",
            ]),
            laws: text_list(&[
                "The application is a workbench, not a dashboard mural or marketing page.",
                "Query state is shared so table, chart, summary metrics, and inspector derive from the same Rust state.",
                "Charts live in bounded panes and stay subordinate to the table or list.",
                "Use left rail, central work surface, and right analysis pane before inventing new geometry.",
                "First builds keep the canonical New Alphabet palette strict and later adjustments may only remap semantic color roles.",
                "Corners use the subtle radius token across panes, controls, and chart containers.",
            ]),
            anti_patterns: text_list(&[
                "Hand-drawn chart primitives in Dioxus when Charton can express the chart.",
                "Full-window charts that push the primary table below the fold.",
                "Oversized cards, welcome states, or web-marketing headers on data screens.",
            ]),
        },
        FlavorContract {
            id: "TauriWorkbench".to_owned(),
            runtime: "Tauri shell with Leptos or Yew frontend".to_owned(),
            purpose: "Desktop workbench binding for a thin web-style frontend and native Rust data commands.".to_owned(),
            stack: text_list(&[
                "Tauri",
                "Leptos or Yew",
                "Polars",
                "Charton",
                "SVG output",
                "Vega-Lite JSON",
            ]),
            required_bindings: text_list(&[
                "Tauri owns the native shell and command boundary.",
                "The frontend owns shell layout, interaction, and direct SVG embedding.",
                "Polars and Charton live in native Rust commands or native state rather than frontend WASM by default.",
                "Frontend requests shaped table slices, chart SVG, and optional chart spec JSON instead of raw datasets.",
            ]),
            laws: text_list(&[
                "Heavy aggregation stays in Rust and never migrates into the frontend view layer.",
                "Use CSS grid for shell geometry with fixed sidebar width, fixed toolbar height, minmax(0, 1fr) main surface, and explicit chart-pane bounds.",
                "Chart transport prefers SVG first, Vega-Lite JSON second, and PNG only for export or fallback.",
                "Charts embed directly in bounded panes, preserve viewBox, use explicit compact height, and never push filters or toolbar controls below the fold.",
                "Command boundaries stay coarse: one command for query refresh, one for chart refresh, and one for export.",
                "The table remains the anchor surface and stays legible at 1440x900.",
                "First builds keep the canonical New Alphabet palette strict and corners use the subtle radius token.",
            ]),
            anti_patterns: text_list(&[
                "Heavy aggregation or schema shaping in frontend WASM.",
                "Dozens of chatty Tauri command calls for one user action.",
                "Ad hoc nested flex shells that weaken the workbench hierarchy.",
                "Charts that dominate the window or collapse the table below the fold.",
            ]),
        },
        FlavorContract {
            id: "DesktopHtmlWorkbench".to_owned(),
            runtime:
                "Desktop shell with Leptos, Yew, Dioxus, or Azul using HTML and CSS layout semantics"
                    .to_owned(),
            purpose: "General dense desktop workbench binding for Rust UI stacks that compose the window as an application surface rather than a website.".to_owned(),
            stack: text_list(&[
                "Tauri",
                "Leptos",
                "Yew",
                "Dioxus",
                "Azul",
                "HTML semantics",
                "CSS grid or disciplined flex",
            ]),
            required_bindings: text_list(&[
                "Build the shell first and treat the window as an application surface rather than a centered page.",
                "Use a desktop shell with a left sidebar between 232 and 256, a top toolbar between 34 and 38, an optional right inspector between 280 and 320, and a central work surface that resolves to minmax(0, 1fr).",
                "Use CSS grid or disciplined flex composition for shell geometry rather than ad hoc nested wrappers.",
                "Keep the first viewport at 1440x900 on the actual work surface instead of on headers, banners, or decorative chrome.",
            ]),
            laws: text_list(&[
                "Page titles stay at or below 20px, section titles stay between 15 and 16px, body text stays at 13px, and meta text stays at 12px.",
                "Controls stay around 32px tall and table or list rows stay between 28 and 32px so density reads as desktop work rather than responsive marketing UI.",
                "Use the gap scale 4, 6, 8, and 12, keep outer padding at 8, and keep panel padding at 8.",
                "Prefer list and detail, table and detail, split panes, compact forms, sticky toolbars, sticky table headers, plain surfaces, and thin separators.",
                "Do not use centered page wrappers, max-width content columns, hero headers, giant empty banners, oversized cards for routine content, or vertical stacks of big full-width panels.",
                "Avoid any area below the fold caused by non-content chrome and make real working rows visible immediately at 1440x900.",
                "Self-audit should report shell grid or pane structure, sidebar width, toolbar height, body font size, visible rows at 1440x900, and any below-the-fold loss caused by chrome.",
                "First builds keep the canonical New Alphabet palette strict and corners use the subtle radius token.",
            ]),
            anti_patterns: text_list(&[
                "Responsive marketing tropes inside a desktop window.",
                "Giant Tailwind-like spacing, decorative gradient backgrounds, or card mosaics.",
                "Centered empty states when data exists or nested wrappers that each add padding.",
                "Chrome-heavy shells that push useful work below the fold.",
            ]),
        },
        FlavorContract {
            id: "FloemWorkbench".to_owned(),
            runtime: "Floem desktop".to_owned(),
            purpose: "Dense desktop workbench binding for Floem with disciplined stacks, wrapper restraint, and native Rust data services.".to_owned(),
            stack: text_list(&["Floem", "Polars", "Charton", "SVG output"]),
            required_bindings: text_list(&[
                "Floem owns the shell through h_stack and v_stack composition rather than free layout invention.",
                "One root style layer sets shared density and typography and child regions override it rarely.",
                "Polars owns file reads, lazy queries, joins, filters, aggregations, table slices, and export-ready datasets.",
                "Charton owns chart grammar, theming, axis policy, legend policy, and SVG-first output.",
            ]),
            laws: text_list(&[
                "Dense shell bindings use body 13, meta 12, section title 16, page title 20, control height 32, toolbar height 36, sidebar width 240, inspector width 300, gap x 8, gap y 6, and panel padding 8.",
                "Use container only for a real style boundary, clip boundary, or region identity and report wrapper count per major region.",
                "Use virtual_stack for long lists and keep scrolling inside the main content region.",
                "Rows should read as desktop rows rather than mobile cards and the first render must show real work instead of decorative empty space.",
                "The table remains primary, the chart remains explanatory, and no chart should push the working table below the fold.",
                "No spacing value should exceed 12 unless the separation is structurally necessary and explicitly explained.",
                "Business aggregation never lives in the view layer and chart geometry is never hand-assembled when Charton can express it declaratively.",
                "First builds keep the canonical New Alphabet palette strict and corners use the subtle radius token.",
            ]),
            anti_patterns: text_list(&[
                "Casual container wrappers that add padding without region identity.",
                "Marketing-card layouts, giant headers, or decorative empty regions.",
                "Heavy aggregation or chart logic in the Floem view layer.",
                "Global scrolling that weakens the fixed workbench shell.",
            ]),
        },
        FlavorContract {
            id: "GpuiWorkbench".to_owned(),
            runtime: "GPUI desktop".to_owned(),
            purpose: "Dense desktop tool binding for GPUI with disciplined pane structure, compact controls, and work-first surfaces.".to_owned(),
            stack: text_list(&["GPUI", "Polars", "Charton", "SVG output"]),
            required_bindings: text_list(&[
                "GPUI owns the application shell through horizontal and vertical flex composition rather than freeform canvas behavior.",
                "The shell keeps a fixed left navigation width, a flexible center pane, an optional fixed right inspector, and one compact top toolbar row.",
                "Polars owns file reads, lazy queries, joins, filters, aggregations, table slices, and export-ready datasets.",
                "Charton owns chart grammar, theming, axis policy, legend policy, and SVG-first output.",
            ]),
            laws: text_list(&[
                "Default text reads around 13px, page titles stay at or below 20px, and compact controls stay around 32px tall.",
                "Use gaps mostly 4, 6, or 8 and padding mostly 6 or 8 so density stays tight and predictable.",
                "Scrolling lives in the content region rather than across the whole shell.",
                "Prefer virtualized list or table components for dense datasets and report virtualization usage in self-audit.",
                "Prefer dock and split-pane patterns for multi-surface tools and simple surfaces over card piles.",
                "Leaf controls shrink to content or fixed compact sizes and full-size leaf widgets appear only when the leaf is the real editor or table.",
                "The center pane must show useful content immediately at 1440x900.",
                "First builds keep the canonical New Alphabet palette strict and corners use the subtle radius token.",
            ]),
            anti_patterns: text_list(&[
                "Freeform canvas composition that weakens structured pane hierarchy.",
                "Giant panes with one tiny child.",
                "Repeated banner alerts, oversized iconography, or decorative chrome that steals vertical space.",
                "Nested padding on every wrapper or giant flex children used as decorative whitespace.",
            ]),
        },
        FlavorContract {
            id: "Relm4Workbench".to_owned(),
            runtime: "Relm4 with GTK4 and libadwaita".to_owned(),
            purpose: "Native desktop workbench binding for Relm4 with GTK structure, compact density, and serious data surfaces.".to_owned(),
            stack: text_list(&["Relm4", "GTK4", "libadwaita", "Polars", "Charton", "SVG output"]),
            required_bindings: text_list(&[
                "Use ApplicationWindow as the root shell and a compact HeaderBar or top toolbar as the primary action row.",
                "Use Paned for two- or three-pane shells and ScrolledWindow around main content regions.",
                "Use ListView or ColumnView for datasets, Stack or StackSwitcher for major modes, and dialogs or popovers only for secondary actions.",
                "Polars owns file reads, lazy queries, joins, filters, aggregations, table slices, and export-ready datasets.",
                "Charton owns chart grammar, theming, axis policy, legend policy, and SVG-first output.",
            ]),
            laws: text_list(&[
                "Sidebar width-request stays between 232 and 256 and inspector width-request stays between 280 and 320.",
                "Spacing and margins stay mostly between 6 and 8 so controls read as compact desktop controls rather than tablet controls.",
                "Assign css-classes to all major regions and theme with GTK CSS instead of per-widget styling hacks.",
                "Use ColumnView for multi-column data rather than nested boxes pretending to be a table.",
                "Do not nest Box inside Box inside Box to fake grid structure and do not rely on card-heavy framing.",
                "Dense rows ellipsize long labels, keep row height stable, and avoid unexpected wrapping.",
                "Primary actions stay visible without scrolling and scrolling remains local to the main content region.",
                "Use subtle borders or separators where useful and avoid giant title blocks or oversized empty states.",
                "First builds keep the canonical New Alphabet palette strict and corners use the subtle radius token when CSS rounding is applied.",
            ]),
            anti_patterns: text_list(&[
                "Nested box hierarchies used as fake tables or grids.",
                "Wrapped row labels that expand dense list height unexpectedly.",
                "Card-heavy framing, oversized empty states, or decorative chrome that weakens the native workbench feel.",
                "Per-widget style hacks that bypass shared GTK CSS classes.",
            ]),
        },
    ]
}

fn primitive_contracts() -> Vec<PrimitiveContract> {
    vec![
        primitive_contract(
            "AppShell",
            "Root frame that binds density and surface intent.",
            &[],
            &["density", "intent"],
            &[
                option_group(
                    "density",
                    &["density.calm", "density.regular", "density.dense"],
                ),
                option_group("intent", &["editorial", "workspace"]),
            ],
            &["Every serious screen begins inside AppShell before any local composition."],
            &["EditorialAnchorExample", "WorkspaceAnchorExample"],
        ),
        primitive_contract(
            "PageGrid",
            "Canonical page geometry with fixed breakpoint columns.",
            &[],
            &["intent"],
            &[option_group("intent", &["editorial", "workspace"])],
            &[
                "PageGrid must contain a named main region and may support rails, detail, or action bands.",
            ],
            &["EditorialStructureExample", "WorkflowStructureExample"],
        ),
        primitive_contract(
            "Region",
            "Named page regions with explicit span behavior.",
            &["kind", "placement"],
            &["tag"],
            &[
                option_group(
                    "kind",
                    &["full", "main", "support", "rail", "detail", "action_band"],
                ),
                option_group(
                    "placement",
                    &[
                        "lead",
                        "main",
                        "support",
                        "rail_start",
                        "rail_end",
                        "detail",
                        "action_band",
                    ],
                ),
                option_group("tag", &["main", "section", "aside", "nav", "header"]),
            ],
            &["Regions remain structural and must not be repurposed as decorative wrappers."],
            &["EditorialAnchorExample", "WorkspaceAnchorExample"],
        ),
        primitive_contract(
            "Rail",
            "Bounded side structure for navigation or filters.",
            &[],
            &["width", "side"],
            &[
                option_group(
                    "width",
                    &[
                        "layout.rail.narrow.width",
                        "layout.rail.default.width",
                        "layout.rail.broad.width",
                    ],
                ),
                option_group("side", &["start", "end"]),
            ],
            &["Rails support the main region; they do not replace it or multiply without need."],
            &["WorkflowStructureExample", "SettingsSurfaceExample"],
        ),
        primitive_contract(
            "Stack",
            "Vertical rhythm governed by spacing tokens.",
            &[],
            &["spacing"],
            &[option_group(
                "spacing",
                &[
                    "spacing.stack.tight",
                    "spacing.stack.default",
                    "spacing.stack.loose",
                ],
            )],
            &["Use Stack instead of ad hoc wrappers when local rhythm is needed."],
            &["EditorialStructureExample", "WorkflowStructureExample"],
        ),
        primitive_contract(
            "Row",
            "Local horizontal alignment for dense operational groupings.",
            &[],
            &["align", "distribution", "gap"],
            &[
                option_group("align", &["start", "center", "baseline"]),
                option_group("distribution", &["start", "between"]),
                option_group("gap", &["spacing.stack.tight", "spacing.stack.default"]),
            ],
            &["Rows stay local inside the main region and do not replace page geometry."],
            &["WorkflowStructureExample"],
        ),
        primitive_contract(
            "ColumnGroup",
            "Finite multi-column subdivision inside a named region.",
            &[],
            &["preset"],
            &[option_group("preset", &["two", "three", "main_support"])],
            &["ColumnGroup is for internal subdivision, not for rebuilding the page grid."],
            &["EditorialStructureExample"],
        ),
        primitive_contract(
            "Panel",
            "Bounded surface with explicit hierarchy and state.",
            &[],
            &["strength", "state"],
            &[
                option_group("strength", &["default", "strong"]),
                option_group("state", &["default", "loading", "success", "error"]),
            ],
            &["Panel hierarchy depends on borders and state tokens rather than ornament."],
            &["EditorialSurfaceExample", "SettingsSurfaceExample"],
        ),
        primitive_contract(
            "Band",
            "Full-width structural framing surface.",
            &[],
            &["strength"],
            &[option_group("strength", &["default", "strong"])],
            &["Bands remain typographic and structural rather than promotional."],
            &["EditorialSurfaceExample", "WorkflowStructureExample"],
        ),
        primitive_contract(
            "SectionHeader",
            "Shared structural heading for panels, bands, and regions.",
            &["title"],
            &["subtitle", "annotation"],
            &[],
            &["SectionHeader keeps hierarchy typographic and quiet across the system."],
            &["EditorialSurfaceExample", "SettingsSurfaceExample"],
        ),
        primitive_contract(
            "Divider",
            "Explicit separation marker for bounded surfaces.",
            &[],
            &["strength"],
            &[option_group("strength", &["default", "strong"])],
            &["Use Divider when the structure needs a visible seam, not extra space."],
            &["SettingsSurfaceExample"],
        ),
    ]
}

fn component_contracts() -> Vec<ComponentContract> {
    vec![
        component_contract(
            "Button",
            "Primary and secondary action trigger with explicit activity states.",
            &["Panel", "SectionHeader"],
            &[
                "default",
                "loading",
                "disabled",
                "hover",
                "active",
                "focus_visible",
            ],
            &["semantic_name", "keyboard", "focus_visible"],
            &[
                option_group("priority", &["primary", "secondary", "tertiary"]),
                option_group("type", &["button", "submit", "reset"]),
            ],
            &["WorkflowActionExample", "EditorialActionExample"],
        ),
        component_contract(
            "LinkAction",
            "Semantic link action for secondary or navigational operations.",
            &["Panel", "SectionHeader"],
            &["default", "hover", "active", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[option_group(
                "priority",
                &["primary", "secondary", "tertiary"],
            )],
            &["WorkflowActionExample", "EditorialActionExample"],
        ),
        component_contract(
            "TextField",
            "Single-line text input with explicit label, help, and message binding.",
            &["Panel", "Stack"],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["SettingsFieldExample", "FormFieldExample"],
        ),
        component_contract(
            "Textarea",
            "Multi-line text input with the same field-state law as TextField.",
            &["Panel", "Stack"],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["FormFieldExample"],
        ),
        component_contract(
            "Select",
            "Finite choice picker bound to named options only.",
            &["Panel", "Stack"],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["SettingsChoiceExample", "FormChoiceExample"],
        ),
        component_contract(
            "Checkbox",
            "Boolean choice control with native semantics.",
            &["Panel", "Stack"],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["SettingsChoiceExample", "FormChoiceExample"],
        ),
        component_contract(
            "RadioGroup",
            "Single-choice group for named alternatives.",
            &["Panel", "Stack"],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["FormChoiceExample"],
        ),
        component_contract(
            "Switch",
            "Binary workflow toggle with explicit text and state.",
            &["Panel", "Stack"],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["SettingsChoiceExample"],
        ),
        component_contract(
            "StatusBadge",
            "Compact status marker that pairs severity text with structure.",
            &["Panel", "Stack"],
            &["info", "success", "warning", "error"],
            &["semantic_name", "color_independent_meaning"],
            &[option_group(
                "severity",
                &["info", "success", "warning", "error"],
            )],
            &["EditorialStatusExample", "WorkflowStatusExample"],
        ),
        component_contract(
            "InlineAlert",
            "Structural status message for action bands and bounded surfaces.",
            &["Band", "Panel"],
            &["info", "success", "warning", "error"],
            &["semantic_name", "color_independent_meaning"],
            &[option_group(
                "severity",
                &["info", "success", "warning", "error"],
            )],
            &["EditorialStatusExample", "WorkflowStatusExample"],
        ),
        component_contract(
            "EmptyState",
            "Explicit absence surface with next-action text.",
            &["Panel", "Stack"],
            &["empty"],
            &["semantic_name", "color_independent_meaning"],
            &[],
            &["ReviewDataExample", "DashboardDataExample"],
        ),
        component_contract(
            "Table",
            "Dense data surface with explicit loading, empty, error, wrap, and truncate behavior.",
            &["Panel", "SectionHeader"],
            &["default", "loading", "empty", "error"],
            &["semantic_name", "color_independent_meaning"],
            &[option_group("cell_mode", &["truncate", "wrap"])],
            &["DashboardDataExample", "ReviewDataExample"],
        ),
        component_contract(
            "MetricBlock",
            "Typographic operational summary block with optional context and note.",
            &["Panel", "Stack"],
            &["default"],
            &["semantic_name", "color_independent_meaning"],
            &[],
            &["DashboardDataExample"],
        ),
        component_contract(
            "Pagination",
            "Previous and next navigation with explicit page position.",
            &["Panel", "Stack"],
            &["default", "disabled"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[option_group("direction", &["previous", "next"])],
            &["DashboardDataExample"],
        ),
        component_contract(
            "NavIndex",
            "Section and archive navigation surface with current-item semantics.",
            &["Rail", "Panel"],
            &["default", "current"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["DocumentationNavigationExample", "SearchWorkflowExample"],
        ),
        component_contract(
            "CommandBar",
            "Primary and secondary action structure for workflow surfaces.",
            &["Band", "Panel"],
            &["default", "loading", "disabled"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["ReviewQueueCommandExample"],
        ),
        component_contract(
            "FilterRail",
            "Bounded filter group surface with explicit zero-result state.",
            &["Rail", "Panel"],
            &["default", "zero_result"],
            &["semantic_name", "keyboard", "focus_visible"],
            &[],
            &["SearchWorkflowExample"],
        ),
        component_contract(
            "DetailPane",
            "Context inspection surface with loading and unavailable states.",
            &["Region", "Panel"],
            &["default", "loading", "unavailable"],
            &["semantic_name", "color_independent_meaning"],
            &[],
            &["ReviewQueueCommandExample", "SearchWorkflowExample"],
        ),
    ]
}

fn recipe_contracts() -> Vec<RecipeContract> {
    vec![
        recipe_contract(
            "BlogIndex",
            IntentKind::Editorial,
            "Archive-first publication surface for essays and notes.",
            &["main"],
            &["support"],
            &[
                "Taxonomy and archive support remain optional.",
                "The surface must read as archive or publication rather than marketing landing page.",
            ],
            &[
                "AppShell",
                "PageGrid",
                "Region",
                "Panel",
                "Stack",
                "SectionHeader",
            ],
            &["EmptyState", "NavIndex"],
            &["BlogIndexExample", "BlogIndexMinimalExample"],
        ),
        recipe_contract(
            "ArticleShell",
            IntentKind::Editorial,
            "Longform article shell that keeps the reading flow primary.",
            &["lead", "main"],
            &["support"],
            &[
                "Metadata, local navigation, and adjacent links stay secondary.",
                "The article body remains the primary reading field.",
            ],
            &[
                "AppShell",
                "PageGrid",
                "Region",
                "Stack",
                "SectionHeader",
                "Panel",
            ],
            &["NavIndex"],
            &["ArticleShellExample", "ArticleShellMinimalExample"],
        ),
        recipe_contract(
            "DocsShell",
            IntentKind::Editorial,
            "Documentation shell with persistent rail and optional detail context.",
            &["rail", "main"],
            &["detail"],
            &[
                "Documentation stays in the editorial family rather than becoming a separate docs-site style.",
                "The table of contents and context detail remain optional.",
            ],
            &["AppShell", "PageGrid", "Rail", "Region", "Panel", "Stack"],
            &["NavIndex", "DetailPane"],
            &["DocsShellExample", "DocsShellMinimalExample"],
        ),
        recipe_contract(
            "SearchResultsWorkspace",
            IntentKind::Workspace,
            "Dense search workspace with query rail, results, actions, and optional detail.",
            &["rail", "main"],
            &["action_band", "detail"],
            &[
                "Search, filtering, and results remain coherent under dense operational use.",
                "The detail pane may disappear but the results field stays primary.",
            ],
            &[
                "AppShell", "PageGrid", "Rail", "Region", "Panel", "Stack", "Band",
            ],
            &[
                "TextField",
                "FilterRail",
                "Table",
                "Pagination",
                "InlineAlert",
                "CommandBar",
                "DetailPane",
            ],
            &[
                "SearchResultsWorkspaceExample",
                "SearchResultsWorkspaceLoadingExample",
                "SearchResultsWorkspaceZeroResultsExample",
            ],
        ),
        recipe_contract(
            "ReviewQueue",
            IntentKind::Workspace,
            "Queue, decision, and inspection surface for repeated review work.",
            &["action_band", "main", "detail"],
            &["rail"],
            &[
                "Queue and detail must coexist without collapsing into generic admin chrome.",
                "Optional rail structures remain bounded and secondary.",
            ],
            &[
                "AppShell", "PageGrid", "Rail", "Region", "Panel", "Stack", "Band",
            ],
            &[
                "CommandBar",
                "InlineAlert",
                "Table",
                "FilterRail",
                "DetailPane",
                "NavIndex",
            ],
            &[
                "ReviewQueueExample",
                "ReviewQueueLoadingExample",
                "ReviewQueueEmptyExample",
                "ReviewQueueUnavailableDetailExample",
            ],
        ),
        recipe_contract(
            "SettingsWorkspace",
            IntentKind::Workspace,
            "Sectioned settings workspace with native form controls and optional context.",
            &["rail", "main"],
            &["action_band", "detail"],
            &[
                "Settings panels stay narrow and structural rather than template-driven.",
                "Context help remains optional and secondary.",
            ],
            &[
                "AppShell", "PageGrid", "Rail", "Region", "Panel", "Stack", "Band",
            ],
            &[
                "NavIndex",
                "TextField",
                "Select",
                "Switch",
                "Checkbox",
                "InlineAlert",
            ],
            &["SettingsWorkspaceExample"],
        ),
        recipe_contract(
            "DashboardShell",
            IntentKind::Workspace,
            "Operational summary surface with metrics, dense tables, and optional context.",
            &["main"],
            &["action_band", "detail"],
            &[
                "Metrics remain typographic and quiet rather than ornamental cards.",
                "The summary table keeps the same explicit state law as other dense surfaces.",
            ],
            &["AppShell", "PageGrid", "Region", "Panel", "Stack", "Band"],
            &["MetricBlock", "Table", "InlineAlert"],
            &["DashboardShellExample"],
        ),
    ]
}

fn composition_rules() -> Vec<CompositionRule> {
    vec![
        composition_rule(
            "P-001",
            CompositionRuleKind::Allowed,
            "AppShell -> PageGrid -> Region(main)",
            "Every serious screen starts with a named primary region.",
        ),
        composition_rule(
            "P-002",
            CompositionRuleKind::Allowed,
            "PageGrid -> Rail -> Region(main)",
            "Side structure may support the primary region without replacing it.",
        ),
        composition_rule(
            "P-003",
            CompositionRuleKind::Allowed,
            "Region(main) -> Stack -> ColumnGroup",
            "Editorial subdivision stays inside the primary region.",
        ),
        composition_rule(
            "P-004",
            CompositionRuleKind::Allowed,
            "Region(main) -> Stack -> Row",
            "Workflow alignment stays local and token-driven.",
        ),
        composition_rule(
            "P-005",
            CompositionRuleKind::Allowed,
            "Panel -> SectionHeader -> Divider -> content",
            "Bounded surfaces remain explicit and quiet.",
        ),
        composition_rule(
            "P-006",
            CompositionRuleKind::Allowed,
            "Band -> SectionHeader",
            "Full-width framing surfaces stay structural and typographic.",
        ),
        composition_rule(
            "P-101",
            CompositionRuleKind::Disallowed,
            "PageGrid without Region(main)",
            "The grid becomes decorative scaffolding.",
        ),
        composition_rule(
            "P-102",
            CompositionRuleKind::Disallowed,
            "multiple rails on one side",
            "Side structure stops being bounded.",
        ),
        composition_rule(
            "P-103",
            CompositionRuleKind::Disallowed,
            "Rail inside ColumnGroup",
            "Page structure is being collapsed into local layout.",
        ),
        composition_rule(
            "P-104",
            CompositionRuleKind::Disallowed,
            "arbitrary spacing wrappers between Stack or Row children",
            "Rhythm is being repaired locally instead of governed by tokens.",
        ),
        composition_rule(
            "P-105",
            CompositionRuleKind::Disallowed,
            "Panel using color or motion as its only hierarchy signal",
            "Structure is being replaced by decoration.",
        ),
    ]
}

fn state_contracts() -> Vec<StateContract> {
    vec![
        state_contract(
            "state.global.surface",
            &["surfaces"],
            &["default", "loading", "empty", "error", "success"],
            &[
                "Global surface states remain part of the constitution rather than implementation leftovers.",
            ],
        ),
        state_contract(
            "state.global.interactive",
            &["interactive_components"],
            &["hover", "focus_visible", "active", "disabled"],
            &["Interactive states are first-class across buttons, links, and controls."],
        ),
        state_contract(
            "state.table",
            &["Table"],
            &["default", "loading", "empty", "error"],
            &[
                "Dense tables must define both structural cell behavior and empty or failure behavior.",
            ],
        ),
        state_contract(
            "state.detail_pane",
            &["DetailPane"],
            &["default", "loading", "unavailable"],
            &["Detail inspection remains explicit even when nothing can be shown."],
        ),
        state_contract(
            "state.filter_rail",
            &["FilterRail"],
            &["default", "zero_result"],
            &["Filters must explain when current criteria collapse the result set."],
        ),
        state_contract(
            "state.fields",
            &[
                "TextField",
                "Textarea",
                "Select",
                "Checkbox",
                "RadioGroup",
                "Switch",
            ],
            &["default", "error", "success", "disabled", "focus_visible"],
            &["Field controls keep feedback and focus visible in the same semantic grammar."],
        ),
        state_contract(
            "state.actions",
            &["Button", "LinkAction"],
            &[
                "default",
                "loading",
                "disabled",
                "hover",
                "active",
                "focus_visible",
            ],
            &["Action controls expose activity rather than hiding it behind style."],
        ),
    ]
}

fn anti_patterns() -> Vec<AntiPattern> {
    vec![
        anti_pattern(
            "AP-001",
            "Ad hoc layout",
            "Ad hoc layout rules outside named grid behavior break the constitutional geometry.",
            "Return the screen to PageGrid, named regions, and finite primitive combinations.",
        ),
        anti_pattern(
            "AP-002",
            "Spacing repair",
            "Arbitrary spacing values repair local rhythm instead of using token law.",
            "Delete local spacing wrappers and replace them with Stack, Row, or token-bound props.",
        ),
        anti_pattern(
            "AP-003",
            "Decorative wrapper",
            "Decorative wrappers with no structural responsibility obscure the real surface law.",
            "Delete the wrapper unless it owns a named primitive or semantic role.",
        ),
        anti_pattern(
            "AP-004",
            "Decorative naming",
            "Components named after style, palette, or mood dissolve the role-based system.",
            "Rename the surface by structure, intent, or product role.",
        ),
        anti_pattern(
            "AP-005",
            "Variant sprawl",
            "Open-ended variants and boolean prop sprawl weaken finite APIs.",
            "Replace freeform variants with finite enums or a narrower semantic split.",
        ),
        anti_pattern(
            "AP-006",
            "Prompt invented layout",
            "Prompt-to-prompt layout invention without recipe or primitive fit abandons the doctrine.",
            "Choose a recipe or allowed primitive composition before writing new UI code.",
        ),
        anti_pattern(
            "AP-007",
            "Default library identity",
            "Component-library defaults treated as product identity flatten the system into generic SaaS.",
            "Recompose from New Alphabet primitives, components, and recipes instead of imported taste.",
        ),
        anti_pattern(
            "AP-008",
            "Per-screen exception",
            "Per-screen exceptions weaken family resemblance and hide drift.",
            "Prefer the shared grammar or add a documented constitutional expansion.",
        ),
        anti_pattern(
            "AP-009",
            "Ornament over hierarchy",
            "Color or ornament doing work that hierarchy, type, or structure should do is not allowed.",
            "Move hierarchy back into structure, typography, and semantic status text.",
        ),
        anti_pattern(
            "AP-010",
            "AI taste simulation",
            "AI-generated output that simulates taste instead of enforcing law is outside the project.",
            "Require schema-backed recipes, state coverage, and validation before accepting generated output.",
        ),
        anti_pattern(
            "AP-011",
            "Implementation drift",
            "Recipes, primitives, and components outside the published inventory bypass the constitutional system.",
            "Return to the published inventory or expand the doctrine explicitly before generating more structure.",
        ),
    ]
}

fn validation_rules() -> Vec<ValidationRule> {
    vec![
        validation_rule(
            "V-001",
            ValidationCategory::Composition,
            Severity::Error,
            "Every surface must include a named main region or an equivalent required recipe field.",
            "Add Region(main) or the missing required recipe regions before adjusting local details.",
        ),
        validation_rule(
            "V-002",
            ValidationCategory::Composition,
            Severity::Error,
            "Recipe regions must stay inside the allowed required and optional set.",
            "Delete the invalid region or switch to the correct recipe before adding more structure.",
        ),
        validation_rule(
            "V-003",
            ValidationCategory::Spacing,
            Severity::Error,
            "Spacing must use named tokens only.",
            "Replace arbitrary spacing values with Stack, Row, Panel, or foundation spacing tokens.",
        ),
        validation_rule(
            "V-004",
            ValidationCategory::Naming,
            Severity::Warning,
            "Surface and component names must be role-based rather than decorative.",
            "Rename the target by structure or product intent rather than mood or ornament.",
        ),
        validation_rule(
            "V-005",
            ValidationCategory::StateCoverage,
            Severity::Error,
            "Components with state contracts must declare the full required state set.",
            "Add the missing states to the manifest or simplify the surface until the contract is true.",
        ),
        validation_rule(
            "V-006",
            ValidationCategory::Accessibility,
            Severity::Error,
            "Interactive components require accessible names, keyboard support, and visible focus.",
            "Use the component accessibility checklist and restore missing semantic coverage.",
        ),
        validation_rule(
            "V-007",
            ValidationCategory::Accessibility,
            Severity::Error,
            "Status meaning must remain legible without color alone.",
            "Add explicit text or semantic status markup so severity is readable without color.",
        ),
        validation_rule(
            "V-008",
            ValidationCategory::AntiPatternUsage,
            Severity::Error,
            "Style escape hatches and invented layouts are constitutional drift.",
            "Delete the escape hatch, choose the nearest valid recipe, and re-validate.",
        ),
        validation_rule(
            "V-009",
            ValidationCategory::Composition,
            Severity::Error,
            "Recipes must include the canonical primitive scaffolding they depend on.",
            "Add the missing primitive scaffolding or narrow the recipe claim until the manifest matches the real structure.",
        ),
        validation_rule(
            "N-001",
            ValidationCategory::Composition,
            Severity::Note,
            "Valid surfaces are called out explicitly so validation can confirm constitutional alignment as well as drift.",
            "No repair is required.",
        ),
    ]
}

fn schema_documents() -> Vec<SchemaDocument> {
    vec![
        SchemaDocument {
            id: "new-alphabet.contract".to_owned(),
            title: "New Alphabet Contract Bundle".to_owned(),
            layer: LayerKind::Contract,
            document: contract_bundle_schema(),
        },
        SchemaDocument {
            id: "new-alphabet.foundations".to_owned(),
            title: "New Alphabet Foundations".to_owned(),
            layer: LayerKind::Foundation,
            document: foundations_schema(),
        },
        SchemaDocument {
            id: "new-alphabet.flavors".to_owned(),
            title: "New Alphabet Runtime Flavors".to_owned(),
            layer: LayerKind::Flavor,
            document: flavors_schema(),
        },
        SchemaDocument {
            id: "new-alphabet.primitives".to_owned(),
            title: "New Alphabet Primitives".to_owned(),
            layer: LayerKind::Primitive,
            document: primitives_schema(),
        },
        SchemaDocument {
            id: "new-alphabet.components".to_owned(),
            title: "New Alphabet Components".to_owned(),
            layer: LayerKind::Component,
            document: components_schema(),
        },
        SchemaDocument {
            id: "new-alphabet.recipes".to_owned(),
            title: "New Alphabet Recipes".to_owned(),
            layer: LayerKind::Recipe,
            document: recipes_schema(),
        },
        SchemaDocument {
            id: "new-alphabet.project".to_owned(),
            title: "New Alphabet Project Manifest".to_owned(),
            layer: LayerKind::Project,
            document: project_schema(),
        },
    ]
}

fn prompt_intents() -> Vec<PromptIntent> {
    vec![
        PromptIntent {
            id: "prompt.blog".to_owned(),
            level: PromptLevel::Sparse,
            prompt: "Build me a blog for longform essays and notes using New Alphabet."
                .to_owned(),
            recommended_flavor: Some("LeptosSsr".to_owned()),
            recommended_recipe: "BlogIndex".to_owned(),
            plan_outline: text_list(&[
                "Choose BlogIndex for the archive surface and ArticleShell for reading flow.",
                "Keep taxonomy and archive support optional and secondary.",
                "Validate archive naming, support-region usage, and empty-state coverage.",
            ]),
            validation_focus: text_list(&["composition", "state_coverage", "naming"]),
        },
        PromptIntent {
            id: "prompt.review_workspace".to_owned(),
            level: PromptLevel::Moderate,
            prompt:
                "Build me a B2B admin workspace for reviewing submissions with a left rail, dense table, and right-side detail pane."
                    .to_owned(),
            recommended_flavor: Some("LeptosSsr".to_owned()),
            recommended_recipe: "ReviewQueue".to_owned(),
            plan_outline: text_list(&[
                "Choose ReviewQueue because the prompt names queue, action, and detail structure.",
                "Bind filters and navigation to the optional rail rather than custom dashboard chrome.",
                "Validate table, detail, and action state coverage before adding examples.",
            ]),
            validation_focus: text_list(&[
                "composition",
                "state_coverage",
                "accessibility",
            ]),
        },
        PromptIntent {
            id: "prompt.review_workspace_dense".to_owned(),
            level: PromptLevel::High,
            prompt:
                "Build a dense review workspace with left navigation rail, center results table, right detail pane, persistent action strip, loading and dirty states, and a calm editorial tone."
                    .to_owned(),
            recommended_flavor: Some("LeptosSsr".to_owned()),
            recommended_recipe: "ReviewQueue".to_owned(),
            plan_outline: text_list(&[
                "Choose ReviewQueue because the prompt explicitly names navigation, results, detail, and action structure.",
                "Keep the action strip in action_band, results in main, and dirty feedback adjacent to the detail path rather than decorative chrome.",
                "Validate loading, unavailable, focus, and state-change coverage before exporting the scaffold.",
            ]),
            validation_focus: text_list(&[
                "composition",
                "state_coverage",
                "accessibility",
                "naming",
            ]),
        },
        PromptIntent {
            id: "prompt.settings".to_owned(),
            level: PromptLevel::High,
            prompt:
                "Use New Alphabet to scaffold a settings workspace with section navigation and editable panels."
                    .to_owned(),
            recommended_flavor: Some("LeptosSsr".to_owned()),
            recommended_recipe: "SettingsWorkspace".to_owned(),
            plan_outline: text_list(&[
                "Choose SettingsWorkspace and keep navigation in the rail and editable panels in the main region.",
                "Use native form controls only and keep context help optional.",
                "Validate field-state coverage, naming, and accessibility bindings before export.",
            ]),
            validation_focus: text_list(&[
                "composition",
                "state_coverage",
                "accessibility",
            ]),
        },
        PromptIntent {
            id: "prompt.docs".to_owned(),
            level: PromptLevel::Moderate,
            prompt:
                "Explain which recipe I should use for a documentation site with article pages and archive navigation."
                    .to_owned(),
            recommended_flavor: Some("LeptosSsr".to_owned()),
            recommended_recipe: "DocsShell".to_owned(),
            plan_outline: text_list(&[
                "Use DocsShell for the documentation index and ArticleShell for the reading pages.",
                "Keep navigation persistent in the rail and move supporting context to the optional detail region.",
                "Validate family resemblance against editorial recipes rather than inventing docs-only chrome.",
            ]),
            validation_focus: text_list(&["composition", "naming"]),
        },
    ]
}

fn reference_examples() -> Vec<ReferenceExample> {
    vec![
        reference_example(
            "example.flavor.leptos_ssr",
            LayerKind::Flavor,
            "LeptosSsr",
            "docs/flavors.md",
            "Canonical web runtime binding that keeps SSR, hydration, and recipe geometry under one law.",
        ),
        reference_example(
            "example.flavor.dioxus_desktop_workbench",
            LayerKind::Flavor,
            "DioxusDesktopWorkbench",
            "docs/flavors.md",
            "Desktop workbench binding for Dioxus, Polars, and Charton under New Alphabet law.",
        ),
        reference_example(
            "example.flavor.desktop_html_workbench",
            LayerKind::Flavor,
            "DesktopHtmlWorkbench",
            "docs/flavors.md",
            "General desktop-shell binding for Leptos, Yew, Dioxus, or Azul when HTML and CSS semantics should still read as a dense native workbench.",
        ),
        reference_example(
            "example.flavor.floem_workbench",
            LayerKind::Flavor,
            "FloemWorkbench",
            "docs/flavors.md",
            "Dense Floem workbench binding with host-specific density and wrapper discipline under New Alphabet law.",
        ),
        reference_example(
            "example.flavor.gpui_workbench",
            LayerKind::Flavor,
            "GpuiWorkbench",
            "docs/flavors.md",
            "Dense GPUI workbench binding with structured panes, compact density, and virtualization-first lists under New Alphabet law.",
        ),
        reference_example(
            "example.flavor.relm4_workbench",
            LayerKind::Flavor,
            "Relm4Workbench",
            "docs/flavors.md",
            "Native Relm4 and GTK4 workbench binding with paned shells, ColumnView discipline, and GTK CSS region law under New Alphabet law.",
        ),
        reference_example(
            "example.primitive.editorial_anchor",
            LayerKind::Primitive,
            "EditorialAnchorExample",
            "crates/new-alphabet-primitives/src/examples.rs",
            "Minimal editorial anchor proving AppShell, PageGrid, and Region(main).",
        ),
        reference_example(
            "example.primitive.workflow_structure",
            LayerKind::Primitive,
            "WorkflowStructureExample",
            "crates/new-alphabet-primitives/src/examples.rs",
            "Workflow composition proving rails, rows, and named page geometry.",
        ),
        reference_example(
            "example.component.search_workflow",
            LayerKind::Component,
            "SearchWorkflowExample",
            "crates/new-alphabet-components/src/examples.rs",
            "Workflow chrome example tying nav, filters, and dense results together.",
        ),
        reference_example(
            "example.component.accessibility_coverage",
            LayerKind::Component,
            "AccessibilityCoverageExample",
            "crates/new-alphabet-components/src/examples.rs",
            "Coverage example bound to the machine-readable accessibility checklist.",
        ),
        reference_example(
            "example.recipe.blog_index",
            LayerKind::Recipe,
            "BlogIndexExample",
            "crates/new-alphabet-recipes/src/editorial.rs",
            "Archive-first editorial recipe proof.",
        ),
        reference_example(
            "example.recipe.docs_shell",
            LayerKind::Recipe,
            "DocsShellExample",
            "crates/new-alphabet-recipes/src/editorial.rs",
            "Documentation shell proof with rail and optional detail context.",
        ),
        reference_example(
            "example.recipe.review_queue",
            LayerKind::Recipe,
            "ReviewQueueExample",
            "crates/new-alphabet-recipes/src/workflow.rs",
            "Dense workflow proof with action band, queue, rail, and detail.",
        ),
        reference_example(
            "example.recipe.settings_workspace",
            LayerKind::Recipe,
            "SettingsWorkspaceExample",
            "crates/new-alphabet-recipes/src/workflow.rs",
            "Settings proof with section navigation and editable panels.",
        ),
        reference_example(
            "example.recipe.dashboard_shell",
            LayerKind::Recipe,
            "DashboardShellExample",
            "crates/new-alphabet-recipes/src/workflow.rs",
            "Dashboard proof with metric blocks, summary table, and optional context help.",
        ),
    ]
}

fn contract_bundle_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/contract-bundle-0.1.0.json",
        "title": "New Alphabet Contract Bundle",
        "type": "object",
        "required": [
            "bundle_format_version",
            "doctrine",
            "foundations",
            "flavors",
            "primitives",
            "components",
            "recipes",
            "composition_rules",
            "state_contracts",
            "anti_patterns",
            "validation_rules",
            "schemas",
            "prompt_intents",
            "generation_sequence",
            "repair_sequence"
        ],
        "properties": {
            "bundle_format_version": {"type": "string"},
            "doctrine": {"type": "object"},
            "foundations": {"type": "array"},
            "flavors": {"type": "array"},
            "primitives": {"type": "array"},
            "components": {"type": "array"},
            "recipes": {"type": "array"},
            "composition_rules": {"type": "array"},
            "state_contracts": {"type": "array"},
            "anti_patterns": {"type": "array"},
            "validation_rules": {"type": "array"},
            "schemas": {"type": "array"},
            "examples": {"type": "array"},
            "prompt_intents": {"type": "array"},
            "generation_sequence": {
                "type": "array",
                "items": {"type": "string"}
            },
            "repair_sequence": {
                "type": "array",
                "items": {"type": "string"}
            }
        }
    })
}

fn foundations_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/foundations-0.1.0.json",
        "title": "New Alphabet Foundations",
        "type": "array",
        "items": {
            "type": "object",
            "required": ["id", "title", "tokens"],
            "properties": {
                "id": {"type": "string"},
                "title": {"type": "string"},
                "tokens": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "description", "values"],
                        "properties": {
                            "id": {"type": "string"},
                            "description": {"type": "string"},
                            "values": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "required": ["name", "value"],
                                    "properties": {
                                        "name": {"type": "string"},
                                        "value": {"type": "string"}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn flavors_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/flavors-0.1.0.json",
        "title": "New Alphabet Runtime Flavors",
        "type": "array",
        "items": {
            "type": "object",
            "required": [
                "id",
                "runtime",
                "purpose",
                "stack",
                "required_bindings",
                "laws",
                "anti_patterns"
            ]
        }
    })
}

fn primitives_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/primitives-0.1.0.json",
        "title": "New Alphabet Primitives",
        "type": "array",
        "items": {
            "type": "object",
            "required": [
                "id",
                "purpose",
                "required_props",
                "optional_props",
                "finite_options",
                "composition_notes",
                "example_ids"
            ]
        }
    })
}

fn components_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/components-0.1.0.json",
        "title": "New Alphabet Components",
        "type": "array",
        "items": {
            "type": "object",
            "required": [
                "id",
                "purpose",
                "built_from",
                "required_states",
                "accessibility",
                "finite_options",
                "example_ids"
            ]
        }
    })
}

fn recipes_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/recipes-0.1.0.json",
        "title": "New Alphabet Recipes",
        "type": "array",
        "items": {
            "type": "object",
            "required": [
                "id",
                "intent",
                "purpose",
                "required_regions",
                "optional_regions",
                "adaptation_limits",
                "primitives",
                "components",
                "example_ids"
            ]
        }
    })
}

fn project_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://schemas.newalphabet.dev/project-0.1.0.json",
        "title": "New Alphabet Project Manifest",
        "type": "object",
        "required": ["schema_version", "project_name", "surfaces"],
        "properties": {
            "schema_version": {"const": bundle_format_version()},
            "project_name": {"type": "string"},
            "surfaces": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": [
                        "id",
                        "name",
                        "intent",
                        "recipe",
                        "regions",
                        "primitives",
                        "components",
                        "spacing_tokens",
                        "custom_spacing_values",
                        "accessibility",
                        "style_escape_hatch",
                        "invented_layout"
                    ]
                }
            }
        }
    })
}

fn primitive_contract(
    id: &str,
    purpose: &str,
    required_props: &[&str],
    optional_props: &[&str],
    finite_options: &[ContractOptionGroup],
    composition_notes: &[&str],
    example_ids: &[&str],
) -> PrimitiveContract {
    PrimitiveContract {
        id: id.to_owned(),
        purpose: purpose.to_owned(),
        required_props: text_list(required_props),
        optional_props: text_list(optional_props),
        finite_options: finite_options.to_vec(),
        composition_notes: text_list(composition_notes),
        example_ids: text_list(example_ids),
    }
}

fn component_contract(
    id: &str,
    purpose: &str,
    built_from: &[&str],
    required_states: &[&str],
    accessibility: &[&str],
    finite_options: &[ContractOptionGroup],
    example_ids: &[&str],
) -> ComponentContract {
    let derived_accessibility = COMPONENT_ACCESSIBILITY_CHECKS
        .iter()
        .find(|check| check.component == id)
        .map(|check| {
            check
                .rules
                .iter()
                .map(|rule| rule.id().to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    ComponentContract {
        id: id.to_owned(),
        purpose: purpose.to_owned(),
        built_from: text_list(built_from),
        required_states: text_list(required_states),
        accessibility: if derived_accessibility.is_empty() {
            text_list(accessibility)
        } else {
            derived_accessibility
        },
        finite_options: finite_options.to_vec(),
        example_ids: text_list(example_ids),
    }
}

fn recipe_contract(
    id: &str,
    intent: IntentKind,
    purpose: &str,
    required_regions: &[&str],
    optional_regions: &[&str],
    adaptation_limits: &[&str],
    primitives: &[&str],
    components: &[&str],
    example_ids: &[&str],
) -> RecipeContract {
    RecipeContract {
        id: id.to_owned(),
        intent,
        purpose: purpose.to_owned(),
        required_regions: text_list(required_regions),
        optional_regions: text_list(optional_regions),
        adaptation_limits: text_list(adaptation_limits),
        primitives: text_list(primitives),
        components: text_list(components),
        example_ids: text_list(example_ids),
    }
}

fn composition_rule(
    id: &str,
    kind: CompositionRuleKind,
    composition: &str,
    reason: &str,
) -> CompositionRule {
    CompositionRule {
        id: id.to_owned(),
        kind,
        composition: composition.to_owned(),
        reason: reason.to_owned(),
    }
}

fn state_contract(
    id: &str,
    applies_to: &[&str],
    required_states: &[&str],
    notes: &[&str],
) -> StateContract {
    StateContract {
        id: id.to_owned(),
        applies_to: text_list(applies_to),
        required_states: text_list(required_states),
        notes: text_list(notes),
    }
}

fn anti_pattern(id: &str, title: &str, summary: &str, repair: &str) -> AntiPattern {
    AntiPattern {
        id: id.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        repair: repair.to_owned(),
    }
}

fn validation_rule(
    id: &str,
    category: ValidationCategory,
    default_severity: Severity,
    summary: &str,
    repair: &str,
) -> ValidationRule {
    ValidationRule {
        id: id.to_owned(),
        category,
        default_severity,
        summary: summary.to_owned(),
        repair: repair.to_owned(),
    }
}

fn option_group(name: &str, values: &[&str]) -> ContractOptionGroup {
    ContractOptionGroup {
        name: name.to_owned(),
        values: text_list(values),
    }
}

fn named_value(name: &str, value: impl Into<String>) -> NamedValue {
    NamedValue {
        name: name.to_owned(),
        value: value.into(),
    }
}

fn reference_example(
    id: &str,
    layer: LayerKind,
    target: &str,
    source_path: &str,
    summary: &str,
) -> ReferenceExample {
    ReferenceExample {
        id: id.to_owned(),
        layer,
        target: target.to_owned(),
        source_path: source_path.to_owned(),
        summary: summary.to_owned(),
    }
}

fn text_list(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn span_value(span: new_alphabet_foundation::SpanBehavior) -> String {
    match span {
        new_alphabet_foundation::SpanBehavior::Columns(columns) => columns.to_string(),
        new_alphabet_foundation::SpanBehavior::Stacked => "stacked".to_owned(),
    }
}
