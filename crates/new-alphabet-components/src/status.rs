use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusSeverity {
    Info,
    Success,
    Warning,
    Error,
}

impl StatusSeverity {
    fn id(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }
}

#[component]
pub fn StatusBadge(label: &'static str, severity: StatusSeverity) -> impl IntoView {
    view! {
        <span
            class="na-status-badge"
            data-severity=severity.id()
        >
            <span class="na-status-badge__prefix">{severity.label()}</span>
            <span class="na-status-badge__label">{label}</span>
        </span>
    }
}

#[component]
pub fn InlineAlert(
    title: &'static str,
    message: &'static str,
    severity: StatusSeverity,
) -> impl IntoView {
    view! {
        <section
            class="na-inline-alert"
            role="status"
            data-severity=severity.id()
        >
            <p class="na-inline-alert__title">
                <strong>{severity.label()}</strong>
                " "
                {title}
            </p>
            <p class="na-inline-alert__message">{message}</p>
        </section>
    }
}

#[component]
pub fn EmptyState(
    title: &'static str,
    message: &'static str,
    next_action: &'static str,
) -> impl IntoView {
    view! {
        <section class="na-empty-state" role="status" data-kind="empty">
            <h3>{title}</h3>
            <p>{message}</p>
            <p class="na-empty-state__action">{next_action}</p>
        </section>
    }
}
