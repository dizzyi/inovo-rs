use inovo_rs::util::scan_for_all_psu;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("roslibrust=error")
        .init();

    let record = scan_for_all_psu().await;
    println!("{:#?}", record);
}
