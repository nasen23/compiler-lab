// This type system is incomplete, as it may have some custom types like structs,
// enums, typedefs.
use crate::ast::VarDef;

pub enum TyKind {
    // incomplete type 'void' could not be directly used
    // It is either function return type or behind a pointer
    Void,
    Char(Sign),
    Short(Sign),
    Int(Sign),
    Long(Sign),
    LLong(Sign),
    Float,
    Double,
    Array(Array),
    Func(Func),
    Pointer(Pointer),
    Struct(Struct),
    Union(Union),
    Enum,
}

pub enum Sign {
    Signed,
    Unsigned
}

pub enum StorClass {
    Typedef,
    Extern,
    Static,
    Auto,
    Register
}

pub struct Array {
    // initialized array must have a explicit size
    // array in function param is okay not to have a length
    pub tyk: Box<TyKind>,
    pub len: Option<u32>
}


pub struct Pointer {
    pub tyk: Box<TyKind>
}

pub struct Struct {
    pub name: String,
    pub mem: Vec<VarDef>
}

pub struct Union {
    pub name: String,
    pub mem: Vec<VarDef>,
    pub size: usize
}

pub struct Func {
    pub ret: Box<TyKind>,
    pub param: Vec<TyKind>,
    pub args: bool
}

pub struct Ty {
    pub const_: bool,
    pub kind: TyKind
}

use Sign::*;

impl Ty {
    pub const fn new(kind: TyKind, const_: bool) -> Ty { Ty { const_, kind } }

    pub const fn void() -> Ty { Ty::new(TyKind::Void, false) }
    pub const fn char() -> Ty { Ty::new(TyKind::Char(Signed), false) }
    pub const fn int() -> Ty { Ty::new(TyKind::Int(Signed), false) }
    pub const fn float() -> Ty { Ty::new(TyKind::Float, false) }
    pub const fn double() -> Ty { Ty::new(TyKind::Double, false) }
    pub fn array(kind: TyKind, len: Option<u32>) -> Ty {
        Ty::new(TyKind::Array(Array { tyk: Box::new(kind), len }), false)
    }
    pub fn pointer(kind: TyKind) -> Ty {
        Ty::new(TyKind::Pointer(Pointer { tyk: Box::new(kind) }), false)
    }

    pub fn is_const(&self) -> bool { self.const_ }
}
