use crate::errors::TransitionError;
use crate::shipment::ShipmentStatus;

pub fn validate_transition(
    from: ShipmentStatus,
    to: ShipmentStatus,
    office_changed: bool,
) -> Result<(), TransitionError> {
    // Terminal states reject all
    if from.is_terminal() {
        return Err(TransitionError::TerminalState { from });
    }

    // Office hop policy
    if office_changed && to != ShipmentStatus::InTransit {
        return Err(TransitionError::OfficeHopNotAllowed { from, to });
    }

    use ShipmentStatus::*;

    let allowed = matches!(
        (from, to),
        // forward progression
        (New, Accepted)
            | (Accepted, Processed)
            | (Processed, InTransit)
            | (InTransit, Delivered)
            // cancellation
            | (New, Cancelled)
            | (Accepted, Cancelled)
            | (Processed, Cancelled)
            | (InTransit, Cancelled)
    );

    if allowed {
        Ok(())
    } else {
        Err(TransitionError::InvalidTransition { from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ShipmentStatus::*;

    #[test]
    fn allowed_forward_transitions_pass() {
        let cases = [
            (New, Accepted),
            (Accepted, Processed),
            (Processed, InTransit),
            (InTransit, Delivered),
        ];

        for (from, to) in cases {
            assert!(
                validate_transition(from, to, false).is_ok(),
                "expected {:?} -> {:?} to be allowed",
                from,
                to
            )
        }
    }

    #[test]
    fn cancellation_is_allowed_from_non_terminal_states() {
        let cases = [New, Accepted, Processed, InTransit];

        for from in cases {
            assert!(
                validate_transition(from, Cancelled, false).is_ok(),
                "expected {:?} -> Cancelled to be allowed",
                from
            )
        }
    }

    #[test]
    fn terminal_states_reject_all_transitions() {
        let cases = [Delivered, Cancelled];

        for from in cases {
            let err = validate_transition(from, New, false).unwrap_err();
            assert!(
                matches!(err, TransitionError::TerminalState { .. }),
                "expected terminal state error for {:?}",
                from
            );
        }
    }

    #[test]
    fn invalid_transitions_are_rejected() {
        let cases = [
            (New, Processed),
            (Accepted, InTransit),
            (Processed, Delivered),
            (New, Delivered),
        ];

        for (from, to) in cases {
            let err = validate_transition(from, to, false).unwrap_err();
            assert!(
                matches!(err, TransitionError::InvalidTransition { .. }),
                "expected invalid transition {:?} -> {:?}",
                from,
                to
            );
        }
    }

    #[test]
    fn office_hop_is_only_allowed_when_transitioning_to_in_transit() {
        // allowed:
        assert!(
            validate_transition(Processed, InTransit, true).is_ok(),
            "office hop should be allowed when going to IN_TRANSIT"
        );

        // disallowed:
        let err = validate_transition(New, Accepted, true).unwrap_err();
        assert!(
            matches!(err, TransitionError::OfficeHopNotAllowed { .. }),
            "office hop should be rejected outside IN_TRANSIT"
        )
    }
}
