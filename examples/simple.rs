use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::{process, sync::Arc};
use termui::{
    context::TerminalContext,
    element::{Attribute, DivBuilder, PBuilder},
    handler::Handler,
    rx::State,
    style::{Color, Property, Selector, Style},
};

//-------------------------------------------------------------------------------
// Functions
//-------------------------------------------------------------------------------

fn main() -> Result<()> {
    let style = Style::from_iter([Selector::Class(
        "root".to_string(),
        vec![
            Property::BackgroundColor(Color::Rgb(0, 0, 0)),
            Property::Color(Color::Rgb(255, 255, 255)),
        ],
    )]);

    let message = Arc::new(State::new("Hello, World!".to_string()));
    let message_clone = Arc::clone(&message);
    let on_key_handler = Handler::new(move |event: &KeyEvent| {
        match event.code {
            KeyCode::Char('q') => {
                process::exit(0);
            }
            KeyCode::Char('h') => {
                message_clone.set("Hello, World!".to_string())?;
            }
            KeyCode::Char('w') => {
                message_clone.set("World, Hello!".to_string())?;
            }
            _ => {}
        }

        Ok(())
    });

    let root = DivBuilder::new()
        .attr(Attribute::Class("root".to_string()))
        .attr(Attribute::OnKeyEvent(on_key_handler))
        .child(PBuilder::new().content_rx(message.as_ref()).build())
        .build();

    TerminalContext::new(root, style)?.run()
}
