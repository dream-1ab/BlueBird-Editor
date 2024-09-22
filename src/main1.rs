use std::{env, error, fs::File, io::Read};

use rquickjs::{Object, Value};






fn main() {
    let mut runtime = rquickjs::Runtime::new().unwrap();
    let mut source_code = {
        let mut file = File::open(env::args().skip(1).take(1).next().unwrap()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    };
    let context = rquickjs::Context::full(&runtime).unwrap();
    context.with(|ctx| {
        let global = ctx.globals();
        {
            let ctx = ctx.clone();
            global.set("print", rquickjs::Function::new(ctx.clone(), |a: i32|{
                println!("{}", a);
            })).unwrap();
        }
        
        let result = ctx.eval::<(), _>(source_code).unwrap();
        
    });
    // let result = context. (None, Script::new("rust://file.js", &source_code)).unwrap();

    // let fun = JsValueFacade::new_function("print", |args| {
    //     println!("{}", args.first().unwrap().get_str());
    //     Ok(JsValueFacade::Undefined)
    // }, 1);

    // println!("{:?}", result);

    println!("Done.");
}
