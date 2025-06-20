use std::{
    ops::{Add, Mul, Neg, Sub},
    panic,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    VAL_BOOL(bool),
    VAL_NIL,
    VAL_NUMBER(f64),
    VAL_STRING(Rc<String>),
}
/*
impl Clone for Value{
    fn clone(&self) -> Self {
       match self.type_v {
           ValueType::VAL_NUMBER(b)=> Value::from(b),
           ValueType::VAL_NIL => Value::from(ValueType::VAL_NIL),
           _=>panic!("Cant be cloned")
       }
    }
}*/
#[derive(Debug, Clone)]
pub struct Value {
    pub type_v: ValueType,
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self {
            type_v: ValueType::VAL_NUMBER(value),
        }
    }
}

impl From<ValueType> for Value {
    fn from(value: ValueType) -> Self {
        Self { type_v: value }
    }
}

impl Value {
    pub fn bool_value(value: bool) -> Self {
        Self::from(ValueType::VAL_BOOL(value))
    }
    pub fn nil_value() -> Self {
        Self::from(ValueType::VAL_NIL)
    }
    pub fn number_value(value: f64) -> Self {
        Self::from(ValueType::VAL_NUMBER(value))
    }
    pub fn obj_value(str: &str) -> Self {
        Self::from(ValueType::VAL_STRING(Rc::new(str.to_string())))
    }

    pub fn as_bool(&self) -> bool {
        if let ValueType::VAL_BOOL(b) = self.type_v {
            b
        } else {
            panic!("Tried to extract bool from non-bool Value: {:?}", self);
        }
    }

    pub fn as_number(&self) -> f64 {
        if let ValueType::VAL_NUMBER(n) = self.type_v {
            n
        } else {
            panic!("Tried to extract number from non-number Value: {:?}", self);
        }
    }

    pub fn as_obj(&self) -> Rc<String> {
        if let ValueType::VAL_STRING(str) = self.type_v.clone() {
            str.clone()
        } else {
            panic!("Tried to extract object from non obj value");
        }
    }
    pub fn is_bool(&self) -> bool {
        matches!(self.type_v, ValueType::VAL_BOOL(_))
    }
    pub fn is_obj(&self) -> bool {
        matches!(self.type_v, ValueType::VAL_STRING(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self.type_v, ValueType::VAL_NUMBER(_))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self.type_v, ValueType::VAL_NIL)
    }
}

#[derive(Debug)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: vec![] }
    }
}

impl Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self.type_v {
            ValueType::VAL_NUMBER(n) => Value::from(-n),
            a => panic!("-{:?} is not valid", a),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self.type_v, rhs.type_v) {
            (ValueType::VAL_NUMBER(n), ValueType::VAL_NUMBER(n2)) => Value::from(n / n2),
            a => panic!("{:?}/{:?} is not valid", a.0, a.1),
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.type_v, rhs.type_v) {
            (ValueType::VAL_NUMBER(n), ValueType::VAL_NUMBER(n2)) => Value::from(n + n2),
            (ValueType::VAL_STRING(a), ValueType::VAL_STRING(b)) => Value::from(
                ValueType::VAL_STRING(Rc::new(format!("{}{}", a.as_str(), b.as_str()))),
            ),
            a => panic!("{:?}+{:?} is not valid", a.0, a.1),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self.type_v, rhs.type_v) {
            (ValueType::VAL_NUMBER(n), ValueType::VAL_NUMBER(n2)) => Value::from(n - n2),
            a => panic!("{:?}-{:?} is not valid", a.0, a.1),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self.type_v, rhs.type_v) {
            (ValueType::VAL_NUMBER(n), ValueType::VAL_NUMBER(n2)) => Value::from(n * n2),
            a => panic!("{:?}*{:?} is not valid", a.0, a.1),
        }
    }
}
