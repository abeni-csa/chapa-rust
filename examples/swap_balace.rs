#[tokio::main]
async fn main() {
    use chapa_rust::{
        client::ChapaClient,
        config::ChapaConfigBuilder,
        models::{balances::SwapOptions, bank::Currency},
    };
    dotenvy::dotenv().ok();
    let config = ChapaConfigBuilder::new().build().unwrap();
    let client = ChapaClient::from_config(config).unwrap();
    let option = SwapOptions {
        amount: 100,
        from: Currency::USD,
        to: Currency::ETB,
    };
    let swap = client.swap_currency(option).await;
    match swap {
        Ok(swap) => println!("{:#?}", swap),
        Err(e) => eprintln!("{:#?}", e),
    }
}
