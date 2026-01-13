#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipmentStatus {
    New,
    Accepted,
    Processed,
    InTransit,
    Delivered,
    Cancelled,
}

impl ShipmentStatus {
    pub fn is_terminal(self) -> bool {
        matches!(self, ShipmentStatus::Delivered | ShipmentStatus::Cancelled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_statuses_are_correct() {
        // Terminal statuses
        assert!(ShipmentStatus::Delivered.is_terminal());
        assert!(ShipmentStatus::Cancelled.is_terminal());

        // Non-terminal statuses
        assert!(!ShipmentStatus::New.is_terminal());
        assert!(!ShipmentStatus::Accepted.is_terminal());
        assert!(!ShipmentStatus::Processed.is_terminal());
        assert!(!ShipmentStatus::InTransit.is_terminal());
    }
}
