use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::payment::{CreateSubaccountOptions, SplitType},
};
use mockito::{Matcher, Server};
use serde_json::json;

#[tokio::test]
async fn test_create_subaccount() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/subaccount")
        .match_header("authorization", Matcher::Regex(r"^Bearer .+$".to_string()))
        .with_status(200)
        .with_body(
            serde_json::to_string(&json!({
                "status": "success",
                "message": "Subaccount created",
                "data": {
                     "id": "837b4e5e-57c8-4e39-b2df-66e7886b8bdb"
               }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("test-key")
        .build()
        .unwrap();
    let client = ChapaClient::from_config(config).unwrap();

    let subaccount = CreateSubaccountOptions {
        account_name: "Abebe Bikila ".to_string(),
        bank_code: 128,
        account_number: "0123456789".to_string(),
        split_type: Some(SplitType::PERCENTAGE),
        split_value: Some(0.2),
    };
    let response = client.create_subaccount(subaccount).await.unwrap();

    assert_eq!(response.status.as_str(), "success");
    assert!(!response.message.is_null());
    assert!(response.data.is_some());

    mock.assert_async().await;
}
