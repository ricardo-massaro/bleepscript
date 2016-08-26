
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
            Expression::FuncDef(ref d)      => d.fmt_indent(f, indent),
            Expression::Assignment(ref a)   => a.fmt_indent(f, indent),
            Expression::BinaryOp(ref op)    => op.fmt_indent(f, indent), 
            Expression::PrefixOp(ref op)    => op.fmt_indent(f, indent),
            Expression::FuncCall(ref c)     => c.fmt_indent(f, indent),
        }
    }
}

impl DebugIndent for FuncCall {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(self.func.fmt_indent(f, indent));
        try!(write!(f, "("));
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

impl DebugIndent for Assignment {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        try!(write!(f, "<{}@{}> = ", self.var_index, self.env_index));
        self.val.fmt_indent(f, indent)
    }
}

impl DebugIndent for Statement {
    fn fmt_indent(&self, f : &mut fmt::Formatter, indent : usize) -> Result<(), fmt::Error> {
        match *self {
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
        if let Some((vi, ei, ref val)) = self.var {
            try!(write!(f, "{1:0$}", indent + 2, ""));
            try!(write!(f, "var <{}@{}>", vi, ei));
            if let Some(ref val) = *val {
                try!(write!(f, " = "));
                try!((*val).fmt_indent(f, indent + 2));
            }
            try!(writeln!(f, ";"));
        };
        for s in &self.stmts {
            try!(write!(f, "{1:0$}", indent + 2, ""));
            try!(s.fmt_indent(f, indent + 2));
            try!(writeln!(f, ""));
        }
        write!(f, "{1:0$}}}", indent, "")
    }
}

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