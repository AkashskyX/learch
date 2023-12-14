use tantivy::{schema::*, Index, Document, Result as TantivyResult};
use walkdir::WalkDir;
use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use std::fs;
use std::io::Error as IoError;
use tantivy::TantivyError;

pub fn create_index() -> TantivyResult<Index> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_bool_field("is_dir", STORED);

    let schema = schema_builder.build();
    let index_path = PathBuf::from("/Users/sky/Documents/GitHub/learch/1");

    if !index_path.exists() {
        fs::create_dir_all(&index_path)?;
    }

    let index = Index::create_in_dir(&index_path, schema)?;
    Ok(index)
}

pub fn index_files(app: AppHandle, index: &Index, root_path: &str) -> TantivyResult<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let total_files = WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .count();
    
    let mut indexed_files = 0;

    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let title = path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        let path_str = path.to_string_lossy().to_string();
        let is_dir = path.is_dir();

        let mut doc = Document::new();
        doc.add_text(index.schema().get_field("title").unwrap(), &title);
        doc.add_text(index.schema().get_field("path").unwrap(), &path_str);
        doc.add_bool(index.schema().get_field("is_dir").unwrap(), is_dir);

        index_writer.add_document(doc);

        indexed_files += 1;
        if indexed_files % 100 == 0 { // Update progress every 100 files
            app.emit_all("index-progress", format!("{} / {}", indexed_files, total_files))
                .map_err(|_| "Failed to emit progress event");
        }
    }

    index_writer.commit().map_err(|e| e.to_string());
    Ok(())
}
