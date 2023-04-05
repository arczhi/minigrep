use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //读取文件内容
    let contents = fs::read_to_string(config.file_path)?;

    // println!("[minigrep] READ:\n{content}");

    for line in search(&config.query, &contents) {
        println!("line {}: {}", line.line_num, line.word);
    }

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(Args: &[String]) -> Result<Config, &str> {
        if Args.len() < 3 {
            return Err("no enough arguments !");
        }
        //要检索的字符串
        let query = Args[1].clone();
        //目标文件的路径
        let file_path = Args[2].clone();

        Ok(Config { query, file_path })
    }
}

struct SearchResult {
    word: String,
    line_num: usize,
}

//用例测试
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let query = "safe, fast, productive.";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }

// 失败用例
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

//成功用例
//文件检索
fn search(query: &str, contents: &str) -> Vec<SearchResult> {
    //结果集
    let mut result: Vec<SearchResult> = Vec::new();
    //扫描到的行数
    let mut line_num = 1;

    //遍历字符串的每一行
    for line in contents.lines() {
        //匹配文件中关键词的大小写形式
        if line.to_uppercase().contains(query) || line.to_lowercase().contains(query) {
            result.push(SearchResult {
                word: line.to_string(),
                line_num: line_num,
            });
        }
        //行数自增
        line_num += 1;
    }

    //返回结果集
    result
}
