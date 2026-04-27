#[tokio::main]
async fn main() {
    use chapa_rust::client::ChapaClient;
    use chapa_rust::config::ChapaConfigBuilder;
    use chapa_rust::models::transfer::BulkTransferOptions;
    use chapa_rust::models::{bank::Currency, transfer::InitateTransferOptions};
    dotenvy::dotenv().ok();
    let config = ChapaConfigBuilder::new().build().unwrap();
    let client = ChapaClient::from_config(config).unwrap();
    let options = BulkTransferOptions {
        title: "This Month Salary!".to_string(),
        currency: Currency::ETB,
        bulk_data: vec![
            InitateTransferOptions {
                account_name: "Israel Goytom".to_string(),
                account_number: "09xxxxxxxx".to_string(),
                amount: "100".to_string(),
                reference: "b1111124".to_string(),
                bank_code: 128,
            },
            InitateTransferOptions {
                account_name: "Yesehrur Goytom".to_string(),
                account_number: "09xxxxxxxx".to_string(),
                amount: "120".to_string(),
                reference: "b1111124".to_string(),
                bank_code: 128,
            },
        ],
    };

    let result = client.bulk_transfer(options).await;
    match result {
        Ok(banks) => println!("{:#?}", banks),
        Err(e) => eprintln!("{:#?}", e),
    }
}
