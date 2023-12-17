```html
G[program]:
program → ExtDefList                                                                                                     
ExtDefList→ExtDef  ExtDefList | ε
ExtDef→Specifier  ExtDecList ; |Specifier  FunDec  CompSt                                                                                         
Specifier→int | float                                    
ExtDecList→VarDec | VarDec , ExtDecList                            
VarDec→ID|VarDec [ INT ]
FucDec→ID ( VarList )  | ID ( )                                        
VarList→ParamDec , VarList   |   ParamDec                        
ParamDec→Specifier VarDec                                                    
CompSt→{ DefList  StmList }                                            
StmList→Stmt  StmList | ε                            
Stmt→Exp ;  |  CompSt  | return Exp ;                            
| if ( Exp ) Stmt   | if ( Exp ) Stmt else Stmt   | while ( Exp ) Stmt        
DefList→Def DefList  |    ε                                    
Def→Specifier DecList ;                                                
DecList→Dec  | Dec , DecList                    
Dec→VarDec  |  VarDec = Exp

Exp →Exp =Exp  | Exp && Exp |  Exp || Exp   | Exp < Exp | Exp <= Exp
| Exp == Exp | Exp != Exp    | Exp > Exp | Exp >= Exp
| Exp + Exp    | Exp - Exp  | Exp * Exp    | Exp / Exp    | ID | INT | FLOAT
| ( Exp )        | - Exp  |  ! Exp  | ID ( Args )  | ID ( )
Args→Exp , Args  | Exp | EXP[Exp] | EXP.ID
```