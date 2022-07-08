use crate::error::{Error, ErrorType, Result};
use crate::parser::{ASTInfo, Ast, Operand, Type};
use either::Either;
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    passes::PassManager,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    types::{BasicMetadataTypeEnum, BasicType},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    IntPredicate, OptimizationLevel,
};
use std::{borrow::Borrow, collections::HashMap, path::Path};

pub struct CodeBuilder<'ctx> {
    /// A Context is a container for all LLVM entities including Modules.
    context: &'ctx Context,
    /// llvm `Module`: Each module directly contains a list of globals variables,
    /// a list of functions, a list of libraries (or other modules) this module depends on,
    /// a symbol table, and various data about the target's characteristics.
    module: Module<'ctx>,
    /// This provides a uniform API for creating instructions and inserting
    /// them into a basic block: either at the end of a BasicBlock,
    /// or at a specific iterator location in a block.
    builder: Builder<'ctx>,

    /// Global variables. Map variables' name to it's type and pointer.
    global_variables: HashMap<String, (Type, PointerValue<'ctx>)>,
    /// Global functions.  Map functions' name to it's type and pointer.
    global_functions: HashMap<String, (Type, FunctionValue<'ctx>)>,
    /// Local variables. It represents the nesting of scopes.
    variables_stack: Vec<HashMap<String, (Type, PointerValue<'ctx>)>>,
    /// The function that code builder is generating.
    current_function: Option<(Type, FunctionValue<'ctx>)>,
    /// For optimize
    fpm: Option<PassManager<FunctionValue<'ctx>>>,
}

impl<'ctx> CodeBuilder<'ctx> {
    pub fn new<T>(context: &'ctx Context, name: T, ast: &Vec<Ast>, opt: bool) -> Result<Self>
    where
        T: Borrow<str>,
    {
        let builder = context.create_builder();
        let module = context.create_module(name.borrow());

        // Create FPM
        let mut fpm = None;
        if opt {
            let temp = PassManager::create(&module);

            temp.add_instruction_combining_pass();
            temp.add_reassociate_pass();
            temp.add_gvn_pass();
            temp.add_cfg_simplification_pass();
            temp.add_promote_memory_to_register_pass();

            temp.initialize();
            fpm = Some(temp)
        }

        let mut codegen = Self {
            context,
            module,
            builder,
            global_variables: HashMap::new(),
            variables_stack: Vec::new(),
            global_functions: HashMap::new(),
            current_function: None,
            fpm,
        };

        codegen.generate(ast)?;
        Ok(codegen)
    }

    /// Build llvm-ir assembly file
    pub fn build_llvmir(&self, path: &Path) {
        self.module.print_to_file(path).unwrap();
    }

    /// Build native assembly file in this machine
    pub fn build_asm(&self, path: &Path) {
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        let triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        let target = Target::from_triple(&triple).unwrap();
        let machine = target
            .create_target_machine(
                &triple,
                &cpu,
                &features,
                OptimizationLevel::None,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        // write assembly code
        machine
            .write_to_file(&self.module, FileType::Assembly, path)
            .unwrap();
    }

    fn generate(&mut self, ast: &Vec<Ast>) -> Result<()> {
        let input = self.context.i32_type().fn_type(&[], false);
        let input = self
            .module
            .add_function("input", input, Some(Linkage::External));
        self.global_functions
            .insert("input".to_string(), (Type::Int, input));

        let output = self
            .context
            .void_type()
            .fn_type(&[self.context.i32_type().into()], false);
        let output = self
            .module
            .add_function("output", output, Some(Linkage::External));
        self.global_functions
            .insert("output".to_string(), (Type::Void, output));

        for i in ast {
            match &i.info {
                ASTInfo::FunctionDec(type_, name, params, body) => {
                    self.gen_function(i.position, type_, name, params, body)?
                }
                ASTInfo::VariableDec(type_, name) => {
                    self.gen_global_variable(i.position, type_, name)?
                }
                _ => panic!(),
            }
        }
        Ok(())
    }

    fn gen_global_variable(
        &mut self,
        position: (usize, usize),
        type_: &Type,
        name: &str,
    ) -> Result<()> {
        if self.global_variables.contains_key(name) || self.global_functions.contains_key(name) {
            Err(Error::new(position, ErrorType::VariableRedefinition))?
        }
        let v = self
            .module
            .add_global(type_.to_llvm_basic_type(self.context), None, name);
        v.set_initializer(&self.context.i32_type().const_int(0, false));
        self.global_variables
            .insert(name.to_string(), (*type_, v.as_pointer_value()));
        Ok(())
    }

    fn gen_function(
        &mut self,
        position: (usize, usize),
        type_: &Type,
        name: &str,
        params: &[(Type, String)],
        body: &Ast,
    ) -> Result<()> {
        if self.global_variables.contains_key(name) || self.global_functions.contains_key(name) {
            Err(Error::new(position, ErrorType::FunctionRedefinition))?
        }

        let param_types: Vec<BasicMetadataTypeEnum<'ctx>> = params
            .iter()
            .map(|(param_type, _)| param_type.to_llvm_basic_metadata_type(self.context))
            .collect();
        let ty = match type_ {
            Type::Void => self.context.void_type().fn_type(&param_types[..], false),
            other => other
                .to_llvm_basic_type(self.context)
                .fn_type(&param_types[..], false),
        };

        let function = self.module.add_function(name, ty, None);
        self.global_functions
            .insert(name.to_string(), (*type_, function));
        let basic_block = self.context.append_basic_block(function, "entry");

        let mut p = HashMap::new();
        for (index, arg) in function.get_param_iter().enumerate() {
            let (arg_type, arg_name) = &params[index];
            arg.set_name(arg_name);

            self.builder.position_at_end(basic_block);

            // alloc variable on stack
            let ptr = self
                .builder
                .build_alloca(arg_type.to_llvm_basic_type(self.context), "");
            self.builder
                .build_store(ptr, function.get_nth_param(index as u32).unwrap());

            p.insert(arg_name.clone(), (*arg_type, ptr));
        }
        self.variables_stack.push(p);
        self.current_function = Some((*type_, function));

        self.builder.position_at_end(basic_block);
        self.gen_block_stmt(body)?;
        if self.no_terminator() {
            self.builder.build_return(None);
        }

        self.variables_stack.pop();

        // Optimize on function level
        if let Some(fpm) = &self.fpm {
            fpm.run_on(&function);
        }
        Ok(())
    }

    fn gen_block_stmt(&mut self, ast: &Ast) -> Result<()> {
        let info = &ast.info;

        if let ASTInfo::BlockStmt(variables, statements) = info {
            self.variables_stack.push(HashMap::new());
            for var in variables {
                let var_position = var.position;
                let var_info = &var.info;
                if let ASTInfo::VariableDec(type_, name) = var_info {
                    if self.variables_stack.last().unwrap().contains_key(name) {
                        Err(Error::new(var_position, ErrorType::VariableRedefinition))?
                    };
                    let v = self
                        .builder
                        .build_alloca(type_.to_llvm_basic_type(self.context), name);
                    // 对于int a[len]这样的声明, 我们转换成int* a进行后续的使用.
                    if let Type::IntArray(_) = type_ {
                        let pv = self.builder.build_alloca(
                            self.context
                                .i32_type()
                                .ptr_type(inkwell::AddressSpace::Generic),
                            name,
                        );
                        let value = unsafe {
                            self.builder.build_in_bounds_gep(
                                v,
                                &[
                                    self.context.i32_type().const_int(0, false),
                                    self.context.i32_type().const_int(0, false),
                                ],
                                name,
                            )
                        };
                        self.builder.build_store(pv, value);
                        self.variables_stack
                            .last_mut()
                            .unwrap()
                            .insert(name.clone(), (Type::IntPtr, pv));
                    } else {
                        self.variables_stack
                            .last_mut()
                            .unwrap()
                            .insert(name.clone(), (*type_, v));
                    }
                }
            }

            for stmt in statements {
                self.gen_statement(stmt)?;
            }
            self.variables_stack.pop();
        }
        Ok(())
    }

    fn gen_statement(&mut self, stmt: &Ast) -> Result<()> {
        match &stmt.info {
            ASTInfo::BlockStmt(_, _) => self.gen_block_stmt(stmt)?,
            ASTInfo::SelectionStmt(cond, then_stmt, else_stmt) => {
                let comparison = self.gen_expression(cond)?.1.into_int_value();
                let comparison = self.builder.build_int_truncate(
                    comparison,
                    self.context.bool_type(),
                    "condition",
                );
                let current_block = self.builder.get_insert_block().unwrap();

                let then_block = self
                    .context
                    .insert_basic_block_after(current_block, "then_block");

                let destination_block = self
                    .context
                    .insert_basic_block_after(then_block, "if_dest_block");
                match else_stmt {
                    Some(else_stmt) => {
                        let else_block = self
                            .context
                            .prepend_basic_block(destination_block, "else_block");

                        self.builder
                            .build_conditional_branch(comparison, then_block, else_block);
                        self.builder.position_at_end(then_block);
                        self.gen_statement(then_stmt)?;
                        if self.no_terminator() {
                            self.builder.build_unconditional_branch(destination_block);
                        }

                        self.builder.position_at_end(else_block);
                        self.gen_statement(else_stmt)?;
                        if self.no_terminator() {
                            self.builder.build_unconditional_branch(destination_block);
                        }
                    }
                    None => {
                        self.builder.build_conditional_branch(
                            comparison,
                            then_block,
                            destination_block,
                        );
                        self.builder.position_at_end(then_block);
                        self.gen_statement(then_stmt)?;
                        if self.no_terminator() {
                            self.builder.build_unconditional_branch(destination_block);
                        }
                    }
                };

                self.builder.position_at_end(destination_block);
            }
            ASTInfo::IterationStmt(cond, loop_stmt) => {
                let current_block = self.builder.get_insert_block().unwrap();
                let loop_head = self
                    .context
                    .insert_basic_block_after(current_block, "loop_head");
                self.builder.build_unconditional_branch(loop_head);

                let loop_body = self
                    .context
                    .insert_basic_block_after(loop_head, "loop_body");
                let destination_block = self
                    .context
                    .insert_basic_block_after(loop_body, "loop_dest_block");

                self.builder.position_at_end(loop_head);
                let comparison = self.gen_expression(cond)?.1.into_int_value();
                let comparison = self.builder.build_int_truncate(
                    comparison,
                    self.context.bool_type(),
                    "condition",
                );
                self.builder
                    .build_conditional_branch(comparison, loop_body, destination_block);

                self.builder.position_at_end(loop_body);
                self.gen_statement(loop_stmt)?;
                self.builder.build_unconditional_branch(loop_head);

                self.builder.position_at_end(destination_block);
            }
            ASTInfo::ReturnStmt(ret_value) => {
                let func_return_type = self.current_function.unwrap().0;
                match ret_value {
                    Some(ast) => {
                        let (type_, value) = self.gen_expression(ast)?;
                        if type_ == func_return_type {
                            self.builder.build_return(Some(&value));
                        } else {
                            Err(Error::new(ast.position, ErrorType::MismatchedTypeFunction))?
                        }
                    }
                    None => {
                        if func_return_type == Type::Void {
                            self.builder.build_return(None);
                        } else {
                            Err(Error::new(stmt.position, ErrorType::MismatchedTypeFunction))?
                        }
                    }
                }
            }
            ASTInfo::AssignmentExpr(var, expr) => {
                self.gen_assignment_expr(var, expr)?;
            }
            ASTInfo::BinaryExpr(op, lhs, rhs) => {
                self.gen_binary_expr(op, lhs, rhs)?;
            }
            ASTInfo::CallExpr(name, arguments) => {
                self.gen_function_call(stmt.position, name, arguments)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn gen_expression(&self, ast: &Ast) -> Result<(Type, BasicValueEnum)> {
        match &ast.info {
            ASTInfo::AssignmentExpr(var, expr) => self.gen_assignment_expr(var, expr),
            ASTInfo::BinaryExpr(op, lhs, rhs) => self.gen_binary_expr(op, lhs, rhs),
            ASTInfo::CallExpr(name, arguments) => {
                // 在expression上下文中不应该返回void
                let r = self.gen_function_call(ast.position, name, arguments);
                if r.is_ok() && r.as_ref().unwrap().0 == Type::Void {
                    Err(Error::new(ast.position, ErrorType::ExpressionVoidType))?
                }
                r
            }
            ASTInfo::Variable(name, index) => {
                let (type_, ptr) =
                    self.gen_variable(ast.position, name, &index.as_ref().map(|x| x.as_ref()))?;
                let value = self.builder.build_load(ptr, "");
                Ok((type_, value))
            }
            ASTInfo::IntLiteral(value) => Ok((
                Type::Int,
                self.context
                    .i32_type()
                    .const_int(*value as u64, true)
                    .as_basic_value_enum(),
            )),
            _ => unreachable!(),
        }
    }

    fn gen_binary_expr(
        &self,
        op: &Operand,
        left: &Ast,
        right: &Ast,
    ) -> Result<(Type, BasicValueEnum)> {
        let (lhs, rhs) = (self.gen_expression(left)?.1, self.gen_expression(right)?.1);
        let lhs = match lhs {
            BasicValueEnum::IntValue(i) => i,
            BasicValueEnum::PointerValue(p) => {
                self.builder
                    .build_ptr_to_int(p, self.context.i32_type(), "")
            }
            _ => unreachable!(),
        };
        let rhs = match rhs {
            BasicValueEnum::IntValue(i) => i,
            BasicValueEnum::PointerValue(p) => {
                self.builder
                    .build_ptr_to_int(p, self.context.i32_type(), "")
            }
            _ => unreachable!(),
        };

        let value = match op {
            Operand::Add => self.builder.build_int_add(lhs, rhs, ""),
            Operand::Sub => self.builder.build_int_sub(lhs, rhs, ""),
            Operand::Mul => self.builder.build_int_mul(lhs, rhs, ""),
            Operand::Div => self.builder.build_int_signed_div(lhs, rhs, ""),
            Operand::Mod => self.builder.build_int_signed_rem(lhs, rhs, ""),
            Operand::Ge => self
                .builder
                .build_int_compare(IntPredicate::SGE, lhs, rhs, ""),
            Operand::Le => self
                .builder
                .build_int_compare(IntPredicate::SLE, lhs, rhs, ""),
            Operand::Gt => self
                .builder
                .build_int_compare(IntPredicate::SGT, lhs, rhs, ""),
            Operand::Lt => self
                .builder
                .build_int_compare(IntPredicate::SLT, lhs, rhs, ""),
            Operand::Eq => self
                .builder
                .build_int_compare(IntPredicate::EQ, lhs, rhs, ""),
            Operand::Ne => self
                .builder
                .build_int_compare(IntPredicate::NE, lhs, rhs, ""),
            Operand::Band => self.builder.build_and(lhs, rhs, ""),
            Operand::Bor => self.builder.build_or(lhs, rhs, ""),
            Operand::Bxor => self.builder.build_xor(lhs, rhs, ""),
            Operand::Land => {
                let a = self.builder.build_and(lhs, rhs, "");
                let b = self.context.i32_type().const_int(0, false);
                self.builder.build_int_compare(IntPredicate::NE, a, b, "")
            }
            Operand::Lor => {
                let a = self.builder.build_or(lhs, rhs, "");
                let b = self.context.i32_type().const_int(0, false);
                self.builder.build_int_compare(IntPredicate::NE, a, b, "")
            }
            Operand::LShift => self.builder.build_left_shift(lhs, rhs, ""),
            Operand::RShift => self.builder.build_right_shift(lhs, rhs, true, ""),
        };
        // 其实，更C语言的做法是在运算符两边类型不同的时候进行隐式转换
        // 不过，我这里将所有的类型都转换成了i32类型
        let value = if value.get_type().get_bit_width() != 32 {
            self.builder
                .build_int_z_extend_or_bit_cast(value, self.context.i32_type(), "")
                .as_basic_value_enum()
        } else {
            value.as_basic_value_enum()
        };

        Ok((Type::Int, value))
    }

    fn gen_function_call(
        &self,
        position: (usize, usize),
        name: &str,
        arguments: &Vec<Ast>,
    ) -> Result<(Type, BasicValueEnum)> {
        let mut args = Vec::new();
        for argument in arguments {
            let arg = self.gen_expression(argument)?.1;
            args.push(arg.into())
        }
        let function = self.global_functions.get(name);
        match function {
            Some((type_, function)) => {
                let return_value = self.builder.build_call(*function, &args[..], name);
                match return_value.try_as_basic_value() {
                    Either::Left(value) => {
                        if value.get_type() == type_.to_llvm_basic_type(self.context) {
                            Ok((*type_, value))
                        } else {
                            Err(Error::new(position, ErrorType::MismatchedType))?
                        }
                    }
                    Either::Right(_) => Ok((
                        Type::Void,
                        self.context
                            .i32_type()
                            .const_int(0, false)
                            .as_basic_value_enum(),
                    )),
                }
            }
            None => Err(Error::new(position, ErrorType::FunctionNotDefined))?,
        }
    }

    fn gen_assignment_expr(&self, var: &Ast, expr: &Ast) -> Result<(Type, BasicValueEnum)> {
        let var_info = &var.info;
        if let ASTInfo::Variable(name, index) = var_info {
            let (type_left, ptr) =
                self.gen_variable(var.position, name, &index.as_ref().map(|x| x.as_ref()))?;
            let (type_right, value) = self.gen_expression(expr)?;
            if type_left == type_right {
                self.builder.build_store(ptr, value);
                Ok((type_left, value.as_basic_value_enum()))
            } else {
                Err(Error::new(var.position, ErrorType::MismatchedType))?
            }
        } else {
            unreachable!()
        }
    }

    fn gen_variable(
        &self,
        position: (usize, usize),
        name: &str,
        index: &Option<&Ast>,
    ) -> Result<(Type, PointerValue)> {
        let (type_, ptr) = self.get_name_ptr(position, name)?;
        match type_ {
            Type::Int => Ok((type_, ptr)),
            Type::Void => Err(Error::new(position, ErrorType::ExpressionVoidType))?,
            Type::IntPtr => {
                if let Some(index) = index {
                    let (index_type, index) = self.gen_expression(index)?;
                    if index_type == Type::Int {
                        let ptr = self.builder.build_load(ptr, "").into_pointer_value();
                        unsafe {
                            let ptr = self.builder.build_in_bounds_gep(
                                ptr,
                                &[index.into_int_value()],
                                "",
                            );
                            Ok((Type::Int, ptr))
                        }
                    } else {
                        Err(Error::new(position, ErrorType::IndexNotInt))?
                    }
                } else {
                    Ok((type_, ptr))
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_name_ptr(&self, position: (usize, usize), name: &str) -> Result<(Type, PointerValue)> {
        for domain in self.variables_stack.iter().rev() {
            if let Some(ptr) = domain.get(name) {
                return Ok(*ptr);
            }
        }
        if let Some(ptr) = self.global_variables.get(name) {
            return Ok(*ptr);
        }
        Err(Error::new(position, ErrorType::VariableNotDefined))?
    }

    fn no_terminator(&self) -> bool {
        self.builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
    }
}

#[cfg(test)]
mod test_parse {
    use std::{
        fs::{self, File},
        io::Read,
        os::unix::prelude::OsStringExt,
        path::Path,
    };

    use inkwell::context::Context;

    use super::CodeBuilder;

    fn codegen_ok_test(ok_path: &Path) {
        for source in fs::read_dir(ok_path).unwrap() {
            let source = source.unwrap();
            if source.file_type().unwrap().is_file()
                && source.file_name().into_vec().ends_with(b".c")
            {
                let mut file = File::open(source.path()).unwrap();

                println!("Test source code file {:?}", source);
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();

                let ast = super::Ast::parse(buf).unwrap();
                let context = Context::create();
                let codegen = CodeBuilder::new(&context, "test", &ast, false)
                    .expect("Source code file test failed");
                codegen.build_llvmir(Path::new("test.ll"));
            }
        }
    }

    #[test]
    fn codegen_test() {
        codegen_ok_test(Path::new("test/algorithm/"));
        codegen_ok_test(Path::new("test/ok/"));
        codegen_ok_test(Path::new("test/with_output/"));
    }
}
