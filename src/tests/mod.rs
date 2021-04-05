use crate::{Field, Panel, PanelKind};

#[test]
fn test_field() {
    use PanelKind::*;

    Field::new_slice(&[
        &[Panel::new(Home), Panel::new(Draw), Panel::new(Home)],
        &[Panel::new(Bonus), Panel::new(Empty), Panel::new(Drop)],
        &[Panel::new(Home), Panel::new(Encounter), Panel::new(Home)],
    ]);
}

#[test]
fn test_fld_read() {
    use crate::format::fld;
    use std::io::Cursor;

    // use Training Program as our test field
    const TRAINING_PROGRAM: &'static [u8] = include_bytes!("field_training.fld");

    fld::decode(fld::S15, Cursor::new(TRAINING_PROGRAM))
        .unwrap();
}

#[test]
fn test_fldx_read() {
    use crate::format::fldx;
    use std::io::Cursor;

    // use Training Program as our test field
    const TRAINING_PROGRAM: &'static [u8] = include_bytes!("field_training.fldx");

    fldx::decode(Cursor::new(TRAINING_PROGRAM))
        .unwrap();
}
