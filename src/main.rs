use clap::Parser;
use env_logger::{Builder, Target};
use log::{debug, error, info};
use rlua::{Context, Error, Function, Lua, Table, Value};
use std::fs;
use std::io::Write;
use std::os::unix;
use std::path::Path;
use std::process::Command;
use which::which;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// The lua module you want to run though configz
    #[arg(short, long)]
    module: String,
}

struct Configz;

fn to_liquid_object(input: Value) -> liquid::model::Value {
    match input {
        Value::Boolean(value) => liquid::model::Value::Scalar(liquid::model::ScalarCow::new(value)),
        Value::Integer(value) => liquid::model::Value::Scalar(liquid::model::ScalarCow::new(value)),
        Value::Number(value) => liquid::model::Value::Scalar(liquid::model::ScalarCow::new(value)),
        Value::String(value) => liquid::model::Value::Scalar(liquid::model::ScalarCow::new(
            String::from_utf8(value.as_bytes().to_vec()).unwrap(),
        )),
        Value::Table(value) => {
            let is_array = match value.contains_key(1) {
                Ok(result) => result,
                Err(_) => false,
            };

            if is_array {
                let mut list = Vec::new();
                for pair in value.pairs::<String, Value>() {
                    let (_, v) = pair.unwrap();
                    list.push(to_liquid_object(v))
                }

                liquid::model::Value::Array(list)
            } else {
                let mut table = liquid::model::Object::new();
                for pair in value.pairs::<String, Value>() {
                    let (k, v) = pair.unwrap();
                    table.insert(k.into(), to_liquid_object(v));
                }

                liquid::model::Value::Object(table)
            }
        }
        _ => liquid::model::Value::Nil,
    }
}

impl Configz {
    pub fn debug<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, message: String| {
                debug!("{}", message);
                Ok(true)
            })
            .unwrap()
    }

    pub fn info<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, message: String| {
                info!("{}", message);
                Ok(true)
            })
            .unwrap()
    }

    pub fn error<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, message: String| {
                error!("{}", message);
                Ok(true)
            })
            .unwrap()
    }

    pub fn template<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, (destination, config): (String, Table)| {
                let template_file: String = match config.get("source") {
                    Ok(content) => content,
                    Err(_) => {
                        error!("[{}] missing required parameter source", destination);
                        return Ok(false);
                    }
                };

                let liquid = match liquid::ParserBuilder::with_stdlib().build() {
                    Ok(value) => value,
                    Err(err) => {
                        error!("[{}] unable to build tempalte engin, {}", destination, err);
                        return Ok(false);
                    }
                };

                let template_content = match fs::read_to_string(&template_file) {
                    Ok(value) => value,
                    Err(_) => {
                        error!(
                            "[{}] unable to read template file {}",
                            destination, template_file
                        );
                        return Ok(false);
                    }
                };

                let template = match liquid.parse(&template_content) {
                    Ok(value) => value,
                    Err(err) => {
                        error!(
                            "[{}] unable to parse tempalte {} {}",
                            destination, template_file, err
                        );
                        return Ok(false);
                    }
                };

                let mut globals = liquid::model::Object::new();
                let user_data = config.get("data");
                if user_data.is_ok() {
                    globals.insert("data".into(), to_liquid_object(user_data.unwrap()));
                }

                let content = match template.render(&globals) {
                    Ok(value) => value,
                    Err(err) => {
                        error!("[{}] unable to render template {}", destination, err);
                        return Ok(false);
                    }
                };

                let mut file = match fs::File::create(&destination) {
                    Ok(file) => file,
                    Err(_) => {
                        error!("[{}] unable to create the the file", destination);
                        return Ok(false);
                    }
                };

                info!(
                    "[{}] created from template, {}",
                    destination, &template_file
                );
                file.write_all(&content.as_bytes()).unwrap();
                Ok(true)
            })
            .unwrap()
    }

    pub fn file<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, (destination, config): (String, Table)| {
                let source_result: Result<String, Error> = config.get("source");
                if source_result.is_err() {
                    error!("[{}] missing required config 'source'", destination);
                    return Ok(false);
                }

                let source = source_result.unwrap();
                let result = fs::copy(source.clone(), destination.clone());

                match result {
                    Ok(_) => {
                        info!("[{}] copied to {}", destination, source);
                        Ok(true)
                    }
                    Err(err) => {
                        error!("[{}] filed '{}'", destination, err);
                        Ok(false)
                    }
                }
            })
            .unwrap()
    }

    pub fn link<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, (destination, config): (String, Table)| {
                let source_result: Result<String, Error> = config.get("source");
                if source_result.is_err() {
                    error!("[{}] missing required config 'source'", destination);
                    return Ok(false);
                }

                let source = source_result.unwrap();
                let result = unix::fs::symlink(source.clone(), destination.clone());

                match result {
                    Ok(_) => {
                        info!("[{}] linked to {}", destination, source);
                        Ok(true)
                    }
                    Err(err) => {
                        error!("[{}] filed '{}'", destination, err);
                        Ok(false)
                    }
                }
            })
            .unwrap()
    }

    pub fn directory<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, destination: String| {
                let result = fs::create_dir_all(destination.clone());

                match result {
                    Ok(_) => {
                        info!("[{}] created directory", destination);
                        Ok(true)
                    }
                    Err(err) => {
                        error!("[{}] filed '{}'", destination, err);
                        Ok(false)
                    }
                }
            })
            .unwrap()
    }

    pub fn run<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, command: String| {
                let mut process = Command::new("sh");
                process.arg("-c").arg(command.clone());

                let result = process.output();
                match result {
                    Ok(output) => {
                        if output.status.success() {
                            info!("[{}] exited successfully", command);
                            Ok((true, String::from_utf8(output.stdout).unwrap()))
                        } else {
                            error!("[{}] exited with {}", command, output.status);
                            Ok((
                                output.status.success(),
                                String::from_utf8(output.stderr).unwrap(),
                            ))
                        }
                    }
                    Err(err) => {
                        error!("[{}] filed '{}'", command, err);
                        Ok((false, String::from("")))
                    }
                }
            })
            .unwrap()
    }

    pub fn download<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, (destination, config): (String, Table)| {
                let url_result: Result<String, Error> = config.get("url");
                if url_result.is_err() {
                    error!("[{}] missing required config 'source'", destination);
                    return Ok(false);
                }

                let response_result = reqwest::blocking::get(url_result.unwrap().clone());
                if response_result.is_err() {
                    error!(
                        "[{}] failed to download '{}'",
                        destination,
                        response_result.unwrap_err()
                    );
                    return Ok(false);
                }

                let path = Path::new(&destination);
                let file_result = fs::File::create(path);
                if file_result.is_err() {
                    error!(
                        "[{}] unable to create file",
                        String::from(path.to_str().unwrap())
                    );
                    return Ok(false);
                }

                let result = file_result
                    .unwrap()
                    .write_all(&response_result.unwrap().bytes().unwrap());
                match result {
                    Ok(_) => {
                        info!("[{}] downloaded successfully", destination);
                        Ok(true)
                    }
                    Err(err) => {
                        error!("[{}] filed '{}'", destination, err);
                        Ok(false)
                    }
                }
            })
            .unwrap()
    }

    pub fn get_executable<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, executable: String| match which(executable) {
                Ok(path) => Ok((true, String::from(path.to_str().unwrap()))),
                Err(_) => Ok((false, String::from(""))),
            })
            .unwrap()
    }

    pub fn is_file<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, path: String| Ok(Path::new(&path).is_file()))
            .unwrap()
    }

    pub fn is_directory<'lua>(context: Context<'lua>) -> Function {
        context
            .create_function(|_, path: String| Ok(Path::new(&path).is_dir()))
            .unwrap()
    }
}

fn main() {
    let mut builder = Builder::from_default_env();
    builder.format_target(false);
    builder.format_module_path(false);
    builder.target(Target::Stdout);
    // TODO(ade): Sort our how to do this as a default filter this overrides the env variable
    // builder.filter_level(log::LevelFilter::Info);
    builder.init();

    let args = Args::parse();
    debug!("Running module, {}", args.module);

    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let configz = lua_ctx.create_table().unwrap();
        let globals = lua_ctx.globals();

        // LOGGING
        configz.set("debug", Configz::debug(lua_ctx)).unwrap();
        configz.set("info", Configz::info(lua_ctx)).unwrap();
        configz.set("error", Configz::error(lua_ctx)).unwrap();

        // RESOURCES
        configz.set("file", Configz::file(lua_ctx)).unwrap();
        configz.set("link", Configz::link(lua_ctx)).unwrap();
        configz.set("download", Configz::download(lua_ctx)).unwrap();
        configz.set("template", Configz::template(lua_ctx)).unwrap();
        configz
            .set("directory", Configz::directory(lua_ctx))
            .unwrap();
        configz.set("run", Configz::run(lua_ctx)).unwrap();

        // HELPERS
        configz.set("is_file", Configz::is_file(lua_ctx)).unwrap();
        configz
            .set("is_directory", Configz::is_directory(lua_ctx))
            .unwrap();

        configz
            .set("get_executable", Configz::get_executable(lua_ctx))
            .unwrap();

        globals.set("configz", configz).unwrap();

        let result = lua_ctx
            .load(&format!("require('{}')", args.module).to_string())
            .exec();

        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
            }
        }
    })
}
