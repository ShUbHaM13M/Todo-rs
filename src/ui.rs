use crate::app::{App, CurrentScreen};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, Paragraph},
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
        .border_type(BorderType::Rounded);

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
            completed = completed.content("[x] ").style(Style::default().green());
            label = label.set_style(Style::default().bold().crossed_out());
        }
        let list_item = ListItem::new(Line::from(vec![completed, label]));
        list_items.push(list_item);
    }

    let list = List::new(list_items)
        .block(header_block)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, chunk, &mut app.selected_todo);
}

fn render_footer(chunk: Rect, frame: &mut Frame) {
    let current_navigation_text = vec![];
    let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(mode_footer, chunk);
}

fn render_main(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.size());

    render_body(app, chunks[0], frame);
    render_footer(chunks[1], frame);
}

fn render_add_todo_popup(app: &App, frame: &mut Frame) {
    let popup_block = Block::default()
        .title(" Add new todo ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let todo_label = Paragraph::new(app.todo_input.clone()).block(popup_block);
    let area = centered_rect(60, TODO_INPUT_HEIGHT, frame.size());

    frame.render_widget(todo_label, area);
}

pub fn render(app: &mut App, frame: &mut Frame) {
    render_main(app, frame);

    if let CurrentScreen::AddTodo = &app.current_screen {
        render_add_todo_popup(app, frame);
    }
}
