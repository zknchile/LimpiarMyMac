//! Pantallas en texto plano. Paleta de día lluvioso, con un velo. Los botones llevan sombra. No hay vidrio ni HTML.

use std::collections::HashSet;

use futures::StreamExt;
use gpui::{
    App, ClickEvent, Context, IntoElement, ParentElement, Render, ScrollHandle, Styled, Window,
    div, point, prelude::*, px, rgb, rgba,
};

use crate::model::{CleanItem, CleanKind, CleanProgress, CleanReport, Risk, ScanProgress, ScanReport};
use crate::place::{self, KeptPlace, Place};
use crate::selection::{self, SelectionSummary};
use crate::Session;

const SUMMARY: &str = "summary";

/// Nube `#e2e8f0`. Fondo.
const CLOUD: u32 = 0xe2e8f0;
/// La misma nube al 88 %. Un velo, no vidrio.
const VEIL: u32 = 0xe2e8f0e0;
/// Tinta `#0f172a`. Títulos y botón principal.
const INK: u32 = 0x0f172a;
/// Pizarra `#1e293b`. Texto.
const SLATE: u32 = 0x1e293b;
/// Ceniza `#475569`. Texto secundario y borde.
const ASH: u32 = 0x475569;
/// Pelo `#94a3b8`. Línea. No es texto: sobre la nube queda en 2.08:1.
const HAIR: u32 = 0x94a3b8;
/// Franja del menú. Nube al 80 % y pelo al 20 %: `#d2dae5`. Un poco más oscura que el velo.
const CAP: u32 = 0xd2dae5e0;
/// Alto medido de esa franja en esta Mac, el 2026-11-16.
const CAP_HEIGHT: f32 = 32.;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Scanning,
    Results,
    Cleaning,
    Done,
}

pub struct Cristal {
    session: Session,
    screen: Screen,
    scan_progress: Option<ScanProgress>,
    clean_progress: Option<CleanProgress>,
    selection: HashSet<String>,
    active: String,
    error: String,
    confirm: bool,
    result: Option<CleanReport>,
    list_scroll: ScrollHandle,
    sidebar_scroll: ScrollHandle,
    focus_id: String,
}

impl Cristal {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
            screen: Screen::Home,
            scan_progress: None,
            clean_progress: None,
            selection: HashSet::new(),
            active: SUMMARY.into(),
            error: String::new(),
            confirm: false,
            result: None,
            list_scroll: ScrollHandle::new(),
            sidebar_scroll: ScrollHandle::new(),
            focus_id: String::new(),
        }
    }

    fn start_scan(&mut self, cx: &mut Context<Self>) {
        if self.session.try_begin().is_err() {
            return;
        }
        self.screen = Screen::Scanning;
        self.confirm = false;
        self.error.clear();
        self.scan_progress = Some(ScanProgress {
            stage: "Preparando".into(),
            detail: "Esto no borra nada".into(),
            stage_index: 0,
            stage_count: 7,
            found_bytes: 0,
        });
        cx.notify();

        cx.spawn(async move |this, cx| {
            let (tx, mut rx) = futures::channel::mpsc::unbounded();
            let scanned = cx.background_spawn(async move {
                crate::scan::run(|progress| {
                    let _ = tx.unbounded_send(progress);
                })
            });
            let this_progress = this.clone();
            let progress_task = cx.spawn(async move |cx| {
                while let Some(progress) = rx.next().await {
                    let _ = this_progress.update(cx, |view, cx| {
                        view.scan_progress = Some(progress);
                        cx.notify();
                    });
                }
            });
            let result = scanned.await;
            progress_task.await;
            let _ = this.update(cx, |view, cx| {
                view.finish_scan(result);
                cx.notify();
            });
        })
        .detach();
    }

    fn finish_scan(&mut self, result: Result<ScanReport, String>) {
        self.session.end();
        match result {
            Ok(report) => {
                self.selection = report
                    .categories
                    .iter()
                    .flat_map(|category| category.items.iter())
                    .filter(|item| item.default_selected)
                    .map(|item| item.id.clone())
                    .collect();
                self.session.store_report(report);
                self.active = SUMMARY.into();
                self.focus_id.clear();
                self.screen = Screen::Results;
                self.list_scroll.set_offset(point(px(0.), px(0.)));
                self.sidebar_scroll.set_offset(point(px(0.), px(0.)));
            }
            Err(error) => {
                self.error = error;
                self.screen = Screen::Home;
            }
        }
    }

    fn start_clean(&mut self, cx: &mut Context<Self>) {
        if self.selection.is_empty() {
            return;
        }
        let Some(report) = self.session.report() else {
            self.error = "Primero analiza la Mac.".into();
            self.confirm = false;
            return;
        };
        if self.session.try_begin().is_err() {
            return;
        }
        let home = match dirs::home_dir() {
            Some(home) => home,
            None => {
                self.session.end();
                self.error = "No se encontró tu carpeta de usuario.".into();
                self.confirm = false;
                return;
            }
        };
        let ids: Vec<String> = self.selection.iter().cloned().collect();
        self.confirm = false;
        self.screen = Screen::Cleaning;
        self.clean_progress = Some(CleanProgress {
            name: "Preparando".into(),
            done: 0,
            total: ids.len() as u32,
        });
        cx.notify();

        cx.spawn(async move |this, cx| {
            let (tx, mut rx) = futures::channel::mpsc::unbounded();
            let cleaned = cx.background_spawn(async move {
                crate::clean::execute(&home, &report, &ids, |progress| {
                    let _ = tx.unbounded_send(progress);
                })
            });
            let this_progress = this.clone();
            let progress_task = cx.spawn(async move |cx| {
                while let Some(progress) = rx.next().await {
                    let _ = this_progress.update(cx, |view, cx| {
                        view.clean_progress = Some(progress);
                        cx.notify();
                    });
                }
            });
            let result = cleaned.await;
            progress_task.await;
            let _ = this.update(cx, |view, cx| {
                view.finish_clean(result);
                cx.notify();
            });
        })
        .detach();
    }

    fn finish_clean(&mut self, result: Result<CleanReport, String>) {
        self.session.end();
        match result {
            Ok(report) => {
                self.result = Some(report);
                self.screen = Screen::Done;
            }
            Err(error) => {
                self.error = error;
                self.screen = Screen::Results;
            }
        }
    }

    fn toggle(&mut self, id: String, on: bool) {
        let before = self.capture_place();
        if on {
            self.selection.insert(id.clone());
        } else {
            self.selection.remove(&id);
        }
        self.focus_id = id;
        self.apply_kept(&place::place_to_keep(
            Some(&before),
            "results",
            &self.active,
            self.confirm,
        ));
    }

    fn open_category(&mut self, id: String) {
        let before = self.capture_place();
        self.active = id;
        self.apply_kept(&place::place_to_keep(
            Some(&before),
            "results",
            &self.active,
            false,
        ));
    }

    fn open_confirm(&mut self) {
        if self.selected_items().is_empty() {
            return;
        }
        let before = self.capture_place();
        self.confirm = true;
        self.apply_kept(&place::place_to_keep(
            Some(&before),
            "results",
            &self.active,
            true,
        ));
    }

    fn capture_place(&self) -> Place {
        Place {
            screen: screen_name(self.screen).into(),
            active: self.active.clone(),
            list: Some(scroll_top(&self.list_scroll)),
            sidebar: Some(scroll_top(&self.sidebar_scroll)),
            focus_id: self.focus_id.clone(),
        }
    }

    fn apply_kept(&mut self, kept: &KeptPlace) {
        if let Some(list) = kept.list {
            self.list_scroll
                .set_offset(point(px(0.), px(-(list as f32))));
        } else if self.screen == Screen::Results {
            self.list_scroll.set_offset(point(px(0.), px(0.)));
        }
        if let Some(sidebar) = kept.sidebar {
            self.sidebar_scroll
                .set_offset(point(px(0.), px(-(sidebar as f32))));
        }
        self.focus_id = kept.focus_id.clone();
    }

    fn items(&self) -> Vec<CleanItem> {
        self.session
            .report()
            .map(|report| {
                report
                    .categories
                    .into_iter()
                    .flat_map(|category| category.items)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn selected_items(&self) -> Vec<CleanItem> {
        self.items()
            .into_iter()
            .filter(|item| self.selection.contains(&item.id))
            .collect()
    }

    fn summary_of(&self, items: &[CleanItem]) -> SelectionSummary {
        selection::selection_of(
            items.iter().map(|item| (item.id.as_str(), item.bytes)),
            &self.selection,
        )
    }
}

impl Render for Cristal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = if self.confirm && self.screen == Screen::Results {
            self.confirm_view(cx)
        } else {
            match self.screen {
                Screen::Home => self.home_view().into_any_element(),
                Screen::Scanning => self.scanning_view().into_any_element(),
                Screen::Cleaning => self.cleaning_view().into_any_element(),
                Screen::Done => self.done_view(cx).into_any_element(),
                Screen::Results => self.results_view(cx).into_any_element(),
            }
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgba(VEIL))
            .text_color(rgb(SLATE))
            .child(self.cap())
            .child(div().flex_1().overflow_hidden().child(body))
            .child(self.dock(cx))
    }
}

impl Cristal {
    fn cap(&self) -> impl IntoElement {
        div()
            .h(px(CAP_HEIGHT))
            .w_full()
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(CAP))
            .text_color(rgb(INK))
            .text_size(px(13.))
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .child("Cristal")
    }

    fn dock(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let busy = self.session.is_busy();
        let label = if self.session.has_report() {
            "Analizar de nuevo"
        } else {
            "Analizar esta Mac"
        };
        let scan = cx.listener(|this, _: &ClickEvent, _, cx| {
            this.start_scan(cx);
        });
        div()
            .flex()
            .flex_none()
            .justify_end()
            .items_center()
            .gap_3()
            .p_4()
            .border_t_1()
            .border_color(rgb(HAIR))
            .bg(rgba(VEIL))
            .child(button("analyze", label, !busy, true, scan))
            .child(button("quit", "Salir", true, false, |_, _, cx| cx.quit()))
    }

    fn home_view(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_12()
            .child(heading("Cristal"))
            .child("Encuentra cachés, restos de apps borradas y basura de los navegadores. Tú decides qué se mueve a la Papelera.")
            .child("Primero analiza. Tú eliges. Va a la Papelera.")
            .when(!self.error.is_empty(), |this| this.child(self.error.clone()))
    }

    fn scanning_view(&self) -> impl IntoElement {
        let progress = self.scan_progress.clone().unwrap_or(ScanProgress {
            stage: "Analizando".into(),
            detail: String::new(),
            stage_index: 1,
            stage_count: 7,
            found_bytes: 0,
        });
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_12()
            .child(heading(progress.stage))
            .child(progress.detail)
            .child(format!(
                "{} encontrados · paso {} de {}",
                selection::format_bytes(progress.found_bytes),
                progress.stage_index.max(1),
                progress.stage_count.max(1)
            ))
            .child("Todavía no se borra nada.")
    }

    fn cleaning_view(&self) -> impl IntoElement {
        let progress = self.clean_progress.clone().unwrap_or(CleanProgress {
            name: "Preparando".into(),
            done: 0,
            total: 0,
        });
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_12()
            .child(heading("Moviendo a la Papelera"))
            .child(progress.name)
            .child(format!("{} de {}", progress.done, progress.total))
    }

    fn done_view(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let result = self.result.clone().unwrap_or(CleanReport {
            moved: 0,
            bytes: 0,
            failures: Vec::new(),
            emptied_trash: false,
            notes: Vec::new(),
        });
        let open = cx.listener(|this, _: &ClickEvent, _, cx| {
            if let Err(error) = crate::open_trash() {
                this.error = error;
                cx.notify();
            }
        });
        let mut column = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_12()
            .child(heading("Listo"))
            .child(format!(
                "{} elementos · {}. {}",
                result.moved,
                selection::format_bytes(result.bytes),
                if result.emptied_trash {
                    "La Papelera se vació."
                } else {
                    "Puedes restaurarlos desde la Papelera."
                }
            ));
        for note in result.notes {
            column = column.child(note);
        }
        if let Some(failure) = result.failures.first() {
            column = column.child(format!(
                "{} no se pudieron mover. {} {}",
                result.failures.len(),
                failure.path,
                failure.message
            ));
        }
        column.child(button("open-trash", "Abrir Papelera", true, false, open))
    }

    fn results_view(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(report) = self.session.report() else {
            return div().p_12().child("No hay informe.").into_any_element();
        };
        let selected = self.summary_of(&self.items());
        let nav = self.nav(&report, cx);
        let pane = if self.active == SUMMARY {
            self.summary_pane(&report, cx)
        } else {
            self.category_pane(&report, cx)
        };
        let open_confirm = cx.listener(|this, _: &ClickEvent, _, cx| {
            this.open_confirm();
            cx.notify();
        });
        div()
            .flex()
            .size_full()
            .child(
                div()
                    .id("sidebar")
                    .w(px(240.))
                    .h_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.sidebar_scroll)
                    .p_2()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(nav),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .child(self.banner())
                    .child(pane)
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .justify_between()
                            .items_center()
                            .p_4()
                            .border_t_1()
                            .border_color(rgb(HAIR))
                            .bg(rgba(VEIL))
                            .child(format!(
                                "{} · {} seleccionados",
                                selection::format_bytes(selected.bytes),
                                selected.count
                            ))
                            .child(button(
                                "open-confirm",
                                "Revisar y limpiar",
                                selected.count > 0,
                                true,
                                open_confirm,
                            )),
                    ),
            )
            .into_any_element()
    }

    fn nav(&mut self, report: &ScanReport, cx: &mut Context<Self>) -> Vec<gpui::AnyElement> {
        let mut rows = Vec::new();
        let selected = self.summary_of(&self.items());
        rows.push(self.nav_button(
            SUMMARY,
            "Resumen",
            &selection::format_bytes(selected.bytes),
            cx,
        ));
        for category in &report.categories {
            let picked = self.summary_of(&category.items);
            let meta = format!(
                "{} de {}",
                selection::format_bytes(picked.bytes),
                selection::format_bytes(picked.total)
            );
            rows.push(self.nav_button(&category.id, &category.title, &meta, cx));
        }
        rows
    }

    fn nav_button(
        &mut self,
        id: &str,
        title: &str,
        meta: &str,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let target = id.to_string();
        let open = cx.listener(move |this, _: &ClickEvent, _, cx| {
            this.open_category(target.clone());
            cx.notify();
        });
        let active = self.active == id;
        div()
            .id(format!("nav-{id}"))
            .w_full()
            .p_2()
            .rounded_md()
            .cursor_pointer()
            .bg(if active { rgb(SLATE) } else { rgba(VEIL) })
            .text_color(if active { rgb(CLOUD) } else { rgb(SLATE) })
            .child(title.to_string())
            .child(
                div()
                    .text_color(if active { rgb(CLOUD) } else { rgb(ASH) })
                    .child(meta.to_string()),
            )
            .on_click(open)
            .into_any_element()
    }

    fn summary_pane(&mut self, report: &ScanReport, cx: &mut Context<Self>) -> gpui::AnyElement {
        let selected = self.summary_of(&self.items());
        let total: u64 = self.items().iter().map(|item| item.bytes).sum();
        let mut rows = Vec::new();
        for category in &report.categories {
            let id = category.id.clone();
            let picked = self.summary_of(&category.items);
            let open = cx.listener(move |this, _: &ClickEvent, _, cx| {
                this.open_category(id.clone());
                cx.notify();
            });
            rows.push(
                div()
                    .id(format!("summary-{}", category.id))
                    .w_full()
                    .p_2()
                    .cursor_pointer()
                    .border_b_1()
                    .border_color(rgb(HAIR))
                    .child(category.title.clone())
                    .child(format!(
                        "{} de {} seleccionados",
                        picked.count,
                        category.items.len()
                    ))
                    .child(selection::format_bytes(picked.bytes))
                    .on_click(open)
                    .into_any_element(),
            );
        }
        div()
            .id("list")
            .flex_1()
            .overflow_y_scroll()
            .track_scroll(&self.list_scroll)
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child("Selección segura, lista para revisar")
            .child(selection::format_bytes(selected.bytes))
            .children(report.notes.iter().cloned())
            .child(format!(
                "{} encontrados en total · {} apps instaladas · {} s",
                selection::format_bytes(total),
                report.apps_found,
                report.elapsed_ms / 1000
            ))
            .children(rows)
            .into_any_element()
    }

    fn category_pane(&mut self, report: &ScanReport, cx: &mut Context<Self>) -> gpui::AnyElement {
        let Some(category) = report
            .categories
            .iter()
            .find(|category| category.id == self.active)
        else {
            return div().p_4().child("Esa categoría no está.").into_any_element();
        };
        let picked = self.summary_of(&category.items);
        let all_on = !category.items.is_empty()
            && category
                .items
                .iter()
                .all(|item| self.selection.contains(&item.id));
        let ids: Vec<String> = category.items.iter().map(|item| item.id.clone()).collect();
        let toggle_all = cx.listener(move |this, _: &ClickEvent, _, cx| {
            let before = this.capture_place();
            if ids.iter().all(|id| this.selection.contains(id)) {
                for id in &ids {
                    this.selection.remove(id);
                }
            } else {
                for id in &ids {
                    this.selection.insert(id.clone());
                }
            }
            this.apply_kept(&place::place_to_keep(
                Some(&before),
                "results",
                &this.active.clone(),
                this.confirm,
            ));
            cx.notify();
        });
        let mut rows = Vec::new();
        for item in &category.items {
            rows.push(self.item_row(item, cx));
        }
        div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_hidden()
            .child(
                div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(category.title.clone())
                    .child(category.description.clone())
                    .child(format!(
                        "{} seleccionados · {}",
                        picked.count,
                        selection::format_bytes(picked.bytes)
                    ))
                    .child(button(
                        "toggle-category",
                        if all_on {
                            "Quitar esta lista"
                        } else {
                            "Marcar esta lista"
                        },
                        !category.items.is_empty(),
                        false,
                        toggle_all,
                    )),
            )
            .child(
                div()
                    .id("list")
                    .flex_1()
                    .overflow_y_scroll()
                    .track_scroll(&self.list_scroll)
                    .p_2()
                    .flex()
                    .flex_col()
                    .children(rows),
            )
            .into_any_element()
    }

    fn item_row(&mut self, item: &CleanItem, cx: &mut Context<Self>) -> gpui::AnyElement {
        let id = item.id.clone();
        let on = self.selection.contains(&item.id);
        let toggle = cx.listener(move |this, _: &ClickEvent, _, cx| {
            let next = !this.selection.contains(&id);
            this.toggle(id.clone(), next);
            cx.notify();
        });
        let path = item.delete_paths.first().cloned();
        let reveal = path.map(|path| {
            let open = cx.listener(move |this, _: &ClickEvent, _, cx| {
                if let Err(error) = crate::reveal(std::path::Path::new(&path)) {
                    this.error = error;
                }
                cx.notify();
            });
            button(
                format!("reveal-{}", item.id),
                "Ver",
                true,
                false,
                open,
            )
        });
        let extra = if item.delete_paths.len() > 1 {
            format!(" · {} carpetas", item.delete_paths.len())
        } else {
            String::new()
        };
        let when = age(item.modified_secs);
        div()
            .id(format!("item-{}", item.id))
            .w_full()
            .p_2()
            .border_b_1()
            .border_color(rgb(HAIR))
            .flex()
            .gap_2()
            .child(
                div()
                    .id(format!("check-{}", item.id))
                    .cursor_pointer()
                    .child(if on { "[x]" } else { "[ ]" })
                    .on_click(toggle),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .child(item.name.clone())
                    .child(note(item.reason.clone()))
                    .child(note(format!(
                        "{}{}{}",
                        item.display_path,
                        extra,
                        if when.is_empty() {
                            String::new()
                        } else {
                            format!(" · {when}")
                        }
                    )))
                    .children(reveal),
            )
            .child(note(risk_label(item.risk)))
            .child(note(selection::format_bytes(item.bytes)))
            .into_any_element()
    }

    fn confirm_view(&mut self, cx: &mut Context<Self>) -> gpui::AnyElement {
        let picked = self.selected_items();
        let bytes = picked.iter().map(|item| item.bytes).sum();
        let empties = picked.iter().any(|item| item.kind == CleanKind::EmptyTrash);
        let has_review = picked.iter().any(|item| item.risk == Risk::Review);
        let report = self.session.report();
        let mut lines = Vec::new();
        if let Some(report) = report {
            for category in report.categories {
                let items: Vec<_> = category
                    .items
                    .iter()
                    .filter(|item| self.selection.contains(&item.id))
                    .cloned()
                    .collect();
                if items.is_empty() {
                    continue;
                }
                let total: u64 = items.iter().map(|item| item.bytes).sum();
                lines.push(format!(
                    "{} · {} · {}",
                    category.title,
                    items.len(),
                    selection::format_bytes(total)
                ));
            }
        }
        let mut largest = picked.clone();
        largest.sort_by_key(|item| std::cmp::Reverse(item.bytes));
        for item in largest.into_iter().take(5) {
            lines.push(format!(
                "{} · {}",
                item.name,
                selection::format_bytes(item.bytes)
            ));
        }
        let cancel = cx.listener(|this, _: &ClickEvent, _, cx| {
            this.confirm = false;
            cx.notify();
        });
        let confirm = cx.listener(|this, _: &ClickEvent, _, cx| {
            this.start_clean(cx);
        });
        let mut column = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_12()
            .child(heading(if empties {
                "Esto incluye vaciar la Papelera"
            } else {
                "Mover a la Papelera"
            }))
            .child(format!(
                "{} elementos · {}. {}",
                picked.len(),
                selection::format_bytes(bytes),
                if empties {
                    "Vaciar la Papelera no se puede deshacer."
                } else {
                    "Después puedes restaurarlos desde la Papelera."
                }
            ))
            .children(lines);
        if has_review {
            column = column.child("Incluiste elementos marcados como Revisar. Usa Ver si no estás seguro.");
        }
        column
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(button("cancel-clean", "Cancelar", true, false, cancel))
                    .child(button(
                        "confirm-clean",
                        if empties {
                            "Limpiar y vaciar"
                        } else {
                            "Mover a la Papelera"
                        },
                        !picked.is_empty(),
                        true,
                        confirm,
                    )),
            )
            .into_any_element()
    }

    fn banner(&self) -> gpui::AnyElement {
        if self.error.is_empty() {
            div().into_any_element()
        } else {
            div().p_3().child(self.error.clone()).into_any_element()
        }
    }
}

fn button(
    id: impl Into<gpui::ElementId>,
    label: &str,
    enabled: bool,
    primary: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> gpui::AnyElement {
    let bg = if primary && enabled {
        rgb(INK)
    } else {
        rgba(VEIL)
    };
    let color = if !enabled {
        rgb(ASH)
    } else if primary {
        rgb(CLOUD)
    } else {
        rgb(SLATE)
    };
    let edge = if primary && enabled { rgb(INK) } else { rgb(ASH) };
    let mut button = div()
        .id(id)
        .px_4()
        .py_2p5()
        .rounded_md()
        .border_1()
        .border_color(edge)
        .bg(bg)
        .text_color(color)
        .child(label.to_string());
    if enabled {
        button = button
            .shadow(button_shadow(primary, false))
            .cursor_pointer()
            .on_click(on_click)
            .hover(move |style| style.shadow(button_shadow(primary, true)))
            .active(|style| style.shadow(pressed_shadow()));
    } else {
        button = button.opacity(0.55);
    }
    button.into_any_element()
}

/// Tinta o nube con alfa. No es un sexto color.
fn tone(hex: u32, alpha: u8) -> gpui::Hsla {
    rgba((hex << 8) | u32::from(alpha)).into()
}

fn button_shadow(primary: bool, lifted: bool) -> Vec<gpui::BoxShadow> {
    let ink = tone(
        INK,
        if primary {
            if lifted { 92 } else { 82 }
        } else if lifted {
            56
        } else {
            46
        },
    );
    if primary {
        vec![
            gpui::BoxShadow::new(px(0.), px(if lifted { 5. } else { 3. }), ink)
                .blur_radius(px(if lifted { 12. } else { 8. })),
            gpui::BoxShadow::new(px(0.), px(1.), tone(CLOUD, 56)).inset(),
        ]
    } else {
        vec![gpui::BoxShadow::new(
            px(0.),
            px(if lifted { 2. } else { 1. }),
            ink,
        )
        .blur_radius(px(if lifted { 6. } else { 4. }))]
    }
}

fn pressed_shadow() -> Vec<gpui::BoxShadow> {
    vec![gpui::BoxShadow::new(px(0.), px(2.), tone(INK, 51))
        .blur_radius(px(4.))
        .inset()]
}

fn screen_name(screen: Screen) -> &'static str {
    match screen {
        Screen::Home => "home",
        Screen::Scanning => "scanning",
        Screen::Results => "results",
        Screen::Cleaning => "cleaning",
        Screen::Done => "done",
    }
}

fn scroll_top(handle: &ScrollHandle) -> f64 {
    -f64::from(handle.offset().y.as_f32())
}

fn heading(text: impl Into<String>) -> gpui::AnyElement {
    div()
        .text_color(rgb(INK))
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .child(text.into())
        .into_any_element()
}

fn note(text: impl Into<String>) -> gpui::AnyElement {
    div()
        .text_color(rgb(ASH))
        .child(text.into())
        .into_any_element()
}

fn risk_label(risk: Risk) -> &'static str {
    match risk {
        Risk::Safe => "Seguro",
        Risk::Review => "Revisar",
        Risk::Permanent => "Permanente",
    }
}

fn age(secs: Option<i64>) -> String {
    let Some(secs) = secs else {
        return String::new();
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(secs);
    let days = (now - secs) as f64 / 86400.0;
    if days < 1.0 {
        "hoy".into()
    } else if days < 2.0 {
        "ayer".into()
    } else {
        format!("hace {} días", days.floor() as i64)
    }
}
