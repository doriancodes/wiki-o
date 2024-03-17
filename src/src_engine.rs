use std::env::current_dir;

use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};

use anyhow::{Ok, Result};

use crate::logging::text;

pub struct Engine {
    schema: Schema,
    index: Index,
}

impl Engine {
    pub fn new(path: &String) -> Result<Engine> {
        let index_path = MmapDirectory::open(current_dir()?.join(path))?;

        let mut schema_builder = Schema::builder();

        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);

        let schema = schema_builder.build();
        let index = Index::open_or_create(index_path, schema.clone())?;
        Ok(Engine { schema, index })
    }
}

pub struct WriteOperation {
    pub engine: Engine,
}

impl WriteOperation {
    fn set_index_writer(&mut self) -> Result<IndexWriter> {
        let index_writer = self.engine.index.writer(50_000_000)?;
        Ok(index_writer)
    }

    pub fn build_index(&mut self, documents: Vec<WDocument>) -> Result<()> {
        let mut index_writer = self.set_index_writer()?;

        let title = &self.engine.schema.get_field("title")?;
        let body = &self.engine.schema.get_field("body")?;
        text("Adding documents to the index...".to_string());

        documents.iter().for_each(|doc| {
            println!("Adding document: {}", doc.title);
            let doc = doc!(
                *title => doc.title.clone(),
                *body => doc.body.clone()
            );
            index_writer.add_document(doc).unwrap(); //todo map to anyhow error
        });

        text("Committing...".to_string());
        index_writer.commit()?;
        text("Done.".to_string());

        Ok(())
    }
}

pub struct ReadOperation {
    pub engine: Engine,
}

impl ReadOperation {
    pub fn search(&self, search_str: &str) -> Result<()> {
        let title = Schema::get_field(&self.engine.schema, "title")?;
        let body = Schema::get_field(&self.engine.schema, "body")?;

        let reader = &self
            .engine
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(&self.engine.index, vec![title, body]);

        let query = query_parser.parse_query(search_str)?;
        // We can now perform our query.
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        for (_score, doc_address) in top_docs {
            let retrieved_doc: Document = searcher.doc(doc_address)?;
            println!(
                "{}, score: {}",
                self.engine.schema.to_json(&retrieved_doc),
                _score
            );
        }

        Ok(())
    }
}

pub struct WDocument {
    pub title: String,
    pub body: String,
}
