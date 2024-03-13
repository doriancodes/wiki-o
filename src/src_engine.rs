use std::cell::Cell;
use std::env::current_dir;

use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};

use anyhow::{Ok, Result};

pub struct SearchEngine {
    schema: Schema,
    index: Index,
    index_writer: Cell<IndexWriter>,
}

impl SearchEngine {
    pub fn new() -> Result<SearchEngine>{
            let index_path = MmapDirectory::open(current_dir()?.join("temp"))?;
                        
            let mut schema_builder = Schema::builder();
        
            schema_builder.add_text_field("title", TEXT | STORED);
        
            schema_builder.add_text_field("body", TEXT);
        
            let schema = schema_builder.build();

            println!("Schema: {:?}", schema);
        
            let index = Index::open_or_create(index_path, schema.clone())?;

            println!("Index: {:?}", index);
        
            let index_writer: IndexWriter = index.writer(50_000_000)?;
        
       Ok( SearchEngine {
            schema,
            index,
            index_writer: Cell::new(index_writer),
        })
           
    }
    
    pub fn search(&self) -> Result<()> {

        let title = Schema::get_field(&self.schema, "title")?;
        let body = Schema::get_field(&self.schema, "body")?;
       // # Searching
        //
        // ### Searcher
        //
        // A reader is required first in order to search an index.
        // It acts as a `Searcher` pool that reloads itself,
        // depending on a `ReloadPolicy`.
        //
        // For a search server you will typically create one reader for the entire lifetime of your
        // program, and acquire a new searcher for every single request.
        //
        // In the code below, we rely on the 'ON_COMMIT' policy: the reader
        // will reload the index automatically after each commit.
        let reader = &self.index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;
    
        // We now need to acquire a searcher.
        //
        // A searcher points to a snapshotted, immutable version of the index.
        //
        // Some search experience might require more than
        // one query. Using the same searcher ensures that all of these queries will run on the
        // same version of the index.
        //
        // Acquiring a `searcher` is very cheap.
        //
        // You should acquire a searcher every time you start processing a request and
        // and release it right after your query is finished.
        let searcher = reader.searcher();
    
        // ### Query
    
        // The query parser can interpret human queries.
        // Here, if the user does not specify which
        // field they want to search, tantivy will search
        // in both title and body.
        let query_parser = QueryParser::for_index(&self.index, vec![title, body]);
    
        // `QueryParser` may fail if the query is not in the right
        // format. For user facing applications, this can be a problem.
        // A ticket has been opened regarding this problem.
        let query = query_parser.parse_query("sea whale")?;
    
        // A query defines a set of documents, as
        // well as the way they should be scored.
        //
        // A query created by the query parser is scored according
        // to a metric called Tf-Idf, and will consider
        // any document matching at least one of our terms.
    
        // ### Collectors
        //
        // We are not interested in all of the documents but
        // only in the top 10. Keeping track of our top 10 best documents
        // is the role of the `TopDocs` collector.
    
        // We can now perform our query.
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    
        // The actual documents still need to be
        // retrieved from Tantivy's store.
        //
        // Since the body field was not configured as stored,
        // the document returned will only contain
        // a title.
        for (_score, doc_address) in top_docs {
            let retrieved_doc: Document = searcher.doc(doc_address)?;
            println!("{}, score: {}", self.schema.to_json(&retrieved_doc), _score);
        }
    
        // We can also get an explanation to understand
        // how a found document got its score.
        let query = query_parser.parse_query("title:sea^20 body:whale^70")?;
    
        let (_score, doc_address) = searcher
            .search(&query, &TopDocs::with_limit(1))?
            .into_iter()
            .next()
            .unwrap();
    
    
    
        Ok(())
    }

    pub fn build_index(&mut self) -> Result<()> {


    // ### Adding documents
       //
       // We can create a document manually, by setting the fields
       // one by one in a Document object.
       let title = &self.schema.get_field("title").unwrap();
       let body = &self.schema.get_field("body").unwrap();
   
        println!("Adding documents to the index...");
       let mut old_man_doc = Document::default();
       old_man_doc.add_text(*title, "The Old Man and the Sea");
       old_man_doc.add_text(
           *body,
           "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
            eighty-four days now without taking a fish.",
       );
   
       // ... and add it to the `IndexWriter`.
       self.index_writer.get_mut().add_document(old_man_doc)?;
   
       // For convenience, tantivy also comes with a macro to
       // reduce the boilerplate above.
       self.index_writer.get_mut().add_document(doc!(
       *title => "Of Mice and Men",
       *body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
               bank and runs deep and green. The water is warm too, for it has slipped twinkling \
               over the yellow sands in the sunlight before reaching the narrow pool. On one \
               side of the river the golden foothill slopes curve up to the strong and rocky \
               Gabilan Mountains, but on the valley side the water is lined with trees—willows \
               fresh and green with every spring, carrying in their lower leaf junctures the \
               debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
               limbs and branches that arch over the pool"
       ))?;
   
       // Multivalued field just need to be repeated.
       self.index_writer.get_mut().add_document(doc!(
       *title => "Frankenstein",
       *title => "The Modern Prometheus",
       *body => "You will rejoice to hear that no disaster has accompanied the commencement of an \
                enterprise which you have regarded with such evil forebodings.  I arrived here \
                yesterday, and my first task is to assure my dear sister of my welfare and \
                increasing confidence in the success of my undertaking."
       ))?;
   
       // This is an example, so we will only index 3 documents
       // here. You can check out tantivy's tutorial to index
       // the English wikipedia. Tantivy's indexing is rather fast.
       // Indexing 5 million articles of the English wikipedia takes
       // around 3 minutes on my computer!
   
       // ### Committing
       //
       // At this point our documents are not searchable.
       //
       //
       // We need to call `.commit()` explicitly to force the
       // `index_writer` to finish processing the documents in the queue,
       // flush the current index to the disk, and advertise
       // the existence of new documents.
       //
       // This call is blocking.
       println!("Committing...");
       self.index_writer.get_mut().commit()?;
       println!("Done.");
   
       // If `.commit()` returns correctly, then all of the
       // documents that have been added are guaranteed to be
       // persistently indexed.
       //
       // In the scenario of a crash or a power failure,
       // tantivy behaves as if it has rolled back to its last
       // commit.
   
       Ok(())
   
   
        

    }

    
}