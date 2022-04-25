pub trait Saver: Sync + Send {
    fn save(&self, title: String, data: Vec<u8>, at: u64);
    fn load(&self, title: String) -> Option<Vec<u8>>;
}