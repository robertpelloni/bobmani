use crate::arrowvortex::notes::ExpandedNote;

/// Contains notes for a single column, sorted by row.
#[derive(Debug, Clone, Default)]
pub struct NoteCol {
    notes: Vec<ExpandedNote>,
}

impl NoteCol {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { notes: Vec::with_capacity(capacity) }
    }

    pub fn clear(&mut self) {
        self.notes.clear();
    }

    pub fn append(&mut self, note: ExpandedNote) {
        self.notes.push(note);
    }

    pub fn remove_marked_notes(&mut self) {
        self.notes.retain(|n| n.row >= 0);
    }

    pub fn end_row(&self) -> i32 {
        self.notes.last().map(|n| n.endrow).unwrap_or(0)
    }

    pub fn size(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }
}
