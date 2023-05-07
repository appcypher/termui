//! Context is a container for all the information to manage the terminal UI.

use crate::{
    element::Element,
    geom::{Point, Size},
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
    cusor_pos: Point,
    /// The root element.
    root_element: Arc<dyn Element>,
    /// The style sheet to apply to the elements.
    style_sheet: StyleSheet,
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
    pub fn new(root_element: Arc<dyn Element>, style_sheet: StyleSheet) -> Result<Self> {
        Ok(Self {
            size: terminal::size()?.into(),
            cusor_pos: cursor::position()?.into(),
            root_element,
            style_sheet,
        })
    }

    /// Runs the event loop.
    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        // self.clear()?;
        // self.render(self.root_element, parent_xxx)?;

        loop {
            // self.update_term_info()?;
            if event::poll(Duration::from_millis(100))? {
                // Check for new events.
                if let Event::Key(event) = event::read()? {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        break;
                    }

                    if event.code == KeyCode::Tab {
                        // self.set_next_focus(&root_widget)?;
                    }

                    // TODO(appcypher): Handle events.
                }
            }

            // self.clear()?;
            // self.render(self.root_element, parent_bounds)?;
        }

        terminal::disable_raw_mode()?;
        println!();

        Ok(())
    }

    pub fn clear() -> Result<()> {
        eprintln!("terminal size: {:?}", terminal::size()?);
        eprintln!("remainder terminal draw area: {:?}", terminal::size()?);
        // let out = &mut stdout();

        // for y in self.cursor_position.y..self.size.height {
        //     out.queue(MoveTo(0, y))?;
        //     out.queue(Clear(ClearType::CurrentLine))?;
        // }

        // out.queue(MoveTo(0, self.cursor_position.y))?;
        // out.flush()?;

        // Ok(())
        todo!()
    }
}
