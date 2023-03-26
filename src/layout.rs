use std::{fs, io::Read};
use std::path;

/// 文件行
pub struct FileLines(Vec<String>);

impl FileLines {
    // 从给定的文件中读取，按行存储
    pub fn new(file_path: &path::Path) -> std::io::Result<Self> {
        let mut file_lines = FileLines(vec![]);
        let mut contents = String::new();
        let mut f = fs::File::open(file_path)?;
        let size = f.read_to_string(&mut contents)?;
        for line in contents.lines() {
            file_lines.0.push(line.to_string());
        }

        println!("read size:{}, lines:{}", size, file_lines.0.len());
        Ok(file_lines)
    }

    // 
}
