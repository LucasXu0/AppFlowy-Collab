use collab_database::database::{gen_database_id, Database};
use collab_database::entity::FieldType;
use collab_database::template::builder::DatabaseTemplateBuilder;
use collab_database::template::entity::CELL_DATA;

#[tokio::test]
async fn create_template_test() {
  let expected_field_type = [
    FieldType::RichText,
    FieldType::SingleSelect,
    FieldType::MultiSelect,
    FieldType::DateTime,
    FieldType::Checklist,
    FieldType::LastEditedTime,
  ];

  let expected_cell_len = [6, 6, 6, 4, 2, 2];
  let expected_field_name = ["name", "status", "user", "time", "tasks", "last modified"];

  let template = DatabaseTemplateBuilder::new()
    .create_field("name", FieldType::RichText, true, |field_builder| {
      field_builder
        .create_cell("1th")
        .create_cell("2th")
        .create_cell("3th")
    })
    .create_field("status", FieldType::SingleSelect, false, |field_builder| {
      field_builder
        .create_cell("In Progress")
        .create_cell("Done")
        .create_cell("Not Started")
        .create_cell("In Progress")
        .create_cell("In Progress")
    })
    .create_field("user", FieldType::MultiSelect, false, |field_builder| {
      field_builder
        .create_cell("Lucas, Tom")
        .create_cell("Lucas")
        .create_cell("Tom")
        .create_cell("")
        .create_cell("Lucas, Tom, Nathan")
    })
    .create_field("time", FieldType::DateTime, false, |field_builder| {
      field_builder
        .create_cell("2024/08/22")
        .create_cell("2024-08-22")
        .create_cell("August 22, 2024")
        .create_cell("2024-08-22 03:30 PM")
    })
    .create_field("tasks", FieldType::Checklist, false, |field_builder| {
      field_builder
        .create_checklist_cell(vec!["A", "B"], vec!["A"])
        .create_checklist_cell(vec!["1", "2", "3"], Vec::<String>::new())
        .create_checklist_cell(vec!["task1", "task2"], vec!["task1", "task2"])
    })
    .create_field(
      "last modified",
      FieldType::LastEditedTime,
      false,
      |field_builder| {
        field_builder
          .create_cell("2024/08/22")
          .create_cell("2024-08-22")
          .create_cell("August 22, 2024")
          .create_cell("2024-08-22 03:30 PM")
      },
    )
    .build();

  assert_eq!(template.rows.len(), 5);
  for (index, row) in template.rows.iter().enumerate() {
    assert_eq!(row.cells.len(), expected_cell_len[index]);
  }
  assert_eq!(template.fields.len(), 6);

  let database_id = gen_database_id();
  let database = Database::create_with_template(&database_id, template)
    .await
    .unwrap();

  // Assert num of fields
  let fields = database.get_fields_in_view(database.get_inline_view_id().as_str(), None);
  assert_eq!(fields.len(), 6);
  for (index, field) in fields.iter().enumerate() {
    assert_eq!(field.field_type, expected_field_type[index].clone() as i64);
    assert_eq!(field.name, expected_field_name[index]);
  }

  // Assert num of rows
  let rows = database.get_all_rows().await;
  assert_eq!(rows.len(), 5);
  for row in rows.iter() {
    for field in &fields {
      let cell = row
        .cells
        .get(&field.id)
        .and_then(|cell| cell.get(CELL_DATA).cloned());
      println!("data: {:?}", cell);
    }
    println!("\n");
  }
}
