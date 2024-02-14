use std::{env, fs};
use std::error::Error;
use std::path::Path;
use gedcomx::Gedcomx;

pub struct File {
    filename: String,
}
impl File {
    pub fn new(filename: String) -> Self {
        File { filename }
    }

    // Read the json gedcom file and return a GedCom struct
    // Check if the file is well formated
    pub fn read(&self) -> Result<Gedcomx, Box<dyn Error>> {
        let path = Path::new(&self.filename);

        // Check if the path exists
        if !path.exists() {
            return Err("Path does not exist".into());
        }

        // Ensure the file has a '.json' extension
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => {
                // Read the file to a string
                let contents = fs::read_to_string(&self.filename)?;
                // Deserialize the JSON string into Gedcomx
                let gx = Gedcomx::from_json_str(&contents)
                    .map_err(|_| "Failed to deserialize GEDCOM X document")?;
                println!(
                    "Successfully deserialized GEDCOM X document from JSON with {} people inside!",
                    gx.persons.len()
                );
                Ok(gx)
            },
            _ => Err("The file extension is not a JSON file.".into()),
        }
    }
}

#[test]
fn test_read() {
    let current_path = env::current_dir();
    let file = File::new(current_path.unwrap().to_str().unwrap().to_owned() + "/assets/files/gedcom.json");
    file.read();
}
