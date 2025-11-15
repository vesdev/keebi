use directories::ProjectDirs;
use eyre::bail;
use hebi4::{prelude::*, value::ValueRaw};
mod module;

#[derive(argh::FromArgs)]
#[argh(description = "Simple keyboard/mouse macro scripts")]
struct Args {
    #[argh(positional)]
    script: String,

    #[argh(positional)]
    args: Vec<String>,
}

fn main() -> eyre::Result<()> {
    let args: Args = argh::from_env();

    let Some(dirs) = ProjectDirs::from("", "", "keebi") else {
        bail!("Could not locate keebi config directory");
    };

    let script_path = dirs.config_dir().join(format!("{}.hi", args.script));

    let src = std::fs::read_to_string(script_path)?;

    eval(&src, args.args)?;
    Ok(())
}

fn eval(src: &str, args: Vec<String>) -> HebiResult<ValueRaw> {
    let opts = EmitOptions::default();
    let module = Module::compile_with(None, src, opts)?;

    let native_module = module::module(args);

    let mut result = Ok(Default::default());
    Hebi::new().with(|mut vm| {
        let loaded_module = vm.load(&module);
        vm.register(&native_module);
        match vm.run(&loaded_module) {
            Ok(v) => result = Ok(v),
            Err(err) => {
                result = err.into();
            }
        }
    });

    result
}
