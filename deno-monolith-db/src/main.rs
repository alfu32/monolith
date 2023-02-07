use std::env;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::Extension;
use std::rc::Rc;
use monolith_db::monolith::MonolithBackend;
use monolith_db::{id128_parse, Record};

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[op]
fn op_db_open(dbname:String) -> Result<MonolithBackend, AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());
    Ok(db)
}
#[op]
fn op_db_read(dbname:String,data:String) -> Result<(), AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());
    let result = db.read_all().unwrap();
    let (_,_,_,id)  = id128_parse(data.as_str());

    result.iter().filter(|r| (r.id == id )).for_each(|x| println!("{:#?}", x.to_json()));;
    db.close();
    Ok(())
}

#[op]
fn op_db_delete(dbname:String,data:String) -> Result<(), AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());

    let result = db.read_all().unwrap();
    let (_,_,_,id)  = id128_parse(data.as_str());

    result.iter().filter(|r| (r.id == id))
        .for_each(|x| {
            let mut y = x.clone();
            y.delete();
            db.write(y.clone()).expect("Could not delete record ");//+x.to_json().as_str()+".");
            println!("deleted {:#?}", y.to_json())
        });
    db.close();
    Ok(())
}

#[op]
fn op_db_create(dbname:String,data:String) -> Result<(), AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());
    let rr = Record::new(data.as_bytes()).tag("system".as_bytes());
    db.write(*rr.clone());
    println!("{:#?}", rr.id);
    db.close();
    Ok(())
}

#[op]
fn op_db_write(dbname:String,data:String) -> Result<(), AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());
    let record = Record::from_csv(data);
    db.write(record.clone()).expect("could not write record");
    println!("writing {} to {}", record.to_json(), dbname);;
    db.close();
    Ok(())
}

#[op]
fn op_db_read_all(dbname:String,data:String) -> Result<(), AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());
    let result = db.read_all().unwrap();
    match data.as_str() {
        "--json" => {
            result.iter().for_each(|x| println!("{}", x.to_json()));
        }
        _ => {
            println!("id;created;updated;deleted;owner;tag;data;checksum");
            result.iter().for_each(|x| println!("{}", x.to_csv()));
        }
    };
    db.close();
    Ok(())
}

#[op]
fn op_db_write_from_file(dbname:String,data:String)  -> Result<(), AnyError> {

    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), dbname.as_str());
    println!("OP_WRITE_FROM_FILE {} on {}", data,dbname);
    db.close();
    Ok(())
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder("runtime")
        .ops(vec![
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
            op_db_open::decl(),
            op_db_read::decl(),
            op_db_delete::decl(),
            op_db_create::decl(),
            op_db_write::decl(),
            op_db_read_all::decl(),
            op_db_write_from_file::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./runtime.deno.js");
    js_runtime
        .execute_script("[runjs:runtime.deno.js]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js(args[1].as_str())) {
        eprintln!("error: {}", error);
    }
}