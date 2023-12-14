use tantivy::{schema::*, Index, Result};
use std::path::PathBuf;
use std::fs;

pub fn create_index() -> Result<Index> {
    // Define the schema for the index
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_bool_field("is_dir", STORED);

    let schema = schema_builder.build();

    // Specify the path for the index
    let index_path = PathBuf::from("/Users/sky/Documents/GitHub/learch/1");

    // Create the directory if it does not exist
    if !index_path.exists() {
        fs::create_dir_all(&index_path)?;
    }

    // Uncomment these lines to use the application directory provided by Tauri
    // let app_path = app_dir().ok_or("Could not find the app directory")?;
    // let index_path = app_path.join("index");

    // Create or open the index at the specified path
    let index = Index::create_in_dir(&index_path, schema)?;

    Ok(index)
}
