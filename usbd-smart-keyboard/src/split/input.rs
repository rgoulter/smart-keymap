use keyberon::layout::Event;

/// The [Event] for the LHS split keyboard half.
pub fn event_transform_lhs(e: Event) -> Event {
    e
}

/// The [Event] for the RHS split keyboard half.
pub fn event_transform_rhs<const COLS: usize>(e: Event) -> Event {
    e.transform(|i, j| (i, j + COLS as u8))
}
