use barter_bot::bot::bot_demo;


#[tokio::main]
async fn main() {
    // tonic_build::compile_protos("proto/mk_data.proto").unwrap();
    let _ = bot_demo::bot_run().await;
    println!("机器人启动6666!");
}
