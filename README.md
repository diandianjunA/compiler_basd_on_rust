本实验自定义的语言为C语言的子集

- 数据类型支持char类型、int类型和float类型，同时支持字符串给char型一维数组赋值。
- 基本运算支持算术运算、比较运算、自增自减运算、复合赋值运算和逻辑运算。
- 控制语句支持if语句、while语句、do while语句、break、continue语句、for语句、switch语句。
- 支持多维数组、函数调用、单行注释和多行注释

语言文法：

```
program:declaration_list
declaration_list:declaration declaration_list'
declaration_list':declaration declaration_list'|None
declaration:type Identifier dec_stmt
dec_stmt:var' var_dec_stmt|fun_dec_stmt
type: Int|Float|Char
var_dec_stmt:Equals expression SemiColon|SemiColon
fun_dec_stmt:LeftParen params RightParen compound_stmt
params:param_list|Void
param_list:param param_list'
param_list':Comma param param_list'| None
param:type var
compound_stmt:OpenBrace statement_list CloseBrace
local_declarations:type Identifier var' var_dec_stmt
statement_list:statement statement_list'|None
statement_list':statement statement_list'|None
statement:var_or_call_stmt SemiColon|for_stmt|while_stmt|do_stmt|if_stmt|switch_stmt|break_stmt|local_declarations|continue_stmt|return_stmt
if_stmt:If LeftParen expression RightParen compound_stmt else_stmt_n
else_stmt_n:else_stmt|None
else_stmt:Else compound_stmt
while_stmt:While LeftParen expression RightParen compound_stmt SemiColon
for_stmt:For LeftParen for_expression RightParen compound_stmt SemiColon
for_expression:expression_assign SemiColon expression SemiColon expression_assign
expression_assign:var_or_call_stmt|None
do_stmt:Do compound_stmt While LeftParen expression RightParen SemiColon
break_stmt:Break SemiColon
continue_stmt:Continue SemiColon
switch_stmt:Switch LeftParen expression RightParen OpenBrace case_default_stmt CloseBrace
case_default_stmt:case_stmt_n default_stmt
case_stmt_n:case_stmt case_stmt_n'
case_stmt_n':case_stmt_n|None
case_stmt:Case expression Colon statement_list
default_stmt:Default Colon statement_list|None
return_stmt:Return expression SemiColon
var:Identifier var'
var':LeftBracket Integer(i64) RightBracket var'|None
expression:additive_expression expression'
expression':logical_op additive_expression expression'|None
relational_op:LessThanEquals|LessThan|GreaterThanEquals|GreaterThan|EqualsEquals|BangEquals
logical_op:And|Or|Not
additive_expression:simple_expression additive_expression'
additive_expression':relational_op simple_expression additive_expression'|None
simple_expression:term simple_expression'
simple_expression':add_op term simple_expression'|None
add_op:Plus|Minus
term:factor term'
term':mul_op factor term'|None
mul_op:Asterisk|Slash
factor:LeftParen expression RightParen|var_or_call|value
var_or_call:Identifier var_or_call_remain
var_or_call_remain:LeftParen args RightParen|var'
value:Integer(i64)|FloatNumber(f64)|Character(char)|String(String)|True|False
args:arg_list|None
arg_list:expression arg_list'
arg_list':Comma expression arg_list'|None
call_stmt:Identifier LeftParen args RightParen SemiColon
self_op:PlusPlus|MinusMinus
var_or_call_stmt:Identifier var_or_call_stmt_remain
var_or_call_stmt_remain:LeftParen args RightParen|var' var_stmt
var_stmt:self_op|Equals expression
```