use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct ModelCallArgs {
    store_id: i32,
    genre_id: i32,
    request_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Cp01OutputCtx {
    model_call_args: ModelCallArgs,
}

fn parse_ctx_json(path: &str) -> Cp01OutputCtx {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let ctx: Cp01OutputCtx = serde_json::from_reader(reader).unwrap();
    return ctx;
}
fn print_as_json(ctx: &Cp01OutputCtx) -> serde_json::Result<()> {
    let json_str = serde_json::to_string(&ctx)?;
    println!("{}", json_str);
    Ok(())
}
fn main() {
    let ctx = parse_ctx_json("./test_fixtures/cp01.json");
    println!("-----(Deserialized Oject)---- \nctx: {:?}\n-----------------\n", ctx);
    print_as_json(&ctx).unwrap();
}
