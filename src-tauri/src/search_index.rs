use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use tantivy::{schema::*, Index, Document, Result as TantivyResult};
use walkdir::WalkDir;
use tauri::{AppHandle, Manager};
use std::path::{PathBuf, Path};
use std::fs;




pub fn index_search(index: &Index, query_str: &str) -> TantivyResult<Vec<String>> {
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![index.schema().get_field("title").unwrap(), index.schema().get_field("path").unwrap()]);

    let query = query_parser.parse_query(query_str)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    let title_field = index.schema().get_field("title").unwrap();
    let path_field = index.schema().get_field("path").unwrap();
    let is_dir_field = index.schema().get_field("is_dir").unwrap();

    let results = top_docs.into_iter()
        .map(|(_score, doc_address)| {
            let doc = searcher.doc(doc_address)?;
            let title = doc.get_first(title_field).and_then(|f| f.as_text()).unwrap_or_default().to_string();
            let path = doc.get_first(path_field).and_then(|f| f.as_text()).unwrap_or_default().to_string();
            let is_dir = doc.get_first(is_dir_field).and_then(|f| f.as_bool()).unwrap_or_default();

            Ok(format!(r#"{{"title": "{}", "path": "{}", "is_dir": {}}}"#, title, path, is_dir))
        })
        .collect::<TantivyResult<Vec<String>>>();

    results
}




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





pub fn get_index_operational_data(index_path: String) -> TantivyResult<String> {
    let index = Index::open_in_dir(&index_path)?;
    let reader = index.reader()?;

    let searcher = reader.searcher();
    let segment_readers = searcher.segment_readers();

    let mut data = Vec::new();

    for (i, segment_reader) in segment_readers.iter().enumerate() {
        let segment_info = format!("Segment {}: {} documents", i, segment_reader.num_docs());
        data.push(segment_info);
    }

    // Calculate the directory size
    let dir_size = calculate_dir_size(&index_path)?;

    data.push(format!("Directory Size: {} bytes", dir_size));

    // Convert the Vec<String> to a single String
    let result = data.join("\n");
    Ok(result)
}


fn calculate_dir_size(dir_path: &str) -> TantivyResult<u64> {
    let dir_path = Path::new(dir_path);
    let mut total_size = 0;

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            total_size += calculate_dir_size(&path.to_string_lossy())?;
        } else {
            let metadata = entry.metadata()?;
            total_size += metadata.len();
        }
    }

    Ok(total_size)
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

        let _ = index_writer.add_document(doc);

        indexed_files += 1;
        if indexed_files % 100 == 0 { // Update progress every 100 files
            let _ = app.emit_all("index-progress", format!("{} / {}", indexed_files, total_files))
                .map_err(|_| "Failed to emit progress event");
        }
    }

    let _ = index_writer.commit().map_err(|e| e.to_string());
    Ok(())
}
