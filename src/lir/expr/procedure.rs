use crate::asm::{AssemblyProgram, CoreOp, A, FP, SP};
use crate::lir::{Compile, Env, Error, Expr, GetSize, GetType, Type, TypeCheck};
use std::sync::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    static ref LAMBDA_COUNT: Mutex<usize> = Mutex::new(0);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Procedure {
    mangled_name: String,
    args: Vec<(String, Type)>,
    ret: Type,
    body: Box<Expr>,
    /// Has this procedure been compiled yet?
    pub compiled: bool,
}

impl Procedure {
    /// Construct a new procedure with a given list of arguments and their types,
    /// a return type, and the body of the procedure.
    pub fn new(args: Vec<(String, Type)>, ret: Type, body: impl Into<Expr>) -> Self {
        let mut lambda_count = LAMBDA_COUNT.lock().unwrap();
        *lambda_count += 1;
        Self {
            mangled_name: format!("__LAMBDA_{lambda_count}"),
            args,
            ret,
            body: Box::new(body.into()),
            compiled: false,
        }
    }

    /// Get the mangled name of the procedure.
    /// The procedure's mangled name is used to store the procedure in the environment.
    pub fn get_name(&self) -> &str {
        &self.mangled_name
    }

    /// Push this procedure's label to the stack.
    pub fn push_label(&self, output: &mut dyn AssemblyProgram) {
        // Push the procedure label address onto the stack
        output.op(CoreOp::SetLabel(A, self.mangled_name.clone()));
        output.op(CoreOp::Push(A, 1));
    }
}

impl TypeCheck for Procedure {
    fn type_check(&self, env: &Env) -> Result<(), Error> {
        let mut new_env = env.new_scope();
        new_env.define_args(self.args.clone())?;

        self.body.get_type(&new_env)?.equals(&self.ret, env)?;
        self.body.type_check(&new_env)
    }
}

impl GetType for Procedure {
    fn get_type_checked(&self, _env: &Env, _i: usize) -> Result<Type, Error> {
        Ok(Type::Proc(
            self.args.iter().map(|(_, t)| t.clone()).collect(),
            Box::new(self.ret.clone()),
        ))
    }
}

impl Compile for Procedure {
    fn compile_expr(self, env: &mut Env, output: &mut dyn AssemblyProgram) -> Result<(), Error> {
        // Compile the contents of the procedure under a new environment
        let mut new_env = env.new_scope();

        // Declare the arguments and get their size
        let args_size = new_env.define_args(self.args)?;
        // Get the size of the return value to leave on the stack
        let ret_size = self.ret.get_size(env)?;
        // Declare the function body
        output.op(CoreOp::Fn(self.mangled_name.clone()));
        // Execute the body to leave the return value
        self.body.compile_expr(&mut new_env, output)?;

        // Overwrite the arguments with the return value
        output.op(CoreOp::Copy {
            dst: FP.deref().offset(1 - args_size as isize),
            src: SP.deref().offset(1 - ret_size as isize),
            size: ret_size,
        });
        // Decrement the stack pointer by the difference between the size of the
        // arguments and return value, to leave the return value on the stack.
        output.op(CoreOp::Pop(None, args_size));
        // End the function body
        output.op(CoreOp::End);

        // Push the procedure label address onto the stack
        output.op(CoreOp::SetLabel(A, self.mangled_name));
        output.op(CoreOp::Push(A, 1));
        Ok(())
    }
}