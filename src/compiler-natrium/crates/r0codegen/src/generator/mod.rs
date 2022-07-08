mod util;

use crate::{
    code::{BasicBlock, JumpInst},
    err::{CompileError, CompileErrorKind, WithSpan},
    scope::{Scope, Symbol, SymbolIdGenerator},
    ty::{FuncTy, Ty},
};
use ast::FuncStmt;
use bit_set::BitSet;
use indexmap::{IndexMap, IndexSet};
use r0syntax::{
    ast,
    span::Span,
    util::Mut,
    util::{MutWeak, P},
};
use r0vm::{opcodes::Op, s0};
use smol_str::SmolStr;
use std::cell::RefCell;
use util::BBArranger;

static RET_VAL_KEY: &str = "$ret";
static FUNC_VAL_KEY: &str = "$func";

type CompileResult<T> = std::result::Result<T, CompileError>;
type BB = usize;

pub fn compile(tree: &ast::Program) -> CompileResult<s0::S0> {
    let global_sym_gen = RefCell::new(SymbolIdGenerator::new());
    let mut global_scope = Scope::new(&global_sym_gen);
    let global_entries = Mut::new(GlobalEntries {
        functions: indexmap::indexset! {"_start".into()},
        values: IndexMap::new(),
    });

    let mut funcs = vec![];

    create_lib_func(&mut global_scope);

    for decl in &tree.decls {
        let (var_id, ty) = add_decl_scope(decl, &mut global_scope)?;
        global_entries
            .borrow_mut()
            .values
            .insert(var_id, vec![0u8; ty.size()]);
    }
    {
        global_entries
            .borrow_mut()
            .functions
            .insert("_start".into());
    }
    for func in &tree.funcs {
        if !global_entries
            .borrow_mut()
            .functions
            .insert(func.name.name.clone())
        {
            return Err(CompileError(
                CompileErrorKind::DuplicateSymbol(func.name.name.as_str().into()),
                Some(func.name.span),
            ));
        }
        let func = compile_func(func, &mut global_scope, global_entries.clone())?;
        funcs.push(func);
    }

    let start = compile_start_func(tree, &mut global_scope, global_entries.clone())?;
    funcs.insert(0, start);

    let mut global_entries = Mut::take_inner(global_entries).unwrap_or_else(|_| panic!());

    let s0 = s0::S0 {
        globals: global_entries
            .values
            .drain(..)
            .map(|(_, val)| s0::GlobalValue {
                is_const: false,
                bytes: val,
            })
            .collect(),
        functions: funcs,
    };

    Ok(s0)
}

fn create_lib_func(scope: &mut Scope) {
    scope.insert(
        "putint".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![P(Ty::Int)],
                ret: P(Ty::Void),
            }),
            true,
        ),
    );
    scope.insert(
        "putstr".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![P(Ty::Int)],
                ret: P(Ty::Void),
            }),
            true,
        ),
    );
    scope.insert(
        "putdouble".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![P(Ty::Double)],
                ret: P(Ty::Void),
            }),
            true,
        ),
    );
    scope.insert(
        "putchar".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![P(Ty::Int)],
                ret: P(Ty::Void),
            }),
            true,
        ),
    );
    scope.insert(
        "putln".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![],
                ret: P(Ty::Void),
            }),
            true,
        ),
    );
    scope.insert(
        "getchar".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![],
                ret: P(Ty::Int),
            }),
            true,
        ),
    );
    scope.insert(
        "getint".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![],
                ret: P(Ty::Int),
            }),
            true,
        ),
    );
    scope.insert(
        "getdouble".into(),
        Symbol::new(
            Ty::Func(FuncTy {
                params: vec![],
                ret: P(Ty::Double),
            }),
            true,
        ),
    );
}

struct GlobalEntries {
    functions: IndexSet<SmolStr>,
    values: IndexMap<u64, Vec<u8>>,
}

impl GlobalEntries {
    pub fn function_id(&self, func_name: &str) -> Option<u32> {
        self.functions.get_index_of(func_name).map(|x| x as u32)
    }

    pub fn value_id(&self, symbol_id: u64) -> Option<u32> {
        self.values.get_index_of(&symbol_id).map(|x| x as u32)
    }

    pub fn insert_string_literal(&mut self, s: &str, val_id: u64) -> u32 {
        self.values.insert(val_id, s.as_bytes().into());
        self.value_id(val_id).unwrap()
    }
}

fn compile_start_func(
    tree: &ast::Program,
    global_scope: &mut Scope,
    global_entries: Mut<GlobalEntries>,
) -> CompileResult<s0::FnDef> {
    let start_func = FuncStmt {
        name: ast::Ident {
            name: "_start".into(),
            span: Span::default(),
        },
        params: vec![],
        ret_ty: ast::TyDef {
            name: "void".into(),
            params: None,
            span: Span::default(),
        },
        body: ast::BlockStmt {
            span: Span::default(),
            stmts: tree
                .decls
                .iter()
                .cloned()
                .filter_map(|decl: ast::DeclStmt| {
                    Some(ast::Stmt::Expr(ast::Expr::Assign(ast::AssignExpr {
                        span: decl.span,
                        allow_assign_const: true,
                        lhs: P::new(ast::Expr::Ident(decl.name)),
                        rhs: decl.val?,
                    })))
                })
                .chain(
                    vec![
                        ast::Stmt::Expr(ast::Expr::Call(ast::CallExpr {
                            span: Span::default(),
                            func: ast::Ident {
                                span: Span::default(),
                                name: "main".into(),
                            },
                            params: vec![],
                        })),
                        ast::Stmt::Return(ast::ReturnStmt {
                            val: None,
                            span: Span::default(),
                        }),
                    ]
                    .into_iter(),
                )
                .collect(),
        },
        span: Span::default(),
    };
    let mut func = compile_func(&start_func, global_scope, global_entries)?;
    // remove the last 'ret'
    func.ins.pop();
    Ok(func)
}

macro_rules! check_type_eq {
    ($lhs:expr, $rhs:expr, $span:expr) => {
        if $lhs != $rhs {
            return Err(CompileError(
                CompileErrorKind::TypeMismatch {
                    expected: $lhs.to_string(),
                    got: Some($rhs.to_string()),
                },
                Some($span),
            ));
        }
    };
}

fn compile_func(
    func: &FuncStmt,
    global_scope: &mut Scope,
    global_entries: Mut<GlobalEntries>,
) -> CompileResult<s0::FnDef> {
    let ret_ty = P::new(get_ty(&func.ret_ty)?);

    let params = func
        .params
        .iter()
        .map(|param| Ok(P::new(get_ty_nonvoid(&param.ty)?)))
        .collect::<Result<Vec<_>, _>>()?;
    let func_ty = FuncTy {
        params,
        ret: ret_ty,
    };

    global_scope.insert(func.name.name.clone(), Symbol::new(Ty::Func(func_ty), true));
    global_entries
        .borrow_mut()
        .functions
        .insert(func.name.name.clone());

    let mut fc = FuncCodegen::new(func, global_scope, global_entries);
    fc.compile()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Place {
    Arg(u32),
    Loc(u32),
}

struct FuncCodegen<'f> {
    func: &'f FuncStmt,
    global_scope: &'f Scope<'f>,
    global_entries: Mut<GlobalEntries>,
    basic_blocks: Vec<BasicBlock>,
    break_continue_positions: Vec<(BB, BB)>,
    place_mapping: IndexMap<u64, Place>,
    arg_top: u32,
    loc_top: u32,
}

impl<'f> FuncCodegen<'f> {
    pub fn new(
        func: &'f FuncStmt,
        scope: &'f Scope<'f>,
        global_entries: Mut<GlobalEntries>,
    ) -> FuncCodegen<'f> {
        FuncCodegen {
            func,
            global_scope: scope,
            global_entries,
            basic_blocks: vec![],
            break_continue_positions: vec![],
            place_mapping: IndexMap::new(),
            arg_top: 0,
            loc_top: 0,
        }
    }

    pub fn compile(mut self) -> CompileResult<s0::FnDef> {
        self.compile_func()
    }

    fn new_bb(&mut self) -> BB {
        let bb_id = self.basic_blocks.len();
        self.basic_blocks.push(BasicBlock::new());
        bb_id
    }

    fn append_code(&mut self, bb_id: BB, code: Op) {
        if let Some(bb) = self.basic_blocks.get_mut(bb_id) {
            bb.code.push(code);
        } else {
            panic!("Non-existent basic block: {}", bb_id);
        }
    }

    fn set_jump(&mut self, bb_id: BB, jump: JumpInst) {
        if let Some(bb) = self.basic_blocks.get_mut(bb_id) {
            if matches!(bb.jump, JumpInst::Undefined) {
                bb.jump = jump;
            } else {
                panic!("Double-set jump instruction")
            }
        } else {
            panic!("Non-existent basic block: {}", bb_id);
        }
    }

    fn compile_func(mut self) -> CompileResult<s0::FnDef> {
        let mut scope = Scope::new_with_parent(self.global_scope);

        let (ret_slots, param_slots) = self.add_params(&mut scope)?;

        let start_bb = self.new_bb();

        let end_bb = self.compile_block_without_scope(&self.func.body, start_bb, &mut scope)?;

        if scope.find(RET_VAL_KEY).unwrap().ty == Ty::Void {
            self.set_jump(end_bb, JumpInst::Return);
        }

        let arrange = self.bb_arrange(start_bb)?;

        let start_offset = arrange
            .iter()
            .map(|a| (*a, &self.basic_blocks[*a]))
            .fold((0, IndexMap::new()), |(acc, mut map), (bb_id, bb)| {
                map.insert(bb_id, acc);
                let acc = acc
                    + bb.code.len()
                    + match bb.jump {
                        JumpInst::Undefined => 0,
                        JumpInst::Unreachable => 0,
                        JumpInst::Return => 1,
                        JumpInst::Jump(_) => 1,
                        JumpInst::JumpIf(..) => 2,
                    };
                (acc, map)
            })
            .1;

        let mut result_code = vec![];
        for bb in arrange {
            let bb = &mut self.basic_blocks[bb];
            result_code.append(&mut bb.code);
            match bb.jump {
                JumpInst::Return => result_code.push(Op::Ret),
                JumpInst::Jump(id) => {
                    result_code.push(Op::Br(
                        start_offset[&id] as i32 - result_code.len() as i32 - 1,
                    ));
                }
                JumpInst::JumpIf(bb_true, bb_false) => {
                    result_code.push(Op::BrTrue(
                        start_offset[&bb_true] as i32 - result_code.len() as i32 - 1,
                    ));
                    result_code.push(Op::Br(
                        start_offset[&bb_false] as i32 - result_code.len() as i32 - 1,
                    ));
                }
                _ => {}
            }
        }

        let name_global_id = {
            let name_val_id = self.global_scope.get_new_id();
            self.global_entries
                .borrow_mut()
                .insert_string_literal(&self.func.name.name, name_val_id)
        };

        Ok(s0::FnDef {
            name: name_global_id,
            ret_slots: ret_slots as u32,
            param_slots: param_slots as u32,
            loc_slots: self.loc_top,
            ins: result_code,
        })
    }

    fn bb_arrange(&self, start: BB) -> CompileResult<Vec<BB>> {
        // dbg!(&self.basic_blocks);
        let mut arr_state = BBArranger::new(&self.basic_blocks);

        arr_state
            .construct_arrangement(start)
            .with_span(self.func.name.span)?;

        let arrange = arr_state.arrange();

        Ok(arrange)
    }

    fn add_params(&mut self, scope: &mut Scope) -> CompileResult<(usize, usize)> {
        let ret_ty = get_ty(&self.func.ret_ty)?;
        let ret_size = ret_ty.size_slot();
        let ret_id = scope
            .insert(RET_VAL_KEY.into(), Symbol::new(ret_ty, false))
            .expect("Return value should be valid");
        self.place_mapping.insert(ret_id, Place::Arg(self.arg_top));
        self.arg_top += ret_size as u32;

        for param in &self.func.params {
            let param_ty = get_ty_nonvoid(&param.ty)?;
            let param_size = param_ty.size_slot();

            let param_id = scope
                .insert(
                    param.name.name.clone(),
                    Symbol::new(param_ty, param.is_const),
                )
                .ok_or_else(|| CompileError {
                    kind: CompileErrorKind::DuplicateSymbol(param.name.name.as_str().into()),
                    span: Some(param.name.span),
                })?;
            self.place_mapping
                .insert(param_id, Place::Arg(self.arg_top));
            self.arg_top += param_size as u32;
        }

        Ok((ret_size, self.arg_top as usize - ret_size))
    }

    fn get_place(&self, var_id: u64) -> Option<Place> {
        self.place_mapping.get(&var_id).cloned()
    }

    fn compile_block(
        &mut self,
        blk: &ast::BlockStmt,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        let mut block_scope = Scope::new_with_parent(scope);
        self.compile_block_without_scope(blk, bb_id, &mut block_scope)
    }

    fn compile_block_without_scope(
        &mut self,
        blk: &ast::BlockStmt,
        bb_id: BB,
        scope: &mut Scope,
    ) -> CompileResult<BB> {
        let mut cur_bb_id = bb_id;
        for stmt in &blk.stmts {
            cur_bb_id = self.compile_stmt(stmt, cur_bb_id, scope)?;
        }
        Ok(cur_bb_id)
    }

    fn compile_stmt(
        &mut self,
        stmt: &ast::Stmt,
        bb_id: BB,
        scope: &mut Scope,
    ) -> CompileResult<BB> {
        match stmt {
            ast::Stmt::Block(blk) => self.compile_block(blk, bb_id, scope),
            ast::Stmt::While(stmt) => self.compile_while(stmt, bb_id, scope),
            ast::Stmt::If(stmt) => self.compile_if(stmt, bb_id, scope),
            ast::Stmt::Expr(expr) => self.compile_expr_stmt(expr, bb_id, scope),
            ast::Stmt::Decl(stmt) => self.compile_decl(stmt, bb_id, scope),
            ast::Stmt::Return(stmt) => self.compile_return(stmt, bb_id, scope),
            ast::Stmt::Break(span) => self.compile_break(*span, bb_id, scope),
            ast::Stmt::Continue(span) => self.compile_continue(*span, bb_id, scope),
            ast::Stmt::Empty(_span) => Ok(bb_id),
        }
    }

    fn compile_expr_stmt(
        &mut self,
        expr: &ast::Expr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        let ty = self.compile_expr(expr, bb_id, scope)?;
        if ty.size_slot() > 0 {
            self.append_code(bb_id, Op::PopN(ty.size_slot() as u32));
        }
        Ok(bb_id)
    }

    fn compile_while(
        &mut self,
        stmt: &ast::WhileStmt,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        /*
         * break -v      v----------------------------------\  v- continue
         * begin --> [bb:A cond] -true--> [bb:B body bb:C] -/  /--> [bb:D next]
         *               \-false------------------------------/
         */

        let cond_bb = self.new_bb();
        let body_bb = self.new_bb();
        let next_bb = self.new_bb();

        self.break_continue_positions.push((cond_bb, next_bb));

        self.compile_expr(stmt.cond.as_ref(), cond_bb, scope)?;
        self.set_jump(bb_id, JumpInst::Jump(cond_bb));
        self.set_jump(cond_bb, JumpInst::JumpIf(body_bb, next_bb));

        let body_end_bb = self.compile_block(stmt.body.as_ref(), body_bb, scope)?;
        self.set_jump(body_end_bb, JumpInst::Jump(cond_bb));

        self.break_continue_positions.pop();

        Ok(next_bb)
    }

    fn compile_if(&mut self, stmt: &ast::IfStmt, bb_id: BB, scope: &Scope) -> CompileResult<BB> {
        /*
         * begin --> [bb:A1 cond] -true--> [bb:B1 body] -\
         *           |false                              |
         *           V                                   |
         *           [bb:A2 cond] -true--> [bb:B2 body] -|
         *           |false                              |
         *           ...                                 |
         *           V                                   |
         *           [bb:C else_body]----------------------> end
         */
        let end_bb = self.new_bb();

        self.compile_expr(stmt.cond.as_ref(), bb_id, scope)?;

        let bb_true = self.new_bb();
        let bb_true_end = self.compile_block(stmt.if_block.as_ref(), bb_true, scope)?;

        // body -> end
        self.set_jump(bb_true_end, JumpInst::Jump(end_bb));

        let bb_false = match &stmt.else_block {
            ast::IfElseBlock::None => end_bb,
            ast::IfElseBlock::If(stmt) => {
                let else_bb = self.new_bb();
                let else_end = self.compile_if(stmt.as_ref(), else_bb, scope)?;
                self.set_jump(else_end, JumpInst::Jump(end_bb));
                else_bb
            }
            ast::IfElseBlock::Block(block) => {
                let else_bb = self.new_bb();
                let else_end = self.compile_block(block.as_ref(), else_bb, scope)?;
                self.set_jump(else_end, JumpInst::Jump(end_bb));
                else_bb
            }
        };

        self.set_jump(bb_id, JumpInst::JumpIf(bb_true, bb_false));

        Ok(end_bb)
    }

    fn compile_decl(
        &mut self,
        stmt: &ast::DeclStmt,
        bb_id: BB,
        scope: &mut Scope,
    ) -> CompileResult<BB> {
        let (val_id, ty) = add_decl_scope(stmt, scope)?;
        // add value to stack
        self.place_mapping.insert(val_id, Place::Loc(self.loc_top));
        let var_size = ty.size_slot();
        self.loc_top += var_size as u32;

        if let Some(val) = stmt.val.clone() {
            let assign_expr = ast::AssignExpr {
                span: Span::default(),
                allow_assign_const: true,
                lhs: P::new(ast::Expr::Ident(stmt.name.clone())),
                rhs: val,
            };
            self.compile_assign_expr(&assign_expr, bb_id, scope)?;
        }
        Ok(bb_id)
    }

    fn compile_return(
        &mut self,
        stmt: &ast::ReturnStmt,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<BB> {
        let func_ty = self
            .global_scope
            .find(&self.func.name.name)
            .expect("Function type")
            .ty
            .get_func()
            .unwrap();
        let ret_ty = &*func_ty.ret;

        if matches!(ret_ty, Ty::Void) {
            // void return
            if stmt.val.is_some() {
                return Err(CompileError {
                    kind: CompileErrorKind::TypeMismatch {
                        expected: "void".into(),
                        got: None,
                    },
                    span: Some(stmt.span),
                });
            }
        } else {
            // non-void return
            if stmt.val.is_none() {
                return Err(CompileError {
                    kind: CompileErrorKind::TypeMismatch {
                        expected: ret_ty.to_string(),
                        got: Some(Ty::Void.to_string()),
                    },
                    span: Some(stmt.span),
                });
            } else {
                let ret_id = scope.find(RET_VAL_KEY).unwrap().id;
                let offset = self.get_place(ret_id).unwrap();

                self.append_code(bb_id, op_load_address(offset));
                let ty = self.compile_expr(stmt.val.as_deref().unwrap(), bb_id, scope)?;
                if ty != *ret_ty {
                    return Err(CompileError(
                        CompileErrorKind::TypeMismatch {
                            expected: ret_ty.to_string(),
                            got: ty.to_string().into(),
                        },
                        Some(stmt.span),
                    ));
                }
                self.append_code(bb_id, store_ty(ret_ty));
            }
        }

        self.set_jump(bb_id, JumpInst::Return);
        Ok(self.new_bb())
    }

    fn compile_break(&mut self, span: Span, bb_id: BB, _scope: &Scope) -> CompileResult<BB> {
        let (_, next) = self
            .break_continue_positions
            .last()
            .ok_or_else(|| CompileError(CompileErrorKind::NoBreakContext, Some(span)))?;
        let break_target = *next;
        self.set_jump(bb_id, JumpInst::Jump(break_target));
        Ok(self.new_bb())
    }

    fn compile_continue(&mut self, span: Span, bb_id: BB, _scope: &Scope) -> CompileResult<BB> {
        let (next, _) = self
            .break_continue_positions
            .last()
            .ok_or_else(|| CompileError(CompileErrorKind::NoContinueContext, Some(span)))?;
        let continue_target = *next;
        self.set_jump(bb_id, JumpInst::Jump(continue_target));
        Ok(self.new_bb())
    }

    fn compile_expr(&mut self, expr: &ast::Expr, bb_id: BB, scope: &Scope) -> CompileResult<Ty> {
        match expr {
            ast::Expr::Ident(expr) => self.compile_ident_expr(expr, bb_id, scope),
            ast::Expr::Assign(expr) => self.compile_assign_expr(expr, bb_id, scope),
            ast::Expr::As(expr) => self.compile_as_expr(expr, bb_id, scope),
            ast::Expr::Literal(expr) => self.compile_literal_expr(expr, bb_id, scope),
            ast::Expr::Unary(expr) => self.compile_unary_expr(expr, bb_id, scope),
            ast::Expr::Binary(expr) => self.compile_binary_expr(expr, bb_id, scope),
            ast::Expr::Call(expr) => self.compile_call_expr(expr, bb_id, scope),
        }
    }

    fn get_l_value_addr(
        &mut self,
        expr: &ast::Expr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<(Ty, bool)> {
        match expr {
            ast::Expr::Ident(i) => self.gen_ident_addr(i, bb_id, scope),
            _ => Err(CompileError(CompileErrorKind::NotLValue, Some(expr.span()))),
        }
    }

    fn gen_ident_addr(
        &mut self,
        i: &ast::Ident,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<(Ty, bool)> {
        let (sym, is_global) = scope.find_is_global(&i.name).ok_or_else(|| {
            CompileError(
                CompileErrorKind::NoSuchSymbol(i.name.to_string()),
                Some(i.span),
            )
        })?;

        if is_global {
            let global_val_id = self
                .global_entries
                .borrow()
                .value_id(sym.id)
                .expect("Reference to non-existent global value");

            self.append_code(bb_id, Op::GlobA(global_val_id));
        } else {
            let var_id = sym.id;
            self.append_code(bb_id, op_load_address(self.get_place(var_id).unwrap()));
        }
        Ok((sym.ty.clone(), sym.is_const))
    }

    fn compile_assign_expr(
        &mut self,
        expr: &ast::AssignExpr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        let (lhs_ty, is_const) = self.get_l_value_addr(expr.lhs.as_ref(), bb_id, scope)?;
        let rhs_ty = self.compile_expr(expr.rhs.as_ref(), bb_id, scope)?;

        check_type_eq!(lhs_ty, rhs_ty, expr.rhs.span());
        if !expr.allow_assign_const && is_const {
            return Err(CompileError(
                CompileErrorKind::AssignToConst,
                Some(expr.lhs.span()),
            ));
        }

        self.append_code(bb_id, store_ty(&lhs_ty));

        Ok(Ty::Void)
    }

    fn compile_binary_expr(
        &mut self,
        expr: &ast::BinaryExpr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        let lhs_ty = self.compile_expr(expr.lhs.as_ref(), bb_id, scope)?;
        let rhs_ty = self.compile_expr(expr.rhs.as_ref(), bb_id, scope)?;

        check_type_eq!(lhs_ty, rhs_ty, expr.rhs.span());

        let code = binary_op_op(expr.op, &lhs_ty).ok_or_else(|| {
            CompileError(
                CompileErrorKind::InvalidCalculation(lhs_ty.to_string()),
                Some(expr.rhs.span()),
            )
        })?;

        for code in code {
            self.append_code(bb_id, *code);
        }

        let result_ty = binary_op_result_ty(expr.op, &lhs_ty).ok_or_else(|| {
            CompileError(
                CompileErrorKind::InvalidCalculation(lhs_ty.to_string()),
                Some(expr.rhs.span()),
            )
        })?;

        Ok(result_ty)
    }

    fn compile_unary_expr(
        &mut self,
        expr: &ast::UnaryExpr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        let lhs_ty = self.compile_expr(expr.expr.as_ref(), bb_id, scope)?;

        let code = unary_op_op(expr.op, &lhs_ty).ok_or_else(|| {
            CompileError(
                CompileErrorKind::InvalidCalculation(lhs_ty.to_string()),
                Some(expr.expr.span()),
            )
        })?;

        for code in code {
            self.append_code(bb_id, *code);
        }

        let result_ty = unary_op_result_ty(expr.op, &lhs_ty).ok_or_else(|| {
            CompileError(
                CompileErrorKind::InvalidCalculation(lhs_ty.to_string()),
                Some(expr.expr.span()),
            )
        })?;

        Ok(result_ty)
    }

    fn compile_as_expr(
        &mut self,
        expr: &ast::AsExpr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        let lhs_ty = self.compile_expr(expr.val.as_ref(), bb_id, scope)?;
        let rhs_ty = get_ty_nonvoid(&expr.ty)?;

        let code = as_expr_op(&lhs_ty, &rhs_ty).ok_or_else(|| {
            CompileError(
                CompileErrorKind::InvalidCalculation(lhs_ty.to_string()),
                Some(expr.ty.span),
            )
        })?;

        for code in code {
            self.append_code(bb_id, *code);
        }

        Ok(rhs_ty)
    }

    fn compile_literal_expr(
        &mut self,
        expr: &ast::LiteralExpr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        match &expr.kind {
            ast::LiteralKind::Integer(i) => {
                self.append_code(bb_id, Op::Push(*i));
                Ok(Ty::Int)
            }
            ast::LiteralKind::Float(f) => {
                self.append_code(bb_id, Op::Push(unsafe { std::mem::transmute_copy(f) }));
                Ok(Ty::Double)
            }
            ast::LiteralKind::String(s) => {
                let val_id = scope.get_new_id();
                let glob_id = self
                    .global_entries
                    .borrow_mut()
                    .insert_string_literal(s, val_id);
                self.append_code(bb_id, Op::Push(glob_id as u64));
                Ok(Ty::Int)
            }
            ast::LiteralKind::Char(c) => {
                self.append_code(bb_id, Op::Push(*c as u64));
                Ok(Ty::Int)
            }
        }
    }

    fn compile_call_expr(
        &mut self,
        expr: &ast::CallExpr,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        let mut expr_tys = vec![];

        let func_name = &expr.func.name;
        let func_sig = self.global_scope.find(&func_name).ok_or_else(|| {
            CompileError(
                CompileErrorKind::NoSuchSymbol(func_name.to_string()),
                Some(expr.func.span),
            )
        })?;

        let func_ty = func_sig.ty.get_func().ok_or_else(|| {
            CompileError(
                CompileErrorKind::TypeMismatch {
                    expected: "function".into(),
                    got: Some(func_sig.ty.to_string()),
                },
                Some(expr.func.span),
            )
        })?;

        self.append_code(bb_id, Op::StackAlloc(func_ty.ret.size_slot() as u32));

        for sub in &expr.params {
            let ty = self.compile_expr(sub, bb_id, scope)?;
            expr_tys.push(ty);
        }

        if expr_tys.len() != func_ty.params.len() {
            return Err(CompileError(
                CompileErrorKind::FuncParamSizeMismatch(expr_tys.len(), func_ty.params.len()),
                Some(expr.span),
            ));
        }

        for ((expr_ty, param_ty), expr_span) in expr_tys
            .iter()
            .zip(func_ty.params.iter().map(|x| x.as_ref()))
            .zip(expr.params.iter().map(|x| x.span()))
        {
            check_type_eq!(expr_ty, param_ty, expr_span);
        }

        let ret_ty = func_ty.ret.as_ref().clone();

        let func_id = self.global_entries.borrow().function_id(func_name);
        if let Some(id) = func_id {
            self.append_code(bb_id, Op::Call(id));
        } else {
            let val_id = scope.get_new_id();
            let glob_id = self
                .global_entries
                .borrow_mut()
                .insert_string_literal(func_name, val_id);
            self.append_code(bb_id, Op::CallName(glob_id));
        }

        Ok(ret_ty)
    }

    fn compile_ident_expr(
        &mut self,
        expr: &ast::Ident,
        bb_id: BB,
        scope: &Scope,
    ) -> CompileResult<Ty> {
        let (ty, _) = self.gen_ident_addr(expr, bb_id, scope)?;
        self.append_code(bb_id, load_ty(&ty));
        Ok(ty)
    }
}

fn add_decl_scope(decl: &ast::DeclStmt, scope: &mut Scope) -> CompileResult<(u64, Ty)> {
    let ty = get_ty(&decl.ty)?;
    if matches!(ty, Ty::Void) {
        return Err(CompileError(
            CompileErrorKind::VoidTypeVariable,
            Some(decl.span),
        ));
    }
    let name = decl.name.name.clone();

    let symbol = Symbol::new(ty.clone(), decl.is_const);

    let symbol = scope.insert(name, symbol);
    match symbol {
        Some(u) => Ok((u, ty)),
        None => Err(CompileError {
            kind: CompileErrorKind::DuplicateSymbol(decl.name.name.as_str().into()),
            span: Some(decl.name.span),
        }),
    }
}

fn get_ty(ty: &ast::TyDef) -> CompileResult<Ty> {
    Ok(match ty.name.as_str() {
        "int" => Ty::Int,
        "double" => Ty::Double,
        "void" => Ty::Void,
        _ => {
            return Err(CompileError {
                kind: CompileErrorKind::UnknownType(ty.name.as_str().into()),
                span: Some(ty.span),
            })
        }
    })
}

fn get_ty_nonvoid(ty: &ast::TyDef) -> CompileResult<Ty> {
    Ok(match ty.name.as_str() {
        "int" => Ty::Int,
        "double" => Ty::Double,
        "void" => {
            return Err(CompileError {
                kind: CompileErrorKind::VoidTypeVariable,
                span: Some(ty.span),
            })
        }
        _ => {
            return Err(CompileError {
                kind: CompileErrorKind::UnknownType(ty.name.as_str().into()),
                span: Some(ty.span),
            })
        }
    })
}

fn op_load_address(place: Place) -> Op {
    match place {
        Place::Arg(x) => Op::ArgA(x),
        Place::Loc(x) => Op::LocA(x),
    }
}

fn load_ty(ty: &Ty) -> Op {
    match ty {
        Ty::Int => Op::Load64,
        Ty::Double => Op::Load64,
        Ty::Bool => Op::Load64,
        Ty::Addr => Op::Load64,
        Ty::Func(_) => panic!("Invalid type"),
        Ty::Void => Op::Pop,
    }
}

fn store_ty(ty: &Ty) -> Op {
    match ty {
        Ty::Int => Op::Store64,
        Ty::Double => Op::Store64,
        Ty::Bool => Op::Store64,
        Ty::Addr => Op::Store64,
        Ty::Func(_) => panic!("Invalid type"),
        Ty::Void => Op::Pop,
    }
}

fn binary_op_op(op: ast::BinaryOp, ty: &Ty) -> Option<&[Op]> {
    match ty {
        Ty::Int | Ty::Addr => Some(match op {
            ast::BinaryOp::Add => &[Op::AddI],
            ast::BinaryOp::Sub => &[Op::SubI],
            ast::BinaryOp::Mul => &[Op::MulI],
            ast::BinaryOp::Div => &[Op::DivI],
            ast::BinaryOp::Gt => &[Op::CmpI, Op::SetGt],
            ast::BinaryOp::Lt => &[Op::CmpI, Op::SetLt],
            ast::BinaryOp::Ge => &[Op::CmpI, Op::SetLt, Op::Not],
            ast::BinaryOp::Le => &[Op::CmpI, Op::SetGt, Op::Not],
            ast::BinaryOp::Eq => &[Op::CmpI, Op::Not],
            ast::BinaryOp::Neq => &[Op::CmpI],
        }),
        Ty::Double => Some(match op {
            ast::BinaryOp::Add => &[Op::AddF],
            ast::BinaryOp::Sub => &[Op::SubF],
            ast::BinaryOp::Mul => &[Op::MulF],
            ast::BinaryOp::Div => &[Op::DivF],
            ast::BinaryOp::Gt => &[Op::CmpF, Op::SetGt],
            ast::BinaryOp::Lt => &[Op::CmpF, Op::SetLt],
            ast::BinaryOp::Ge => &[Op::CmpF, Op::SetLt, Op::Not],
            ast::BinaryOp::Le => &[Op::CmpF, Op::SetGt, Op::Not],
            ast::BinaryOp::Eq => &[Op::CmpF, Op::Not],
            ast::BinaryOp::Neq => &[Op::CmpF],
        }),
        Ty::Bool | Ty::Func(_) | Ty::Void => None,
    }
}

fn unary_op_op(op: ast::UnaryOp, ty: &Ty) -> Option<&[Op]> {
    match ty {
        Ty::Int => Some(match op {
            ast::UnaryOp::Neg => &[Op::NegI],
            ast::UnaryOp::Pos => &[],
        }),
        Ty::Double => Some(match op {
            ast::UnaryOp::Neg => &[Op::NegF],
            ast::UnaryOp::Pos => &[],
        }),
        Ty::Addr | Ty::Bool | Ty::Func(_) | Ty::Void => None,
    }
}

fn as_expr_op(from_ty: &Ty, to_ty: &Ty) -> Option<&'static [Op]> {
    match from_ty {
        Ty::Int | Ty::Addr => match to_ty {
            Ty::Int | Ty::Addr => Some(&[]),
            Ty::Double => Some(&[Op::IToF]),
            Ty::Bool => Some(&[]),
            _ => None,
        },
        Ty::Double => match to_ty {
            Ty::Int => Some(&[Op::FToI]),
            Ty::Double => Some(&[]),
            Ty::Bool => Some(&[]),
            _ => None,
        },
        Ty::Bool | Ty::Func(_) | Ty::Void => None,
    }
}

fn binary_op_result_ty(op: ast::BinaryOp, ty: &Ty) -> Option<Ty> {
    match ty {
        Ty::Int | Ty::Double | Ty::Addr => match op {
            ast::BinaryOp::Add => Some(ty.clone()),
            ast::BinaryOp::Sub => Some(ty.clone()),
            ast::BinaryOp::Mul => Some(ty.clone()),
            ast::BinaryOp::Div => Some(ty.clone()),
            ast::BinaryOp::Gt => Some(Ty::Bool),
            ast::BinaryOp::Lt => Some(Ty::Bool),
            ast::BinaryOp::Ge => Some(Ty::Bool),
            ast::BinaryOp::Le => Some(Ty::Bool),
            ast::BinaryOp::Eq => Some(Ty::Bool),
            ast::BinaryOp::Neq => Some(Ty::Bool),
        },
        Ty::Bool | Ty::Func(_) | Ty::Void => None,
    }
}

fn unary_op_result_ty(_op: ast::UnaryOp, ty: &Ty) -> Option<Ty> {
    match ty {
        Ty::Int | Ty::Double => Some(ty.clone()),
        Ty::Addr | Ty::Bool | Ty::Func(_) | Ty::Void => None,
    }
}
