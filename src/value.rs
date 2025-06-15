pub type Value=f64;

#[derive(Debug)]
pub struct ValueArray{
   pub values:Vec<Value>
}

impl ValueArray {
    pub fn new()->Self{
        Self { values: vec![] }
    }
}
