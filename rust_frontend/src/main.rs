mod rustlexer;
mod rustparser;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::input_stream::InputStream;
use antlr4rust::tree::ParseTree;
use antlr4rust::token_stream::TokenStream;
use rustlexer::RustLexer;
use rustparser::{RustParser, RustParserContext};

use std::env;
use std::fs;

fn main() {
    // 读取输入文件
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <源代码文件>", args[0]);
        return;
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("无法读取文件");

    // 构建 ANTLR 输入流
    let input = InputStream::new(code.as_str());

    // 词法分析
    let mut lexer = RustLexer::new(input);
    let token_stream = CommonTokenStream::new(&mut lexer);

    // 语法分析
    let mut parser = RustParser::new(token_stream);

    // 解析入口（根据 RustParser.g4 的规则名，通常是 'crate' 或 'source_file'）
    let result = parser.crate_(); // 或 parser.source_file_();

    // 输出 TokenStream
    println!("=== TokenStream ===");
    for token in parser.get_token_stream().get_tokens() {
        println!("{:?}", token);
    }

    // 输出 ParseTree（LISP 树格式）
    println!("\n=== ParseTree (LISP) ===");
    println!("{}", result.to_string_tree(&*parser));
}
