use crate::panel::*;

use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// A field, stored on the heap as a row-major flattened array.
///
/// Most of a field's power comes from [`PanelRef::offset`] and
/// [`PanelMut::offset`]. These functions allow relative indexing into the
/// field:
///
/// ```
/// use citrus_common::{Field, Panel, PanelKind::*};
///
/// let mut field = Field::new_slice(&[
///     &[Panel::new(Draw), Panel::new(Encounter)],
///     &[Panel::new(Bonus), Panel::new(Drop)],
/// ]);
///
/// // we can grab that Drop panel from an index to the Draw panel!
/// let mut panel = field.get_mut(0, 0)
///     .offset(1, 1).unwrap();
///
/// assert_eq!(panel.kind, Drop);
///
/// // we can also modify the panel's kind...
/// panel.kind = Drop2x;
/// // ...and watch it reflect on the field!
/// assert_eq!(field.get(1, 1).kind, Drop2x);
/// ```
pub struct Field {
    data: Vec<Panel>,
    width: usize,
    height: usize,
}

impl Field {
    /// Creates a new, empty field.
    pub const fn new() -> Field {
        Field {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    /// Creates a new field from a row-major vector.
    pub fn new_vec(data: Vec<Panel>, width: usize, height: usize) -> Field {
        assert!(data.len() == width * height, 
            "data does not match size requirements");

        Field { data, width, height }
    }

    /// Creates a new field from row-major nested iterators.
    ///
    /// The iterators must be [`ExactSizeIterator`]; this will automatically
    /// resolve the height and width from the iterators.
    ///
    /// # Panics
    /// Will panic if the inner iterators do not all have the same length.
    pub fn new_iter<I, J>(mut iter: I) -> Field where 
        I: Iterator<Item = J> + ExactSizeIterator,
        J: Iterator<Item = Panel> + ExactSizeIterator {
        // check height and width
        let height = iter.len();

        if let Some(subiter) = iter.next() {
            let width = subiter.len();

            Field {
                data: subiter.chain(
                    iter.inspect(|s| {
                        assert!(s.len() == width, 
                            "all sub-iterators must have the same length");
                    })
                    .flatten()
                ).collect(),
                width, height,
            }
        } else {
            // return null field
            Field::new()
        }
    }

    /// Creates a new field from row-major slice-of-slices.
    ///
    /// # Panics
    /// Will panic if the inner iterators do not all have the same length.
    pub fn new_slice(slice: &[&[Panel]]) -> Field {
        Field::new_iter(
            slice.into_iter().map(|subslice| {
                // take ownership of panels
                subslice.into_iter().map(|p| p.clone())
            })
        )
    }

    /// Gets the width of a field.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of a field.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Indexes the field immutably.
    pub fn get(&self, x: usize, y: usize) -> PanelRef {
        PanelRef::new(self, x, y)
    }

    /// Indexes the field mutably.
    pub fn get_mut(&mut self, x: usize, y: usize) -> PanelMut {
        PanelMut::new(self, x, y)
    }

    /// Gets an iterator over all of the positions on the field, row-major.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + DoubleEndedIterator {
        let Field { width, height, .. } = *self;

        (0..height)
            .map(move |y| (0..width).map(move |x| {
                (x, y)
            }))
            .flatten()
    }

    /// Gets an iterator over all of the panels in a row.
    pub fn row_iter(&self, y: usize) -> impl Iterator<Item = PanelRef> + DoubleEndedIterator + ExactSizeIterator {
        (0..self.width)
            .map(move |x| self.get(x, y))
    }

    /// Gets an iterator over all of the rows in a field.
    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = PanelRef>> + DoubleEndedIterator + ExactSizeIterator {
        (0..self.height)
            .map(move |y| self.row_iter(y))
    }

    /// Gets an iterator over all of the panels in a column.
    pub fn column_iter(&self, x: usize) -> impl Iterator<Item = PanelRef> + DoubleEndedIterator + ExactSizeIterator {
        (0..self.height)
            .map(move |y| self.get(x, y))
    }

    /// Gets an iterator over all of the columns in a field.
    pub fn columns_iter(&self) -> impl Iterator<Item = impl Iterator<Item = PanelRef>> + DoubleEndedIterator + ExactSizeIterator {
        (0..self.width)
            .map(move |x| self.column_iter(x))
    }

    /// Rebuilds backtrack exits, using the normal exits as a reference.
    pub fn build_backtrack(&mut self) {
        // reset all backtrack exits
        for (x, y) in self.iter() {
            self.get_mut(x, y).exits_backtrack = Exits::none();
        }

        for (x, y) in self.iter() {
            // get mut ref
            let panel = self.get_mut(x, y);

            // alter adjacent panels
            // south
            let panel = if panel.exits & Exits::SOUTH {
                match panel.offset(0, -1) {
                    Ok(mut adjacent) => {
                        adjacent.exits_backtrack |= Exits::NORTH;
                        adjacent.offset(0, 1).unwrap()
                    },
                    Err(panel) => panel,
                }
            } else {
                panel
            };

            // north
            let panel = if panel.exits & Exits::NORTH {
                match panel.offset(0, 1) {
                    Ok(mut adjacent) => {
                        adjacent.exits_backtrack |= Exits::SOUTH;
                        adjacent.offset(0, -1).unwrap()
                    },
                    Err(panel) => panel,
                }
            } else {
                panel
            };

            // west
            let panel = if panel.exits & Exits::WEST {
                match panel.offset(-1, 0) {
                    Ok(mut adjacent) => {
                        adjacent.exits_backtrack |= Exits::EAST;
                        adjacent.offset(1, 0).unwrap()
                    },
                    Err(panel) => panel,
                }
            } else {
                panel
            };

            // east
            if panel.exits & Exits::EAST {
                match panel.offset(1, 0) {
                    Ok(mut adjacent) => {
                        adjacent.exits_backtrack |= Exits::WEST;
                        adjacent.offset(-1, 0).unwrap()
                    },
                    Err(panel) => panel,
                }
            } else {
                panel
            };
        }
    }

    fn flatten_index(&self, x: usize, y: usize) -> usize {
        // flatten
        y * self.width + x
    }
}

/// Used to refer to a panel on a field.
pub struct PanelRef<'a> {
    field: &'a Field,
    x: usize,
    y: usize,
}

/// Used to refer to a panel on a field mutably.
pub struct PanelMut<'a> {
    field: &'a mut Field,
    x: usize,
    y: usize,
}

impl<'a> PanelRef<'a> {
    /// Creates a new `PanelRef`.
    ///
    /// You shouldn't call this directly; use [`Field::get`] instead.
    pub fn new(field: &'a Field, x: usize, y: usize) -> PanelRef<'a> {
        // do bounds checks
        assert!(x < field.width(), "x ({}) is out of bounds ", x);
        assert!(y < field.height(), "y ({}) is out of bounds ", x);

        PanelRef { field, x, y }
    }
    
    /// Offsets a `PanelRef` by a certain vector, returning `Err(self)` if it 
    /// would index out of bounds.
    pub fn offset(self, x_offset: i64, y_offset: i64) -> Result<PanelRef<'a>, PanelRef<'a>> {
        // deconstruct
        let PanelRef { field, x, y } = self;

        match offset_common(field, x, y, x_offset, y_offset) {
            Some((x, y)) => Ok(PanelRef { field, x, y }),
            None => Err(PanelRef { field, x, y }),
        }
    }
}

impl<'a> Deref for PanelRef<'a> {
    type Target = Panel;

    fn deref(&self) -> &Panel {
        let idx = self.field.flatten_index(self.x, self.y);
        &self.field.data[idx]
    }
}

impl<'a> PanelMut<'a> {
    /// Creates a new `PanelMut`.
    ///
    /// You shouldn't call this directly; use [`Field::get_mut`] instead.
    pub fn new(field: &'a mut Field, x: usize, y: usize) -> PanelMut<'a> {
        // do bounds checks
        assert!(x < field.width(), "x ({}) is out of bounds ", x);
        assert!(y < field.height(), "y ({}) is out of bounds ", x);

        PanelMut { field, x, y }
    }
    
    /// Offsets a `PanelMut` by a certain vector, returning `Err(self)` if it 
    /// would index out of bounds.
    pub fn offset(self, x_offset: i64, y_offset: i64) -> Result<PanelMut<'a>, PanelMut<'a>> {
        // deconstruct
        let PanelMut { field, x, y } = self;

        match offset_common(field, x, y, x_offset, y_offset) {
            Some((x, y)) => Ok(PanelMut { field, x, y }),
            None => Err(PanelMut { field, x, y }),
        }
    }
}

impl<'a> Deref for PanelMut<'a> {
    type Target = Panel;

    fn deref(&self) -> &Panel {
        let idx = self.field.flatten_index(self.x, self.y);
        &self.field.data[idx]
    }
}

impl<'a> DerefMut for PanelMut<'a> {
    fn deref_mut(&mut self) -> &mut Panel {
        let idx = self.field.flatten_index(self.x, self.y);
        &mut self.field.data[idx]
    }
}

impl<'a> Debug for PanelRef<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { f.write_str("PanelRef") }
}

impl<'a> Debug for PanelMut<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { f.write_str("PanelMut") }
}

#[inline]
fn offset_common(
    field: &Field, 
    x: usize, y: usize, 
    xo: i64, yo: i64,
) -> Option<(usize, usize)> {
    // offset
    let x = (x as i64) + xo;
    let y = (y as i64) + yo;

    if x >= 0 && y >= 0 {
        let x = x as usize;
        let y = y as usize;

        if x < field.width() && y < field.height() {
            Some((x, y))
        } else {
            None
        }
    } else {
        None
    }
}
