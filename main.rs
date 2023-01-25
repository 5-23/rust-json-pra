mod easy_json;
use easy_json::json::*;
use easy_json::object::*;



fn main(){
    let a = json! {
        "a": "10",
        "b": {
            "c": "a",
            "d": "a"
        }
    };
    println!("org: \n{}\n\n", a);
    println!("dbg: \n{:?}", a);
}
