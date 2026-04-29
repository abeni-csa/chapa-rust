use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::{balances::SwapOptions, bank::Currency},
};
use mockito::{Matcher, Server};
use serde_json::json;

#[tokio::test]
async fn test_get_balances_success() {
    let mut server = Server::new_async().await;

    let success = server
        .mock("GET", "/v1/balances")
        .match_header("authorization", Matcher::Regex(r"^Bearer .+$".to_string()))
        .with_status(200)
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "status": "success",
            "message": "Balances fetched",
            "data": [
                {"currency": "ETB", "available_balance": 1000.0, "ledger_balance": 1200.0},
                {"currency": "USD", "available_balance": 50.0, "ledger_balance": 50.0}
            ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let client = ChapaClient::from_config(config).unwrap();

    let response = client.get_balances().await.unwrap();
    assert!(response.data.is_some());
    assert!(response.data.is_some());

    success.assert_async().await;
}

#[tokio::test]
async fn test_get_balances_failure() {
    let mut server = Server::new_async().await;

    let failure = server
        .mock("GET", "/v1/balances")
        .match_header("authorization", Matcher::Regex(r"^Bearer .+$".to_string()))
        .with_status(200)
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Invalid API Key",
              "status": "failed",
              "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let client = ChapaClient::from_config(config).unwrap();

    let response = client.get_balances().await.unwrap();
    assert_eq!(response.status, "failed");
    assert!(response.data.is_none());

    failure.assert_async().await;
}

#[tokio::test]
async fn test_swap_currency() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/swap")
        .match_header("authorization", Matcher::Regex(r"^Bearer .+$".to_string()))
        .with_status(200)
        .with_body(
            serde_json::to_string(&json!({
              "message": "Swap has been made successfully.",
              "status": "success",
              "data": {
                "status": "Success",
                "ref_id": "SWPfSqc5BiwcC",
                "from_currency": "USD",
                "to_currency": "ETB",
                "amount": 1,
                "exchanged_amount": 127,
                "charge": 0,
                "rate": 127,
                "created_at": "2025-04-23T08:50:46.000000Z",
                "updated_at": "2025-04-23T08:50:46.000000Z"
            }
             }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let client = ChapaClient::from_config(config).unwrap();

    let options = SwapOptions {
        amount: 100,
        from: Currency::USD,
        to: Currency::ETB,
    };

    let response = client.swap_currency(options).await.unwrap();
    assert!(response.data.is_some());

    assert_eq!(response.status, "success");

    mock.assert_async().await;
}
