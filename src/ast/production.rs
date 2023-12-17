use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::NonTerminals::{AdditiveExpression, AdditiveExpression1, AddOp, ArgList, ArgList1, Args, BreakStmt, CaseDefaultStmt, CaseStmt, CaseStmtN, CaseStmtN1, CompoundStmt, ContinueStmt, Declaration, DeclarationList, DeclarationList1, DecStmt, DefaultStmt, DoStmt, ElseStmtN, Expression, Expression1, ExpressionAssign, Factor, ForExpression, ForStmt, FunDecStmt, IfStmt, LocalDeclarations, LogicalOp, MulOp, Param, ParamList, ParamList1, Params, Program, RelationalOp, ReturnStmt, SelfOp, SimpleExpression, SimpleExpression1, Statement, StatementList, StatementList1, SwitchStmt, Term, Term1, Type, Value, Var, Var1, VarDecStmt, VarOrCall, VarOrCallRemain, VarOrCallStmt, VarOrCallStmtRemain, VarStmt, WhileStmt};
use crate::ast::{TreeNode};
use crate::ast::lexer::Terminals;
use crate::ast::lexer::Terminals::{Break, Case, Char, CloseBrace, Colon, Comma, Continue, Default, Do, Else, Equals, Float, For, Identifier, If, Int, LeftParen, OpenBrace, RightParen, SemiColon, Switch, Void, While, Return, LeftBracket, RightBracket, LessThan, LessThanEquals, GreaterThanEquals, GreaterThan, EqualsEquals, BangEquals, And, Or, Not, Plus, Minus, Asterisk, Slash, Integer, FloatNumber, String, Character, True, False, PlusPlus, MinusMinus};

pub struct Stack {
    pub stack: Vec<Rc<RefCell<TreeNode>>>,
    pub tree_root: Rc<RefCell<TreeNode>>,
}

impl Stack {
    
    pub fn new() -> Self {
        let eof = TreeNode::new_value(Terminals::Eof, "".to_string());
        let eof_pointer = Rc::new(RefCell::from(eof));
        let tree_root = TreeNode::new_non_terminal(Program);
        let tree_root_pointer = Rc::new(RefCell::from(tree_root));
        Self {
            stack: vec![eof_pointer,tree_root_pointer.clone()],
            tree_root: tree_root_pointer.clone(),
        }
    }
    
    pub fn last(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.stack.last().cloned()
    }
    
    pub fn pop(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        self.stack.pop()
    }
    
    pub fn production0(&mut self) {

    }
    pub fn production1(&mut self, program: Rc<RefCell<TreeNode>>) {
        let declaration_list  = TreeNode::new_non_terminal(DeclarationList);
        let declaration_list_pointer = Rc::new(RefCell::from(declaration_list));
        program.borrow_mut().child.push(declaration_list_pointer.clone());
        self.stack.push(declaration_list_pointer.clone());
    }

    pub fn production2(&mut self, declaration_list: Rc<RefCell<TreeNode>>) {
        let declaration = TreeNode::new_non_terminal(Declaration);
        let declaration_list1 = TreeNode::new_non_terminal(DeclarationList1);
        let declaration_list1_pointer = Rc::new(RefCell::from(declaration_list1));
        let declaration_pointer = Rc::new(RefCell::from(declaration));
        declaration_list.borrow_mut().child.push(declaration_pointer.clone());        
        declaration_list.borrow_mut().child.push(declaration_list1_pointer.clone());
        self.stack.push(declaration_list1_pointer.clone());
        self.stack.push(declaration_pointer.clone());
    }

    pub fn production3(&mut self, declaration_list1: Rc<RefCell<TreeNode>>) {
        let declaration = TreeNode::new_non_terminal(Declaration);
        let declaration_list1_ = TreeNode::new_non_terminal(DeclarationList1);
        let declaration_list1_pointer = Rc::new(RefCell::from(declaration_list1_));
        let declaration_pointer = Rc::new(RefCell::from(declaration));
        declaration_list1.borrow_mut().child.push(declaration_pointer.clone());
        declaration_list1.borrow_mut().child.push(declaration_list1_pointer.clone());
        self.stack.push(declaration_list1_pointer.clone());
        self.stack.push(declaration_pointer.clone());
    }

    pub fn production4(&mut self, declaration: Rc<RefCell<TreeNode>>) {
        let type_ = TreeNode::new_non_terminal(Type);
        let identifier = TreeNode::new_value(Identifier("".to_string()), "".to_string());
        let dec_stmt = TreeNode::new_non_terminal(DecStmt);
        let type_pointer = Rc::new(RefCell::from(type_));
        let identifier_pointer = Rc::new(RefCell::from(identifier));
        let dec_stmt_pointer = Rc::new(RefCell::from(dec_stmt));
        declaration.borrow_mut().child.push(type_pointer.clone());
        declaration.borrow_mut().child.push(identifier_pointer.clone());
        declaration.borrow_mut().child.push(dec_stmt_pointer.clone());
        self.stack.push(dec_stmt_pointer.clone());
        self.stack.push(identifier_pointer.clone());
        self.stack.push(type_pointer.clone());
    }

    pub fn production5(&mut self, dec_stmt: Rc<RefCell<TreeNode>>) {
        let var1 = TreeNode::new_non_terminal(Var1);
        let var_dec_stmt = TreeNode::new_non_terminal(VarDecStmt);
        let var1_pointer = Rc::new(RefCell::from(var1));
        let var_dec_stmt_pointer = Rc::new(RefCell::from(var_dec_stmt));
        dec_stmt.borrow_mut().child.push(var1_pointer.clone());
        dec_stmt.borrow_mut().child.push(var_dec_stmt_pointer.clone());
        self.stack.push(var_dec_stmt_pointer.clone());
        self.stack.push(var1_pointer.clone());
    }

    pub fn production6(&mut self, dec_stmt: Rc<RefCell<TreeNode>>) {
        let fun_dec_stmt = TreeNode::new_non_terminal(FunDecStmt);
        let fun_dec_stmt_pointer = Rc::new(RefCell::from(fun_dec_stmt));
        dec_stmt.borrow_mut().child.push(fun_dec_stmt_pointer.clone());
        self.stack.push(fun_dec_stmt_pointer.clone());
    }

    pub fn production7(&mut self, type_: Rc<RefCell<TreeNode>>) {
        let int = TreeNode::new_value(Int, "".to_string());
        let int_pointer = Rc::new(RefCell::from(int));
        type_.borrow_mut().child.push(int_pointer.clone());
        self.stack.push(int_pointer.clone());
    }

    pub fn production8(&mut self, type_: Rc<RefCell<TreeNode>>) {
        let float = TreeNode::new_value(Float, "".to_string());
        let float_pointer = Rc::new(RefCell::from(float));
        type_.borrow_mut().child.push(float_pointer.clone());
        self.stack.push(float_pointer.clone());
    }

    pub fn production9(&mut self, type_: Rc<RefCell<TreeNode>>) {
        let char = TreeNode::new_value(Char, "".to_string());
        let char_pointer = Rc::new(RefCell::from(char));
        type_.borrow_mut().child.push(char_pointer.clone());
        self.stack.push(char_pointer.clone());
    }

    pub fn production10(&mut self, var_dec_stmt: Rc<RefCell<TreeNode>>) {
        let equals = TreeNode::new_value(Equals,"=".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let equals_pointer = Rc::new(RefCell::from(equals));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        var_dec_stmt.borrow_mut().child.push(expression_pointer.clone());
        self.stack.push(semicolon_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(equals_pointer.clone());
    }

    pub fn production11(&mut self, var1: Rc<RefCell<TreeNode>>) {
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        self.stack.push(semicolon_pointer.clone());
    }

    pub fn production12(&mut self, fun_dec_stmt: Rc<RefCell<TreeNode>>) {
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let params = TreeNode::new_non_terminal(Params);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let compound_stmt = TreeNode::new_non_terminal(CompoundStmt);
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let params_pointer = Rc::new(RefCell::from(params));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let compound_stmt_pointer = Rc::new(RefCell::from(compound_stmt));
        fun_dec_stmt.borrow_mut().child.push(params_pointer.clone());
        fun_dec_stmt.borrow_mut().child.push(compound_stmt_pointer.clone());
        self.stack.push(compound_stmt_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(params_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
    }

    pub fn production13(&mut self, params: Rc<RefCell<TreeNode>>) {
        let param_list = TreeNode::new_non_terminal(ParamList);
        let param_list_pointer = Rc::new(RefCell::from(param_list));
        params.borrow_mut().child.push(param_list_pointer.clone());
        self.stack.push(param_list_pointer.clone());
    }

    pub fn production14(&mut self, params: Rc<RefCell<TreeNode>>) {
        let void = TreeNode::new_value(Void,"void".to_string());
        let void_pointer = Rc::new(RefCell::from(void));
        params.borrow_mut().child.push(void_pointer.clone());
        self.stack.push(void_pointer.clone())
    }

    pub fn production15(&mut self, param_list: Rc<RefCell<TreeNode>>) {
        let param = TreeNode::new_non_terminal(Param);
        let param_list1 = TreeNode::new_non_terminal(ParamList1);
        let param_list1_pointer = Rc::new(RefCell::from(param_list1));
        let param_pointer = Rc::new(RefCell::from(param));
        param_list.borrow_mut().child.push(param_pointer.clone());
        param_list.borrow_mut().child.push(param_list1_pointer.clone());
        self.stack.push(param_list1_pointer.clone());
        self.stack.push(param_pointer.clone());
    }

    pub fn production16(&mut self, param_list1: Rc<RefCell<TreeNode>>) {
        let comma = TreeNode::new_value(Comma,",".to_string());
        let param = TreeNode::new_non_terminal(Param);
        let param_list1_ = TreeNode::new_non_terminal(ParamList1);
        let param_list1_pointer = Rc::new(RefCell::from(param_list1_));
        let param_pointer = Rc::new(RefCell::from(param));
        let comma_pointer = Rc::new(RefCell::from(comma));
        param_list1.borrow_mut().child.push(param_pointer.clone());
        param_list1.borrow_mut().child.push(param_list1_pointer.clone());
        self.stack.push(param_list1_pointer.clone());
        self.stack.push(param_pointer.clone());
        self.stack.push(comma_pointer.clone());
    }

    pub fn production17(&mut self, param: Rc<RefCell<TreeNode>>) {
        let type_ = TreeNode::new_non_terminal(Type);
        let var = TreeNode::new_non_terminal(Var);
        let type_pointer = Rc::new(RefCell::from(type_));
        let var_pointer = Rc::new(RefCell::from(var));
        param.borrow_mut().child.push(type_pointer.clone());
        param.borrow_mut().child.push(var_pointer.clone());
        self.stack.push(var_pointer.clone());
        self.stack.push(type_pointer.clone());
    }

    pub fn production18(&mut self, compound_stmt: Rc<RefCell<TreeNode>>) {
        let open_brace = TreeNode::new_value(OpenBrace,"{".to_string());
        let statement_list = TreeNode::new_non_terminal(StatementList);
        let close_brace = TreeNode::new_value(CloseBrace,"}".to_string());
        let open_brace_pointer = Rc::new(RefCell::from(open_brace));
        let statement_list_pointer = Rc::new(RefCell::from(statement_list));
        let close_brace_pointer = Rc::new(RefCell::from(close_brace));
        compound_stmt.borrow_mut().child.push(statement_list_pointer.clone());
        self.stack.push(close_brace_pointer.clone());
        self.stack.push(statement_list_pointer.clone());
        self.stack.push(open_brace_pointer.clone());
    }

    pub fn production19(&mut self, local_declarations: Rc<RefCell<TreeNode>>) {
        let type_ = TreeNode::new_non_terminal(Type);
        let identifier = TreeNode::new_value(Identifier("".to_string()),"".to_string());
        let var1 = TreeNode::new_non_terminal(Var1);
        let var_dec_stmt = TreeNode::new_non_terminal(VarDecStmt);
        let type_pointer = Rc::new(RefCell::from(type_));
        let identifier_pointer = Rc::new(RefCell::from(identifier));
        let var1_pointer = Rc::new(RefCell::from(var1));
        let var_dec_stmt_pointer = Rc::new(RefCell::from(var_dec_stmt));
        local_declarations.borrow_mut().child.push(type_pointer.clone());
        local_declarations.borrow_mut().child.push(identifier_pointer.clone());
        local_declarations.borrow_mut().child.push(var1_pointer.clone());
        local_declarations.borrow_mut().child.push(var_dec_stmt_pointer.clone());
        self.stack.push(var_dec_stmt_pointer.clone());
        self.stack.push(var1_pointer.clone());
        self.stack.push(identifier_pointer.clone());
        self.stack.push(type_pointer.clone());
    }

    pub fn production20(&mut self, statement_list: Rc<RefCell<TreeNode>>) {
        let statement = TreeNode::new_non_terminal(Statement);
        let statement_list1 = TreeNode::new_non_terminal(StatementList1);
        let statement_list1_pointer = Rc::new(RefCell::from(statement_list1));
        let statement_pointer = Rc::new(RefCell::from(statement));
        statement_list.borrow_mut().child.push(statement_pointer.clone());
        statement_list.borrow_mut().child.push(statement_list1_pointer.clone());
        self.stack.push(statement_list1_pointer.clone());
        self.stack.push(statement_pointer.clone());
    }

    pub fn production21(&mut self, statement_list1: Rc<RefCell<TreeNode>>) {
        let statement = TreeNode::new_non_terminal(Statement);
        let statement_list1_ = TreeNode::new_non_terminal(StatementList1);
        let statement_list1_pointer = Rc::new(RefCell::from(statement_list1_));
        let statement_pointer = Rc::new(RefCell::from(statement));
        statement_list1.borrow_mut().child.push(statement_pointer.clone());
        statement_list1.borrow_mut().child.push(statement_list1_pointer.clone());
        self.stack.push(statement_list1_pointer.clone());
        self.stack.push(statement_pointer.clone());
    }

    pub fn production22(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let var_or_call_stmt = TreeNode::new_non_terminal(VarOrCallStmt);
        let semi_colon = TreeNode::new_value(SemiColon,";".to_string());
        let var_or_call_stmt_pointer = Rc::new(RefCell::from(var_or_call_stmt));
        let semi_colon_pointer = Rc::new(RefCell::from(semi_colon));
        statement.borrow_mut().child.push(var_or_call_stmt_pointer.clone());
        self.stack.push(semi_colon_pointer.clone());
        self.stack.push(var_or_call_stmt_pointer.clone());
    }

    pub fn production23(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let for_stmt = TreeNode::new_non_terminal(ForStmt);
        let for_stmt_pointer = Rc::new(RefCell::from(for_stmt));
        statement.borrow_mut().child.push(for_stmt_pointer.clone());
        self.stack.push(for_stmt_pointer.clone());
    }

    pub fn production24(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let while_stmt = TreeNode::new_non_terminal(WhileStmt);
        let while_stmt_pointer = Rc::new(RefCell::from(while_stmt));
        statement.borrow_mut().child.push(while_stmt_pointer.clone());
        self.stack.push(while_stmt_pointer.clone());
    }

    pub fn production25(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let do_stmt = TreeNode::new_non_terminal(DoStmt);
        let do_stmt_pointer = Rc::new(RefCell::from(do_stmt));
        statement.borrow_mut().child.push(do_stmt_pointer.clone());
        self.stack.push(do_stmt_pointer.clone());
    }

    pub fn production26(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let if_stmt = TreeNode::new_non_terminal(IfStmt);
        let if_stmt_pointer = Rc::new(RefCell::from(if_stmt));
        statement.borrow_mut().child.push(if_stmt_pointer.clone());
        self.stack.push(if_stmt_pointer.clone());
    }

    pub fn production27(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let switch_stmt = TreeNode::new_non_terminal(SwitchStmt);
        let switch_stmt_pointer = Rc::new(RefCell::from(switch_stmt));
        statement.borrow_mut().child.push(switch_stmt_pointer.clone());
        self.stack.push(switch_stmt_pointer.clone());
    }

    pub fn production28(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let break_stmt = TreeNode::new_non_terminal(BreakStmt);
        let break_stmt_pointer = Rc::new(RefCell::from(break_stmt));
        statement.borrow_mut().child.push(break_stmt_pointer.clone());
        self.stack.push(break_stmt_pointer.clone());
    }

    pub fn production29(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let local_declarations = TreeNode::new_non_terminal(LocalDeclarations);
        let local_declarations_pointer = Rc::new(RefCell::from(local_declarations));
        statement.borrow_mut().child.push(local_declarations_pointer.clone());
        self.stack.push(local_declarations_pointer.clone());
    }

    pub fn production30(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let continue_stmt = TreeNode::new_non_terminal(ContinueStmt);
        let continue_stmt_pointer = Rc::new(RefCell::from(continue_stmt));
        statement.borrow_mut().child.push(continue_stmt_pointer.clone());
        self.stack.push(continue_stmt_pointer.clone());
    }

    pub fn production31(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let return_stmt = TreeNode::new_non_terminal(ReturnStmt);
        let return_stmt_pointer = Rc::new(RefCell::from(return_stmt));
        statement.borrow_mut().child.push(return_stmt_pointer.clone());
        self.stack.push(return_stmt_pointer.clone());
    }

    pub fn production32(&mut self, if_stmt: Rc<RefCell<TreeNode>>) {
        let if_ = TreeNode::new_value(If,"if".to_string());
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let compound_stmt = TreeNode::new_non_terminal(CompoundStmt);
        let else_stmt_n = TreeNode::new_non_terminal(ElseStmtN);
        let if_pointer = Rc::new(RefCell::from(if_));
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let compound_stmt_pointer = Rc::new(RefCell::from(compound_stmt));
        let else_stmt_n_pointer = Rc::new(RefCell::from(else_stmt_n));
        if_stmt.borrow_mut().child.push(expression_pointer.clone());
        if_stmt.borrow_mut().child.push(compound_stmt_pointer.clone());
        if_stmt.borrow_mut().child.push(else_stmt_n_pointer.clone());
        self.stack.push(else_stmt_n_pointer.clone());
        self.stack.push(compound_stmt_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
        self.stack.push(if_pointer.clone());
    }

    pub fn production33(&mut self, else_stmt_n: Rc<RefCell<TreeNode>>) {
        let else_stmt = TreeNode::new_non_terminal(ElseStmtN);
        let else_stmt_pointer = Rc::new(RefCell::from(else_stmt));
        else_stmt_n.borrow_mut().child.push(else_stmt_pointer.clone());
        self.stack.push(else_stmt_pointer.clone());
    }

    pub fn production34(&mut self, else_stmt: Rc<RefCell<TreeNode>>) {
        let else_ = TreeNode::new_value(Else,"else".to_string());
        let compound_stmt = TreeNode::new_non_terminal(CompoundStmt);
        let else_pointer = Rc::new(RefCell::from(else_));
        let compound_stmt_pointer = Rc::new(RefCell::from(compound_stmt));
        else_stmt.borrow_mut().child.push(compound_stmt_pointer.clone());
        self.stack.push(compound_stmt_pointer.clone());
        self.stack.push(else_pointer.clone());
    }

    pub fn production35(&mut self, while_stmt: Rc<RefCell<TreeNode>>) {
        let while_ = TreeNode::new_value(While,"while".to_string());
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let compound_stmt = TreeNode::new_non_terminal(CompoundStmt);
        let while_pointer = Rc::new(RefCell::from(while_));
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let compound_stmt_pointer = Rc::new(RefCell::from(compound_stmt));
        while_stmt.borrow_mut().child.push(expression_pointer.clone());
        while_stmt.borrow_mut().child.push(compound_stmt_pointer.clone());
        self.stack.push(compound_stmt_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
        self.stack.push(while_pointer.clone());
    }

    pub fn production36(&mut self, for_stmt: Rc<RefCell<TreeNode>>) {
        let for_ = TreeNode::new_value(For,"for".to_string());
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let for_expression = TreeNode::new_non_terminal(ForExpression);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let compound_stmt = TreeNode::new_non_terminal(CompoundStmt);
        let for_pointer = Rc::new(RefCell::from(for_));
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let for_expression_pointer = Rc::new(RefCell::from(for_expression));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let compound_stmt_pointer = Rc::new(RefCell::from(compound_stmt));
        for_stmt.borrow_mut().child.push(for_expression_pointer.clone());
        for_stmt.borrow_mut().child.push(compound_stmt_pointer.clone());
        self.stack.push(compound_stmt_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(for_expression_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
        self.stack.push(for_pointer.clone());
    }

    pub fn production37(&mut self, expression_assign: Rc<RefCell<TreeNode>>) {
        let var_or_call_stmt = TreeNode::new_non_terminal(VarOrCallStmt);
        let var_or_call_stmt_pointer = Rc::new(RefCell::from(var_or_call_stmt));
        expression_assign.borrow_mut().child.push(var_or_call_stmt_pointer.clone());
        self.stack.push(var_or_call_stmt_pointer.clone());
    }

    pub fn production40(&mut self, do_stmt: Rc<RefCell<TreeNode>>) {
        let do_ = TreeNode::new_value(Do,"do".to_string());
        let compound_stmt = TreeNode::new_non_terminal(CompoundStmt);
        let while_ = TreeNode::new_value(While,"while".to_string());
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let do_pointer = Rc::new(RefCell::from(do_));
        let compound_stmt_pointer = Rc::new(RefCell::from(compound_stmt));
        let while_pointer = Rc::new(RefCell::from(while_));
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        do_stmt.borrow_mut().child.push(compound_stmt_pointer.clone());
        do_stmt.borrow_mut().child.push(expression_pointer.clone());
        self.stack.push(semicolon_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
        self.stack.push(while_pointer.clone());
        self.stack.push(compound_stmt_pointer.clone());
        self.stack.push(do_pointer.clone());
    }

    pub fn production41(&mut self, break_stmt: Rc<RefCell<TreeNode>>) {
        let break_ = TreeNode::new_value(Break,"break".to_string());
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let break_pointer = Rc::new(RefCell::from(break_));
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        self.stack.push(semicolon_pointer.clone());
        self.stack.push(break_pointer.clone());
    }

    pub fn production42(&mut self, continue_stmt: Rc<RefCell<TreeNode>>) {
        let continue_ = TreeNode::new_value(Continue,"continue".to_string());
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let continue_pointer = Rc::new(RefCell::from(continue_));
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        self.stack.push(semicolon_pointer.clone());
        self.stack.push(continue_pointer.clone());
    }

    pub fn production43(&mut self, switch_stmt: Rc<RefCell<TreeNode>>) {
        let switch_ = TreeNode::new_value(Switch,"switch".to_string());
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let open_brace = TreeNode::new_value(OpenBrace,"{".to_string());
        let case_default_stmt = TreeNode::new_non_terminal(CaseDefaultStmt);
        let close_brace = TreeNode::new_value(CloseBrace,"}".to_string());
        let switch_pointer = Rc::new(RefCell::from(switch_));
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let open_brace_pointer = Rc::new(RefCell::from(open_brace));
        let case_default_stmt_pointer = Rc::new(RefCell::from(case_default_stmt));
        let close_brace_pointer = Rc::new(RefCell::from(close_brace));
        switch_stmt.borrow_mut().child.push(expression_pointer.clone());
        switch_stmt.borrow_mut().child.push(case_default_stmt_pointer.clone());
        self.stack.push(close_brace_pointer.clone());
        self.stack.push(case_default_stmt_pointer.clone());
        self.stack.push(open_brace_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
        self.stack.push(switch_pointer.clone());
    }

    pub fn production44(&mut self, case_default_stmt: Rc<RefCell<TreeNode>>) {
        let case_stmt_n = TreeNode::new_non_terminal(CaseStmtN);
        let default_stmt = TreeNode::new_non_terminal(DefaultStmt);
        let case_stmt_n_pointer = Rc::new(RefCell::from(case_stmt_n));
        let default_stmt_pointer = Rc::new(RefCell::from(default_stmt));
        case_default_stmt.borrow_mut().child.push(case_stmt_n_pointer.clone());
        case_default_stmt.borrow_mut().child.push(default_stmt_pointer.clone());
        self.stack.push(default_stmt_pointer.clone());
        self.stack.push(case_stmt_n_pointer.clone());
    }

    pub fn production45(&mut self, case_stmt_n: Rc<RefCell<TreeNode>>) {
        let case_stmt = TreeNode::new_non_terminal(CaseStmt);
        let case_stmt_n1 = TreeNode::new_non_terminal(CaseStmtN1);
        let case_stmt_pointer = Rc::new(RefCell::from(case_stmt));
        let case_stmt_n1_pointer = Rc::new(RefCell::from(case_stmt_n1));
        case_stmt_n.borrow_mut().child.push(case_stmt_pointer.clone());
        case_stmt_n.borrow_mut().child.push(case_stmt_n1_pointer.clone());
        self.stack.push(case_stmt_n1_pointer.clone());
        self.stack.push(case_stmt_pointer.clone());
    }

    pub fn production46(&mut self, case_stmt_n1: Rc<RefCell<TreeNode>>) {
        let case_stmt_n = TreeNode::new_non_terminal(CaseStmtN);
        let case_stmt_n_pointer = Rc::new(RefCell::from(case_stmt_n));
        case_stmt_n1.borrow_mut().child.push(case_stmt_n_pointer.clone());
        self.stack.push(case_stmt_n_pointer.clone());
    }

    pub fn production47(&mut self, case_stmt: Rc<RefCell<TreeNode>>) {
        let case_ = TreeNode::new_value(Case,"case".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let colon = TreeNode::new_value(Colon,":".to_string());
        let statement_list = TreeNode::new_non_terminal(StatementList);
        let case_pointer = Rc::new(RefCell::from(case_));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let colon_pointer = Rc::new(RefCell::from(colon));
        let statement_list_pointer = Rc::new(RefCell::from(statement_list));
        case_stmt.borrow_mut().child.push(expression_pointer.clone());
        case_stmt.borrow_mut().child.push(statement_list_pointer.clone());
        self.stack.push(statement_list_pointer.clone());
        self.stack.push(colon_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(case_pointer.clone());
    }

    pub fn production48(&mut self, default_stmt: Rc<RefCell<TreeNode>>) {
        let default_ = TreeNode::new_value(Default,"default".to_string());
        let colon = TreeNode::new_value(Colon,":".to_string());
        let statement_list = TreeNode::new_non_terminal(StatementList);
        let default_pointer = Rc::new(RefCell::from(default_));
        let colon_pointer = Rc::new(RefCell::from(colon));
        let statement_list_pointer = Rc::new(RefCell::from(statement_list));
        default_stmt.borrow_mut().child.push(statement_list_pointer.clone());
        self.stack.push(statement_list_pointer.clone());
        self.stack.push(colon_pointer.clone());
        self.stack.push(default_pointer.clone());
    }

    pub fn production49(&mut self, return_stmt: Rc<RefCell<TreeNode>>) {
        let return_ = TreeNode::new_value(Return,"return".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let return_pointer = Rc::new(RefCell::from(return_));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        return_stmt.borrow_mut().child.push(expression_pointer.clone());
        self.stack.push(semicolon_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(return_pointer.clone());
    }

    pub fn production50(&mut self, var: Rc<RefCell<TreeNode>>) {
        let identifier = TreeNode::new_value(Identifier("".to_string()),"".to_string());
        let var1 = TreeNode::new_non_terminal(Var1);
        let identifier_pointer = Rc::new(RefCell::from(identifier));
        let var1_pointer = Rc::new(RefCell::from(var1));
        var.borrow_mut().child.push(identifier_pointer.clone());
        var.borrow_mut().child.push(var1_pointer.clone());
        self.stack.push(var1_pointer.clone());
        self.stack.push(identifier_pointer.clone());
    }

    pub fn production51(&mut self, var1: Rc<RefCell<TreeNode>>) {
        let left_bracket = TreeNode::new_value(LeftBracket,"[".to_string());
        let expression = TreeNode::new_value(Integer(0), "0".to_string());
        let right_bracket = TreeNode::new_value(RightBracket,"]".to_string());
        let var1_ = TreeNode::new_non_terminal(Var1);
        let left_bracket_pointer = Rc::new(RefCell::from(left_bracket));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let right_bracket_pointer = Rc::new(RefCell::from(right_bracket));
        let var1_pointer = Rc::new(RefCell::from(var1_));
        var1.borrow_mut().child.push(expression_pointer.clone());
        var1.borrow_mut().child.push(var1_pointer.clone());
        self.stack.push(var1_pointer.clone());
        self.stack.push(right_bracket_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(left_bracket_pointer.clone());
    }

    pub fn production52(&mut self, expression: Rc<RefCell<TreeNode>>) {
        let additive_expression = TreeNode::new_non_terminal(AdditiveExpression);
        let expression1 = TreeNode::new_non_terminal(Expression1);
        let additive_expression_pointer = Rc::new(RefCell::from(additive_expression));
        let expression1_pointer = Rc::new(RefCell::from(expression1));
        expression.borrow_mut().child.push(additive_expression_pointer.clone());
        expression.borrow_mut().child.push(expression1_pointer.clone());
        self.stack.push(expression1_pointer.clone());
        self.stack.push(additive_expression_pointer.clone());
    }

    pub fn production53(&mut self, expression1: Rc<RefCell<TreeNode>>) {
        let logical_op = TreeNode::new_non_terminal(LogicalOp);
        let additive_expression = TreeNode::new_non_terminal(AdditiveExpression);
        let expression1_ = TreeNode::new_non_terminal(Expression1);
        let logical_op_pointer = Rc::new(RefCell::from(logical_op));
        let additive_expression_pointer = Rc::new(RefCell::from(additive_expression));
        let expression1_pointer = Rc::new(RefCell::from(expression1_));
        expression1.borrow_mut().child.push(logical_op_pointer.clone());
        expression1.borrow_mut().child.push(additive_expression_pointer.clone());
        expression1.borrow_mut().child.push(expression1_pointer.clone());
        self.stack.push(expression1_pointer.clone());
        self.stack.push(additive_expression_pointer.clone());
        self.stack.push(logical_op_pointer.clone());
    }

    pub fn production54(&mut self, relational_op: Rc<RefCell<TreeNode>>) {
        let less_than_equals = TreeNode::new_value(LessThanEquals,"<=".to_string());
        let relational_op_pointer = Rc::new(RefCell::from(less_than_equals));
        relational_op.borrow_mut().child.push(relational_op_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production55(&mut self, relational_op: Rc<RefCell<TreeNode>>) {
        let less_than = TreeNode::new_value(LessThan,"<".to_string());
        let relational_op_pointer = Rc::new(RefCell::from(less_than));
        relational_op.borrow_mut().child.push(relational_op_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production56(&mut self, relational_op: Rc<RefCell<TreeNode>>) {
        let greater_than_equals = TreeNode::new_value(GreaterThanEquals,">=".to_string());
        let relational_op_pointer = Rc::new(RefCell::from(greater_than_equals));
        relational_op.borrow_mut().child.push(relational_op_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production57(&mut self, relational_op: Rc<RefCell<TreeNode>>) {
        let greater_than = TreeNode::new_value(GreaterThan,">".to_string());
        let relational_op_pointer = Rc::new(RefCell::from(greater_than));
        relational_op.borrow_mut().child.push(relational_op_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production58(&mut self, relational_op: Rc<RefCell<TreeNode>>) {
        let equals_equals = TreeNode::new_value(EqualsEquals,"==".to_string());
        let relational_op_pointer = Rc::new(RefCell::from(equals_equals));
        relational_op.borrow_mut().child.push(relational_op_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production59(&mut self, relational_op: Rc<RefCell<TreeNode>>) {
        let bang_equals = TreeNode::new_value(BangEquals,"!=".to_string());
        let relational_op_pointer = Rc::new(RefCell::from(bang_equals));
        relational_op.borrow_mut().child.push(relational_op_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production60(&mut self, logical_op: Rc<RefCell<TreeNode>>) {
        let logical_and = TreeNode::new_value(And,"&&".to_string());
        let logical_op_pointer = Rc::new(RefCell::from(logical_and));
        logical_op.borrow_mut().child.push(logical_op_pointer.clone());
        self.stack.push(logical_op_pointer.clone());
    }

    pub fn production61(&mut self, logical_op: Rc<RefCell<TreeNode>>) {
        let logical_or = TreeNode::new_value(Or,"||".to_string());
        let logical_op_pointer = Rc::new(RefCell::from(logical_or));
        logical_op.borrow_mut().child.push(logical_op_pointer.clone());
        self.stack.push(logical_op_pointer.clone());
    }

    pub fn production62(&mut self, logical_op: Rc<RefCell<TreeNode>>) {
        let logical_not = TreeNode::new_value(Not,"!".to_string());
        let logical_op_pointer = Rc::new(RefCell::from(logical_not));
        logical_op.borrow_mut().child.push(logical_op_pointer.clone());
        self.stack.push(logical_op_pointer.clone());
    }

    pub fn production63(&mut self, additive_expression: Rc<RefCell<TreeNode>>) {
        let simple_expression = TreeNode::new_non_terminal(SimpleExpression);
        let additive_expression1 = TreeNode::new_non_terminal(AdditiveExpression1);
        let simple_expression_pointer = Rc::new(RefCell::from(simple_expression));
        let additive_expression1_pointer = Rc::new(RefCell::from(additive_expression1));
        additive_expression.borrow_mut().child.push(simple_expression_pointer.clone());
        additive_expression.borrow_mut().child.push(additive_expression1_pointer.clone());
        self.stack.push(additive_expression1_pointer.clone());
        self.stack.push(simple_expression_pointer.clone());
    }

    pub fn production64(&mut self, additive_expression1: Rc<RefCell<TreeNode>>) {
        let relational_op = TreeNode::new_non_terminal(RelationalOp);
        let simple_expression = TreeNode::new_non_terminal(SimpleExpression);
        let additive_expression1_ = TreeNode::new_non_terminal(AdditiveExpression1);
        let relational_op_pointer = Rc::new(RefCell::from(relational_op));
        let simple_expression_pointer = Rc::new(RefCell::from(simple_expression));
        let additive_expression1_pointer = Rc::new(RefCell::from(additive_expression1_));
        additive_expression1.borrow_mut().child.push(relational_op_pointer.clone());
        additive_expression1.borrow_mut().child.push(simple_expression_pointer.clone());
        additive_expression1.borrow_mut().child.push(additive_expression1_pointer.clone());
        self.stack.push(additive_expression1_pointer.clone());
        self.stack.push(simple_expression_pointer.clone());
        self.stack.push(relational_op_pointer.clone());
    }

    pub fn production65(&mut self, simple_expression: Rc<RefCell<TreeNode>>) {
        let term = TreeNode::new_non_terminal(Term);
        let simple_expression1 = TreeNode::new_non_terminal(SimpleExpression1);
        let term_pointer = Rc::new(RefCell::from(term));
        let simple_expression1_pointer = Rc::new(RefCell::from(simple_expression1));
        simple_expression.borrow_mut().child.push(term_pointer.clone());
        simple_expression.borrow_mut().child.push(simple_expression1_pointer.clone());
        self.stack.push(simple_expression1_pointer.clone());
        self.stack.push(term_pointer.clone());
    }

    pub fn production66(&mut self, simple_expression1: Rc<RefCell<TreeNode>>) {
        let add_op = TreeNode::new_non_terminal(AddOp);
        let term = TreeNode::new_non_terminal(Term);
        let simple_expression1_ = TreeNode::new_non_terminal(SimpleExpression1);
        let add_op_pointer = Rc::new(RefCell::from(add_op));
        let term_pointer = Rc::new(RefCell::from(term));
        let simple_expression1_pointer = Rc::new(RefCell::from(simple_expression1_));
        simple_expression1.borrow_mut().child.push(add_op_pointer.clone());
        simple_expression1.borrow_mut().child.push(term_pointer.clone());
        simple_expression1.borrow_mut().child.push(simple_expression1_pointer.clone());
        self.stack.push(simple_expression1_pointer.clone());
        self.stack.push(term_pointer.clone());
        self.stack.push(add_op_pointer.clone());
    }

    pub fn production67(&mut self, add_op: Rc<RefCell<TreeNode>>) {
        let plus = TreeNode::new_value(Plus,"+".to_string());
        let add_op_pointer = Rc::new(RefCell::from(plus));
        add_op.borrow_mut().child.push(add_op_pointer.clone());
        self.stack.push(add_op_pointer.clone());
    }

    pub fn production68(&mut self, add_op: Rc<RefCell<TreeNode>>) {
        let minus = TreeNode::new_value(Minus,"-".to_string());
        let add_op_pointer = Rc::new(RefCell::from(minus));
        add_op.borrow_mut().child.push(add_op_pointer.clone());
        self.stack.push(add_op_pointer.clone());
    }

    pub fn production69(&mut self, term: Rc<RefCell<TreeNode>>) {
        let factor = TreeNode::new_non_terminal(Factor);
        let term1 = TreeNode::new_non_terminal(Term1);
        let factor_pointer = Rc::new(RefCell::from(factor));
        let term1_pointer = Rc::new(RefCell::from(term1));
        term.borrow_mut().child.push(factor_pointer.clone());
        term.borrow_mut().child.push(term1_pointer.clone());
        self.stack.push(term1_pointer.clone());
        self.stack.push(factor_pointer.clone());
    }

    pub fn production70(&mut self, term1: Rc<RefCell<TreeNode>>) {
        let mul_op = TreeNode::new_non_terminal(MulOp);
        let factor = TreeNode::new_non_terminal(Factor);
        let term1_ = TreeNode::new_non_terminal(Term1);
        let mul_op_pointer = Rc::new(RefCell::from(mul_op));
        let factor_pointer = Rc::new(RefCell::from(factor));
        let term1_pointer = Rc::new(RefCell::from(term1_));
        term1.borrow_mut().child.push(mul_op_pointer.clone());
        term1.borrow_mut().child.push(factor_pointer.clone());
        term1.borrow_mut().child.push(term1_pointer.clone());
        self.stack.push(term1_pointer.clone());
        self.stack.push(factor_pointer.clone());
        self.stack.push(mul_op_pointer.clone());
    }

    pub fn production71(&mut self, mul_op: Rc<RefCell<TreeNode>>) {
        let star = TreeNode::new_value(Asterisk,"*".to_string());
        let mul_op_pointer = Rc::new(RefCell::from(star));
        mul_op.borrow_mut().child.push(mul_op_pointer.clone());
        self.stack.push(mul_op_pointer.clone());
    }

    pub fn production72(&mut self, mul_op: Rc<RefCell<TreeNode>>) {
        let slash = TreeNode::new_value(Slash,"/".to_string());
        let mul_op_pointer = Rc::new(RefCell::from(slash));
        mul_op.borrow_mut().child.push(mul_op_pointer.clone());
        self.stack.push(mul_op_pointer.clone());
    }

    pub fn production73(&mut self, factor: Rc<RefCell<TreeNode>>) {
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        factor.borrow_mut().child.push(expression_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
    }

    pub fn production74(&mut self, factor: Rc<RefCell<TreeNode>>) {
        let var_or_call = TreeNode::new_non_terminal(VarOrCall);
        let var_or_call_pointer = Rc::new(RefCell::from(var_or_call));
        factor.borrow_mut().child.push(var_or_call_pointer.clone());
        self.stack.push(var_or_call_pointer.clone());
    }

    pub fn production75(&mut self, factor: Rc<RefCell<TreeNode>>) {
        let value = TreeNode::new_non_terminal(Value);
        let value_pointer = Rc::new(RefCell::from(value));
        factor.borrow_mut().child.push(value_pointer.clone());
        self.stack.push(value_pointer.clone());
    }

    pub fn production76(&mut self, var_or_call: Rc<RefCell<TreeNode>>) {
        let identifier = TreeNode::new_value(Identifier("".to_string()),"".to_string());
        let var_or_call_remain = TreeNode::new_non_terminal(VarOrCallRemain);
        let identifier_pointer = Rc::new(RefCell::from(identifier));
        let var_or_call_remain_pointer = Rc::new(RefCell::from(var_or_call_remain));
        var_or_call.borrow_mut().child.push(identifier_pointer.clone());
        var_or_call.borrow_mut().child.push(var_or_call_remain_pointer.clone());
        self.stack.push(var_or_call_remain_pointer.clone());
        self.stack.push(identifier_pointer.clone());
    }

    pub fn production77(&mut self, var_or_call_remain: Rc<RefCell<TreeNode>>) {
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let args = TreeNode::new_non_terminal(Args);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let args_pointer = Rc::new(RefCell::from(args));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        var_or_call_remain.borrow_mut().child.push(args_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(args_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
    }

    pub fn production78(&mut self, var_or_call_remain: Rc<RefCell<TreeNode>>) {
        let var1 = TreeNode::new_non_terminal(Var1);
        let var1_pointer = Rc::new(RefCell::from(var1));
        var_or_call_remain.borrow_mut().child.push(var1_pointer.clone());
        self.stack.push(var1_pointer.clone());
    }

    pub fn production79(&mut self, value: Rc<RefCell<TreeNode>>) {
        let integer = TreeNode::new_value(Integer(0),"".to_string());
        let integer_pointer = Rc::new(RefCell::from(integer));
        value.borrow_mut().child.push(integer_pointer.clone());
        self.stack.push(integer_pointer.clone());
    }

    pub fn production80(&mut self, value: Rc<RefCell<TreeNode>>) {
        let float = TreeNode::new_value(FloatNumber(0.0),"".to_string());
        let float_pointer = Rc::new(RefCell::from(float));
        value.borrow_mut().child.push(float_pointer.clone());
        self.stack.push(float_pointer.clone());
    }

    pub fn production81(&mut self, value: Rc<RefCell<TreeNode>>) {
        let char_ = TreeNode::new_value(Character(' '),"".to_string());
        let char_pointer = Rc::new(RefCell::from(char_));
        value.borrow_mut().child.push(char_pointer.clone());
        self.stack.push(char_pointer.clone());
    }

    pub fn production82(&mut self, value: Rc<RefCell<TreeNode>>) {
        let string = TreeNode::new_value(String("".to_string()),"".to_string());
        let string_pointer = Rc::new(RefCell::from(string));
        value.borrow_mut().child.push(string_pointer.clone());
        self.stack.push(string_pointer.clone());
    }

    pub fn production83(&mut self, value: Rc<RefCell<TreeNode>>) {
        let true_ = TreeNode::new_value(True,"true".to_string());
        let true_pointer = Rc::new(RefCell::from(true_));
        value.borrow_mut().child.push(true_pointer.clone());
        self.stack.push(true_pointer.clone());
    }

    pub fn production84(&mut self, value: Rc<RefCell<TreeNode>>) {
        let false_ = TreeNode::new_value(False,"false".to_string());
        let false_pointer = Rc::new(RefCell::from(false_));
        value.borrow_mut().child.push(false_pointer.clone());
        self.stack.push(false_pointer.clone());
    }

    pub fn production85(&mut self, args: Rc<RefCell<TreeNode>>) {
        let arg_list = TreeNode::new_non_terminal(ArgList);
        let arg_list_pointer = Rc::new(RefCell::from(arg_list));
        args.borrow_mut().child.push(arg_list_pointer.clone());
        self.stack.push(arg_list_pointer.clone());
    }

    pub fn production86(&mut self, arg_list: Rc<RefCell<TreeNode>>) {
        let expression = TreeNode::new_non_terminal(Expression);
        let arg_list1 = TreeNode::new_non_terminal(ArgList1);
        let expression_pointer = Rc::new(RefCell::from(expression));
        let arg_list1_pointer = Rc::new(RefCell::from(arg_list1));
        arg_list.borrow_mut().child.push(expression_pointer.clone());
        arg_list.borrow_mut().child.push(arg_list1_pointer.clone());
        self.stack.push(arg_list1_pointer.clone());
        self.stack.push(expression_pointer.clone());
    }

pub fn production87(&mut self, arg_list1: Rc<RefCell<TreeNode>>) {
        let comma = TreeNode::new_value(Comma,",".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let arg_list1_ = TreeNode::new_non_terminal(ArgList1);
        let comma_pointer = Rc::new(RefCell::from(comma));
        let expression_pointer = Rc::new(RefCell::from(expression));
        let arg_list1_pointer = Rc::new(RefCell::from(arg_list1_));
        arg_list1.borrow_mut().child.push(expression_pointer.clone());
        arg_list1.borrow_mut().child.push(arg_list1_pointer.clone());
        self.stack.push(arg_list1_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(comma_pointer.clone());
    }

    pub fn production88(&mut self, call_stmt: Rc<RefCell<TreeNode>>) {
        let identifier = TreeNode::new_value(Identifier("".to_string()),"".to_string());
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let args = TreeNode::new_non_terminal(Args);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let semi_colon = TreeNode::new_value(SemiColon,";".to_string());
        let identifier_pointer = Rc::new(RefCell::from(identifier));
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let args_pointer = Rc::new(RefCell::from(args));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        let semi_colon_pointer = Rc::new(RefCell::from(semi_colon));
        call_stmt.borrow_mut().child.push(identifier_pointer.clone());
        call_stmt.borrow_mut().child.push(args_pointer.clone());
        self.stack.push(semi_colon_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(args_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
        self.stack.push(identifier_pointer.clone());
    }

    pub fn production89(&mut self, self_op: Rc<RefCell<TreeNode>>) {
        let plus_plus = TreeNode::new_value(PlusPlus,"++".to_string());
        let self_op_pointer = Rc::new(RefCell::from(plus_plus));
        self_op.borrow_mut().child.push(self_op_pointer.clone());
        self.stack.push(self_op_pointer.clone());
    }

    pub fn production90(&mut self, self_op: Rc<RefCell<TreeNode>>) {
        let minus_minus = TreeNode::new_value(MinusMinus,"--".to_string());
        let self_op_pointer = Rc::new(RefCell::from(minus_minus));
        self_op.borrow_mut().child.push(self_op_pointer.clone());
        self.stack.push(self_op_pointer.clone());
    }

    pub fn production91(&mut self, var_or_call_stmt: Rc<RefCell<TreeNode>>) {
        let identifier = TreeNode::new_value(Identifier("".to_string()),"".to_string());
        let var_or_call_stmt_remain = TreeNode::new_non_terminal(VarOrCallStmtRemain);
        let identifier_pointer = Rc::new(RefCell::from(identifier));
        let var_or_call_stmt_remain_pointer = Rc::new(RefCell::from(var_or_call_stmt_remain));
        var_or_call_stmt.borrow_mut().child.push(identifier_pointer.clone());
        var_or_call_stmt.borrow_mut().child.push(var_or_call_stmt_remain_pointer.clone());
        self.stack.push(var_or_call_stmt_remain_pointer.clone());
        self.stack.push(identifier_pointer.clone());
    }

    pub fn production92(&mut self, var_or_call_stmt_remain: Rc<RefCell<TreeNode>>) {
        let left_paren = TreeNode::new_value(LeftParen,"(".to_string());
        let args = TreeNode::new_non_terminal(Args);
        let right_paren = TreeNode::new_value(RightParen,")".to_string());
        let left_paren_pointer = Rc::new(RefCell::from(left_paren));
        let args_pointer = Rc::new(RefCell::from(args));
        let right_paren_pointer = Rc::new(RefCell::from(right_paren));
        var_or_call_stmt_remain.borrow_mut().child.push(args_pointer.clone());
        self.stack.push(right_paren_pointer.clone());
        self.stack.push(args_pointer.clone());
        self.stack.push(left_paren_pointer.clone());
    }

    pub fn production93(&mut self, var_or_call_stmt_remain: Rc<RefCell<TreeNode>>) {
        let var1 = TreeNode::new_non_terminal(Var1);
        let var_stmt = TreeNode::new_non_terminal(VarStmt);
        let var1_pointer = Rc::new(RefCell::from(var1));
        let var_stmt_pointer = Rc::new(RefCell::from(var_stmt));
        var_or_call_stmt_remain.borrow_mut().child.push(var1_pointer.clone());
        var_or_call_stmt_remain.borrow_mut().child.push(var_stmt_pointer.clone());
        self.stack.push(var_stmt_pointer.clone());
        self.stack.push(var1_pointer.clone());
    }

    pub fn production94(&mut self, var_stmt: Rc<RefCell<TreeNode>>) {
        let self_op = TreeNode::new_non_terminal(SelfOp);
        let self_op_pointer = Rc::new(RefCell::from(self_op));
        var_stmt.borrow_mut().child.push(self_op_pointer.clone());
        self.stack.push(self_op_pointer.clone());
    }

    pub fn production95(&mut self, var_stmt: Rc<RefCell<TreeNode>>) {
        let equals = TreeNode::new_value(Equals,"=".to_string());
        let expression = TreeNode::new_non_terminal(Expression);
        let equals_pointer = Rc::new(RefCell::from(equals));
        let expression_pointer = Rc::new(RefCell::from(expression));
        var_stmt.borrow_mut().child.push(expression_pointer.clone());
        self.stack.push(expression_pointer.clone());
        self.stack.push(equals_pointer.clone());
    }
    
    pub fn production96(&mut self, for_expression: Rc<RefCell<TreeNode>>) {
        let expression_assign1 = TreeNode::new_non_terminal(ExpressionAssign);
        let semicolon1 = TreeNode::new_value(SemiColon,";".to_string());
        let expression_assign2 = TreeNode::new_non_terminal(Expression);
        let semicolon2 = TreeNode::new_value(SemiColon,";".to_string());
        let expression_assign3 = TreeNode::new_non_terminal(ExpressionAssign);
        let expression_assign1_pointer = Rc::new(RefCell::from(expression_assign1));
        let semicolon1_pointer = Rc::new(RefCell::from(semicolon1));
        let expression_assign2_pointer = Rc::new(RefCell::from(expression_assign2));
        let semicolon2_pointer = Rc::new(RefCell::from(semicolon2));
        let expression_assign3_pointer = Rc::new(RefCell::from(expression_assign3));
        for_expression.borrow_mut().child.push(expression_assign1_pointer.clone());
        for_expression.borrow_mut().child.push(expression_assign2_pointer.clone());
        for_expression.borrow_mut().child.push(expression_assign3_pointer.clone());
        self.stack.push(expression_assign3_pointer.clone());
        self.stack.push(semicolon2_pointer.clone());
        self.stack.push(expression_assign2_pointer.clone());
        self.stack.push(semicolon1_pointer.clone());
        self.stack.push(expression_assign1_pointer.clone());
    }
    
    pub fn production97(&mut self, var_dec_stmt: Rc<RefCell<TreeNode>>) {
        let semicolon = TreeNode::new_value(SemiColon,";".to_string());
        let semicolon_pointer = Rc::new(RefCell::from(semicolon));
        self.stack.push(semicolon_pointer.clone());
    }
}

