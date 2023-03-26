use std::{fs::File, io::{Write, Read}};

fn main() -> std::io::Result<()> {
    // 创建文件（不存在创建，存在则截断文件内容）
    const FILE_PATH: &str  = "./output.txt";
    let mut file = match File::create(FILE_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create file '{}' failed!", FILE_PATH);
            return Err(e);
        },
    };

    // 生成所需内容
    let contents_write = task1::generate_file_contents();

    // 写入文件
    file.write_all(contents_write.as_bytes())?;

    // 读取文件类容
    let mut contents_read = String::new();
    let mut file = File::open(FILE_PATH)?;
    let size = file.read_to_string(&mut contents_read)?;
    println!("read size:{}", size);

    // 按行读取，倒序并写入新文件
    const NEW_FILE_PATH: &str  = "./output_new.txt";
    let mut file = File::create(NEW_FILE_PATH)?;
    let size = task1::write_formated_contents(&mut file, &contents_read)?;
    println!("write size:{}", size);

    Ok(())
}
