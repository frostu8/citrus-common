use num_enum::{IntoPrimitive, TryFromPrimitive};

use std::ops::{BitOr, BitOrAssign, BitAnd};

/// A single panel.
#[derive(Clone)]
pub struct Panel {
    /// The panel's kind.
    pub kind: PanelKind,
    /// The exits a panel has.
    pub exits: Exits,
    /// The exits a panel has during Backtrack.
    ///
    /// Commonly referred to as "entrances," which is misleading, considering
    /// that a panel can have an entrance and an exit on the same direction.
    pub exits_backtrack: Exits,
}

impl Panel {
    /// Creates a new panel from the panel's kind.
    pub const fn new(kind: PanelKind) -> Panel {
        Panel {
            kind,
            exits: Exits::none(),
            exits_backtrack: Exits::none(),
        }
    }

    pub(crate) const fn from_internal(kind: PanelKind, exits: u8) -> Panel {
        Panel {
            kind,
            exits: Exits(exits & 0xF),
            exits_backtrack: Exits((exits >> 4) & 0xF),
        }
    }

    pub(crate) const fn exits_internal(&self) -> u8 {
        (self.exits_backtrack.0 << 4) | self.exits.0
    }
}

/// A panel's type.
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PanelKind {
    Empty = 0x00,
    Neutral = 0x01,
    Home = 0x02,
    Encounter = 0x03,
    Draw = 0x04,
    Bonus = 0x05,
    Drop = 0x06,
    Warp = 0x07,
    Draw2x = 0x08,
    Bonus2x = 0x09,
    Drop2x = 0x0A,
    Deck = 0x12,
    Encounter2x = 0x14,
    Move = 0x15,
    Move2x = 0x16,
    WarpMove = 0x17,
    WarpMove2x = 0x18, // confirmation needed
    Ice = 0x19,
    Heal = 0x1B,
    Heal2x = 0x1C, // confirmation needed
    Damage = 0x20,
    Damage2x = 0x21,
}

/// A panel's exits.
///
/// To combine two directions together into one exit, e.g. make an `Exits` that
/// is both `SOUTH` and `NORTH`, use the `|` operator. To check if an exit has
/// a direction, use the `&` operator.
///
/// # Examples
/// ```
/// use citrus::panel::Exits;
///
/// // check if our exits has a direction set.
/// let exits = Exits::SOUTH;
/// assert!(exits & Exits::SOUTH);
/// assert!(!(exits & Exits::NORTH));
///
/// // make exits that point to north and south
/// let exits = Exits::SOUTH | Exits::NORTH;
/// assert!(exits & Exits::SOUTH);
/// assert!(exits & Exits::NORTH);
/// // we can also mix these together, AOK!
/// assert!(exits & (Exits::SOUTH | Exits::NORTH));
/// ```
#[derive(Clone, Copy)]
pub struct Exits(u8);

impl Exits {
    pub const WEST: Exits = Exits(0b0001);
    pub const NORTH: Exits = Exits(0b0010);
    pub const EAST: Exits = Exits(0b0100);
    pub const SOUTH: Exits = Exits(0b1000);

    /// An `Exits` with no exits.
    pub const fn none() -> Exits {
        Exits(0)
    }

    /// Checks if an `Exits` has a direction, or multiple directions.
    pub const fn has(&self, rhs: Exits) -> bool {
        self.0 & rhs.0 > 0
    }
}

impl PartialEq for Exits {
    fn eq(&self, rhs: &Exits) -> bool {
        self.0 == rhs.0
    }
}

impl BitOr for Exits {
    type Output = Exits;

    fn bitor(self, rhs: Exits) -> Exits {
        Exits(self.0 | rhs.0)
    }
}

impl BitOrAssign for Exits {
    fn bitor_assign(&mut self, rhs: Exits) {
        self.0 |= rhs.0
    }
}

impl BitAnd for Exits {
    type Output = bool;

    fn bitand(self, rhs: Exits) -> bool {
        self.0 & rhs.0 > 0
    }
}

