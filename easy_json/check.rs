use crate::easy_json::object::*;

#[allow(bindings_with_variant_name)]
pub fn display(x: &Object, _i: usize) -> String{
    match x{
        Object::IntSIGNED(i) => format!("{i}"),
        Object::IntUNSIGNED(i) => format!("{i}"),
        Object::Str(s) => format!("\"{s}\""),
        Object::Bool(b) => format!("{b}"),
        Object::Arr(v) => {
            let mut res = "[".to_string();
            let len = v.len();
            let mut i = 0;
            for o in v{
                i += 1;
                res.push_str(&display(o, _i));
                if len != i{
                    res.push_str(", ");
                }
            }
            res.push_str("]");
            res
        },
        Object::Obj(v) => {
            let mut res = "{\n".to_string();
            let len = v.len();
            let mut i = 0;
            for o in v{
                i += 1;
                res.push_str(format!("{}{}: ", " ".repeat(4 * (_i+1)) , o.key).as_str());
                res.push_str(&display(&o.value, _i+1));
                if len != i{
                    res.push_str(",\n");
                }else{
                    res.push_str("\n");
                }
            }
            res.push_str(format!("{}{}", " ".repeat(4 * _i), "}").as_str());
            res
        }
        _ => "null".to_string(),
    }
}

pub fn debug(x: &Object, _i: usize) -> String{
    match x{
        Object::IntSIGNED(i) => format!("signed-int({i})"),
        Object::IntUNSIGNED(i) => format!("unsigned-int({i})"),
        Object::Str(s) => format!("str({s})"),
        Object::Bool(b) => format!("bool({b})"),
        Object::Float(f) => format!("float({f})"),
        Object::Arr(v) => {
            let mut res = "vector[".to_string();
            let len = v.len();
            let mut i = 0;
            for o in v{
                i += 1;
                res.push_str(&debug(o, _i));
                if len != i{
                    res.push_str(", ");
                }
            }
            res.push_str("]");
            res
        },
        Object::Obj(v) => {
            let mut res = "object{\n".to_string();
            let len = v.len();
            let mut i = 0;
            for o in v{
                i += 1;
                res.push_str(format!("{}{}: ", " ".repeat(4 * (_i+1)) , o.key).as_str());
                res.push_str(&debug(&o.value, _i+1));
                if len != i{
                    res.push_str(",\n");
                }else{
                    res.push_str("\n");
                }
            }
            res.push_str(format!("{}{}", " ".repeat(4 * _i), "}").as_str());
            res
        }
        _ => "NULL".to_string(),
    }
}
