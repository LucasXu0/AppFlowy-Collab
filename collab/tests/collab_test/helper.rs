use collab::collab::{Collab, CollabBuilder};
use collab::plugin::disk::CollabStateCachePlugin;
use collab_derive::Collab;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yrs::Map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub(crate) name: String,
    pub(crate) position: Position,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Position {
    pub(crate) title: String,
    pub(crate) level: u8,
}

pub fn make_collab_pair() -> (Collab, Collab, CollabStateCachePlugin) {
    let update_cache = CollabStateCachePlugin::new();
    let mut local_collab = CollabBuilder::new("1".to_string(), 1)
        .with_plugin(update_cache.clone())
        .build();
    // Insert document
    local_collab.insert_json_with_path(vec![], "document", test_document());
    let remote_collab =
        CollabBuilder::from_updates("1".to_string(), 1, update_cache.get_updates().unwrap())
            .build();

    (local_collab, remote_collab, update_cache)
}

#[derive(Collab, Serialize, Deserialize)]
pub struct Document {
    doc_id: String,
    name: String,
    created_at: i64,
    attributes: HashMap<String, String>,
    tasks: HashMap<String, TaskInfo>,
    owner: Owner,
}

#[derive(Collab, Default, Serialize, Deserialize)]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub email: String,
    pub location: Option<String>,
}

#[derive(Collab, Default, Serialize, Deserialize)]
pub struct TaskInfo {
    pub title: String,
    pub repeated: bool,
}

fn test_document() -> Document {
    let owner = Owner {
        id: "owner_id".to_string(),
        name: "nathan".to_string(),
        email: "nathan@appflowy.io".to_string(),
        location: None,
    };

    let mut attributes = HashMap::new();
    attributes.insert("1".to_string(), "task 1".to_string());
    attributes.insert("2".to_string(), "task 2".to_string());

    let mut tasks = HashMap::new();
    tasks.insert(
        "1".to_string(),
        TaskInfo {
            title: "Task 1".to_string(),
            repeated: true,
        },
    );
    tasks.insert(
        "2".to_string(),
        TaskInfo {
            title: "Task 2".to_string(),
            repeated: false,
        },
    );

    Document {
        doc_id: "doc_id".to_string(),
        name: "Hello world".to_string(),
        created_at: 0,
        attributes,
        tasks,
        owner,
    }
}