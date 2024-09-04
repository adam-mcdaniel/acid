use super::{
    Annotation, AssignOp, BinaryOp, ConstExpr, Expr, Mutability, Pattern, PolyProcedure, TernaryOp,
    Type, UnaryOp,
};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// An LIR compilation error.
#[derive(Clone, Debug)]
pub enum Error {
    /// An error with some annotation about the source code that caused the error.
    Annotated(
        /// The error that was caused by the source code.
        Box<Self>,
        /// The location of the source code that caused the error.
        /// This is used for error reporting.
        Annotation,
    ),

    UnimplementedOperator(String),

    /// An error caused by trying to assemble invalid code generated by the compiler.
    /// This should be taken seriously, unless the error is due to an invalid handwritten builtin.
    AssemblyError(crate::asm::Error),

    /// The variant of an enum is not defined.
    VariantNotFound(Type, String),
    /// Tried to access an undefined member of a tuple, struct, or union.
    MemberNotFound(Expr, ConstExpr),
    /// Recursion depth exceeded when trying to evaluate a constant expression.
    RecursionDepthConst(ConstExpr),
    /// Recursion depth exceeded when trying to confirm a type's equality to another type.
    CouldntSimplify(Type, Type),
    /// Recursion depth exceeded when trying to confirm a type's equality to another type.
    RecursionDepthTypeEquality(Type, Type),
    /// Got another type when expecting an integer, bool, or char.
    NonIntegralConst(ConstExpr),
    /// Tried to instantiate a type that cannot be sized.
    /// This is a problem because we cannot manage the stack if we cannot know the size of the type.
    UnsizedType(Type),
    /// Tried to dereference a non-pointer.
    DerefNonPointer(Expr),
    /// Tried to apply a non-procedure to some arguments.
    ApplyNonProc(Expr),
    /// Expected a symbol, but got something else.
    NonSymbol(ConstExpr),
    /// Invalid `Index` expression (incorrect types).
    InvalidIndex(Expr),
    /// Invalid `Refer` expression. The compiler was not able to calculate the address of the expression.
    InvalidRefer(Expr),
    /// Invalid unary operation (negate, not) expression (incorrect types).
    InvalidUnaryOp(Box<dyn UnaryOp>, Expr),
    /// Invalid unary op types (incorrect types).
    InvalidUnaryOpTypes(Box<dyn UnaryOp>, Type),
    /// Invalid binary operation (add, subtract, and, or) expression (incorrect types).
    InvalidBinaryOp(Box<dyn BinaryOp>, Expr, Expr),
    /// Invalid binary op types (incorrect types).
    InvalidBinaryOpTypes(Box<dyn BinaryOp>, Type, Type),
    /// Invalid ternary operation (if) expression (incorrect types).
    InvalidTernaryOp(Box<dyn TernaryOp>, Expr, Expr, Expr),
    /// Invalid ternary op types (incorrect types).
    InvalidTernaryOpTypes(Box<dyn TernaryOp>, Type, Type, Type),
    /// Invalid assignment operation (assign, add_assign, subtract_assign, and_assign, or_assign) expression (incorrect types).
    InvalidAssignOp(Box<dyn AssignOp>, Expr, Expr),
    /// Invalid assign op types (incorrect types).
    InvalidAssignOpTypes(Box<dyn AssignOp>, Type, Type),

    /// Mismatched types
    MismatchedTypes {
        expected: Type,
        found: Type,
        expr: Expr,
    },
    /// Mismatched mutability
    MismatchedMutability {
        expected: Mutability,
        found: Mutability,
        expr: Expr,
    },

    /// A symbol was used, but not defined.
    SymbolNotDefined(String),
    /// A type was used, but not defined.
    TypeNotDefined(String),
    /// Tried to create an array with a negative length.
    NegativeArrayLength(Expr),

    /// Tried to use a pattern that is not valid for the given type.
    InvalidPatternForType(Type, Pattern),
    /// Tried to use a pattern that is not valid for the given expression.
    InvalidPatternForExpr(Expr, Pattern),

    /// Tried to match over an expression that cannot be matched over.
    InvalidMatchExpr(Expr),

    /// Invalid pattern for a match expression.
    NonExhaustivePatterns {
        patterns: Vec<Pattern>,
        expr: Expr,
    },

    /// Invalid type casting expression.
    InvalidAs(Expr, Type, Type),

    /// Invalid constant expression.
    InvalidConstExpr(ConstExpr),

    /// Expression uses an operation unsupported by the target.
    UnsupportedOperation(Expr),

    /// Tried to define a type that already exists.
    TypeRedefined(String),

    /// Unused expression returned a non-None value.
    UnusedExpr(Expr, Type),

    /// Invalid number of template arguments to a type.
    InvalidTemplateArgs(Type),

    /// Tried to apply a non-template type to some arguments.
    ApplyNonTemplate(Type),

    /// Tried to get the size of a template type.
    SizeOfTemplate(Type),

    /// Tried to  compile a polymorphic procedure without monomorphing it.
    CompilePolyProc(PolyProcedure),

    /// Cannot monomorphize a constant expression.
    InvalidMonomorphize(ConstExpr),

    /// Duplicate implementations of a member for a type
    DuplicateMember(Type, String),
}

impl Error {
    /// Annotate an error with some metadata.
    pub fn annotate(mut self, annotation: Annotation) -> Self {
        match &mut self {
            Self::Annotated(err, previous_annotation) => {
                let mut result = annotation.clone();
                result |= previous_annotation.clone();
                *err = Box::new(err.clone().annotate(annotation));
                *previous_annotation = result;
                self
            }
            _ => Self::Annotated(Box::new(self), annotation),
        }
    }
}

/// Create an IR error from an assembly error.
impl From<crate::asm::Error> for Error {
    fn from(e: crate::asm::Error) -> Self {
        Self::AssemblyError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Annotated(err, _) => {
                write!(f, "{err}")
            }
            Self::UnimplementedOperator(op) => {
                write!(f, "unimplemented operator {}", op)
            }

            Self::DuplicateMember(ty, member) => {
                write!(f, "duplicate member {member} of type {ty}")
            }

            Self::MismatchedTypes {
                expected,
                found,
                expr,
            } => {
                write!(
                    f,
                    "mismatched types: expected {}, found {} in {}",
                    expected, found, expr
                )
            }
            Self::MismatchedMutability {
                expected,
                found,
                expr,
            } => {
                write!(
                    f,
                    "mismatched mutability: expected {}, found {} in {}",
                    expected, found, expr
                )
            }
            Self::VariantNotFound(ty, variant) => {
                write!(f, "variant {} not found in {}", variant, ty)
            }
            Self::MemberNotFound(expr, member) => {
                write!(f, "member {} not found in {}", member, expr)
            }
            Self::RecursionDepthConst(expr) => {
                write!(
                    f,
                    "recursion depth exceeded when trying to evaluate {}",
                    expr
                )
            }
            Self::CouldntSimplify(ty1, ty2) => {
                write!(f, "couldn't simplify {} to {}", ty1, ty2)
            }
            Self::RecursionDepthTypeEquality(ty1, ty2) => {
                write!(
                    f,
                    "recursion depth exceeded when trying to confirm {} == {}",
                    ty1, ty2
                )
            }
            Self::NonIntegralConst(expr) => {
                write!(f, "got non-integral constant expression {}", expr)
            }
            Self::UnsizedType(ty) => {
                write!(f, "tried to instantiate unsized type {}", ty)
            }
            Self::DerefNonPointer(expr) => {
                write!(f, "tried to dereference non-pointer {}", expr)
            }
            Self::ApplyNonProc(expr) => {
                write!(f, "tried to apply non-procedure {}", expr)
            }
            Self::NonSymbol(expr) => {
                write!(f, "expected symbol, found {}", expr)
            }
            Self::InvalidIndex(expr) => {
                write!(f, "invalid index expression {}", expr)
            }
            Self::InvalidRefer(expr) => {
                write!(f, "invalid refer expression {}", expr)
            }
            Self::InvalidUnaryOp(op, expr) => {
                write!(f, "invalid unary operation {} {}", op, expr)
            }
            Self::InvalidUnaryOpTypes(op, ty) => {
                write!(f, "invalid unary operation {} for type {}", op, ty)
            }
            Self::InvalidBinaryOp(op, expr1, expr2) => {
                write!(f, "invalid binary operation {} {} {}", op, expr1, expr2)
            }
            Self::InvalidBinaryOpTypes(op, ty1, ty2) => {
                write!(
                    f,
                    "invalid binary operation {} for types {} and {}",
                    op, ty1, ty2
                )
            }
            Self::InvalidTernaryOp(op, expr1, expr2, expr3) => {
                write!(
                    f,
                    "invalid ternary operation {} {} {} {}",
                    op, expr1, expr2, expr3
                )
            }
            Self::InvalidTernaryOpTypes(op, ty1, ty2, ty3) => {
                write!(
                    f,
                    "invalid ternary operation {} for types {}, {}, and {}",
                    op, ty1, ty2, ty3
                )
            }
            Self::InvalidAssignOp(op, expr1, expr2) => {
                write!(f, "invalid assignment operation {} {} {}", op, expr1, expr2)
            }
            Self::InvalidAssignOpTypes(op, ty1, ty2) => {
                write!(
                    f,
                    "invalid assignment operation {} for types {} and {}",
                    op, ty1, ty2
                )
            }
            Self::SymbolNotDefined(sym) => {
                write!(f, "symbol {} not defined", sym)
            }
            Self::TypeNotDefined(ty) => {
                write!(f, "type {} not defined", ty)
            }
            Self::NegativeArrayLength(expr) => {
                write!(f, "negative array length {}", expr)
            }
            Self::InvalidPatternForType(ty, pat) => {
                write!(f, "invalid pattern {} for type {}", pat, ty)
            }
            Self::InvalidPatternForExpr(expr, pat) => {
                write!(f, "invalid pattern {} for expression {}", pat, expr)
            }
            Self::InvalidMatchExpr(expr) => {
                write!(f, "invalid match expression {}", expr)
            }
            Self::NonExhaustivePatterns { patterns, expr } => {
                write!(
                    f,
                    "non-exhaustive patterns {:?} for expression {}",
                    patterns, expr
                )
            }
            Self::InvalidAs(expr, ty1, ty2) => {
                write!(
                    f,
                    "invalid as expression {} for types {} and {}",
                    expr, ty1, ty2
                )
            }
            Self::InvalidConstExpr(expr) => {
                write!(f, "invalid constant expression {}", expr)
            }
            Self::UnsupportedOperation(expr) => {
                write!(f, "unsupported operation {}", expr)
            }
            Self::TypeRedefined(ty) => {
                write!(f, "type {} redefined", ty)
            }
            Self::UnusedExpr(expr, ty) => {
                write!(f, "unused expression {} of type {}", expr, ty)
            }
            Self::InvalidTemplateArgs(ty) => {
                write!(f, "invalid template arguments for type {}", ty)
            }
            Self::ApplyNonTemplate(ty) => {
                write!(f, "tried to apply non-template type {}", ty)
            }
            Self::SizeOfTemplate(ty) => {
                write!(f, "tried to get size of template type {}", ty)
            }
            Self::CompilePolyProc(proc) => {
                write!(f, "tried to compile polymorphic procedure {}", proc)
            }
            Self::AssemblyError(e) => {
                write!(f, "assembly error: {}", e)
            }
            Self::InvalidMonomorphize(expr) => {
                write!(
                    f,
                    "invalid monomorphization of constant expression {}",
                    expr
                )
            }
        }
    }
}
