use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait,
    QueryFilter, Statement,
};
use uuid::Uuid;

use core_eventstore::adapter::append::append_package;
use core_eventstore::schema::{packages, streams};

use core_eventstore_migration::Migrator;
use core_eventstore_migration::MigratorTrait;

#[tokio::test]
async fn append_rules() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/logipack_test".to_string());

    assert!(
        db_url.contains("test"),
        "Refusing to run integration tests against non-test database"
    );

    let db = Database::connect(&db_url).await.unwrap();

    //  Apply migrations (schema must exist)
    Migrator::up(&db, None).await.unwrap();

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
    let v1 = strata::value::Value::Map(std::collections::BTreeMap::from_iter([
        (
            "event".to_owned(),
            strata::value::Value::String("Created".into()),
        ),
        ("seq".to_owned(), strata::value::Value::Int(1)),
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
    let v2 = strata::value::Value::Map(std::collections::BTreeMap::from_iter([
        (
            "event".to_owned(),
            strata::value::Value::String("StatusChanged".into()),
        ),
        ("seq".to_owned(), strata::value::Value::Int(2)),
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

async fn reset_eventstore_db(db: &DatabaseConnection) {
    db.execute(Statement::from_string(
        DbBackend::Postgres,
        "TRUNCATE TABLE packages, streams RESTART IDENTITY CASCADE".to_owned(),
    ))
    .await
    .unwrap();
}
