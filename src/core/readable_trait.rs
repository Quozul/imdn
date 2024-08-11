pub trait ReadableTrait {
    fn get_mime(&self) -> String;
    fn get_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
