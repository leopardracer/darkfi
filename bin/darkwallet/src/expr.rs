use crate::{
    error::{Error, Result},
    //prop::{Property, PropertySubType, PropertyType, PropertySExprValue},
};
use darkfi_serial::{Decodable, Encodable, ReadExt, SerialDecodable, SerialEncodable, WriteExt, serialize};
use std::{io::{Read, Write}, sync::Arc};

#[derive(Clone, Debug, PartialEq, SerialEncodable, SerialDecodable)]
pub enum SExprVal {
    Null,
    Bool(bool),
    Uint32(u32),
    Float32(f32),
    Str(String),
}

impl SExprVal {
    fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }

    fn is_bool(&self) -> bool {
        match self {
            Self::Bool(v) => true,
            _ => false,
        }
    }

    fn is_u32(&self) -> bool {
        match self {
            Self::Uint32(v) => true,
            _ => false,
        }
    }

    fn is_f32(&self) -> bool {
        match self {
            Self::Float32(v) => true,
            _ => false,
        }
    }

    fn is_str(&self) -> bool {
        match self {
            Self::Str(v) => true,
            _ => false,
        }
    }

    fn as_bool(&self) -> Result<bool> {
        match self {
            Self::Bool(v) => Ok(*v),
            _ => Err(Error::PropertyWrongType),
        }
    }

    pub fn as_u32(&self) -> Result<u32> {
        match self {
            Self::Uint32(v) => Ok(*v),
            _ => Err(Error::PropertyWrongType),
        }
    }

    pub fn as_f32(&self) -> Result<f32> {
        match self {
            Self::Float32(v) => Ok(*v),
            _ => Err(Error::PropertyWrongType),
        }
    }

    fn as_str(&self) -> Result<String> {
        match self {
            Self::Str(v) => Ok(v.clone()),
            _ => Err(Error::PropertyWrongType),
        }
    }

    pub fn coerce_f32(&self) -> Result<f32> {
        match self {
            Self::Uint32(v) => Ok(*v as f32),
            Self::Float32(v) => Ok(*v),
            _ => Err(Error::PropertyWrongType),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SExprMachine<'a> {
    pub globals: Vec<(String, SExprVal)>,
    pub stmts: &'a SExprCode,
}

// Each item is a statement
pub type SExprCode = Vec<Op>;

#[derive(Debug, PartialEq)]
pub enum Op {
    Null,
    Add((Box<Op>, Box<Op>)),
    Sub((Box<Op>, Box<Op>)),
    Mul((Box<Op>, Box<Op>)),
    Div((Box<Op>, Box<Op>)),
    ConstBool(bool),
    ConstUint32(u32),
    ConstFloat32(f32),
    ConstStr(String),
    LoadVar(String),
    //StoreVar((String, Box<Op>)),
    Min((Box<Op>, Box<Op>)),
    Max((Box<Op>, Box<Op>)),
    IsEqual((Box<Op>, Box<Op>)),
    LessThan((Box<Op>, Box<Op>)),
    Float32ToUint32(Box<Op>)
}

impl<'a> SExprMachine<'a> {
    pub fn call(&self) -> Result<SExprVal> {
        if self.stmts.is_empty() {
            return Ok(SExprVal::Null)
        }
        for i in 0..(self.stmts.len() - 1) {
            self.eval(&self.stmts[i])?;
        }
        self.eval(self.stmts.last().unwrap())
    }

    fn eval(&self, op: &Op) -> Result<SExprVal> {
        match op {
            Op::Null => Ok(SExprVal::Null),
            Op::Add((lhs, rhs)) => self.add(lhs, rhs),
            Op::Sub((lhs, rhs)) => self.sub(lhs, rhs),
            Op::Mul((lhs, rhs)) => self.mul(lhs, rhs),
            Op::Div((lhs, rhs)) => self.div(lhs, rhs),
            Op::ConstBool(val) => Ok(SExprVal::Bool(*val)),
            Op::ConstUint32(val) => Ok(SExprVal::Uint32(*val)),
            Op::ConstFloat32(val) => Ok(SExprVal::Float32(*val)),
            Op::ConstStr(val) => Ok(SExprVal::Str(val.clone())),
            Op::LoadVar(var) => self.load_var(var),
            //Op::StoreVar((var, val)) => self.store_var(var, val),
            Op::Min((lhs, rhs)) => self.min(lhs, rhs),
            Op::Max((lhs, rhs)) => self.max(lhs, rhs),
            Op::IsEqual((lhs, rhs)) => self.is_equal(lhs, rhs),
            Op::LessThan((lhs, rhs)) => self.less_than(lhs, rhs),
            Op::Float32ToUint32(val) => self.float32_to_uint32(val),
        }
    }

    fn add(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            return Ok(SExprVal::Uint32(lhs.as_u32().unwrap() + rhs.as_u32().unwrap()))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;

        Ok(SExprVal::Float32(lhs + rhs))
    }
    fn sub(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            return Ok(SExprVal::Uint32(lhs.as_u32().unwrap() - rhs.as_u32().unwrap()))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;

        Ok(SExprVal::Float32(lhs - rhs))
    }
    fn mul(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            return Ok(SExprVal::Uint32(lhs.as_u32().unwrap() * rhs.as_u32().unwrap()))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;

        Ok(SExprVal::Float32(lhs * rhs))
    }
    fn div(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        // Always coerce

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;

        Ok(SExprVal::Float32(lhs / rhs))
    }
    fn load_var(&self, var: &str) -> Result<SExprVal> {
        for (name, val) in &self.globals {
            if name == var {
                return Ok(val.clone())
            }
        }
        Err(Error::SExprGlobalNotFound)
    }
    //fn store_var(&mut self, var, val) {
    //}
    fn min(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            let lhs = lhs.as_u32().unwrap();
            let rhs = rhs.as_u32().unwrap();
            let min = if lhs < rhs { lhs } else { rhs };
            return Ok(SExprVal::Uint32(min))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;
        let min = if lhs < rhs { lhs } else { rhs };

        Ok(SExprVal::Float32(min))
    }
    fn max(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            let lhs = lhs.as_u32().unwrap();
            let rhs = rhs.as_u32().unwrap();
            let max = if lhs > rhs { lhs } else { rhs };
            return Ok(SExprVal::Uint32(max))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;
        let max = if lhs > rhs { lhs } else { rhs };

        Ok(SExprVal::Float32(max))
    }
    fn is_equal(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            return Ok(SExprVal::Bool(lhs.as_u32().unwrap() == rhs.as_u32().unwrap()))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;
        let is_equal = (lhs - rhs).abs() < f32::EPSILON;

        Ok(SExprVal::Bool(is_equal))
    }
    fn less_than(&self, lhs: &Op, rhs: &Op) -> Result<SExprVal> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        if lhs.is_u32() && rhs.is_u32() {
            return Ok(SExprVal::Bool(lhs.as_u32().unwrap() < rhs.as_u32().unwrap()))
        }

        let lhs = lhs.coerce_f32()?;
        let rhs = rhs.coerce_f32()?;

        Ok(SExprVal::Bool(lhs < rhs))
    }
    fn float32_to_uint32(&self, val: &Op) -> Result<SExprVal> {
        let val = self.eval(val)?;
        if val.is_u32() {
            return Ok(SExprVal::Uint32(val.as_u32()?))
        }
        Ok(SExprVal::Uint32(val.as_f32()? as u32))
    }
}

impl Encodable for Op {
    fn encode<S: Write>(&self, s: &mut S) -> std::result::Result<usize, std::io::Error> {
        let mut len = 0;
        match self {
            Self::Null => {
                len += 0u8.encode(s)?;
            }
            Self::Add((lhs, rhs)) => {
                len += 1u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::Sub((lhs, rhs)) => {
                len += 2u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::Mul((lhs, rhs)) => {
                len += 3u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::Div((lhs, rhs)) => {
                len += 4u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::ConstBool(val) => {
                len += 5u8.encode(s)?;
                len += val.encode(s)?;
            }
            Self::ConstUint32(val) => {
                len += 6u8.encode(s)?;
                len += val.encode(s)?;
            }
            Self::ConstFloat32(val) => {
                len += 7u8.encode(s)?;
                len += val.encode(s)?;
            }
            Self::ConstStr(val) => {
                len += 8u8.encode(s)?;
                len += val.encode(s)?;
            }
            Self::LoadVar(var) => {
                len += 9u8.encode(s)?;
                len += var.encode(s)?;
            }
            // StoreVar
            Self::Min((lhs, rhs)) => {
                len += 11u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::Max((lhs, rhs)) => {
                len += 12u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::IsEqual((lhs, rhs)) => {
                len += 13u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::LessThan((lhs, rhs)) => {
                len += 14u8.encode(s)?;
                len += lhs.encode(s)?;
                len += rhs.encode(s)?;
            }
            Self::Float32ToUint32(val) => {
                len += 15u8.encode(s)?;
                len += val.encode(s)?;
            }
        }
        Ok(len)
    }
}

impl Decodable for Op {
    fn decode<D: Read>(d: &mut D) -> std::result::Result<Self, std::io::Error> {
        let op_type = d.read_u8()?;
        let self_ = match op_type {
            0 => Self::Null,
            1 => Self::Add((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            2 => Self::Sub((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            3 => Self::Mul((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            4 => Self::Div((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            5 => Self::ConstBool(d.read_bool()?),
            6 => Self::ConstUint32(d.read_u32()?),
            7 => Self::ConstFloat32(d.read_f32()?),
            8 => Self::ConstStr(String::decode(d)?),
            9 => Self::LoadVar(String::decode(d)?),
            // StoreVar
            11 => Self::Min((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            12 => Self::Max((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            13 => Self::IsEqual((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            14 => Self::LessThan((Box::new(Self::decode(d)?), Box::new(Self::decode(d)?))),
            15 => Self::Float32ToUint32(Box::new(Self::decode(d)?)),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid Op type")),
        };
        Ok(self_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darkfi_serial::{serialize, deserialize};

    #[test]
    fn seval() {
        let machine = SExprMachine {
            globals: vec![
                ("sw".to_string(), SExprVal::Uint32(110u32)),
                ("sh".to_string(), SExprVal::Uint32(4u32)),
            ],
            stmts: &vec![Op::Add((
                Box::new(Op::ConstUint32(5)),
                Box::new(Op::Div((
                    Box::new(Op::LoadVar("sw".to_string())),
                    Box::new(Op::ConstUint32(2)),
                ))),
            ))],
        };
        assert_eq!(machine.call().unwrap(), SExprVal::Float32(60.));
    }

    #[test]
    fn encdec_code() {
        let code = Op::Add((
            Box::new(Op::ConstUint32(5)),
            Box::new(Op::Div((
                Box::new(Op::LoadVar("sw".to_string())),
                Box::new(Op::ConstUint32(2)),
            ))),
        ));

        let code_s = serialize(&code);
        let code2 = deserialize::<Op>(&code_s).unwrap();
        assert_eq!(code, code2);
    }
}
