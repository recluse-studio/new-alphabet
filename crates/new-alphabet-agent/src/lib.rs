#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentTaskKind {
    BuildSurface,
    ValidateDrift,
    ExplainSystem,
    RefreshPublicArtifacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentBootstrap {
    pub required_files: &'static [&'static str],
    pub targeted_docs: &'static [&'static str],
    pub public_artifacts: &'static [&'static str],
    pub verification_commands: &'static [&'static str],
}

pub const REQUIRED_SESSION_FILES: &[&str] = &[
    "AGENTS.md",
    "documentation.md",
    "prd.json",
    "progress.txt",
    "schemas/context-bundle-0.1.0.json",
];

pub const BUILD_DOCS: &[&str] = &[
    "docs/foundations.md",
    "docs/primitives.md",
    "docs/components.md",
    "docs/recipes.md",
    "docs/cli.md",
    "docs/agent-contract.md",
];

pub const VALIDATION_DOCS: &[&str] = &[
    "docs/cli.md",
    "docs/agent-contract.md",
    "docs/foundations.md",
    "docs/components.md",
    "docs/recipes.md",
];

pub const EXPLAIN_DOCS: &[&str] = &[
    "docs/foundations.md",
    "docs/primitives.md",
    "docs/components.md",
    "docs/recipes.md",
    "docs/agent-contract.md",
];

pub const PUBLIC_PACKAGE_DOCS: &[&str] = &[
    "docs/agent-contract.md",
    "examples/README.md",
    "schemas/README.md",
    "prompts/README.md",
    "scripts/README.md",
];

pub const PUBLIC_ARTIFACT_PATHS: &[&str] = &[
    "examples/README.md",
    "schemas/README.md",
    "prompts/README.md",
    "scripts/README.md",
    "apps/demo-blog/site/index.html",
    "apps/demo-saas/site/index.html",
];

pub const VERIFICATION_COMMANDS: &[&str] = &[
    "cargo test --workspace",
    "scripts/build-demo-sites.sh",
    "scripts/export-public-artifacts.sh",
];

pub fn bootstrap_for(task: AgentTaskKind) -> AgentBootstrap {
    let targeted_docs = match task {
        AgentTaskKind::BuildSurface => BUILD_DOCS,
        AgentTaskKind::ValidateDrift => VALIDATION_DOCS,
        AgentTaskKind::ExplainSystem => EXPLAIN_DOCS,
        AgentTaskKind::RefreshPublicArtifacts => PUBLIC_PACKAGE_DOCS,
    };

    AgentBootstrap {
        required_files: REQUIRED_SESSION_FILES,
        targeted_docs,
        public_artifacts: PUBLIC_ARTIFACT_PATHS,
        verification_commands: VERIFICATION_COMMANDS,
    }
}

pub fn session_bootstrap_text() -> &'static str {
    include_str!("../../../prompts/session-bootstrap.md")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_bootstrap_includes_required_repo_files() {
        let bootstrap = bootstrap_for(AgentTaskKind::BuildSurface);

        assert_eq!(bootstrap.required_files, REQUIRED_SESSION_FILES);
        assert!(bootstrap.targeted_docs.contains(&"docs/recipes.md"));
        assert!(bootstrap.targeted_docs.contains(&"docs/components.md"));
    }

    #[test]
    fn refresh_bootstrap_includes_public_artifact_indexes() {
        let bootstrap = bootstrap_for(AgentTaskKind::RefreshPublicArtifacts);

        assert!(bootstrap.public_artifacts.contains(&"examples/README.md"));
        assert!(bootstrap.public_artifacts.contains(&"prompts/README.md"));
        assert!(
            bootstrap
                .verification_commands
                .contains(&"scripts/export-public-artifacts.sh")
        );
    }

    #[test]
    fn session_bootstrap_mentions_context_bundle() {
        let bootstrap = session_bootstrap_text();

        assert!(bootstrap.contains("schemas/context-bundle-0.1.0.json"));
        assert!(bootstrap.contains("Choose an existing recipe or allowed primitive composition"));
    }
}
