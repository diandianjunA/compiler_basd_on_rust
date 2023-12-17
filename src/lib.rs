use std::cell::RefCell;
use std::rc::Rc;
use crate::analyzer::analyze::Analyzer;
use crate::ast::printer::{LexerPrinter, MasmPrinter, ParserPrinter, SymbolTablePrinter, ThreeAddressCodePrinter};
use crate::code_gen::optimize::Optimizer;
use crate::code_gen::target_code_gen::Masm;

pub mod ast;
pub mod analyzer;
pub mod code_gen;
mod predict_table;

pub struct Compiler {
    pub lexer_printer: LexerPrinter,
    pub parser_printer: ParserPrinter,
    pub symbol_table_printer: SymbolTablePrinter,
    pub analyzer: Rc<RefCell<Analyzer>>,
    pub three_address_code_printer: ThreeAddressCodePrinter,
    pub masm_printer: MasmPrinter,
    pub optimizer: Optimizer
}

impl Compiler {
    pub fn new(content: String) -> Self {
        let lexer_printer = LexerPrinter::new(content.clone());
        let parser_printer = ParserPrinter::new(content.clone());
        let analyzer = Rc::new(RefCell::new(Analyzer::new(parser_printer.parser.stack.tree_root.clone(), lexer_printer.tokens.last().unwrap().line)));
        let symbol_table_printer = SymbolTablePrinter::new(analyzer.clone());
        let three_address_code_printer = ThreeAddressCodePrinter::new(analyzer.clone());
        let optimizer = Optimizer::new(three_address_code_printer.code_generator.codes.clone());
        let masm = Masm::new(analyzer.clone(), three_address_code_printer.code_generator.codes.clone());
        let masm_printer = MasmPrinter::new(masm);
        Self {
            lexer_printer,
            parser_printer,
            symbol_table_printer,
            analyzer,
            three_address_code_printer,
            masm_printer,
            optimizer
        }
    }
    
    pub fn lexer(&mut self) {
        self.lexer_printer.print();
    }
    
    pub fn parser(&mut self) {
        self.parser_printer.print();
    }
    
    pub fn symbol_table(&mut self) {
        self.symbol_table_printer.print();
    }
    
    pub fn three_address_code(&mut self) {
        self.three_address_code_printer.print();
    }
    
    pub fn masm(&mut self) {
        self.masm_printer.print();
    }
    
    pub fn optimize_tac_code(&mut self) {
        self.optimizer.optimize();
        self.optimizer.print();
    }
    
    pub fn optimize_masm_code(&mut self) {
        self.optimizer.optimize();
        let masm = Masm::new(self.analyzer.clone(), self.optimizer.codes.clone());
        self.masm_printer.masm = masm;
        self.masm_printer.print();
    }
}
