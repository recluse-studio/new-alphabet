#![forbid(unsafe_code)]

mod templates;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use new_alphabet_core::{
    AccessibilitySnapshot, ComponentInstance, IntentKind, ProjectManifest, Severity,
    SurfaceManifest, SurfaceRegion, ValidationMessage,
};
use new_alphabet_schema::{
    bundle_format_version, contract_bundle, serialize_bundle_pretty, validate_manifest,
};

use crate::templates::{
    component_module_name, component_names, normalize, recipe_template, render_component_module,
    render_recipe_module,
};

const MANIFEST_FILE: &str = "new-alphabet.json";

#[derive(Debug)]
pub struct CommandResult {
    pub stdout: String,
    pub wrote_files: Vec<PathBuf>,
}

enum Command {
    Init {
        path: Option<String>,
        name: Option<String>,
    },
    AddRecipe {
        name: String,
    },
    AddComponent {
        name: String,
    },
    Explain {
        item: String,
    },
    Inspect {
        path: Option<String>,
    },
    Validate {
        path: Option<String>,
    },
    ExportContext {
        output: Option<String>,
    },
    Plan {
        intent: String,
    },
}

pub fn run_from<I, S>(args: I, cwd: &Path) -> Result<CommandResult, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args = args.into_iter().map(Into::into).collect::<Vec<_>>();
    let command = parse_command(&args)?;
    run_command(command, cwd)
}

fn parse_command(args: &[String]) -> Result<Command, String> {
    if args.len() <= 1 || matches!(args[1].as_str(), "help" | "--help" | "-h") {
        return Err(help_text());
    }

    match args[1].as_str() {
        "init" => parse_init(args),
        "add" => parse_add(args),
        "explain" => parse_explain(args),
        "inspect" => Ok(Command::Inspect {
            path: args.get(2).cloned(),
        }),
        "validate" => parse_validate(args),
        "export" => parse_export(args),
        "plan" => parse_plan(args),
        _ => Err(help_text()),
    }
}

fn parse_init(args: &[String]) -> Result<Command, String> {
    let mut path = None;
    let mut name = None;
    let mut index = 2;

    while index < args.len() {
        match args[index].as_str() {
            "--path" => {
                let value = args
                    .get(index + 1)
                    .cloned()
                    .ok_or_else(|| "init requires a value after --path".to_owned())?;
                path = Some(value);
                index += 2;
            }
            "--name" => {
                let value = args
                    .get(index + 1)
                    .cloned()
                    .ok_or_else(|| "init requires a value after --name".to_owned())?;
                name = Some(value);
                index += 2;
            }
            _ => return Err("init accepts only --path and --name".to_owned()),
        }
    }

    Ok(Command::Init { path, name })
}

fn parse_add(args: &[String]) -> Result<Command, String> {
    let kind = args
        .get(2)
        .ok_or_else(|| "add requires recipe or component".to_owned())?;
    let name = args
        .get(3)
        .cloned()
        .ok_or_else(|| "add requires a canonical recipe or component name".to_owned())?;

    match kind.as_str() {
        "recipe" => Ok(Command::AddRecipe { name }),
        "component" => Ok(Command::AddComponent { name }),
        _ => Err("add requires recipe or component".to_owned()),
    }
}

fn parse_explain(args: &[String]) -> Result<Command, String> {
    let item = args.get(2).cloned().ok_or_else(|| {
        "explain requires a primitive, component, recipe, token, or rule id".to_owned()
    })?;
    Ok(Command::Explain { item })
}

fn parse_validate(args: &[String]) -> Result<Command, String> {
    match args.get(2).map(String::as_str) {
        None => Ok(Command::Validate { path: None }),
        Some("--path") => Ok(Command::Validate {
            path: Some(
                args.get(3)
                    .cloned()
                    .ok_or_else(|| "validate requires a value after --path".to_owned())?,
            ),
        }),
        Some(value) => Ok(Command::Validate {
            path: Some(value.to_owned()),
        }),
    }
}

fn parse_export(args: &[String]) -> Result<Command, String> {
    if args.get(2).map(String::as_str) != Some("context") {
        return Err("export currently supports only `new-alphabet export context`".to_owned());
    }

    let output = if args.get(3).map(String::as_str) == Some("--output") {
        Some(
            args.get(4)
                .cloned()
                .ok_or_else(|| "export context requires a value after --output".to_owned())?,
        )
    } else {
        None
    };

    Ok(Command::ExportContext { output })
}

fn parse_plan(args: &[String]) -> Result<Command, String> {
    let intent = args
        .iter()
        .skip(2)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_owned();

    if intent.is_empty() {
        return Err("plan requires a product intent string".to_owned());
    }

    Ok(Command::Plan { intent })
}

fn run_command(command: Command, cwd: &Path) -> Result<CommandResult, String> {
    match command {
        Command::Init { path, name } => init_project(cwd, path.as_deref(), name.as_deref()),
        Command::AddRecipe { name } => add_recipe(cwd, &name),
        Command::AddComponent { name } => add_component(cwd, &name),
        Command::Explain { item } => explain_item(&item),
        Command::Inspect { path } => inspect_manifest(cwd, path.as_deref()),
        Command::Validate { path } => validate_project(cwd, path.as_deref()),
        Command::ExportContext { output } => export_context(cwd, output.as_deref()),
        Command::Plan { intent } => plan_intent(&intent),
    }
}

fn init_project(
    cwd: &Path,
    path: Option<&str>,
    name: Option<&str>,
) -> Result<CommandResult, String> {
    let root = resolve_root(cwd, path);
    let mut wrote_files = Vec::new();

    fs::create_dir_all(root.join("src/new_alphabet/components")).map_err(io_error)?;
    fs::create_dir_all(root.join("src/new_alphabet/recipes")).map_err(io_error)?;

    let manifest_path = root.join(MANIFEST_FILE);
    let project_name = name.map(str::to_owned).unwrap_or_else(|| {
        root.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("new-alphabet-project")
            .to_owned()
    });

    let mut manifest = if manifest_path.exists() {
        load_manifest(&manifest_path)?
    } else {
        ProjectManifest {
            schema_version: bundle_format_version().to_owned(),
            project_name: project_name.clone(),
            surfaces: Vec::new(),
        }
    };

    manifest.schema_version = bundle_format_version().to_owned();
    if manifest.project_name.is_empty() {
        manifest.project_name = project_name;
    }

    write_json_file(&manifest_path, &manifest, &mut wrote_files)?;
    ensure_module_file(
        &root.join("src/new_alphabet/mod.rs"),
        &["pub mod components;", "pub mod recipes;"],
        &mut wrote_files,
    )?;
    ensure_module_file(
        &root.join("src/new_alphabet/components/mod.rs"),
        &[],
        &mut wrote_files,
    )?;
    ensure_module_file(
        &root.join("src/new_alphabet/recipes/mod.rs"),
        &[],
        &mut wrote_files,
    )?;

    Ok(CommandResult {
        stdout: describe_writes("init", &wrote_files, &root),
        wrote_files,
    })
}

fn add_recipe(cwd: &Path, name: &str) -> Result<CommandResult, String> {
    let root = cwd.to_path_buf();
    let template = recipe_template(name).ok_or_else(|| {
        "unsupported recipe; use one of blog-index, article-shell, docs-shell, search-results-workspace, review-queue, settings-workspace, or dashboard-shell".to_owned()
    })?;

    let manifest_path = root.join(MANIFEST_FILE);
    let mut manifest = load_manifest(&manifest_path)?;
    let mut wrote_files = Vec::new();

    let module_path = root.join(format!(
        "src/new_alphabet/recipes/{}.rs",
        template.module_name
    ));
    write_new_file(
        &module_path,
        &render_recipe_module(&template),
        &mut wrote_files,
    )?;
    ensure_module_line(
        &root.join("src/new_alphabet/recipes/mod.rs"),
        &format!("pub mod {};", template.module_name),
        &mut wrote_files,
    )?;

    if !manifest
        .surfaces
        .iter()
        .any(|surface| normalize(&surface.recipe) == normalize(&template.canonical_name))
    {
        manifest.surfaces.push(recipe_surface_manifest(
            template.canonical_name.as_str(),
            template.intent,
        ));
        write_json_file(&manifest_path, &manifest, &mut wrote_files)?;
    }

    Ok(CommandResult {
        stdout: describe_writes("add recipe", &wrote_files, &root),
        wrote_files,
    })
}

fn add_component(cwd: &Path, name: &str) -> Result<CommandResult, String> {
    let root = cwd.to_path_buf();
    if !root.join(MANIFEST_FILE).exists() {
        return Err("new-alphabet.json is missing; run `new-alphabet init` first".to_owned());
    }

    let module_name = component_module_name(name).ok_or_else(|| {
        format!(
            "unsupported component; use one of {}",
            component_names().join(", ")
        )
    })?;
    let body = render_component_module(name)
        .ok_or_else(|| "component scaffold generation failed".to_owned())?;
    let mut wrote_files = Vec::new();

    let module_path = root.join(format!("src/new_alphabet/components/{module_name}.rs"));
    write_new_file(&module_path, &body, &mut wrote_files)?;
    ensure_module_line(
        &root.join("src/new_alphabet/components/mod.rs"),
        &format!("pub mod {module_name};"),
        &mut wrote_files,
    )?;

    Ok(CommandResult {
        stdout: describe_writes("add component", &wrote_files, &root),
        wrote_files,
    })
}

fn explain_item(item: &str) -> Result<CommandResult, String> {
    let bundle = contract_bundle();
    let item_key = normalize(item);

    for family in &bundle.foundations {
        if normalize(&family.id) == item_key || normalize(&family.title) == item_key {
            return Ok(CommandResult {
                stdout: format!(
                    "foundation {}\ntitle: {}\ntokens: {}",
                    family.id,
                    family.title,
                    family
                        .tokens
                        .iter()
                        .map(|token| token.id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                wrote_files: Vec::new(),
            });
        }

        if let Some(token) = family
            .tokens
            .iter()
            .find(|token| normalize(&token.id) == item_key)
        {
            let values = token
                .values
                .iter()
                .map(|value| format!("{}={}", value.name, value.value))
                .collect::<Vec<_>>()
                .join(", ");
            return Ok(CommandResult {
                stdout: format!(
                    "foundation token {}\n{}\nvalues: {}",
                    token.id, token.description, values
                ),
                wrote_files: Vec::new(),
            });
        }
    }

    if let Some(primitive) = bundle
        .primitives
        .iter()
        .find(|primitive| normalize(&primitive.id) == item_key)
    {
        return Ok(CommandResult {
            stdout: format!(
                "primitive {}\npurpose: {}\nrequired_props: {}\noptional_props: {}\nnotes: {}",
                primitive.id,
                primitive.purpose,
                list_or_none(&primitive.required_props),
                list_or_none(&primitive.optional_props),
                primitive.composition_notes.join(" | ")
            ),
            wrote_files: Vec::new(),
        });
    }

    if let Some(component) = bundle
        .components
        .iter()
        .find(|component| normalize(&component.id) == item_key)
    {
        return Ok(CommandResult {
            stdout: format!(
                "component {}\npurpose: {}\nrequired_states: {}\naccessibility: {}\nexamples: {}",
                component.id,
                component.purpose,
                list_or_none(&component.required_states),
                list_or_none(&component.accessibility),
                list_or_none(&component.example_ids)
            ),
            wrote_files: Vec::new(),
        });
    }

    if let Some(recipe) = bundle
        .recipes
        .iter()
        .find(|recipe| normalize(&recipe.id) == item_key)
    {
        return Ok(CommandResult {
            stdout: format!(
                "recipe {}\npurpose: {}\nrequired_regions: {}\noptional_regions: {}\nadaptation_limits: {}",
                recipe.id,
                recipe.purpose,
                list_or_none(&recipe.required_regions),
                list_or_none(&recipe.optional_regions),
                recipe.adaptation_limits.join(" | ")
            ),
            wrote_files: Vec::new(),
        });
    }

    if let Some(rule) = bundle
        .composition_rules
        .iter()
        .find(|rule| normalize(&rule.id) == item_key)
    {
        return Ok(CommandResult {
            stdout: format!(
                "composition rule {}\nkind: {:?}\npattern: {}\nreason: {}",
                rule.id, rule.kind, rule.composition, rule.reason
            ),
            wrote_files: Vec::new(),
        });
    }

    if let Some(rule) = bundle
        .validation_rules
        .iter()
        .find(|rule| normalize(&rule.id) == item_key)
    {
        return Ok(CommandResult {
            stdout: format!(
                "validation rule {}\ncategory: {:?}\nsummary: {}\nrepair: {}",
                rule.id, rule.category, rule.summary, rule.repair
            ),
            wrote_files: Vec::new(),
        });
    }

    if let Some(pattern) = bundle
        .anti_patterns
        .iter()
        .find(|pattern| normalize(&pattern.id) == item_key || normalize(&pattern.title) == item_key)
    {
        return Ok(CommandResult {
            stdout: format!(
                "anti-pattern {}\nsummary: {}\nrepair: {}",
                pattern.id, pattern.summary, pattern.repair
            ),
            wrote_files: Vec::new(),
        });
    }

    Err("No matching primitive, component, recipe, rule, or token was found.".to_owned())
}

fn inspect_manifest(cwd: &Path, path: Option<&str>) -> Result<CommandResult, String> {
    let manifest_path = resolve_manifest_path(cwd, path);
    let manifest = load_manifest(&manifest_path)?;
    let surfaces = manifest
        .surfaces
        .iter()
        .map(|surface| {
            format!(
                "{} -> {} [{}] regions={} components={}",
                surface.id,
                surface.recipe,
                match surface.intent {
                    IntentKind::Editorial => "editorial",
                    IntentKind::Workspace => "workspace",
                },
                surface
                    .regions
                    .iter()
                    .map(|region| region.kind.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                surface
                    .components
                    .iter()
                    .map(|component| component.id.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(CommandResult {
        stdout: format!(
            "project {}\nschema_version: {}\nsurfaces: {}\n{}",
            manifest.project_name,
            manifest.schema_version,
            manifest.surfaces.len(),
            surfaces
        ),
        wrote_files: Vec::new(),
    })
}

fn validate_project(cwd: &Path, path: Option<&str>) -> Result<CommandResult, String> {
    let manifest_path = resolve_manifest_path(cwd, path);
    let manifest = load_manifest(&manifest_path)?;
    let report = validate_manifest(&manifest);
    let mut lines = report
        .messages
        .iter()
        .map(format_validation_message)
        .collect::<Vec<_>>();

    let error_count = report
        .messages
        .iter()
        .filter(|message| matches!(message.severity, Severity::Error))
        .count();
    let warning_count = report
        .messages
        .iter()
        .filter(|message| matches!(message.severity, Severity::Warning))
        .count();
    let note_count = report
        .messages
        .iter()
        .filter(|message| matches!(message.severity, Severity::Note))
        .count();

    if lines.is_empty() {
        lines.push(format!(
            "note [validation] {}: manifest conforms to the current contract.",
            manifest_path.display()
        ));
    } else {
        lines.push(format!(
            "note [summary] {}: {} errors, {} warnings, {} notes.",
            manifest_path.display(),
            error_count,
            warning_count,
            note_count
        ));
    }

    Ok(CommandResult {
        stdout: lines.join("\n"),
        wrote_files: Vec::new(),
    })
}

fn export_context(cwd: &Path, output: Option<&str>) -> Result<CommandResult, String> {
    let bundle = serialize_bundle_pretty().map_err(|error| error.to_string())?;
    let mut wrote_files = Vec::new();

    if let Some(output) = output {
        let path = resolve_root(cwd, Some(output));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(io_error)?;
        }
        fs::write(&path, bundle.as_bytes()).map_err(io_error)?;
        wrote_files.push(path.clone());

        Ok(CommandResult {
            stdout: format!("export context wrote {}", path.display()),
            wrote_files,
        })
    } else {
        Ok(CommandResult {
            stdout: bundle,
            wrote_files,
        })
    }
}

fn plan_intent(intent: &str) -> Result<CommandResult, String> {
    let bundle = contract_bundle();
    let intent_words = token_set(intent);
    let best = bundle
        .prompt_intents
        .iter()
        .max_by_key(|prompt| {
            token_set(&prompt.prompt)
                .intersection(&intent_words)
                .count()
        })
        .ok_or_else(|| "No prompt intents are available.".to_owned())?;

    Ok(CommandResult {
        stdout: format!(
            "plan {}\nrecipe: {}\nsteps:\n- {}\nvalidation_focus: {}",
            best.id,
            best.recommended_recipe,
            best.plan_outline.join("\n- "),
            best.validation_focus.join(", ")
        ),
        wrote_files: Vec::new(),
    })
}

fn recipe_surface_manifest(recipe_name: &str, intent: IntentKind) -> SurfaceManifest {
    let bundle = contract_bundle();
    let component_contracts = bundle.components.clone();
    let recipe = bundle
        .recipes
        .into_iter()
        .find(|recipe| recipe.id == recipe_name)
        .expect("recipe to exist in contract bundle");

    let regions = recipe
        .required_regions
        .iter()
        .chain(recipe.optional_regions.iter())
        .map(|region| SurfaceRegion {
            kind: region.clone(),
            placement: region_placement(region),
        })
        .collect::<Vec<_>>();

    let components = recipe
        .components
        .iter()
        .map(|component| ComponentInstance {
            id: component.clone(),
            states: component_contracts
                .iter()
                .find(|candidate| candidate.id == *component)
                .map(|candidate| candidate.required_states.clone())
                .unwrap_or_default(),
            accessible_name: true,
            keyboard_support: true,
            text_equivalent: true,
            custom_variant_props: false,
        })
        .collect::<Vec<_>>();

    SurfaceManifest {
        id: recipe
            .id
            .chars()
            .enumerate()
            .flat_map(|(index, character)| {
                if character.is_uppercase() && index > 0 {
                    ['-', character.to_ascii_lowercase()]
                } else {
                    ['\0', character.to_ascii_lowercase()]
                }
            })
            .filter(|character| *character != '\0')
            .collect(),
        name: recipe
            .id
            .replace("Shell", " shell")
            .replace("Workspace", " workspace"),
        intent,
        recipe: recipe.id,
        regions,
        primitives: recipe.primitives,
        components,
        spacing_tokens: vec![
            "spacing.stack.tight".to_owned(),
            "spacing.stack.default".to_owned(),
            "spacing.panel.content".to_owned(),
        ],
        custom_spacing_values: Vec::new(),
        accessibility: AccessibilitySnapshot {
            semantic_html: true,
            focus_visible: true,
            color_independent_meaning: true,
        },
        style_escape_hatch: false,
        invented_layout: false,
    }
}

fn region_placement(kind: &str) -> String {
    match kind {
        "rail" => "rail_start".to_owned(),
        other => other.to_owned(),
    }
}

fn resolve_root(cwd: &Path, path: Option<&str>) -> PathBuf {
    match path {
        Some(path) => {
            let path = PathBuf::from(path);
            if path.is_absolute() {
                path
            } else {
                cwd.join(path)
            }
        }
        None => cwd.to_path_buf(),
    }
}

fn resolve_manifest_path(cwd: &Path, path: Option<&str>) -> PathBuf {
    let root = resolve_root(cwd, path);
    if root.is_dir() {
        root.join(MANIFEST_FILE)
    } else {
        root
    }
}

fn load_manifest(path: &Path) -> Result<ProjectManifest, String> {
    let content = fs::read_to_string(path).map_err(io_error)?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn write_json_file(
    path: &Path,
    manifest: &ProjectManifest,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }

    let content = serde_json::to_string_pretty(manifest).map_err(|error| error.to_string())?;
    write_if_changed(path, &content, wrote_files)
}

fn write_new_file(
    path: &Path,
    content: &str,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }

    fs::write(path, content.as_bytes()).map_err(io_error)?;
    wrote_files.push(path.to_path_buf());
    Ok(())
}

fn ensure_module_file(
    path: &Path,
    required_lines: &[&str],
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let mut content = if path.exists() {
        fs::read_to_string(path).map_err(io_error)?
    } else {
        String::new()
    };

    let mut changed = !path.exists();
    for line in required_lines {
        if !content.lines().any(|existing| existing.trim() == *line) {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(line);
            content.push('\n');
            changed = true;
        }
    }

    if changed {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(io_error)?;
        }
        fs::write(path, content.as_bytes()).map_err(io_error)?;
        wrote_files.push(path.to_path_buf());
    }

    Ok(())
}

fn ensure_module_line(
    path: &Path,
    line: &str,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    ensure_module_file(path, &[line], wrote_files)
}

fn write_if_changed(
    path: &Path,
    content: &str,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let existing = fs::read_to_string(path).ok();
    if existing.as_deref() == Some(content) {
        return Ok(());
    }

    fs::write(path, content.as_bytes()).map_err(io_error)?;
    wrote_files.push(path.to_path_buf());
    Ok(())
}

fn describe_writes(command: &str, wrote_files: &[PathBuf], root: &Path) -> String {
    if wrote_files.is_empty() {
        return format!("{command}: no file changes");
    }

    let lines = wrote_files
        .iter()
        .map(|path| {
            path.strip_prefix(root)
                .unwrap_or(path)
                .display()
                .to_string()
        })
        .collect::<Vec<_>>();
    format!("{command}: wrote {}", lines.join(", "))
}

fn format_validation_message(message: &ValidationMessage) -> String {
    let severity = match message.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    };

    match &message.repair {
        Some(repair) => format!(
            "{severity} [{}] {}: {}\n  repair: {}",
            message.rule_id, message.target, message.message, repair
        ),
        None => format!(
            "{severity} [{}] {}: {}",
            message.rule_id, message.target, message.message
        ),
    }
}

fn list_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_owned()
    } else {
        values.join(", ")
    }
}

fn token_set(value: &str) -> BTreeSet<String> {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| part.len() > 2)
        .map(|part| part.to_ascii_lowercase())
        .collect()
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}

fn help_text() -> String {
    "new-alphabet <command>\ncommands: init, add recipe <name>, add component <name>, explain <item>, inspect [path], validate [path], export context [--output path], plan <intent>".to_owned()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::run_from;

    #[test]
    fn init_creates_manifest_and_modules() {
        let root = unique_test_dir("init");
        let result =
            run_from(["new-alphabet", "init", "--name", "proof"], &root).expect("init to succeed");

        assert!(result.stdout.contains("new-alphabet.json"));
        assert!(root.join("new-alphabet.json").exists());
        assert!(root.join("src/new_alphabet/mod.rs").exists());
        assert!(root.join("src/new_alphabet/components/mod.rs").exists());
        assert!(root.join("src/new_alphabet/recipes/mod.rs").exists());
    }

    #[test]
    fn init_is_idempotent() {
        let root = unique_test_dir("init-idempotent");
        run_from(["new-alphabet", "init", "--name", "proof"], &root).expect("init");

        let result =
            run_from(["new-alphabet", "init", "--name", "proof"], &root).expect("second init");

        assert_eq!(result.stdout, "init: no file changes");
        let manifest = fs::read_to_string(root.join("new-alphabet.json")).expect("manifest");
        assert!(manifest.contains("\"project_name\": \"proof\""));
    }

    #[test]
    fn add_recipe_updates_manifest_and_writes_scaffold() {
        let root = unique_test_dir("add-recipe");
        run_from(["new-alphabet", "init"], &root).expect("init");
        let result =
            run_from(["new-alphabet", "add", "recipe", "review-queue"], &root).expect("add recipe");

        assert!(result.stdout.contains("review_queue.rs"));
        assert!(
            root.join("src/new_alphabet/recipes/review_queue.rs")
                .exists()
        );
        let scaffold = fs::read_to_string(root.join("src/new_alphabet/recipes/review_queue.rs"))
            .expect("recipe scaffold");
        assert!(scaffold.contains("REQUIRED_PRIMITIVES"));
        assert!(scaffold.contains("REQUIRED_COMPONENTS"));
        assert!(scaffold.contains("REFERENCE_EXAMPLES"));
        assert!(scaffold.contains("DOCUMENTATION_PATHS"));
        let manifest = fs::read_to_string(root.join("new-alphabet.json")).expect("manifest");
        assert!(manifest.contains("\"recipe\": \"ReviewQueue\""));
    }

    #[test]
    fn add_recipe_is_idempotent() {
        let root = unique_test_dir("add-recipe-idempotent");
        run_from(["new-alphabet", "init"], &root).expect("init");
        run_from(["new-alphabet", "add", "recipe", "review-queue"], &root).expect("first add");

        let result =
            run_from(["new-alphabet", "add", "recipe", "review-queue"], &root).expect("second add");

        assert_eq!(result.stdout, "add recipe: no file changes");
    }

    #[test]
    fn add_recipe_rejects_unsupported_name() {
        let root = unique_test_dir("add-recipe-unsupported");
        run_from(["new-alphabet", "init"], &root).expect("init");

        let error = run_from(["new-alphabet", "add", "recipe", "magic-dashboard"], &root)
            .expect_err("unsupported recipe to fail");

        assert!(error.contains("unsupported recipe"));
    }

    #[test]
    fn add_component_writes_component_scaffold() {
        let root = unique_test_dir("add-component");
        run_from(["new-alphabet", "init"], &root).expect("init");
        let result =
            run_from(["new-alphabet", "add", "component", "button"], &root).expect("add component");

        assert!(result.stdout.contains("button.rs"));
        let scaffold = fs::read_to_string(root.join("src/new_alphabet/components/button.rs"))
            .expect("scaffold");
        assert!(scaffold.contains("COMPONENT_ID"));
        assert!(scaffold.contains("REQUIRED_STATES"));
        assert!(scaffold.contains("BUILT_FROM_PRIMITIVES"));
        assert!(scaffold.contains("FOUNDATION_BINDINGS"));
        assert!(scaffold.contains("DOCUMENTATION_PATHS"));
        assert!(scaffold.contains("ActionPriority::Primary"));
    }

    #[test]
    fn add_component_rejects_unsupported_name() {
        let root = unique_test_dir("add-component-unsupported");
        run_from(["new-alphabet", "init"], &root).expect("init");

        let error = run_from(["new-alphabet", "add", "component", "magic-panel"], &root)
            .expect_err("unsupported component to fail");

        assert!(error.contains("unsupported component"));
    }

    #[test]
    fn explain_recipe_reports_framework_terms() {
        let root = unique_test_dir("explain");
        let result = run_from(["new-alphabet", "explain", "ReviewQueue"], &root).expect("explain");

        assert!(result.stdout.contains("required_regions"));
        assert!(result.stdout.contains("adaptation_limits"));
    }

    #[test]
    fn validate_reports_errors_warnings_and_notes() {
        let root = unique_test_dir("validate");
        run_from(["new-alphabet", "init"], &root).expect("init");
        fs::write(
            root.join("new-alphabet.json"),
            "{\n  \"schema_version\": \"0.0.0\",\n  \"project_name\": \"magic\",\n  \"surfaces\": []\n}\n",
        )
        .expect("write manifest");

        let result = run_from(["new-alphabet", "validate"], &root).expect("validate");

        assert!(result.stdout.contains("error"));
        assert!(result.stdout.contains("note [summary]"));
    }

    #[test]
    fn export_context_writes_bundle_when_output_is_given() {
        let root = unique_test_dir("export");
        let result = run_from(
            [
                "new-alphabet",
                "export",
                "context",
                "--output",
                "schemas/context.json",
            ],
            &root,
        )
        .expect("export");

        assert!(result.stdout.contains("export context wrote"));
        assert!(root.join("schemas/context.json").exists());
    }

    #[test]
    fn plan_selects_a_recipe_from_intent_language() {
        let root = unique_test_dir("plan");
        let result = run_from(
            [
                "new-alphabet",
                "plan",
                "Build a dense review workspace with a rail, queue, and detail pane",
            ],
            &root,
        )
        .expect("plan");

        assert!(result.stdout.contains("recipe: ReviewQueue"));
    }

    fn unique_test_dir(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "new-alphabet-cli-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }
}
