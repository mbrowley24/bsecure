



pub fn insert(table: String,
              insert_fields: String,
              insert_values: String,
              return_fields: String

) -> String {

    format!("INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            table,
            insert_fields,
            insert_values,
            return_fields
        )
}