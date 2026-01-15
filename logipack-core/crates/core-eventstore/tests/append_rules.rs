use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, QueryFilter,
    Statement,
};
use uuid::Uuid;

use core_eventstore::adapter::append::append_package;
use core_eventstore::schema::{packages, streams};

use strata::value::Value;
use strata::{int, map, string};

use test_infra::test_db;

#[tokio::test(flavor = "current_thread")]
async fn append_rules_with_macros() {
    let db = test_db().await;
    reset_eventstore_db(&db).await;

    //  Create a stream

    let stream_id = Uuid::new_v4();

    streams::Entity::insert(streams::ActiveModel {
        id: sea_orm::ActiveValue::Set(stream_id),
        kind: sea_orm::ActiveValue::Set("shipment".to_owned()),
        head_hash: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::NotSet,
    })
    .exec(&db)
    .await
    .unwrap();

    //  Append first package
    let v1 = Value::Map(std::collections::BTreeMap::from_iter([
        ("event".to_owned(), Value::String("Created".into())),
        ("seq".to_owned(), Value::Int(1)),
    ]));

    let p1 = append_package(&db, stream_id, "ShipmentCreated", &v1)
        .await
        .unwrap();

    let pkg1 = packages::Entity::find()
        .filter(packages::Column::StreamId.eq(stream_id))
        .filter(packages::Column::Seq.eq(1))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(pkg1.seq, 1);
    assert!(pkg1.prev_hash.is_none());
    assert_eq!(pkg1.hash, p1.hash);

    //  Append second package
    let v2 = Value::Map(std::collections::BTreeMap::from_iter([
        ("event".to_owned(), Value::String("StatusChanged".into())),
        ("seq".to_owned(), Value::Int(2)),
    ]));

    let p2 = append_package(&db, stream_id, "StatusChanged", &v2)
        .await
        .unwrap();

    let pkg2 = packages::Entity::find()
        .filter(packages::Column::StreamId.eq(stream_id))
        .filter(packages::Column::Seq.eq(2))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(pkg2.seq, 2);
    assert_eq!(pkg2.prev_hash.unwrap(), p1.hash);
    assert_eq!(pkg2.hash, p2.hash);

    //  Verify stream head
    let stream = streams::Entity::find_by_id(stream_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(stream.head_hash.unwrap(), p2.hash);
}

#[tokio::test(flavor = "current_thread")]
async fn append_rules() {
    let db = test_db().await;
    reset_eventstore_db(&db).await;

    //  Create a stream
    let stream_id = Uuid::new_v4();

    streams::Entity::insert(streams::ActiveModel {
        id: sea_orm::ActiveValue::Set(stream_id),
        kind: sea_orm::ActiveValue::Set("shipment".to_owned()),
        head_hash: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::NotSet,
    })
    .exec(&db)
    .await
    .unwrap();

    //  Append first package
    let v1 = map! {
        "event" => string!("Created"),
        "seq" => int!(1),
        "occurred_at" => int!(1_700_000_000_000),
    };

    let p1 = append_package(&db, stream_id, "ShipmentCreated", &v1)
        .await
        .unwrap();

    let pkg1 = packages::Entity::find()
        .filter(packages::Column::StreamId.eq(stream_id))
        .filter(packages::Column::Seq.eq(1))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(pkg1.seq, 1);
    assert!(pkg1.prev_hash.is_none());
    assert_eq!(pkg1.hash, p1.hash);

    //  Append second package
    let v2 = map! {
        "event" => string!("StatusChanged"),
        "seq" => int!(2),
        "occurred_at" => int!(1_700_000_000_001),
    };

    let p2 = append_package(&db, stream_id, "StatusChanged", &v2)
        .await
        .unwrap();

    let pkg2 = packages::Entity::find()
        .filter(packages::Column::StreamId.eq(stream_id))
        .filter(packages::Column::Seq.eq(2))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(pkg2.seq, 2);
    assert_eq!(pkg2.prev_hash.unwrap(), p1.hash);
    assert_eq!(pkg2.hash, p2.hash);

    //  Verify stream head
    let stream = streams::Entity::find_by_id(stream_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(stream.head_hash.unwrap(), p2.hash);
}

async fn reset_eventstore_db(db: &DatabaseConnection) {
    // Prefer deterministic cleanup without table-level locks.
    db.execute(Statement::from_string(
        DbBackend::Postgres,
        "DELETE FROM packages".to_owned(),
    ))
    .await
    .unwrap();

    db.execute(Statement::from_string(
        DbBackend::Postgres,
        "DELETE FROM streams".to_owned(),
    ))
    .await
    .unwrap();
}
