use deno_core::{extension, op2, FsModuleLoader, JsRuntime, RuntimeOptions};
use deno_core::error::AnyError;

use tokio::runtime::Builder;

use std::rc::Rc;
use std::env;

#[op2(async)]
#[string]
async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
  let body = reqwest::get(url).await?.text().await?;
  Ok(body)
}

extension!(
    custom_runtime,
    ops = [op_fetch],
);

struct CustomJsRuntime;

impl CustomJsRuntime {
    async fn run(file_path: &str) -> Result<(), AnyError> {
        let main_module =
        deno_core::resolve_path(file_path, &env::current_dir()?)?;
        
        let mut js_runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            extensions: vec![custom_runtime::init_ops()],
            ..Default::default()
        });

        js_runtime.execute_script("[custom_runtime:bootstrap.js]", include_str!("bootstrap.js")).unwrap();
        
        let module_id = js_runtime.load_main_es_module(&main_module).await?;
        let evaluate_result = js_runtime.mod_evaluate(module_id);

        js_runtime.run_event_loop(Default::default()).await?;
        return evaluate_result.await
    }
}



fn main() {
    let runtime = Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap();

      if let Err(error) = runtime.block_on(CustomJsRuntime::run("src/app.js")) {
        eprintln!("error: {}", error);
      }
}