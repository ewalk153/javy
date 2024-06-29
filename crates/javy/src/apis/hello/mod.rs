use crate::quickjs::{context::Intrinsic, prelude::Func, qjs, Ctx, Object};
use anyhow::{Error, Result};

pub struct Hello;

impl Intrinsic for Hello {
    unsafe fn add_intrinsic(ctx: std::ptr::NonNull<qjs::JSContext>) {
        register(Ctx::from_raw(ctx)).expect("`Hello` APIs to succeed")
    }
}

fn register(cx: Ctx) -> Result<()> {
    let globals = cx.globals();
    globals.set("hello", Func::from(hello_world))?;
    let hello_obj = Object::new(cx)?;
    globals.set("world", hello_obj)?;
    // let hello: Object<'_> = globals.get("Hello").expect("Hello global to be defined");
    // math.set("random", Func::from(fastrand::f64))?;

    Ok::<_, Error>(())
}

fn hello_world() {
  println!("Hello world")
}

#[cfg(test)]
mod tests {
    use crate::{
        quickjs::{context::EvalOptions, Value},
        Runtime,
    };
    use anyhow::{Error, Result};

    #[test]
    fn test_hello() -> Result<()> {
        let runtime = Runtime::default();
        runtime.context().with(|this| {
            let mut eval_opts = EvalOptions::default();
            eval_opts.strict = false;
            this.eval_with_options("hello()", eval_opts)?;
            Ok::<_, Error>(())
        })?;

        Ok(())
    }
}