use std::fs;
use std::collections::HashMap;
use gedcomx::Gedcomx;
pub struct File {
    filename: String,
}
impl File {
    pub fn new(filename: String) -> Self {
        File { filename }
    }

    pub fn read(&self) -> Result<(), std::io::Error> {
        let contents = fs::read_to_string(&self.filename)?;

        let gx = Gedcomx::from_json_str(&contents)
            .expect("Failed to deserialize GEDCOM X document");

        println!(
            "Successfully deserialized GEDCOM X document from JSON with {} people inside!",
            gx.persons.len()
        );

        Ok(())
    }
}
#[test]
fn test_read() {

}
