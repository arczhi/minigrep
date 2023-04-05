mod proc;

use proc::{run, Config};

use std::env;
use std::process;

/// `迷你grep工具`
/// a mini grep
/// Usage: minigrep [QUERY] [FILEPATH]
///
///
///

fn main() {
    //获取用户输入的命令行参数
    let Args: Vec<String> = env::args().collect();

    // //打印输入
    // dbg!(Args);

    // | err | { do something } //匹配错误
    let config = Config::build(&Args).unwrap_or_else(|err| {
        println!("parse args error: {err}\nUsage: minigrep [QUERY] [FILEPATH]");
        process::exit(1);
    });

    //运行处理逻辑
    // match run(config) {
    //     Ok(()) => println!("[minigrep] run successfully!"),
    //     Err(e) => {
    //         println!("[minigrep] run error: {e}");
    //         process::exit(1);
    //     }
    // }
    if let Err(e) = run(config) {
        println!("run error: {e}");
        process::exit(1);
    }
}
