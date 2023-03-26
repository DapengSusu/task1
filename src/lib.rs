pub mod layout;

use std::{fs::File, io::{Write, Seek, SeekFrom}};

// 按照指定格式生成字符串内容
pub fn generate_file_contents() -> String {
    let mut contents = String::with_capacity(100);
    for i in 1..=99 {
        contents.push_str(&i.to_string());
        if (i % 3) == 0 {
            contents.push('\n');
            continue;
        }
        contents.push(',');
    }

    contents
}

// 反序按行读取字符串内容，并反序每一行的内容，写入新文件
pub fn write_formated_contents(file: &mut File, contents: &str)
    -> std::io::Result<usize>
{
    let mut size = 0;
    // 按行读取，并反序
    let buf_read: Vec<_> = contents.lines().collect();
    for line in buf_read.iter().rev() {
        // 按字符','分割，再次反序
        let new_line: Vec<_> = line.split(',').collect();
        for ch in new_line.iter().rev() {
            size += file.write(ch.as_bytes())?;
            size += file.write(','.to_string().as_bytes())?;
        }
        // 顶掉行末多余的字符','
        file.seek(SeekFrom::Current(-1))?;
        file.write('\n'.to_string().as_bytes())?;
    }

    Ok(size)
}
