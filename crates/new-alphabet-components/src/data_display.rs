use leptos::prelude::*;
use new_alphabet_foundation::StateToken;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableState {
    Default,
    Loading,
    Empty,
    Error,
}

impl Default for TableState {
    fn default() -> Self {
        Self::Default
    }
}

impl TableState {
    fn id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Loading => "loading",
            Self::Empty => "empty",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableCellMode {
    Truncate,
    Wrap,
}

impl TableCellMode {
    fn id(self) -> &'static str {
        match self {
            Self::Truncate => "truncate",
            Self::Wrap => "wrap",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableColumn {
    pub key: &'static str,
    pub label: &'static str,
    pub mode: TableCellMode,
}

impl TableColumn {
    pub const fn truncate(key: &'static str, label: &'static str) -> Self {
        Self {
            key,
            label,
            mode: TableCellMode::Truncate,
        }
    }

    pub const fn wrap(key: &'static str, label: &'static str) -> Self {
        Self {
            key,
            label,
            mode: TableCellMode::Wrap,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableRow {
    pub id: &'static str,
    pub cells: &'static [&'static str],
}

impl TableRow {
    pub const fn new(id: &'static str, cells: &'static [&'static str]) -> Self {
        Self { id, cells }
    }
}

fn effective_table_state(state: TableState, rows: &[TableRow]) -> TableState {
    if matches!(state, TableState::Default) && rows.is_empty() {
        TableState::Empty
    } else {
        state
    }
}

#[component]
pub fn Table(
    label: &'static str,
    columns: &'static [TableColumn],
    rows: &'static [TableRow],
    #[prop(optional)] state: Option<TableState>,
    #[prop(optional)] loading_message: Option<&'static str>,
    #[prop(optional)] empty_message: Option<&'static str>,
    #[prop(optional)] error_message: Option<&'static str>,
) -> impl IntoView {
    let state = effective_table_state(state.unwrap_or_default(), rows);
    let loading_message = loading_message.unwrap_or("Loading rows.");
    let empty_message = empty_message.unwrap_or("No rows available.");
    let error_message = error_message.unwrap_or("The table is unavailable.");

    let body = match state {
        TableState::Default => view! {
            <table>
                <caption>{label}</caption>
                <thead>
                    <tr>
                        {columns
                            .iter()
                            .map(|column| {
                                view! {
                                    <th
                                        scope="col"
                                        data-column-key=column.key
                                        data-cell-mode=column.mode.id()
                                    >
                                        {column.label}
                                    </th>
                                }
                            })
                            .collect_view()}
                    </tr>
                </thead>
                <tbody>
                    {rows
                        .iter()
                        .map(|row| {
                            view! {
                                <tr data-row-id=row.id>
                                    {columns
                                        .iter()
                                        .enumerate()
                                        .map(|(index, column)| {
                                            let cell = row.cells.get(index).copied().unwrap_or("");
                                            view! {
                                                <td
                                                    data-column-key=column.key
                                                    data-cell-mode=column.mode.id()
                                                    title=if matches!(column.mode, TableCellMode::Truncate) {
                                                        cell
                                                    } else {
                                                        ""
                                                    }
                                                >
                                                    {cell}
                                                </td>
                                            }
                                        })
                                        .collect_view()}
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
        }
        .into_any(),
        TableState::Loading => view! {
            <p
                class="na-table__status"
                role="status"
                data-kind="loading"
                data-state-token=StateToken::LoadingMuted.id()
            >
                {loading_message}
            </p>
        }
        .into_any(),
        TableState::Empty => view! {
            <p class="na-table__status" role="status" data-kind="empty">
                {empty_message}
            </p>
        }
        .into_any(),
        TableState::Error => view! {
            <p class="na-table__status" role="alert" data-kind="error">
                {error_message}
            </p>
        }
        .into_any(),
    };

    view! {
        <section
            class="na-table"
            aria-label=label
            data-state=state.id()
            data-density="dense"
            data-column-count=columns.len().to_string()
        >
            {body}
        </section>
    }
}

#[component]
pub fn MetricBlock(
    label: &'static str,
    value: &'static str,
    #[prop(optional)] context: Option<&'static str>,
    #[prop(optional)] note: Option<&'static str>,
) -> impl IntoView {
    view! {
        <section class="na-metric-block" aria-label=label data-kind="metric">
            <p class="na-metric-block__label">{label}</p>
            <p class="na-metric-block__value">{value}</p>
            {context.map(|text| view! { <p class="na-metric-block__context">{text}</p> })}
            {note.map(|text| view! { <p class="na-metric-block__note">{text}</p> })}
        </section>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationDirection {
    Previous,
    Next,
}

impl PaginationDirection {
    fn id(self) -> &'static str {
        match self {
            Self::Previous => "previous",
            Self::Next => "next",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Previous => "Previous",
            Self::Next => "Next",
        }
    }

    fn rel(self) -> &'static str {
        match self {
            Self::Previous => "prev",
            Self::Next => "next",
        }
    }
}

fn pagination_control(direction: PaginationDirection, href: Option<&'static str>) -> AnyView {
    match href {
        Some(href) => view! {
            <a
                class="na-pagination__control"
                href=href
                rel=direction.rel()
                data-direction=direction.id()
                data-focus-token=StateToken::FocusRing.id()
            >
                {direction.label()}
            </a>
        }
        .into_any(),
        None => view! {
            <span
                class="na-pagination__control"
                aria-disabled="true"
                data-direction=direction.id()
            >
                {direction.label()}
            </span>
        }
        .into_any(),
    }
}

#[component]
pub fn Pagination(
    current_page: usize,
    total_pages: usize,
    #[prop(optional)] previous_href: Option<&'static str>,
    #[prop(optional)] next_href: Option<&'static str>,
) -> impl IntoView {
    let total_pages = total_pages.max(1);
    let current_page = current_page.clamp(1, total_pages);
    let status = format!("Page {current_page} of {total_pages}");

    view! {
        <nav class="na-pagination" aria-label="Pagination" data-kind="pagination">
            {pagination_control(PaginationDirection::Previous, previous_href)}
            <p class="na-pagination__status" aria-live="polite">
                {status}
            </p>
            {pagination_control(PaginationDirection::Next, next_href)}
        </nav>
    }
}
