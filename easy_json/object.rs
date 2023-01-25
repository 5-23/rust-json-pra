
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Object {
    Null,
    IntSIGNED(i128),
    IntUNSIGNED(u128),
    Str(String),
    Bool(bool),
    Float(f64),
    Obj(Vec<Objects>),
    Arr(Vec<Self>)
}
impl From<String> for Object{
    fn from(value: String) -> Self {
        Object::Str(value)
    }
}
impl From<&str> for Object{
    fn from(value: &str) -> Self {
        Object::Str(value.to_string())
    }
}
impl From<bool> for Object{
    fn from(value: bool) -> Self {
        Object::Bool(value)
    }
}

impl From<i128> for Object{
    fn from(value: i128) -> Self {
        Object::IntSIGNED(value)
    }
}
impl From<i64> for Object{
    fn from(value: i64) -> Self {
        Object::IntSIGNED(value as i128)
    }
}
impl From<i32> for Object{
    fn from(value: i32) -> Self {
        Object::IntSIGNED(value as i128)
    }
}
impl From<i16> for Object{
    fn from(value: i16) -> Self {
        Object::IntSIGNED(value as i128)
    }
}
impl From<i8> for Object{
    fn from(value: i8) -> Self {
        Object::IntSIGNED(value as i128)
    }
}


impl From<u128> for Object{
    fn from(value: u128) -> Self {
        Object::IntUNSIGNED(value)
    }
}
impl From<u64> for Object{
    fn from(value: u64) -> Self {
        Object::IntUNSIGNED(value as u128)
    }
}
impl From<u32> for Object{
    fn from(value: u32) -> Self {
        Object::IntUNSIGNED(value as u128)
    }
}
impl From<u16> for Object{
    fn from(value: u16) -> Self {
        Object::IntUNSIGNED(value as u128)
    }
}
impl From<u8> for Object{
    fn from(value: u8) -> Self {
        Object::IntUNSIGNED(value as u128)
    }
}


impl From<f32> for Object{
    fn from(value: f32) -> Self {
        Object::Float(value as f64)
    }
}
impl From<f64> for Object{
    fn from(value: f64) -> Self {
        Object::Float(value)
    }
}

#[allow(unused)]
impl Object{
    pub fn parse_string(self) -> String{
        if let Object::Str(s) = self{
            return s;
        }
        return String::new();
    }
    pub fn parse_int_unsigned(self) -> u128{
        if let Object::IntUNSIGNED(i) = self{
            return i;
        }
        return 0;
    }

    pub fn parse_int(self) -> i128{
        if let Object::IntSIGNED(i) = self{
            return i;
        }
        return 0;
    }
    pub fn parse_float(self) -> f64{
        if let Object::Float(f) = self{
            return f;
        }
        return 0.0;
    }
    pub fn parse_arr(self) -> Vec<Object>{
        if let Object::Arr(v) = self{
            return v;
        }
        return vec![];
    }
    pub fn parse_obj(self) -> Vec<Objects>{
        // let objs = if let Object::Obj(o) = self{
        //     o
        // }else{
        //     vec![]
        // };
        // for obj in &objs{
        //     let a = json!{obj.value};
        //     println!("{a}")
        // }
        // return vec![];
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Objects{
    pub key: String,
    pub value: Object
}