use crate::shipment::ShipmentStatus;

/// Reference to the actor that caused the event.
/// Kept intentionally generic for future expansion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorRef {
    pub id: String,
}

/// Context about the office involved in the event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfficeContext {
    pub office_id: String,
}

/// Domain event emmited when a shipment is created
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShipmentCreated {
    pub shipment_id: String,
    pub actor: ActorRef,
    pub office: Option<OfficeContext>,
    /// Unix timestamp in mins
    pub occured_at: i64,
    pub notes: Option<String>,
}

/// Domain event emmited when a shipment status changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusChanged {
    pub shipment_id: String,
    pub from_status: ShipmentStatus,
    pub to_status: ShipmentStatus,
    pub actor: ActorRef,
    pub office: Option<OfficeContext>,
    /// Unix timestamp in mins
    pub occured_at: i64,
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shipment::ShipmentStatus;

    #[test]
    fn status_changed_event_can_be_constructed() {
        let event = StatusChanged {
            shipment_id: "shipment-1".to_string(),
            from_status: ShipmentStatus::Accepted,
            to_status: ShipmentStatus::Processed,
            actor: ActorRef {
                id: "user-123".to_string(),
            },
            office: Some(OfficeContext {
                office_id: "office-1".to_string(),
            }),
            occured_at: 1_700_000_000_000,
            notes: Some("processed at warehouse".to_string()),
        };

        assert_eq!(event.from_status, ShipmentStatus::Accepted);
        assert_eq!(event.to_status, ShipmentStatus::Processed);
        assert!(event.office.is_some());
    }
}
