//! An app that embeds the writer names itself in `docProps/app.xml` (its own test binary: the
//! name is set once per process).

use std::io::Read;

#[test]
fn the_embedding_app_names_itself_in_app_xml() {
    assert_eq!(bharat_docx::application_name(), "WordCraft", "the default");
    bharat_docx::set_application_name("Example Writer");
    bharat_docx::set_application_name("Other");
    assert_eq!(bharat_docx::application_name(), "Example Writer", "only the first call counts");
    let mut d = bharat_wordmodel::Document::new();
    d.body = vec![bharat_wordmodel::para_block(bharat_wordmodel::Paragraph::with_text("Text", Default::default()))];
    let bytes = bharat_docx::write(&d).unwrap();
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    let mut app = String::new();
    z.by_name("docProps/app.xml").unwrap().read_to_string(&mut app).unwrap();
    assert!(app.contains("<Application>Example Writer</Application>"), "{app}");
}
