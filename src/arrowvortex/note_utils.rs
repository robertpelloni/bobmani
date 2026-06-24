use crate::arrowvortex::notes::{ExpandedNote, NoteType};

pub fn less_than_row_col(a: &ExpandedNote, b: &ExpandedNote) -> bool {
    if a.row != b.row {
        a.row < b.row
    } else {
        a.col < b.col
    }
}

pub fn compare_row_col(a: &ExpandedNote, b: &ExpandedNote) -> i32 {
    if a.row != b.row {
        a.row - b.row
    } else {
        a.col - b.col
    }
}

pub fn less_than_row(a: &ExpandedNote, b: &ExpandedNote) -> bool {
    a.row < b.row
}

pub fn compare_row(a: &ExpandedNote, b: &ExpandedNote) -> i32 {
    a.row - b.row
}

pub fn note_pos(col: i32, n: &ExpandedNote) -> i64 {
    ((n.row as i64) << 32) | (col as i64)
}

pub fn is_note_identical(first: &ExpandedNote, second: &ExpandedNote) -> bool {
    first.row == second.row &&
    first.col == second.col &&
    first.endrow == second.endrow &&
    first.note_type == second.note_type &&
    first.player == second.player
}

pub fn get_note_name(note_type: NoteType, is_plural: bool) -> &'static str {
    match note_type {
        NoteType::StepOrHold => if is_plural { "steps/holds" } else { "step/hold" },
        NoteType::Mine => if is_plural { "mines" } else { "mine" },
        NoteType::Roll => if is_plural { "rolls" } else { "roll" },
        NoteType::Lift => if is_plural { "lifts" } else { "lift" },
        NoteType::Fake => if is_plural { "fakes" } else { "fake" },
    }
}
