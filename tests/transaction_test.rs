use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::{bank::Currency, payment::InitializeOptions},
};
use mockito::{self, Matcher};

#[tokio::test]
async fn test_initialize_transaction() {
    let mut server = mockito::Server::new_async().await;
    let success = server
            .mock("POST", "/v1/transaction/initialize")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Hosted Link",
                "status": "success",
                "data": {
                    "checkout_url": "https://checkout.chapa.co/checkout/payment/V38JyhpTygC9QimkJrdful9oEjih0heIv53eJ1MsJS6xG"
                    }
                }))
                .unwrap(),
            )
            .create_async()
            .await;

    let failure = server
        .mock("POST", "/v1/transaction/initialize")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Authorization required	",
              "status": "failed",
              "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    let transaction_success = InitializeOptions {
        amount: "100".to_string(),
        currency: Currency::ETB,
        email: Some("customer@gmail.com".to_string()),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        tx_ref: String::from("some_generated_tax_ref"),
        ..Default::default()
    };
    let transaction_failure = InitializeOptions {
        ..Default::default()
    };

    // ACT for success
    let response_success = client
        .initialize_transaction(transaction_success)
        .await
        .unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client
        .initialize_transaction(transaction_failure)
        .await
        .unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_verify_transaction() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transaction/verify/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Payment details",
            "status": "success",
            "data": {
                "first_name": "Bilen",
                "last_name": "Gizachew",
                "email": "abebech_bekele@gmail.com",
                "currency": "ETB",
                "amount": 100,
                "charge": 3.5,
                "mode": "test",
                "method": "test",
                "type": "API",
                "status": "success",
                "reference": "6jnheVKQEmy",
                "tx_ref": "chewatatest-6669",
                "customization": {
                    "title": "Payment for my favourite merchant",
                    "description": "I love online payments",
                    "logo": null
                },
                "meta": null,
                "created_at": "2023-02-02T07:05:23.000000Z",
                "updated_at": "2023-02-02T07:05:23.000000Z"
              }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/transaction/verify/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Invalid transaction or Transaction not found	",
            "status": "failed",
            "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // ACT for success
    let response_success = client.verify_transaction("chewatatest-6669").await.unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null()); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.verify_transaction("chewatatest-6669").await.unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null()); // NOTE: check if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_cancel_transaction_success() {
    let mut server = mockito::Server::new_async().await;
    let tx_ref = "tx-456-sdf";

    let success_mock = server
        .mock("PUT", format!("/v1/transaction/cancel/{}", tx_ref).as_str())
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!(
                {
                    "message": "Checkout link expired successfully",
                    "status": "success",
                    "data": {
                        "tx_ref": "tx-456-sdf",
                        "amount": 5,
                        "currency": "ETB",
                        "created_at": "2025-10-22T09:10:03.000000Z",
                        "updated_at": "2025-10-22T09:10:21.000000Z"
                    }
                }
            ))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let client = ChapaClient::from_config(config).unwrap();
    let response = client.cancel_transaction(tx_ref).await.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Checkout link expired successfully");
    assert!(response.data.is_some());

    let data = response.data.unwrap();
    assert_eq!(data.tx_ref, tx_ref);
    assert_eq!(data.amount, 5.0);
    assert_eq!(data.currency.as_str(), "ETB");

    success_mock.assert_async().await;
}

#[tokio::test]
async fn test_cancel_transaction_already_expired() {
    let mut server = mockito::Server::new_async().await;
    let tx_ref = "already-expired-ref";
    let failure_mock = server
        .mock("PUT", format!("/v1/transaction/cancel/{}", tx_ref).as_str())
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .match_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Payment link already expired",
                "status": "failed",
                "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let client = ChapaClient::from_config(config).unwrap();

    let response = client.cancel_transaction(tx_ref).await.unwrap();
    assert_eq!(response.status, "failed");
    assert_eq!(response.message, "Payment link already expired");
    assert!(response.data.is_none());

    failure_mock.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_events_success() {
    let mut server = mockito::Server::new_async().await;
    let ref_id = "chewatatest-6669";
    let success = server
        .mock("GET", format!("/v1/transaction/events/{}", ref_id).as_str())
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transaction events fetched",
                "status": "success",
                "data": [
                    {
                        "item": 23445,
                        "message": "Attempted to make payment with telebirr USSD",
                        "type": "log",
                        "created_at": "2024-07-23T07:31:32.000000Z",
                        "updated_at": "2024-07-23T07:31:32.000000Z"
                    },
                    {
                        "item": 24678,
                        "message": "Redirecting to confirmation page",
                        "type": "log",
                        "created_at": "2024-07-23T07:31:32.000000Z",
                        "updated_at": "2024-07-23T07:31:32.000000Z"
                    }
                ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;
    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();

    let client = ChapaClient::from_config(config).unwrap();

    // ACT for success
    let response_success = client
        .get_transaction_events("chewatatest-6669")
        .await
        .unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null()); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_success.data.is_some());

    success.assert_async().await;
}
#[tokio::test]
async fn test_get_transaction_events_failure() {
    let mut server = mockito::Server::new_async().await;
    let ref_id = "chewatatest-6669";

    let failure = server
        .mock("GET", format!("/v1/transaction/events/{}", ref_id).as_str())
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transaction not found",
                "status": "failed",
                "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();

    let client = ChapaClient::from_config(config).unwrap();

    // ACT for failure
    let response_failure = client
        .get_transaction_events("chewatatest-6669")
        .await
        .unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null()); // NOTE: check if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_failure.data.is_none());
    failure.assert_async().await;
}

#[tokio::test]
async fn test_get_all_transactions_success() {
    let mut server = mockito::Server::new_async().await;

    let success = server
        .mock("GET", "/v1/transactions")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transactions retrieved successfully",
                "status": "success",
                "data": {
                    "transactions": [
                        {
                            "status": "pending",
                            "ref_id": "VcEu3Hf55JU",
                            "type": "Payment Link",
                            "created_at": "2024-07-27T02:22:46.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1301688,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "R6XqfcNVQjW",
                            "type": "Payment Link",
                            "created_at": "2024-06-30T04:31:46.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1145318,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        }
                    ],
                    "pagination": {
                        "per_page": 10,
                        "current_page": 1,
                        "first_page_url": "https://api.chapa.co/v1/transactions?page=1",
                        "next_page_url": "https://api.chapa.co/v1/transactions?page=2",
                        "prev_page_url": null
                    }
                }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();

    let client = ChapaClient::from_config(config).unwrap();
    // ACT for success
    let response_success = client.get_all_transactions().await.unwrap();
    assert_eq!(response_success.status, "success");
    assert_eq!(
        response_success.message,
        "Transactions retrieved successfully"
    ); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_success.data.is_some());

    success.assert_async().await;
}

#[tokio::test]
async fn test_get_all_transactions_failure() {
    let mut server = mockito::Server::new_async().await;

    let failure = server
        .mock("GET", "/v1/transactions")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                        "message": "Invalid API Key or  the business can't accept payments at the moment. Please verify your API key and ensure the account is active and able to process payments.",
                        "status": "failed",
                        "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();

    let client = ChapaClient::from_config(config).unwrap();

    // ACT for failure
    let response_failure = client.get_all_transactions().await.unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());
    failure.assert_async().await;
}
