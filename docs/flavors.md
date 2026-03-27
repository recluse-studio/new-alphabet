# Runtime Flavors

Runtime flavors bind the New Alphabet constitution to a specific Rust UI stack.

They do not create new aesthetics.
They do not loosen the law.
They do not replace recipes.

Use this language:

- the `constitution` is the invariant design law,
- a `runtime flavor` is the host-stack binding,
- a `recipe` is the surface geometry,
- a `runner` is the executable scaffold or app entrypoint for a chosen flavor.

## Law

- flavor changes renderer and host behavior, not hierarchy or tone,
- flavor may bind host-appropriate dense measurements when the constitution remains visibly intact,
- recipes still define region structure,
- semantic components still define state law,
- foundation tokens still define spacing, type, color, border, motion, and radius,
- first builds keep the canonical palette strict,
- later palette adjustments may only remap semantic roles rather than invent new visual freedom,
- entity surfaces and controls use the subtle radius token while the background grid stays sharp.

If the host stack changes but the result stops looking severe, typographic, calm, and explicit, the flavor is invalid.

## Current flavors

### `LeptosSsr`

- runtime: Leptos SSR and hydration
- use when: the surface is web-native and must keep semantic HTML, SSR, and hydration under one law
- stack: Leptos, semantic HTML, SSR, hydration
- rule: hydration must not change layout meaning or weaken recipe structure

### `DioxusDesktopWorkbench`

- runtime: Dioxus desktop
- use when: the surface is a dense desktop workbench with native Rust data services
- stack: Dioxus Desktop, Polars, Charton, SVG output
- shell law: Dioxus owns shell, pane layout, and local interaction state
- data law: Polars owns filtering, shaping, grouping, aggregation, sorting, and export slices
- chart law: Charton owns chart grammar, scales, legends, and SVG rendering output
- hierarchy law: the table or list stays primary, the chart stays explanatory, the inspector stays secondary
- layout law: prefer left rail, central work surface, right analysis pane, and compact top controls before inventing new geometry
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp

### `TauriLeptosWorkbench`

- runtime: Tauri shell with Leptos frontend
- use when: the surface should behave like a desktop workbench with Leptos making shell decisions while Rust keeps heavy data work native
- stack: Tauri, Leptos, Polars, Charton, SVG output, Vega-Lite JSON
- shell law: Tauri owns the native shell and command boundary
- frontend law: Leptos owns shell layout, interaction, and direct SVG embedding
- data law: Polars and Charton live in native Rust commands or native state rather than frontend WASM
- transport law: the frontend requests table slice, chart SVG, and optional chart spec JSON instead of raw datasets
- command law: use one command for query refresh, one for chart refresh, and one for export rather than chatty command fans
- layout law: use CSS grid with sidebar width 248, toolbar height 36, main surface `minmax(0, 1fr)`, and inspector width 300 when the inspector is present
- chart law: charts preserve viewBox, keep explicit compact height, and never force toolbar or filter controls below the fold
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp

### `TauriYewWorkbench`

- runtime: Tauri shell with Yew frontend
- use when: the surface should behave like a desktop workbench with Yew making shell decisions while Rust keeps heavy data work native
- stack: Tauri, Yew, Polars, Charton, SVG output, Vega-Lite JSON
- shell law: Tauri owns the native shell and command boundary
- frontend law: Yew owns shell layout, interaction, and direct SVG embedding
- data law: Polars and Charton live in native Rust commands or native state rather than frontend WASM
- transport law: the frontend requests table slice, chart SVG, and optional chart spec JSON instead of raw datasets
- command law: use one command for query refresh, one for chart refresh, and one for export rather than chatty command fans
- layout law: use CSS grid with sidebar width 248, toolbar height 36, main surface `minmax(0, 1fr)`, and inspector width 300 when the inspector is present
- chart law: charts preserve viewBox, keep explicit compact height, and never force toolbar or filter controls below the fold
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp

### `FloemWorkbench`

- runtime: Floem desktop
- use when: the surface is a dense desktop work application and Floem should own the shell through disciplined stack composition
- stack: Floem, Polars, Charton, SVG output
- shell law: build the shell with h_stack and v_stack and set typography and density once at the root style layer
- density binding: body 13, meta 12, section title 16, page title 20, control 32, toolbar 36, sidebar 240, inspector 300, gap x 8, gap y 6, panel padding 8
- wrapper law: only add container when you need distinct style identity, clip, or region identity and never as casual padding chrome
- list law: use virtual_stack for long lists and keep scrolling inside the main content region
- row law: rows should read as desktop rows rather than mobile cards
- data law: Polars owns file reads, lazy queries, joins, filters, aggregations, table slices, and export-ready datasets
- chart law: Charton owns chart grammar, theming, axis and legend policy, and SVG-first output
- hierarchy law: table stays primary, chart stays explanatory, inspector stays secondary, and no chart pushes the working table below the fold
- spacing law: structural rhythm uses gap x 8 and gap y 6, and values above 12 are invalid in the default dense shell
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp

### `GpuiWorkbench`

- runtime: GPUI desktop
- use when: the surface is a dense desktop tool and GPUI should behave like a structured application shell rather than a freeform canvas
- stack: GPUI, Polars, Charton, SVG output
- shell law: build with horizontal and vertical flex containers, fixed left nav width, flexible center pane, optional fixed right inspector, and one compact top toolbar row
- density binding: default text 13px, page title 20px max, control height 32px, shell gap 8, local control gap 6, inline micro-gap 4, shell padding 8
- scrolling law: scrolling lives in the content region rather than across the shell
- component law: prefer virtualized list and table components for dense datasets and prefer dock or split-pane patterns for multi-surface tools
- sizing law: leaf controls shrink to content or fixed compact sizes and full-size leaf widgets appear only when the leaf is the actual editor or table
- hierarchy law: avoid giant panes with one tiny child and ensure the center pane shows useful content immediately at 1440x900
- tolerance note: GPUI keeps exact 8, 6, and 4 steps because dock surfaces and inline control groups need separate fixed rhythms
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp

### `Relm4Workbench`

- runtime: Relm4 with GTK4 and libadwaita
- use when: the surface should read as a serious native desktop work tool with GTK-native structure rather than a web shell in disguise
- stack: Relm4, GTK4, libadwaita, Polars, Charton, SVG output
- shell law: use ApplicationWindow, a compact HeaderBar or top toolbar, Paned for two- or three-pane shells, and ScrolledWindow around main content
- dataset law: use ListView or ColumnView for datasets, and use ColumnView for real multi-column data instead of nested boxes pretending to be a table
- mode law: use Stack or StackSwitcher for major modes and keep dialogs and popovers secondary
- density binding: sidebar width-request 232 to 256, inspector width-request 280 to 320, shell and panel spacing 8, dense control rhythm 6
- row law: dense row labels should ellipsize rather than wrap and row height should stay stable
- styling law: assign css-classes to all major regions and theme with GTK CSS instead of per-widget hacks
- hierarchy law: primary actions stay visible without scrolling, scrolling stays local to the main content region, and separators do more work than card framing
- tolerance note: the width-request bands remain narrow ranges because GTK and libadwaita widget metrics vary slightly across native control rendering
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp when CSS rounding is applied

### `SlintWorkbench`

- runtime: Slint desktop
- use when: the surface should stay dense and work-first while Slint owns layout structure through its explicit layout system
- stack: Slint, HorizontalLayout, VerticalLayout, GridLayout
- shell law: use layouts for all structure and never build the shell with loose Rectangle primitives
- sizing law: use logical px throughout, set `Window.default-font-size` explicitly, use `preferred-width` or `preferred-height` for sidebar, toolbar, and inspector, and use min or max widths when panes must stay legible
- layout law: set spacing and padding explicitly on every structural layout and do not use percentage widths except at the top shell level
- input law: place `TextInput` only inside constrained layouts
- dense-panel law: prefer `GridLayout` for dense forms and metadata panels and favor `HorizontalLayout` plus `VerticalLayout` for the primary shell before inventing custom geometry
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp where Slint styling applies rounding

### `EguiWorkbench`

- runtime: egui desktop
- use when: the surface should stay compact and work-first while egui owns the shell through panel structure and explicit style installation
- stack: egui, `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Grid`
- startup law: emit `install_compact_style(ctx)` before any screen code
- typography law: set `text_styles` explicitly for `Heading`, `Body`, `Button`, and `Small`
- spacing law: set `spacing.item_spacing`, `spacing.button_padding`, `spacing.interact_size.y`, and `spacing.window_margin` explicitly
- shell law: use `SidePanel`, `TopBottomPanel`, and `CentralPanel` for shell structure and keep scrolling inside central content only
- framing law: avoid nested `Frame::group` calls and avoid more than one framed wrapper around routine content
- density law: use `ui.add_space` only at `4.0` or `8.0` and prefer `0.0` when layout containers already carry the rhythm
- dense-panel law: prefer `Grid` for dense label-value regions and compact form layouts
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp where egui styling applies rounding

### `IcedWorkbench`

- runtime: Iced desktop
- use when: the surface should stay table-first and work-first while Iced owns shell structure through strict fill discipline and bounded panes
- stack: Iced, `Length::Fill`, `Length::FillPortion`, `Scrollable`
- fill law: `Length::Fill` is allowed only on the app root container, the main content pane, scrollable regions, and first-level split panes
- split law: `Length::FillPortion` is allowed only on the first split between major panes
- control law: buttons, labels, chips, form rows, and tool groups use `Shrink` or `Fixed` sizing
- sizing law: sidebar width stays between `Fixed(232.0)` and `Fixed(260.0)`, inspector width stays between `Fixed(280.0)` and `Fixed(340.0)`, and toolbar height stays between `Fixed(34.0)` and `Fixed(38.0)`
- spacing law: `Row` and `Column` spacing uses `8` for structural rhythm and `4` only inside dense inline control groups, and padding uses `8` outside the root shell
- wrapper law: never nest more than two padded containers
- scrolling law: prefer `Scrollable` only around the main body and never around the whole app
- hierarchy law: prefer table or list rows over cards
- tolerance note: the width bands remain narrow ranges because Iced split panes and content tables need a small tolerance band to stay legible across window states
- visual law: first builds keep the canonical New Alphabet palette strict; entity surfaces and controls use the subtle radius token while the background grid stays sharp where Iced styling applies rounding

## Anti-patterns

- treating flavors as theme packs,
- treating runtimes as interchangeable without a declared flavor contract,
- changing palette, spacing, or hierarchy because a host toolkit makes it easy,
- hand-drawing chart primitives when the declared chart engine can express them,
- moving heavy data work into frontend WASM because the transport boundary feels inconvenient,
- casual wrapper containers that exist only to add padding,
- repeated banner alerts, oversized iconography, or decorative chrome that steals vertical space,
- nested GTK box hierarchies used to fake grids or tables,
- centered page wrappers, max-width content columns, hero headers, or giant empty banners inside desktop shells,
- loose Rectangle shells, implicit layout spacing, or unconstrained text inputs in Slint,
- egui screens that skip install_compact_style, depend on default spacing, or stack nested Frame::group wrappers around routine content,
- Iced screens that use Fill on local controls, wrap Scrollable around the whole shell, or turn routine rows into cards,
- broad runtime buckets that hide the real host boundary,
- letting a chart become the page hierarchy.
