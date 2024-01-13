use crate::app::{App, CurrentScreen};
use ratatui::{
    prelude::*,
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph, Scrollbar,
        ScrollbarOrientation,
    },
    Frame,
};

const TODO_INPUT_HEIGHT: u16 = 10;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_body(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let header_block = Block::default()
        .title(" Todo ")
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    if app.todos.len() == 0 {
        let empty_todo = Paragraph::new(" Nothing left to do! 😎").block(header_block);
        frame.render_widget(empty_todo, chunk);
        return;
    }

    let mut list_items: Vec<ListItem> = vec![];
    for (_id, todo) in app.todos.iter() {
        let mut completed = Span::styled("[ ] ", Style::default());
        let mut label = Span::styled(todo.label.clone(), Style::default());
        if todo.completed {
            completed = completed
                .content("[] ")
                .style(Style::default().green().bold());
            label = label.set_style(Style::default().bold().crossed_out());
        }
        let list_item = ListItem::new(Line::from(vec![completed, label]));
        list_items.push(list_item);
    }

    let scrollball = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some(""))
        .end_symbol(Some(""));

    let list = List::new(list_items)
        .block(header_block)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(" ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, chunk, &mut app.selected_todo);
    frame.render_stateful_widget(
        scrollball,
        chunk.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(app: &App, chunk: Rect, frame: &mut Frame) {
    let horizontal_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(chunk);

    let current_mode = vec![match &app.current_screen {
        CurrentScreen::Main => Span::styled("NORMAL", Style::default().white().bold()),
        CurrentScreen::AddTodo => Span::styled("ADD", Style::default().light_cyan().bold()),
        CurrentScreen::DeleteTodo => Span::styled("DELETE", Style::default().red().bold()),
        CurrentScreen::EditTodo => Span::styled("EDIT", Style::default().blue().bold()),
        CurrentScreen::Selection => Span::styled("SELECT", Style::default().green().bold()),
        CurrentScreen::Search => Span::styled("SEARCH", Style::default().yellow().bold()),
    }];
    let mode_footer = Paragraph::new(Line::from(current_mode))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        )
        .alignment(Alignment::Center);

    let current_key_hint = vec![match &app.current_screen {
        CurrentScreen::Main => {
            Span::raw(" <Space> - Toggle | a - Add | e - Edit | d - Delete | q - Quit")
        }
        CurrentScreen::AddTodo => Span::raw(" <Enter> - Add | <Esc> - Cancel"),
        CurrentScreen::DeleteTodo => Span::raw(" y - Yes | n - No | <Esc> - Cancel"),
        CurrentScreen::EditTodo => Span::raw(" <Enter> - Yes | <Esc> - Cancel"),
        CurrentScreen::Selection => Span::raw(" <Esc> | q - Normal "),
        CurrentScreen::Search => Span::raw(" <Esc> - Normal "),
    }];
    let key_hints = Paragraph::new(Line::from(current_key_hint)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double),
    );

    frame.render_widget(mode_footer, horizontal_chunk[0]);
    frame.render_widget(key_hints, horizontal_chunk[1]);
}

fn render_main(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.size());

    render_body(app, chunks[0], frame);
    render_footer(app, chunks[1], frame);
}

fn render_add_todo_popup(app: &App, frame: &mut Frame) {
    let popup_block = Block::default()
        .title("  Add new todo ")
        .style(Style::default().black().on_light_cyan())
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let todo_label = Paragraph::new(app.todo_input.clone()).block(popup_block);
    let area = centered_rect(60, TODO_INPUT_HEIGHT, frame.size());
    frame.render_widget(Clear, area);
    frame.render_widget(todo_label, area);
}

fn render_delete_todo_popup(app: &mut App, frame: &mut Frame) {
    if let Some(selected_todo) = app.get_selected_todo() {
        let popup_block = Block::default()
            .title(" Delete Todo - y/n ")
            .style(Style::default().black().on_light_red())
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        let todo_label = Paragraph::new(selected_todo.label.clone()).block(popup_block);
        let area = centered_rect(60, 20, frame.size());
        frame.render_widget(Clear, area);
        frame.render_widget(todo_label, area);
    }
}

fn render_edit_todo_popup(app: &mut App, frame: &mut Frame) {
    let popup_block = Block::default()
        .title(" Edit - <Enter> to Edit ")
        .style(Style::default().black().on_light_blue())
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let todo_label = Paragraph::new(app.todo_input.clone()).block(popup_block);
    let area = centered_rect(60, 20, frame.size());
    frame.render_widget(Clear, area);
    frame.render_widget(todo_label, area);
}

pub fn render(app: &mut App, frame: &mut Frame) {
    render_main(app, frame);

    match &app.current_screen {
        CurrentScreen::Main => {}
        CurrentScreen::AddTodo => render_add_todo_popup(app, frame),
        CurrentScreen::DeleteTodo => render_delete_todo_popup(app, frame),
        CurrentScreen::EditTodo => render_edit_todo_popup(app, frame),
        // TODO: Implement selection mode
        CurrentScreen::Selection => {}
        CurrentScreen::Search => {}
    }
}
