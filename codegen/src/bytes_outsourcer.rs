use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// A helper structure that stores bytes into files in a given directory.
/// These files will contain the rasterized characters. They can be included with `include_str!`
/// afterwards.
#[derive(Debug)]
pub struct BytesToFileOutsourcer {
    counter: u64,
    out_dir: &'static str,
}

impl BytesToFileOutsourcer {
    pub fn new(out_dir: &'static str) -> Self {
        Self {
            counter: 0,
            out_dir,
        }
    }

    /// Creates a file in the given directory with `bytes` as content.
    pub fn outsource_bytes(&mut self, bytes: &[u8]) -> PathBuf {
        let path = self.generate_path();

        let mut file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.as_path())
            .unwrap();
        file.write_all(bytes).unwrap();

        self.counter += 1;
        path
    }

    fn generate_path(&self) -> PathBuf {
        let filename = format!("{}.txt", self.counter);
        let mut buf = PathBuf::new();
        buf.push(self.out_dir);
        buf.push(filename);
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes_outsourcer::BytesToFileOutsourcer;

    #[test]
    fn test_bytes_outsourcer() {
        let mut outsourcer = BytesToFileOutsourcer::new("target");
        let path = outsourcer.outsource_bytes("hello world".as_bytes());
        assert_eq!("target/0.txt", path.as_os_str().to_str().unwrap());
    }
}
