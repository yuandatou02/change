use character_converter::traditional_to_simplified;
use async_fs::File;
use futures_lite::io::BufReader;
use std::io::{stdin, stdout, Write};
use futures_lite::{AsyncBufReadExt, AsyncWriteExt, StreamExt};

#[tokio::main]
async fn main() {
    print!("请输入文件路径:");
    // 刷新缓冲区
    stdout().flush().unwrap();
    // 输入文件路径
    let mut file_path = String::new();
    stdin()
        .read_line(&mut file_path)
        .expect("请输入正确的文件路径");
    println!("转化开始请稍等...");
    // 读取文件内容
    let file = File::open(file_path.trim()).await.expect("文件不存在");
    // 创建读写缓冲区
    let reader = BufReader::new(file);
    // 创建新文件
    let mut new_file = File::create("simplified.txt").await.expect("文件创建失败！");
    // 读取多行
    let mut lines = reader.lines();
    // 一行一行翻译
    while let Some(line) = lines.next().await {
        match line {
            Ok(content) => {
                // 将行读取到的内容进行转化
                let simplified = traditional_to_simplified(&content);
                // 将转化后的内容追加写入新文件
                new_file
                    .write_all(simplified.as_bytes()).await
                    .expect("写入文件失败");
                new_file.write_all(b"\n").await.expect("写入换行符失败");
            }
            Err(e) => println!("读取行时发生错误: {}", e),
        }
    }
    println!("转换完成！");
}
