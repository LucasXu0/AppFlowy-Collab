use crate::disk::script::Script::*;
use crate::disk::script::{disk_plugin_with_db, CollabPersistenceTest};
use assert_json_diff::assert_json_eq;
use collab::preclude::CollabBuilder;
use collab_entity::CollabType;
use collab_plugins::local_storage::kv::doc::CollabKVAction;
use collab_plugins::local_storage::kv::KVTransactionDB;
use collab_plugins::local_storage::CollabPersistenceConfig;
use std::sync::Arc;

#[tokio::test]
async fn insert_single_change_and_restore_from_disk() {
  let doc_id = "1".to_string();
  let mut test = CollabPersistenceTest::new(CollabPersistenceConfig::new());
  let db = test.db.clone();
  test
    .run_scripts(vec![
      CreateDocumentWithCollabDB {
        id: doc_id.clone(),
        db: db.clone(),
      },
      InsertKeyValue {
        id: doc_id.clone(),
        key: "1".to_string(),
        value: "a".into(),
      },
      CloseDocument {
        id: doc_id.to_string(),
      },
      OpenDocumentWithDiskPlugin {
        id: doc_id.to_string(),
      },
      GetValue {
        id: doc_id,
        key: "1".to_string(),
        expected: Some("a".into()),
      },
    ])
    .await;
}

#[tokio::test]
async fn flush_test() {
  let doc_id = "1".to_string();
  let test = CollabPersistenceTest::new(CollabPersistenceConfig::new());
  let disk_plugin = disk_plugin_with_db(test.uid, test.db.clone(), &doc_id, CollabType::Document);
  let collab = Arc::new(
    CollabBuilder::new(1, &doc_id)
      .with_device_id("1")
      .with_plugin(disk_plugin)
      .build()
      .unwrap(),
  );
  collab.lock().initialize();
  for i in 0..100 {
    collab.lock().insert(&i.to_string(), i.to_string());
  }
  let lock_guard = collab.lock();
  let before_flush_value = lock_guard.to_json_value();
  drop(lock_guard);

  let read = test.db.read_txn();
  let before_flush_updates = read.get_all_updates(test.uid, &doc_id).unwrap();
  collab.lock().flush();
  let after_flush_updates = read.get_all_updates(test.uid, &doc_id).unwrap();

  let lock_guard = collab.lock();
  let after_flush_value = lock_guard.to_json_value();
  drop(lock_guard);
  assert_eq!(before_flush_updates.len(), 100);
  assert_eq!(after_flush_updates.len(), 0);
  assert_json_eq!(before_flush_value, after_flush_value);
}

#[tokio::test]
async fn insert_multiple_changes_and_restore_from_disk() {
  let mut test = CollabPersistenceTest::new(CollabPersistenceConfig::new());
  let doc_id = "1".to_string();
  let db = test.db.clone();
  test
    .run_scripts(vec![
      CreateDocumentWithCollabDB {
        id: doc_id.clone(),
        db: db.clone(),
      },
      InsertKeyValue {
        id: doc_id.clone(),
        key: "1".to_string(),
        value: "a".into(),
      },
      InsertKeyValue {
        id: doc_id.clone(),
        key: "2".to_string(),
        value: "b".into(),
      },
      InsertKeyValue {
        id: doc_id.clone(),
        key: "3".to_string(),
        value: "c".into(),
      },
      InsertKeyValue {
        id: doc_id.clone(),
        key: "4".to_string(),
        value: "d".into(),
      },
      AssertUpdateLen {
        id: doc_id.clone(),
        expected: 4,
      },
      CloseDocument {
        id: doc_id.to_string(),
      },
      OpenDocumentWithDiskPlugin {
        id: doc_id.to_string(),
      },
      GetValue {
        id: doc_id.to_string(),
        key: "1".to_string(),
        expected: Some("a".into()),
      },
      GetValue {
        id: doc_id.to_string(),
        key: "2".to_string(),
        expected: Some("b".into()),
      },
      GetValue {
        id: doc_id.to_string(),
        key: "3".to_string(),
        expected: Some("c".into()),
      },
      GetValue {
        id: doc_id,
        key: "4".to_string(),
        expected: Some("d".into()),
      },
    ])
    .await;
}

#[tokio::test]
async fn insert_multiple_docs() {
  let mut test = CollabPersistenceTest::new(CollabPersistenceConfig::new());
  let db = test.db.clone();
  test
    .run_scripts(vec![
      CreateDocumentWithCollabDB {
        id: "1".to_string(),
        db: db.clone(),
      },
      CreateDocumentWithCollabDB {
        id: "2".to_string(),
        db: db.clone(),
      },
      CreateDocumentWithCollabDB {
        id: "3".to_string(),
        db: db.clone(),
      },
      CreateDocumentWithCollabDB {
        id: "4".to_string(),
        db: db.clone(),
      },
      CreateDocumentWithCollabDB {
        id: "5".to_string(),
        db: db.clone(),
      },
      CreateDocumentWithCollabDB {
        id: "6".to_string(),
        db: db.clone(),
      },
      AssertNumOfDocuments { expected: 6 },
    ])
    .await;
}
