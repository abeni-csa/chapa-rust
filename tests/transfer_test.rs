use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
use mockito::{self, Matcher};

#[tokio::test]
async fn test_get_all_transfers_no_filters() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({

              "message": "Transfer details fetched",
              "status": "success",
              "meta": {
                  "current_page": 1,
                  "first_page_url": "https://api.chapa.co/v1/transfers?page=1",
                  "last_page": 16,
                  "last_page_url": "https://api.chapa.co/v1/transfers?page=16",
                  "next_page_url": "https://api.chapa.co/v1/transfers?page=2",
                  "path": "https://api.chapa.co/v1/transfers?page=1",
                  "per_page": 10,
                  "prev_page_url": null,
                  "to": 10,
                  "total": 159,
                  "error": []
              },
              "data": [
                  {
                      "account_name": "suz",
                      "account_number": "1",
                      "currency": "ETB",
                      "amount": 1,
                      "charge": 0,
                      "transfer_type": "bank",
                      "chapa_reference": "7039636706566",
                      "bank_code": 656,
                      "bank_name": "Awash Bank",
                      "bank_reference": null,
                      "status": "failed/cancelled",
                      "reference": null,
                      "created_at": "2022-10-24T14:46:56.000000Z",
                      "updated_at": "2023-08-07T10:49:59.000000Z"
                  },
                  {
                      "account_name": "Tamiru",
                      "account_number": "1000089352731",
                      "currency": "ETB",
                      "amount": 5,
                      "charge": 0,
                      "transfer_type": "bank",
                      "chapa_reference": "FFY5w8lEYRU",
                      "bank_code": 893,
                      "bank_name": "Cooperative Bank of Oromia (COOP)",
                      "bank_reference": null,
                      "status": "failed/cancelled",
                      "reference": "FFY5w8lEYRU",
                      "created_at": "2023-05-24T13:48:42.000000Z",
                      "updated_at": "2023-10-10T09:21:27.000000Z"
                  }
              ]
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

    let response = client
        .get_all_transfers(None, None, None, None)
        .await
        .unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer details fetched");
    assert!(response.data.is_some());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_all_transfers_with_filters() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/v1/transfers?from_date=2025-01-01&to_date=2025-01-31&currency=ETB&status=success",
        )
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transfer details fetched",
                "status": "success",
                "meta": {
                    "current_page": 1,
                    "first_page_url": "https://api.chapa.co/v1/transfers?page=1",
                    "last_page": 16,
                    "last_page_url": "https://api.chapa.co/v1/transfers?page=16",
                    "next_page_url": "https://api.chapa.co/v1/transfers?page=2",
                    "path": "https://api.chapa.co/v1/transfers?page=1",
                    "per_page": 10,
                    "prev_page_url": null,
                    "to": 10,
                    "total": 159,
                    "error": []
                },
                "data": [
                    {
                        "account_name": "suz",
                        "account_number": "1",
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "7039636706566",
                        "bank_code": 656,
                        "bank_name": "Awash Bank",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": null,
                        "created_at": "2022-10-24T14:46:56.000000Z",
                        "updated_at": "2023-08-07T10:49:59.000000Z"
                    },
                    {
                        "account_name": "suz",
                        "account_number": "1",
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "7039636674388",
                        "bank_code": 656,
                        "bank_name": "Awash Bank",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": "7039636674388",
                        "created_at": "2022-10-24T14:48:06.000000Z",
                        "updated_at": "2023-10-10T09:20:27.000000Z"
                    }
                ]
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

    let response = client
        .get_all_transfers(
            Some("2025-01-01"),
            Some("2025-01-31"),
            Some("ETB"),
            Some("success"),
        )
        .await
        .unwrap();
    assert_eq!(response.status, "success");
    assert!(!response.message.is_empty());
    assert!(response.data.is_some());
    mock.assert_async().await;
}
#[tokio::test]
async fn test_bulk_transfers() {}
