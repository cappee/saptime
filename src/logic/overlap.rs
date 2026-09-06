use crate::logic::dedup::DedupedSlot;

/// Ritorna gli slot già selezionati che vanno in conflitto con `candidate`.
/// Vuoto = nessun conflitto.
pub fn conflicts_with<'a>(candidate: &DedupedSlot, selected: &'a [DedupedSlot]) -> Vec<&'a DedupedSlot> {
    selected
        .iter()
        .filter(|slot| {
            slot.weekday == candidate.weekday
                && slot.start_time < candidate.end_time
                && candidate.start_time < slot.end_time
        })
        .collect()
}