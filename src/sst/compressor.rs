use std::error::Error;

pub trait Compressor: Send + Sync {
    fn compress(&self, buf: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    fn uncompress(&self, buf: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}
