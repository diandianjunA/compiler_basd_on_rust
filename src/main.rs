use std::env;
use std::fs::File;
use std::io::Read;
use compiler::ast::printer::LexerPrinter;
use compiler::analyzer::analyze::Analyzer;
use compiler::Compiler;

fn main() {
    if env::args().len() < 2 {
        panic!("参数数目不对！");
    }
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(name) => name,
        None => panic!("文件名不存在！")
    };
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("文件打开失败: {}", e)
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => panic!("文件读取失败: {}", e)
    };
    let mut compiler = Compiler::new(content);
    if args.contains(&"-l".to_string()){
        compiler.lexer();
    }
    if args.contains(&"-p".to_string()){
        compiler.parser();
    }
    if args.contains(&"-s".to_string()){
        compiler.symbol_table();
    }
    if args.contains(&"-t".to_string()){
        if args.contains(&"-o".to_string()){
            compiler.optimize_tac_code();
        } else {
            compiler.three_address_code();
        }
    }
    if args.contains(&"-m".to_string()){
        if args.contains(&"-o".to_string()){
            compiler.optimize_masm_code();
        } else {
            compiler.masm();
        }
    }
}

#[cfg(test)]
mod tests{
    use std::cell::RefCell;
    use std::rc::Rc;
    use compiler::ast::printer::{MasmPrinter, ParserPrinter, SymbolTablePrinter, ThreeAddressCodePrinter};
    use compiler::code_gen::CodeGen;
    use compiler::code_gen::optimize::Optimizer;
    use compiler::code_gen::target_code_gen::Masm;
    use super::*;

    #[test]
    fn test_lexer(){
        let content = "let a = 1 + 2;\r\nlet b = a + 3;\r\n";
        let lexer_printer = LexerPrinter::new(content.to_string());
        lexer_printer.print();
    }

    #[test]
    fn test_lexer_error(){
        let file_path = "./lexer_error.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let lexer_printer = LexerPrinter::new(content);
        lexer_printer.print();
    }

    #[test]
    fn test_lexer3(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let lexer_printer = LexerPrinter::new(content);
        lexer_printer.print();
    }

    #[test]
    fn test_parser1(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut parser_printer = ParserPrinter::new(content);
        parser_printer.print();
    }

    #[test]
    fn test_parser_error(){
        let file_path = "./parser_error.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut parser_printer = ParserPrinter::new(content);
        parser_printer.print();
    }

    #[test]
    fn test_semantic_error1(){
        let file_path = "./semantic_error.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
        let mut analyzer_printer = SymbolTablePrinter::new(Rc::new(RefCell::new(analyzer)));
        analyzer_printer.print();
    }

    #[test]
    fn test_analyze(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
        let mut analyzer_printer = SymbolTablePrinter::new(Rc::new(RefCell::new(analyzer)));
        analyzer_printer.print();
    }


    #[test]
    fn test_code_gen(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
        let mut three_address_code_printer = ThreeAddressCodePrinter::new(Rc::new(RefCell::new(analyzer)));
        three_address_code_printer.print();
    }

    #[test]
    fn test_target_code_gen(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
        let analyzer = Rc::new(RefCell::new(analyzer));
        let mut code_generator = CodeGen::new(analyzer.clone());
        code_generator.gen_code();
        let masm = Masm::new(analyzer, code_generator.codes.clone());
        let mut masm_printer = MasmPrinter::new(masm);
        masm_printer.print();
    }

    #[test]
    fn test_error2(){
        let file_path = "./test2.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
    }

    #[test]
    fn test_optimize(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
        let mut code_generator = CodeGen::new(Rc::new(RefCell::new(analyzer)));
        code_generator.gen_code();
        let mut optimizer = Optimizer::new(code_generator.codes);
        optimizer.optimize();
        optimizer.print();
    }

    #[test]
    fn test_optimize_masm(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let parser_printer = ParserPrinter::new(content);
        let mut analyzer = Analyzer::new(parser_printer.parser.stack.tree_root.clone(), parser_printer.lexer_printer.tokens.last().unwrap().line);
        analyzer.analyze();
        let analyzer = Rc::new(RefCell::new(analyzer));
        let mut code_generator = CodeGen::new(analyzer.clone());
        code_generator.gen_code();
        let mut optimizer = Optimizer::new(code_generator.codes);
        optimizer.optimize();
        let masm = Masm::new(analyzer, optimizer.codes);
        let mut masm_printer = MasmPrinter::new(masm);
        masm_printer.print();
    }

    #[test]
    fn test_compiler(){
        let file_path = "./right.c";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("文件打开失败: {}", e)
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut compiler = Compiler::new(content);
        compiler.optimize_masm_code();
    }
}
