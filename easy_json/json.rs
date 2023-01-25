use crate::easy_json::*;
use object::*;

use std::fmt;

pub struct Json {
    pub objs: Vec<Objects>
}
impl Json {
    pub fn new() -> Self{
        Self { objs: vec![] }
    }
    pub fn add(&mut self, key: &str, value: Object){
        self.objs.push(Objects { key: key.to_string(), value })
    }
    // pub fn get(&self, name: &str) -> Object {
    //     for i in &self.objs{
    //         if i.key == name.to_string(){
    //             return i.value.clone()
    //         }
    //     }
    //     return Object::Null;
    // }
}



impl fmt::Display for Json{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = "{\n".to_string();
        for i in &self.objs{
            res.push_str("    ");
            res.push_str(&i.key);
            res.push_str(": ");
            
            
            let v = check::display(&i.value, 1);
            res.push_str(&v);
            res.push_str("\n");
        }
        res.push_str("}");
        write!(f, "{res}")
    }
}

impl fmt::Debug for Json{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = "{\n".to_string();
        for i in &self.objs{
            res.push_str("    ");
            res.push_str(&i.key);
            res.push_str(": ");
            
            
            let v = check::debug(&i.value, 1);
            res.push_str(&v);
            res.push_str("\n");
        }
        res.push_str("}");
        write!(f, "{res}")
    }
}

/**
json!{key: value}

# ex
```
json!{
    "int": 1,
    "arr": [1, "a"],
    "object": {
        "arr2": [0; 10]
    }
}
```

*/
#[macro_export]
macro_rules! json {
    () => { Object::Null };
    ($($k: tt : $v: tt), *) => {
        {
            let mut obj = Json::new();
            $(obj.add($k, json!($v));)*
            obj
        }
    };
    ({$($k: tt : $v: tt), *}) => {
        Object::Obj(vec![$(Objects{key: $k.to_string(), value: json!($v)}), *])
    };
    ([$($e: tt), *]) => {
        Object::Arr(vec![$(json!($e)), *])
    };
    ([$e: tt ; $n: tt]) => {
        Object::Arr(vec![json!($e); $n])
    };
    (null) => { Object::Null };
    ($e: tt) => {
        Object::from($e)
    };
}