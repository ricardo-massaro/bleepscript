//extern crate time;
extern crate bleepscript;

use std::rc::Rc;
use std::env;
use bleepscript::*;

fn test_function(args : &[Value], _env : &Rc<Env>) -> Result<Value,RunError> {
    match args.get(0) {
        Some(v) => println!("test_function() called from script with argument '{}'", v),
        None => println!("test_function() called from script with no arguments"),
    }
    Ok(Value::Null)
}

fn main() {
    let mut args = env::args();
    args.next();
    let script_filename = match args.next() {
        Some(f) => f,
        None => {
            println!("USAGE: bleep SCRIPT_FILENAME [SCRIPT_ARGS ...]");
            std::process::exit(1);
        }
    };
    //let script_args = args.collect::<Vec<String>>();

    let mut bleep = Bleep::new();
    bleep.set_var("test_function", Value::new_native_func(test_function));

    if let Err(e) = bleep.compile_file(script_filename) {
        println!("{}", e);
        return;
    }
    bleep.dump_bytecode();

    //bleep.dump_env();
    //bleep.dump_funcs();
    
    //let start = time::precise_time_ns();

    //let args = script_args.iter().map(|a| Value::new_string(a)).collect::<Vec<Value>>();
    //let args = [Value::new_vector(&[Value::Number(42.0), Value::Number(19.0)])];
    let mut map = Value::new_map();
    map.set_element(Value::new_string("test"), Value::Number(33.0)).unwrap();
    let args = [map];
    match bleep.call_function("main", &args) {
        Ok(v) => println!("-> {}", v),
        Err(e) => println!("{}", e),
    }
    
    //let end = time::precise_time_ns();
    //println!("time: {}ms", (end - start) / 1_000_000);
}
