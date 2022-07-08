use crate::error::Error;
use inkwell::{
    context::Context,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum},
};
use pest::{iterators::Pair, Parser};
use std::borrow::Borrow;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CParser;

#[derive(Debug)]
pub struct Ast {
    pub position: (usize, usize),
    pub info: ASTInfo,
}

#[derive(Debug)]
pub enum ASTInfo {
    /// type, name, params, block_statements: type name(params) {statements}
    FunctionDec(Type, String, Vec<(Type, String)>, Box<Ast>),
    /// type, name
    VariableDec(Type, String),

    /// variable_declarations, expressions
    BlockStmt(Vec<Ast>, Vec<Ast>),
    /// condition, if_statement, else_statement: if (condition) {if_statements} else {else_statement}
    SelectionStmt(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    /// condition, expressions: while(condition) {expression}
    IterationStmt(Box<Ast>, Box<Ast>),
    /// return value
    ReturnStmt(Option<Box<Ast>>),

    /// var, expression
    AssignmentExpr(Box<Ast>, Box<Ast>),
    /// operation, expression, expression: expression operation expression
    BinaryExpr(Operand, Box<Ast>, Box<Ast>),
    /// name, args
    CallExpr(String, Vec<Ast>),

    /// name, []: name[]
    Variable(String, Option<Box<Ast>>),
    IntLiteral(i32),
}

#[derive(Debug)]
pub enum Operand {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Ge,
    Le,
    Gt,
    Lt,
    Eq,
    Ne,
    Band,
    Bor,
    Bxor,
    Land,
    Lor,
    LShift,
    RShift,
}

impl ToString for Operand {
    fn to_string(&self) -> String {
        match self {
            Self::Add => "Add".to_string(),
            Self::Sub => "Sub".to_string(),
            Self::Mul => "Mul".to_string(),
            Self::Div => "Div".to_string(),
            Self::Mod => "Mod".to_string(),
            Self::Ge => "Ge".to_string(),
            Self::Le => "Le".to_string(),
            Self::Gt => "Gt".to_string(),
            Self::Lt => "Lt".to_string(),
            Self::Eq => "Eq".to_string(),
            Self::Ne => "Ne".to_string(),
            Self::Band => "Band".to_string(),
            Self::Bor => "Bor".to_string(),
            Self::Bxor => "Bxor".to_string(),
            Self::Land => "Land".to_string(),
            Self::Lor => "Lor".to_string(),
            Self::LShift => "LShift".to_string(),
            Self::RShift => "RShift".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Int,
    Void,
    IntArray(usize),
    IntPtr,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Self::Int => "int".to_string(),
            Self::Void => "void".to_string(),
            Self::IntArray(size) => format!("int array[{}]", size),
            Self::IntPtr => "int pointer".to_string(),
        }
    }
}

impl<'ctx> Type {
    pub fn to_llvm_basic_type(self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            Type::Int => context.i32_type().as_basic_type_enum(),
            Type::Void => panic!("Variable have void type"),
            Type::IntArray(size) => context
                .i32_type()
                .array_type(size as u32)
                .as_basic_type_enum(),
            Type::IntPtr => context
                .i32_type()
                // .array_type(0)
                .ptr_type(inkwell::AddressSpace::Generic)
                .as_basic_type_enum(),
        }
    }

    pub fn to_llvm_basic_metadata_type(
        self,
        context: &'ctx Context,
    ) -> BasicMetadataTypeEnum<'ctx> {
        match self {
            Type::Int => BasicMetadataTypeEnum::IntType(context.i32_type()),
            Type::Void => panic!("Variable have void type"),
            Type::IntArray(size) => {
                BasicMetadataTypeEnum::ArrayType(context.i32_type().array_type(size as u32))
            }
            Type::IntPtr => BasicMetadataTypeEnum::PointerType(
                context.i32_type().ptr_type(inkwell::AddressSpace::Generic),
            ),
        }
    }
}

impl Ast {
    /// Turn the source code to AST,
    /// which can be serialized to json.
    pub fn parse<T>(source_code: T) -> Result<Vec<Self>, Error>
    where
        T: Borrow<str>,
    {
        match CParser::parse(Rule::program, source_code.borrow()) {
            Ok(mut root) => {
                let root = root.next().unwrap();
                let mut ast = vec![];
                visit_program(root, &mut ast);
                Ok(ast)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn new(position: (usize, usize), info: ASTInfo) -> Self {
        Self { position, info }
    }
}

fn visit_program(pair: Pair<'_, Rule>, ast: &mut Vec<Ast>) {
    assert_eq!(pair.as_rule(), Rule::program);
    for node in pair.into_inner() {
        match node.as_rule() {
            Rule::func_declaration => {
                visit_func_declaration(node, ast);
            }
            Rule::var_declaration => visit_var_declaration(node, ast),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
}

fn visit_func_declaration(pair: Pair<'_, Rule>, ast: &mut Vec<Ast>) {
    let position = pair.as_span().start_pos().line_col();
    let mut children = pair.into_inner();
    let type_spec = visit_type_spec(children.next().unwrap());
    let id = visit_id(children.next().unwrap());
    let params = visit_params(children.next().unwrap());
    let block_stmt = visit_block_stmt(children.next().unwrap());

    ast.push(Ast::new(
        position,
        ASTInfo::FunctionDec(type_spec, id, params, Box::new(block_stmt)),
    ));
}

fn visit_var_declaration(pair: Pair<'_, Rule>, ast: &mut Vec<Ast>) {
    let position = pair.as_span().start_pos().line_col();
    let mut children = pair.into_inner();
    let mut type_spec = visit_type_spec(children.next().unwrap());
    let id = visit_id(children.next().unwrap());

    for child in children {
        match child.as_rule() {
            Rule::int_literal => {
                let size = visit_int_literal(child) as usize;
                type_spec = Type::IntArray(size);
            }
            _ => unreachable!(),
        }
    }
    ast.push(Ast::new(position, ASTInfo::VariableDec(type_spec, id)));
}

fn visit_int_literal(pair: Pair<'_, Rule>) -> i32 {
    let child = pair.into_inner().next().unwrap();
    match child.as_rule() {
        Rule::bin_literal => i32::from_str_radix(child.as_str(), 2).unwrap(),
        Rule::oct_literal => i32::from_str_radix(child.as_str(), 8).unwrap(),
        Rule::dec_literal => i32::from_str_radix(child.as_str(), 10).unwrap(),
        Rule::hex_literal => i32::from_str_radix(child.as_str(), 16).unwrap(),
        _ => unreachable!(),
    }
}
fn visit_type_spec(pair: Pair<'_, Rule>) -> Type {
    let child = pair.into_inner().next().unwrap();
    match child.as_rule() {
        Rule::int => Type::Int,
        Rule::void => Type::Void,
        _ => unreachable!(),
    }
}

fn visit_id(pair: Pair<'_, Rule>) -> String {
    pair.as_str().to_string()
}

fn visit_params(pair: Pair<'_, Rule>) -> Vec<(Type, String)> {
    let mut params = vec![];
    for node in pair.into_inner() {
        if Rule::param == node.as_rule() {
            params.push(visit_param(node));
        }
    }
    params
}

fn visit_param(pair: Pair<'_, Rule>) -> (Type, String) {
    let mut children = pair.into_inner();
    let mut type_spec = visit_type_spec(children.next().unwrap());
    let id = visit_id(children.next().unwrap());
    if let Some(x) = children.next() && x.as_rule() == Rule::pointer {
        type_spec = Type::IntPtr;
    }
    (type_spec, id)
}
fn visit_block_stmt(pair: Pair<'_, Rule>) -> Ast {
    let position = pair.as_span().start_pos().line_col();
    let children = pair.into_inner();
    let mut vars = vec![];
    let mut statements = vec![];
    for node in children {
        match node.as_rule() {
            Rule::var_declaration => visit_var_declaration(node, &mut vars),
            Rule::statement => visit_statement(node, &mut statements),
            _ => unreachable!(),
        }
    }
    Ast::new(position, ASTInfo::BlockStmt(vars, statements))
}

fn visit_statement(pair: Pair<'_, Rule>, ast: &mut Vec<Ast>) {
    let position = pair.as_span().start_pos().line_col();
    let children = pair.into_inner().next().unwrap();
    match children.as_rule() {
        Rule::block_stmt => {
            ast.push(visit_block_stmt(children));
        }
        Rule::expression_stmt => {
            let children = children.into_inner();
            for node in children {
                match node.as_rule() {
                    Rule::expression => {
                        ast.push(visit_expression(node));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Rule::selection_stmt => {
            let children = children.into_inner();
            let mut is_if = true;
            let mut condition: Option<Box<Ast>> = None;
            let mut if_statement: Vec<Ast> = vec![];
            let mut else_statement: Vec<Ast> = vec![];

            for node in children {
                match node.as_rule() {
                    Rule::expression => {
                        condition = Some(Box::new(visit_expression(node)));
                    }
                    Rule::statement if is_if => {
                        visit_statement(node, &mut if_statement);
                        is_if = false;
                    }
                    Rule::statement if !is_if => {
                        visit_statement(node, &mut else_statement);
                    }
                    _ => unreachable!(),
                }
            }
            let statement = Ast::new(
                position,
                ASTInfo::SelectionStmt(
                    condition.unwrap(),
                    Box::new(if_statement.into_iter().next().unwrap()),
                    else_statement.into_iter().next().map(Box::new),
                ),
            );

            ast.push(statement);
        }
        Rule::iteration_stmt => {
            let children = children.into_inner();
            let mut condition: Option<Box<Ast>> = None;
            let mut loop_statement: Vec<Ast> = vec![];

            for node in children {
                match node.as_rule() {
                    Rule::expression => condition = Some(Box::new(visit_expression(node))),
                    Rule::statement => {
                        visit_statement(node, &mut loop_statement);
                    }
                    _ => unreachable!(),
                }
            }

            let statement = ASTInfo::IterationStmt(
                condition.unwrap(),
                Box::new(loop_statement.into_iter().next().unwrap()),
            );
            ast.push(Ast::new(position, statement));
        }
        Rule::return_stmt => {
            let children = children.into_inner();
            let mut expression: Option<Box<Ast>> = None;
            for node in children {
                match node.as_rule() {
                    Rule::expression => expression = Some(Box::new(visit_expression(node))),
                    _ => unreachable!(),
                }
            }

            let statement = ASTInfo::ReturnStmt(expression);
            ast.push(Ast::new(position, statement));
        }
        _ => unreachable!(),
    }
}

fn visit_expression(mut pair: Pair<'_, Rule>) -> Ast {
    if pair.as_rule() == Rule::expression {
        pair = pair.into_inner().next().unwrap();
    }
    match pair.as_rule() {
        Rule::assignment_expr => visit_assignment_expr(pair),
        Rule::unary_expr => visit_unary_expr(pair),
        _ => visit_binary_expr(pair),
    }
}

fn visit_unary_expr(pair: Pair<'_, Rule>) -> Ast {
    let position = pair.as_span().start_pos().line_col();
    let child = pair.into_inner().next().unwrap();
    match child.as_rule() {
        Rule::var => visit_var(child),
        Rule::int_literal => Ast::new(position, ASTInfo::IntLiteral(visit_int_literal(child))),
        Rule::call_expr => visit_call_expr(child),
        Rule::bracket_expr => visit_bracket_expr(child),
        _ => unreachable!(),
    }
}

fn visit_bracket_expr(pair: Pair<'_, Rule>) -> Ast {
    let mut children = pair.into_inner();
    loop {
        let child = children.next().unwrap();
        if child.as_rule() == Rule::expression {
            return visit_expression(child);
        }
    }
}

fn visit_call_expr(pair: Pair<'_, Rule>) -> Ast {
    let position = pair.as_span().start_pos().line_col();
    let mut children = pair.into_inner();
    let id = visit_id(children.next().unwrap());
    let mut args = vec![];
    visit_args(children.next().unwrap(), &mut args);
    Ast::new(position, ASTInfo::CallExpr(id, args))
}

fn visit_args(pair: Pair<'_, Rule>, args: &mut Vec<Ast>) {
    let children = pair.into_inner();
    for node in children {
        if node.as_rule() == Rule::expression {
            args.push(visit_expression(node));
        }
    }
}

fn visit_assignment_expr(pair: Pair<'_, Rule>) -> Ast {
    let position = pair.as_span().start_pos().line_col();
    let mut children = pair.into_inner();
    let var = visit_var(children.next().unwrap());
    children.next();
    let expression = visit_expression(children.next().unwrap());
    Ast::new(
        position,
        ASTInfo::AssignmentExpr(Box::new(var), Box::new(expression)),
    )
}

fn visit_var(pair: Pair<'_, Rule>) -> Ast {
    let position = pair.as_span().start_pos().line_col();
    let mut children = pair.into_inner();
    let id = children.next().unwrap().as_str().to_string();
    let mut expression = None;
    for node in children {
        if node.as_rule() == Rule::expression {
            expression = Some(Box::new(visit_expression(node)));
        }
    }
    Ast::new(position, ASTInfo::Variable(id, expression))
}

fn visit_binary_expr(pair: Pair<'_, Rule>) -> Ast {
    let position = pair.as_span().start_pos().line_col();
    let mut children = pair.into_inner();
    let mut lhs = visit_expression(children.next().unwrap());

    while let Some(mut expr) = children.next() {
        let op = match expr.as_rule() {
            Rule::op_ge => Operand::Ge,
            Rule::op_le => Operand::Le,
            Rule::op_gt => Operand::Gt,
            Rule::op_lt => Operand::Lt,
            Rule::op_eq => Operand::Eq,
            Rule::op_ne => Operand::Ne,
            Rule::op_add => Operand::Add,
            Rule::op_sub => Operand::Sub,
            Rule::op_mul => Operand::Mul,
            Rule::op_div => Operand::Div,
            Rule::op_mod => Operand::Mod,
            Rule::op_rshift => Operand::RShift,
            Rule::op_lshift => Operand::LShift,
            Rule::op_bit_and => Operand::Band,
            Rule::op_bit_xor => Operand::Bxor,
            Rule::op_bit_or => Operand::Bor,
            Rule::op_or => Operand::Lor,
            Rule::op_and => Operand::Land,
            _ => unreachable!(),
        };
        expr = children.next().unwrap();
        let rhs = visit_expression(expr);
        lhs = Ast::new(
            position,
            ASTInfo::BinaryExpr(op, Box::new(lhs), Box::new(rhs)),
        );
    }
    lhs
}

#[cfg(test)]
mod test_parse {
    use pest::iterators::Pair;
    use pest::Parser;
    use std::fs::File;
    use std::io::Read;

    fn dfs(tabs: &mut Vec<bool>, pair: Pair<'_, super::Rule>) {
        let mut pair = pair.into_inner();
        let mut current = pair.next();
        let mut next = pair.next();
        while let Some(i) = current {
            for tab in tabs.iter() {
                if *tab {
                    print!("│   ");
                } else {
                    print!("    ");
                }
            }
            if next.is_some() {
                print!("├── ");
                tabs.push(true);
            } else {
                print!("└── ");
                tabs.push(false);
            }
            println!("{:?}: {}", i.as_rule(), i.as_str());

            dfs(tabs, i);
            tabs.pop();

            current = next;
            next = pair.next();
        }
    }

    /// print the parse tree, like command tree's style
    pub fn parse_tree_visible(parse_tree: Pair<'_, super::Rule>) {
        dfs(&mut vec![], parse_tree);
    }

    #[test]
    fn parse_tree_test() {
        let mut f = File::open("test/ok/test.c").unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        let root = super::CParser::parse(super::Rule::program, &buf)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(root.as_rule(), super::Rule::program);
        parse_tree_visible(root);
    }
    #[test]
    fn ast_test() {
        let mut f = File::open("test/ok/test.c").unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        let ast = super::Ast::parse(buf);
        for i in &ast {
            println!("{:?}", i);
        }
    }
}
