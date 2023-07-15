use prql_ast::codegen::{SeparatedExprs, WriteOpt, WriteSource};

use super::{TupleField, Ty, TyKind};

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opt = WriteOpt::new_width(u16::MAX);
        f.write_str(&self.write(opt).unwrap())
    }
}

impl std::fmt::Display for TyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opt = WriteOpt::new_width(u16::MAX);
        f.write_str(&self.write(opt).unwrap())
    }
}

impl WriteSource for Ty {
    fn write(&self, opt: WriteOpt) -> Option<String> {
        if let Some(name) = &self.name {
            Some(name.clone())
        } else {
            self.kind.write(opt)
        }
    }
}

impl WriteSource for TyKind {
    fn write(&self, opt: WriteOpt) -> Option<String> {
        use TyKind::*;

        match &self {
            Primitive(prim) => Some(prim.to_string()),
            Union(variants) => {
                let variants: Vec<_> = variants.iter().map(|x| &x.1).collect();

                SeparatedExprs {
                    exprs: &variants,
                    inline: " || ",
                    line_end: " ||",
                }
                .write(opt)
            }
            Singleton(lit) => Some(lit.to_string()),
            Tuple(elements) => SeparatedExprs {
                exprs: elements,
                inline: ", ",
                line_end: ",",
            }
            .write_between("{", "}", opt),
            Set => Some("set".to_string()),
            Array(elem) => Some(format!("[{}]", elem.write(opt)?)),
            Function(func) => {
                let mut r = String::new();

                for t in &func.args {
                    r += &write_opt_ty(t.as_ref(), opt.clone())?;
                    r += " ";
                }
                r += "-> ";
                r += &write_opt_ty((*func.return_ty).as_ref(), opt)?;
                Some(r)
            }
        }
    }
}

fn write_opt_ty(opt_ty: Option<&Ty>, opt: WriteOpt) -> Option<String> {
    match opt_ty {
        Some(ty) => ty.write(opt),
        None => Some("infer".to_string()),
    }
}

impl WriteSource for TupleField {
    fn write(&self, opt: WriteOpt) -> Option<String> {
        match self {
            Self::Wildcard(generic_el) => match generic_el {
                Some(el) => Some(format!("{}..", el.write(opt)?)),
                None => Some("*..".to_string()),
            },
            Self::Single(name, expr) => {
                let mut r = String::new();

                if let Some(name) = name {
                    r += name;
                    r += " = ";
                }
                if let Some(expr) = expr {
                    r += &expr.write(opt)?;
                } else {
                    r += "?";
                }
                Some(r)
            }
        }
    }
}
