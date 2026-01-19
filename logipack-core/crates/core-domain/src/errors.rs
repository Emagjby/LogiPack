use crate::shipment::ShipmentStatus;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TransitionError {
    /// Attempted to change status from a terminal state.
    #[error("terminal state transition from {from}")]
    TerminalState { from: ShipmentStatus },

    /// Transition is not allowed by the status machine.
    #[error("invalid transition from {from} to {to}")]
    InvalidTransition {
        from: ShipmentStatus,
        to: ShipmentStatus,
    },

    /// Office change is not allowed when transitioning to IN_TRANSIT.
    #[error("office hop not allowed from {from} to {to}")]
    OfficeHopNotAllowed {
        from: ShipmentStatus,
        to: ShipmentStatus,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shipment::ShipmentStatus;

    #[test]
    fn transition_error_is_constructible() {
        let err = TransitionError::InvalidTransition {
            from: ShipmentStatus::New,
            to: ShipmentStatus::Delivered,
        };

        match err {
            TransitionError::InvalidTransition { from, to } => {
                assert_eq!(from, ShipmentStatus::New);
                assert_eq!(to, ShipmentStatus::Delivered);
            }
            _ => panic!("unexpected error variant"),
        }
    }
}
