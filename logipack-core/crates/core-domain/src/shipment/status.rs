use std::fmt;

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

impl std::str::FromStr for ShipmentStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "NEW" => Ok(ShipmentStatus::New),
            "ACCEPTED" => Ok(ShipmentStatus::Accepted),
            "PROCESSED" => Ok(ShipmentStatus::Processed),
            "IN_TRANSIT" => Ok(ShipmentStatus::InTransit),
            "DELIVERED" => Ok(ShipmentStatus::Delivered),
            "CANCELLED" => Ok(ShipmentStatus::Cancelled),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ShipmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            ShipmentStatus::New => "NEW",
            ShipmentStatus::Accepted => "ACCEPTED",
            ShipmentStatus::Processed => "PROCESSED",
            ShipmentStatus::InTransit => "IN_TRANSIT",
            ShipmentStatus::Delivered => "DELIVERED",
            ShipmentStatus::Cancelled => "CANCELLED",
        };
        write!(f, "{}", status_str)
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
