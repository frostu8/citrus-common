use crate::{Field, PanelKind::*, Exits};

use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for y in 0..self.height() {
            // print newline
            f.write_str("\n")?;

            for x in 0..self.width() {
                let panel = self.get(x, y);

                // print panel kind
                match panel.kind {
                    Empty => f.write_str("  ")?,
                    Neutral => f.write_str("[]")?,
                    Home => f.write_str("@@")?,
                    Encounter => f.write_str("en")?,
                    Bonus => f.write_str("bs")?,
                    Draw => f.write_str("da")?,
                    Drop => f.write_str("dr")?,
                    Warp => f.write_str("wa")?,
                    WarpMove => f.write_str("wm")?,
                    Move => f.write_str("mo")?,
                    Bonus2x => f.write_str("BS")?,
                    Deck => f.write_str("__")?,
                    _ => f.write_str("??")?,
                }

                let panel_last_exits = panel.exits;

                // get panel next type
                match panel.offset(1, 0) {
                    Ok(panel) => {
                        // check bitflags
                        if panel.exits.has(Exits::WEST) {
                            f.write_str("<")?;
                        } else if panel_last_exits.has(Exits::EAST) {
                            f.write_str(">")?;
                        } else {
                            f.write_str(" ")?;
                        }
                    },
                    // do not print anything otherwise
                    Err(_) => (),
                }
            }

            // print newline
            // to start our connections
            f.write_str("\n")?;

            for x in 0..self.width() {
                let panel = self.get(x, y);
                let panel_last_exits = panel.exits;
                
                match panel.offset(0, 1) {
                    Ok(panel) => {
                        // check bitflags
                        if panel.exits.has(Exits::NORTH) {
                            f.write_str("/\\")?;
                        } else if panel_last_exits.has(Exits::SOUTH) {
                            f.write_str("\\/")?;
                        } else {
                            f.write_str("  ")?;
                        }
                    },
                    Err(_) => (),
                }

                // add whitespace to prepare for the next panel
                f.write_str(" ")?;
            }
        }

        Ok(())
    }
}
