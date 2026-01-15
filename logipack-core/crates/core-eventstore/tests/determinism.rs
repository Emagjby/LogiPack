use core_eventstore::hashing::hash_strata_value;
use std::collections::BTreeMap;
use strata::value::Value;
use strata::{int, map, string};

#[test]
fn strata_hash_value_is_deterministic_with_macros() {
    let v1 = map! {
        "event_type" => string!("StatusChanged"),
        "shipment_id" => string!("shipment-1"),
        "from" => string!("ACCEPTED"),
        "to" => string!("PROCESSED"),
        "occurred_at" => int!(1_700_000_000_000),
    };

    let v2 = map! {
        "event_type" => string!("StatusChanged"),
        "shipment_id" => string!("shipment-1"),
        "from" => string!("ACCEPTED"),
        "to" => string!("PROCESSED"),
        "occurred_at" => int!(1_700_000_000_000),
    };

    let a = hash_strata_value(&v1).unwrap();
    let b = hash_strata_value(&v2).unwrap();

    assert_eq!(
        a.scb, b.scb,
        "canonical bytes must match for identical values"
    );
    assert_eq!(a.hash, b.hash, "hash must match for identical values");
}

#[test]
fn strata_hash_value_is_deterministic_for_same_payload() {
    let v1 = Value::Map(BTreeMap::from_iter([
        (
            "event_type".to_owned(),
            Value::String("StatusChanged".into()),
        ),
        ("shipment_id".to_owned(), Value::String("shipment-1".into())),
        ("from".to_owned(), Value::String("ACCEPTED".into())),
        ("to".to_owned(), Value::String("PROCESSED".into())),
        ("occurred_at".to_owned(), Value::Int(1_700_000_000_000)),
    ]));

    let v2 = Value::Map(BTreeMap::from_iter([
        (
            "event_type".to_owned(),
            Value::String("StatusChanged".into()),
        ),
        ("shipment_id".to_owned(), Value::String("shipment-1".into())),
        ("from".to_owned(), Value::String("ACCEPTED".into())),
        ("to".to_owned(), Value::String("PROCESSED".into())),
        ("occurred_at".to_owned(), Value::Int(1_700_000_000_000)),
    ]));

    let a = hash_strata_value(&v1).unwrap();
    let b = hash_strata_value(&v2).unwrap();

    assert_eq!(
        a.scb, b.scb,
        "canonical bytes must match for identical values"
    );
    assert_eq!(a.hash, b.hash, "hash must match for identical values");
}
