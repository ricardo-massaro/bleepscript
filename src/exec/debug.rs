
use std::fmt;

use super::*;

trait DebugIndent {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error>;
}

impl DebugIndent for FuncDef {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "function ("));
        for n in 0..self.num_params {
            if n > 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "<{}@0>", n));
        }
        try!(write!(f, ") "));
        self.block.fmt_indent(f, indent)
    }
}

impl DebugIndent for Expression {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        match *self {
            Expression::Number(n, _)        => write!(f, "{}", n),
            Expression::String(ref s, _)    => write!(f, "{:?}", **s),
            Expression::Variable(vi, ei, _) => write!(f, "<{}@{}>", vi, ei),
            Expression::Vec(ref v)          => v.fmt_indent(f, indent),
            Expression::Map(ref m)          => m.fmt_indent(f, indent),
            Expression::Element(ref e)      => e.fmt_indent(f, indent),
            Expression::VarAssign(ref a)    => a.fmt_indent(f, indent),
            Expression::ElemAssign(ref a)   => a.fmt_indent(f, indent),
            Expression::BinaryOp(ref op)    => op.fmt_indent(f, indent), 
            Expression::PrefixOp(ref op)    => op.fmt_indent(f, indent),
            Expression::FuncCall(ref c)     => c.fmt_indent(f, indent),
            Expression::FuncDef(ref d)      => d.fmt_indent(f, indent),
        }
    }
}

impl DebugIndent for VecLiteral {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(writeln!(f, "["));
        for i in &self.vec {
            try!(write!(f, "{1:0$}", indent + 2, ""));
            try!(i.fmt_indent(f, indent + 2));
            try!(writeln!(f, ","));
        }
        try!(write!(f, "{1:0$}", indent, ""));
        write!(f, "]")
    }
}

impl DebugIndent for MapLiteral {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(writeln!(f, "{{"));
        for &(ref k, ref v) in &self.entries {
            try!(write!(f, "{1:0$}", indent + 2, ""));
            try!(write!(f, "\"{}\" : ", k));
            try!(v.fmt_indent(f, indent + 2));
            try!(writeln!(f, ","));
        }
        try!(write!(f, "{1:0$}", indent, ""));
        write!(f, "}}")
    }
}

impl DebugIndent for Element {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "("));
        try!(self.container.fmt_indent(f, indent));
        try!(write!(f, ")["));
        try!(self.index.fmt_indent(f, indent));
        write!(f, "]")
    }
}

impl DebugIndent for FuncCall {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "("));
        try!(self.func.fmt_indent(f, indent));
        try!(write!(f, ")("));
        for (n, arg) in self.args.iter().enumerate() {
            if n > 0 {
                try!(write!(f, ", "));
            }
            try!(arg.fmt_indent(f, indent));
        }
        write!(f, ")")
    }
}

impl DebugIndent for BinaryOp {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "("));
        try!(self.left.fmt_indent(f, indent));
        try!(write!(f, " <{}:{}> ", self.val_index, self.env_index));
        try!(self.right.fmt_indent(f, indent));
        write!(f, ")")
    }
}

impl DebugIndent for PrefixOp {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "("));
        try!(write!(f, " <{}:{}> ", self.val_index, self.env_index));
        try!(self.arg.fmt_indent(f, indent));
        write!(f, ")")
    }
}

impl DebugIndent for VarAssign {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "<{}@{}> = ", self.var_index, self.env_index));
        self.val.fmt_indent(f, indent)
    }
}

impl DebugIndent for ElemAssign {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(self.container.fmt_indent(f, indent));
        try!(write!(f, "["));
        try!(self.index.fmt_indent(f, indent));
        try!(write!(f, "] = "));
        self.val.fmt_indent(f, indent)
    }
}

impl DebugIndent for Statement {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        match *self {
            Statement::Empty               => write!(f, ";"),
            Statement::If(ref i)           => i.fmt_indent(f, indent),
            Statement::While(ref w)        => w.fmt_indent(f, indent),
            Statement::Break(_)            => write!(f, "break;"),
            Statement::Return(ref r)       => r.fmt_indent(f, indent),
            Statement::Block(ref b)        => b.fmt_indent(f, indent),
            Statement::Expression(ref e)   => {
                try!(e.fmt_indent(f, indent));
                write!(f, ";")
            },
        }
    }
}

impl DebugIndent for Block {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(writeln!(f, "{{"));
        if self.has_var {
            try!(write!(f, "{1:0$}", indent + 2, ""));
            try!(write!(f, "var <0@0>"));
            if let Some(ref val) = self.var_val {
                try!(write!(f, " = "));
                try!((*val).fmt_indent(f, indent + 2));
            }
            try!(writeln!(f, ";"));
        }
        for s in &self.stmts {
            try!(write!(f, "{1:0$}", indent + 2, ""));
            try!(s.fmt_indent(f, indent + 2));
            try!(writeln!(f, ""));
        }
        write!(f, "{1:0$}}}", indent, "")
    }
}

impl DebugIndent for IfStatement {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "if ("));
        try!(self.test.fmt_indent(f, indent));
        try!(write!(f, ") "));
        try!(self.true_stmt.fmt_indent(f, indent));
        if let Some(ref e) = self.false_stmt {
            try!(write!(f, " else "));
            try!(e.fmt_indent(f, indent));
        };
        Ok(())
    }
}

impl DebugIndent for WhileStatement {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "while ("));
        try!(self.test.fmt_indent(f, indent));
        try!(write!(f, ") "));
        self.stmt.fmt_indent(f, indent)
    }
}

impl DebugIndent for ReturnStatement {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "return"));
        if let Some(ref e) = self.expr {
            try!(write!(f, " "));
            try!(e.fmt_indent(f, indent));
        }
        write!(f, ";")
    }
}

// ================================================
// fmt::Debug

impl fmt::Debug for FuncCall {
    fn fmt(&self, f : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.fmt_indent(f, 0)
    }
}

impl fmt::Debug for FuncDef {
    fn fmt(&self, f : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.fmt_indent(f, 0)
    }
}
