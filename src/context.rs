//! Context is a container for all the information to manage the terminal UI.

use crate::{
    element::Element,
    point::{Point, Size},
    stylesheet::StyleSheet,
};
use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};
use std::{sync::Arc, time::Duration};

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

/// The context for a terminal UI.
#[derive(Clone, Debug)]
pub struct TerminalContext {
    /// The size of the drawable terminal canvas.
    size: Size,
    /// The position of the cursor.
    cursor_position: Point,
    /// The root element.
    root_element: Arc<dyn Element>,
    /// The stylesheet to apply to the elements.
    stylesheet: StyleSheet,
    // TODO(appcypher): Maybe we should use a smarter search instead as focusable attribute on an element may change. Leading to an expensive write of the tree.
    // /// The focus tree.
    // focus_tree: Tree<Arc<dyn Element>>,
    // /// The currently focused element.
    // focus_element: Arc<dyn Element>,
}

//----------------------------------------------------------------
// Methods
//----------------------------------------------------------------

impl TerminalContext {
    /// Creates a new terminal context.
    pub fn new(root_element: Arc<dyn Element>, stylesheet: StyleSheet) -> Result<Self> {
        Ok(Self {
            size: terminal::size()?.into(),
            cursor_position: cursor::position()?.into(),
            root_element,
            stylesheet,
        })
    }

    /// Runs the event loop.
    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.clear()?;
        // self.render(self.root_element, parent_xxx)?;

        loop {
            // self.update_term_info()?;
            if event::poll(Duration::from_millis(100))? {
                // Check for new events.
                if let Event::Key(event) = event::read()? {
                    // TODO(appcypher): Remove this. Let user handle exits themselves.
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        break;
                    }

                    if event.code == KeyCode::Tab {
                        // self.set_next_focus(&root_widget)?;
                    }

                    // TODO(appcypher): Handle events.
                    // TODO(appcypher): Check for exit error from event handler.
                }
            }

            // self.clear()?;
            // self.render(self.root_element, parent_bounds)?;
        }

        terminal::disable_raw_mode()?;
        println!();

        // TODO(appcypher): Handle exit error if there is any.

        Ok(())
    }

    /// Clears the draw area and can make room for drawing content.
    pub fn clear(&self) -> Result<()> {
        eprintln!("terminal size: {:?}", terminal::size()?);
        eprintln!("remainder terminal draw area: {:?}", terminal::size()?);
        // let out = &mut stdout();

        // for y in self.cursor_position.y..self.size.height {
        //     out.queue(MoveTo(0, y))?;
        //     out.queue(Clear(ClearType::CurrentLine))?;
        // }

        // out.queue(MoveTo(0, self.cursor_position.y))?;
        // out.flush()?;

        Ok(())
    }

    /// Updates the terminal information.
    pub fn update_term_info(&mut self) -> Result<()> {
        self.size = terminal::size()?.into();
        self.cursor_position = cursor::position()?.into();
        Ok(())
    }
}
