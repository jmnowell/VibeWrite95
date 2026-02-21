use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn render(f: &mut Frame, area: Rect) {
    if area.height == 0 {
        return;
    }

    let width = area.width as usize;
    let mut ruler = String::with_capacity(width);

    for i in 1..=width {
        let c = match i % 10 {
            0 => '|',
            5 => '+',
            _ => '-',
        };
        ruler.push(c);
    }

    let paragraph = Paragraph::new(ruler)
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black));
    f.render_widget(paragraph, area);
}
