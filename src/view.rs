use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Clear, HighlightSpacing, List, ListItem},
    Frame,
};
use tui_input::Input;

use crate::{project::Project, task::Task, ui::Ui, util::Util, App, ViewMode};

pub struct View {}

impl View {
    pub fn show_new_modal(f: &mut Frame, area: Rect, input: &Input) {
        Ui::create_input("New", f, area, input)
    }

    pub fn show_rename_modal(f: &mut Frame, area: Rect, input: &Input) {
        Ui::create_input("Rename", f, area, input)
    }

    pub fn show_delete_modal(app: &mut App, f: &mut Frame, area: Rect) {
        let title = match app.view_mode {
            ViewMode::DeleteTask => &Task::get_current(app).title,
            ViewMode::DeleteProject => &Project::get_current(app).title,
            _ => "",
        };

        Ui::create_question_modal(
            "Are you sure to delete?",
            format!("\"{}\"", title).as_str(),
            "Delete",
            f,
            area,
        )
    }

    pub fn show_select_task_status_modal(
        app: &mut App,
        status_items: &Vec<ListItem>,
        f: &mut Frame,
        area: Rect,
    ) {
        let area = Ui::create_rect_area(10, 5, area);

        let task_status_list_widget = List::new(status_items.clone())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(Block::bordered().title("Status"));

        f.render_widget(Clear, area);
        f.render_stateful_widget(task_status_list_widget, area, app.use_state())
    }

    pub fn show_items(app: &mut App, items: &Vec<ListItem>, f: &mut Frame, area: Rect) {
        let block: Block = match app.view_mode {
            ViewMode::ViewProjects
            | ViewMode::AddProject
            | ViewMode::RenameProject
            | ViewMode::DeleteProject => Block::bordered(),
            _ => Block::bordered().title(Util::get_spaced_title(&Project::get_current(app).title)),
        };

        // Iterate through all elements in the `items` and stylize them.
        let items = items.clone();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        if app.view_mode != ViewMode::ChangeStatusTask {
            f.render_stateful_widget(items, area, app.use_state());
        } else {
            f.render_widget(items, area)
        }
    }

    pub fn show_footer_helper(app: &mut App, f: &mut Frame, area: Rect) {
        let help_string = match app.view_mode {
            ViewMode::ViewProjects => {
                "<Up/Down> next/prev - <Enter/Right> go to tasks - <n> new - <r> rename - <d> delete - <q> quit"
            }
            ViewMode::RenameProject => "<Enter> confirm - <Esc> cancel",
            ViewMode::AddProject => "<Enter> confirm - <Esc> cancel",
            ViewMode::DeleteProject => "<y> confirm - <n> cancel",

            ViewMode::ViewTasks => {
                "<Up/Down> next/prev - <Esc/Left> go to projects - <Enter> change status - <n> new - <r> rename - <d> delete - <q> quit"
            }
            ViewMode::RenameTask => "<Enter> confirm - <Esc> cancel",
            ViewMode::ChangeStatusTask => "<Up/Down> next/prev - <Enter> confirm - <Esc> cancel",
            ViewMode::AddTask => "<Enter> confirm - <Esc> cancel",
            ViewMode::DeleteTask => "<y> confirm - <n> cancel",
        };

        f.render_widget(
            Block::new().title_bottom(Line::from(help_string).centered()),
            area,
        );
    }
}
