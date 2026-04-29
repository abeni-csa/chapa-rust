//! Client module for interacting with the Chapa API.
//! This module provides the `ChapaClient` struct, which encapsulates
//! methods for initializing transactions, verifying payments, and retrieving bank information.
//! It leverages the `reqwest` crate for HTTP requests and handles authentication
//! using bearer tokens.
//! ## Example
//! ```rust,no_run
//! use chapa_rust::client::ChapaClient;
//! use chapa_rust::config::ChapaConfigBuilder;
//!
//! let chapa_client = ChapaClient::new("your_secret_key").unwrap();
//! // or using a custom config
//! let config = ChapaConfigBuilder::new().build().unwrap();
//! let chapa_client = ChapaClient::from_config(config).unwrap();
//! ```
//! # Errors
//! Errors encountered during API interactions are represented by the
//! [`ChapaError`] enum.
use std::collections::HashMap;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::{
    config::{ChapaConfig, ChapaConfigBuilder},
    error::{ChapaError, Result},
    models::{
        balances::SwapOptions,
        bank::Currency,
        payment::{CreateSubaccountOptions, InitializeOptions},
        response::{
            BulkTransferResponse, GetAllTransactionResponse, GetAllTransfersResponse,
            GetBalanceResponse, GetBanksResponse, GetSingleCurrencyBalanceResponse,
            GetSwapResponse, InitializeResponse, SubaccountResponse, TransactionEventsResponse,
            TransferResponse, VerifyResponse,
        },
        transaction::CancelTransactionResponse,
        transfer::{BulkTransferOptions, InitateTransferOptions},
    },
};

/// Client for interacting with the Chapa API.
/// # Example
/// ```rust,no_run
/// use chapa_rust::client::ChapaClient;
/// let chapa_client = ChapaClient::new("your_secret_key").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ChapaClient {
    http: Client,
    config: ChapaConfig,
}

/// Enum for representing different body formats.
pub enum RequestBody<T> {
    /// In case of No Body Contete like (())
    None,
    /// Representaion of Json body type
    Json(T),
    /// serializes as application/x-www-form-urlencoded
    Form(T),
}
impl ChapaClient {
    /// Creates a new ChapaClient with the provided secret key.
    pub fn new(secret_key: impl Into<String>) -> Result<Self> {
        let config = ChapaConfigBuilder::new().api_key(secret_key).build()?;
        let http = Client::builder().timeout(config.timeout).build()?;
        Ok(Self { http, config })
    }

    /// Creates a new `ChapaClient` from an existing `ChapaConfig`.
    /// You can build a [`ChapaConfig`] using [`ChapaConfigBuilder`].
    pub fn from_config(config: ChapaConfig) -> Result<Self> {
        let http = Client::builder().timeout(config.timeout).build()?;
        Ok(Self { http, config })
    }

    /// Helper function to convert the default_headers of [ChapaConfig] into a HeaderMap for reqwest requests.
    /// # Errors
    /// Returns an error if any header value is invalid.
    fn build_header(headers: &HashMap<String, String>) -> Result<HeaderMap> {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let header_key = HeaderName::try_from(key)
                .map_err(|e| ChapaError::InvalidHeaderName(format!("{}: {}", key, e)))?;
            let header_value = HeaderValue::try_from(value)
                .map_err(|e| ChapaError::InvalidHeaderValue(format!("{}: {}", value, e)))?;

            header_map.insert(header_key, header_value);
        }
        Ok(header_map)
    }

    /// Helper function to make a generic GET or POST request to the Chapa API.
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    async fn make_request<T, K>(
        &self,
        endpoint: &str,
        method: &str,
        body: RequestBody<K>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        K: serde::Serialize,
    {
        let url = format!(
            "{}/{}/{}",
            self.config.base_url, self.config.version, endpoint
        );
        let headers = Self::build_header(&self.config.default_headers)?;
        let method = reqwest::Method::try_from(method)
            .map_err(|e| ChapaError::InvalidHttpMethod(format!("{}: {}", method, e)))?;

        let mut request = self.http.request(method, url);
        request = match body {
            RequestBody::None => request,
            RequestBody::Json(data) => request.json(&data),
            RequestBody::Form(data) => request.form(&data),
        };

        Ok(request
            .bearer_auth(&self.config.api_key)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    /// Retrieves the list of all banks supported by Chapa.
    ///
    /// This function makes a `GET` request to the `/banks` endpoint and
    /// deserializes the JSON response into a [`GetBanksResponse`] struct.
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::client::ChapaClient;
    /// use chapa_rust::config::ChapaConfigBuilder;
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let banks = client.get_banks().await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn get_banks(&mut self) -> Result<GetBanksResponse> {
        let response = self
            .make_request::<GetBanksResponse, ()>("banks", "GET", RequestBody::None)
            .await?;

        Ok(response)
    }

    /// Initializes a new transaction with Chapa.
    ///
    /// Sends a `POST` request to `/transaction/initialize` with transaction
    /// details provided in the [`InitializeOptions`] struct.
    ///
    /// # Parameters
    /// - `transaction`: The transaction details (amount, currency, customer info, etc.)
    ///
    /// # Example
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder, models::payment::InitializeOptions};
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let transaction = InitializeOptions {
    ///         amount: "100".to_string(),
    ///         currency: "ETB".to_string(),
    ///         email: Some("customer@gmail.com".to_string()),
    ///         first_name: Some("John".to_string()),
    ///         last_name: Some("Doe".to_string()),
    ///         tx_ref: String::from("some_generated_tax_ref"),
    ///         ..Default::default()
    ///     };
    /// let response = client.initialize_transaction(transaction).await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    pub async fn initialize_transaction(
        &mut self,
        transaction: InitializeOptions,
    ) -> Result<InitializeResponse> {
        let response = self
            .make_request::<InitializeResponse, InitializeOptions>(
                "transaction/initialize",
                "POST",
                RequestBody::Json(transaction),
            )
            .await?;

        Ok(response)
    }

    /// Verifies the status of a transaction using its reference ID.
    ///
    /// This function makes a `GET` request to `/transaction/verify/{tx_ref}`
    /// and returns the transaction’s verification details.
    ///
    /// # Parameters
    /// - `tx_ref`: A unique reference string identifying the transaction.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let tx_ref = "your_transaction_reference";
    /// let response = client.verify_transaction(tx_ref).await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    pub async fn verify_transaction(&mut self, tx_ref: &str) -> Result<VerifyResponse> {
        let endpoint = format!("transaction/verify/{}", tx_ref);

        let response = self
            .make_request::<VerifyResponse, ()>(endpoint.as_str(), "GET", RequestBody::None)
            .await?;

        Ok(response)
    }

    /// Cancels an active transaction, expiring its checkout link.
    /// This Functions `PUT` to request `transaction/cancel/{tx_ref}`
    /// and returns the transaction’s cancelation details.
    ///
    /// # Arguments
    /// * `tx_ref` - Your transaction reference used when initializing the payment.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let tx_ref = "tx-456-sdf";
    /// let response = client.cancel_transaction(tx_ref).await.unwrap();
    /// }
    /// ```
    pub async fn cancel_transaction(&self, tx_ref: &str) -> Result<CancelTransactionResponse> {
        let endpoint = format!("transaction/cancel/{}", tx_ref);
        let response = self
            .make_request::<CancelTransactionResponse, ()>(
                endpoint.as_str(),
                "PUT",
                RequestBody::None,
            )
            .await?;
        Ok(response)
    }

    /// Retrieves the event timeline for a given transaction reference ID.
    /// This Functions `GET` to request `transaction/events/{ref_id}`
    /// and allows you to view the timeline for a transaction.
    /// A transaction timeline is a list of events that happened to a selected transaction
    ///
    /// # Arguments
    /// * `ref_id` - the reference id to that specific transaction
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let ref_id = "chewatatest-6669";
    /// let response = client.get_transaction_events(ref_id).await.unwrap();
    /// }
    /// ```
    pub async fn get_transaction_events(&self, ref_id: &str) -> Result<TransactionEventsResponse> {
        let endpoint = format!("transaction/events/{}", ref_id);
        let response = self
            .make_request::<TransactionEventsResponse, ()>(
                endpoint.as_str(),
                "GET",
                RequestBody::None,
            )
            .await?;
        Ok(response)
    }
    /// Retrieves a list of all transactions
    ///
    /// This function makes a `GET` request to the `/transactions` endpoint and
    /// deserializes the JSON response into a [`GetTransactionsResponse`] struct.
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///   use chapa_rust::client::ChapaClient;
    ///   use chapa_rust::config::ChapaConfigBuilder;
    ///   dotenvy::dotenv().ok();
    ///   let config = ChapaConfigBuilder::new().build().unwrap();
    ///   let mut client = ChapaClient::from_config(config).unwrap();
    ///   let banks = client.get_all_transactions().await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn get_all_transactions(&self) -> Result<GetAllTransactionResponse> {
        let respose = self
            .make_request::<GetAllTransactionResponse, ()>("transactions", "GET", RequestBody::None)
            .await?;
        Ok(respose)
    }
    ///  Create a subaccountCreate a subaccount    
    /// Sends a `POST` request to `/subaccount` with subaccout data
    /// details provided in the [`CreateSubaccountOptions`] struct.
    ///
    /// # Parameters
    /// - `subaccount`: The subaccout details (account_name, bank_code, split_value, etc.)
    ///
    /// # Example
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{
    ///       client::ChapaClient,
    ///       config::ChapaConfigBuilder,
    ///       models::payment::{CreateSubaccountOptions, SplitType}
    /// };
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let subaccount = CreateSubaccountOption {
    ///         account_name: "Abebe Bikila ".to_string(),
    ///         bank_code: 128,
    ///         account_number: "0123456789".to_string(),
    ///         split_type: Some(SplitType::PERCENTAGE),
    ///         split_value: Some(0.2),
    ///     };
    /// let response = client.create_subaccount(subaccount).await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    pub async fn create_subaccount(
        &self,
        subaccount: CreateSubaccountOptions,
    ) -> Result<SubaccountResponse> {
        let response = self
            .make_request::<SubaccountResponse, CreateSubaccountOptions>(
                "subaccount",
                "POST",
                RequestBody::Json(subaccount),
            )
            .await?;
        Ok(response)
    }

    /// Initiates a single transfer.
    pub async fn initiate_transfer(
        &self,
        options: InitateTransferOptions,
    ) -> Result<TransferResponse> {
        let response = self
            .make_request::<TransferResponse, InitateTransferOptions>(
                "transfers",
                "POST",
                RequestBody::Json(options),
            )
            .await?;
        Ok(response)
    }

    /// Verifies a transfer by its transaction reference.
    pub async fn verify_transfer(&self, tx_ref: &str) -> Result<TransferResponse> {
        let endpoint = format!("transfers/verify/{}", tx_ref);
        let response = self
            .make_request::<TransferResponse, ()>(endpoint.as_str(), "GET", RequestBody::None)
            .await?;

        Ok(response)
    }

    /// Initiates a bulk transfer.
    ///
    /// Sends a `POST` request to `/bulk-transfers` with a list of individual transfers.
    ///
    /// # Parameters
    /// - `options`: Bulk transfer options containing a vector of individual transfer details.
    ///
    /// # Example
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use chapa_rust::client::ChapaClient;
    ///     use chapa_rust::config::ChapaConfigBuilder;
    ///     use chapa_rust::models::transfer::BulkTransferOptions;
    ///     use chapa_rust::models::{bank::Currency, transfer::InitateTransferOptions};
    ///     dotenvy::dotenv().ok();
    ///     let config = ChapaConfigBuilder::new().build().unwrap();
    ///     let client = ChapaClient::from_config(config).unwrap();
    ///     let options = BulkTransferOptions {
    ///         title: "This Month Salary!".to_string(),
    ///         currency: Currency::ETB,
    ///         bulk_data: vec![
    ///             InitateTransferOptions {
    ///                 account_name: "Israel Goytom".to_string(),
    ///                 account_number: "09xxxxxxxx".to_string(),
    ///                 amount: "100".to_string(),
    ///                 reference: "qwertyuuiop".to_string(),
    ///                 bank_code: 128,
    ///             },
    ///             InitateTransferOptions {
    ///                 account_name: "Abenezer Haymanot".to_string(),
    ///                 account_number: "09xxxxxxxx".to_string(),
    ///                 amount: "120".to_string(),
    ///                 reference: "kjl1139471".to_string(),
    ///                 bank_code: 128,
    ///             },
    ///         ],
    ///     };
    ///     let result = client.bulk_transfer(options).await;
    ///     match result {
    ///         Ok(banks) => println!("{:#?}", banks),
    ///         Err(e) => eprintln!("{:#?}", e),
    ///     }
    /// }
    /// ```
    pub async fn bulk_transfer(
        &self,
        options: BulkTransferOptions,
    ) -> Result<BulkTransferResponse> {
        let respose = self
            .make_request::<BulkTransferResponse, BulkTransferOptions>(
                "bulk-transfers",
                "POST",
                RequestBody::Json(options),
            )
            .await?;
        Ok(respose)
    }

    /// Lists all transfers, optionally filtered by date, currency, or status.
    ///
    /// This function sends a `GET` request to `/transfers` with optional query parameters
    /// for filtering the results. The response contains paginated transfer data.
    ///
    /// # Parameters
    /// - `from_date`: Optional start date/time filter (UTC, format `YYYY-MM-DD` or `YYYY-MM-DDTHH:mm:ss`).
    /// - `to_date`: Optional end date/time filter (UTC, must be ≥ `from_date` if both are provided).
    /// - `currency`: Optional currency filter (e.g., `"ETB"`, `"USD"`).
    /// - `status`: Optional status filter (e.g., `"success"`, `"pending"`, `"failed"`).
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     use chapa_rust::client::ChapaClient;
    ///     use chapa_rust::config::ChapaConfigBuilder;
    ///     dotenvy::dotenv().ok();
    ///     let config = ChapaConfigBuilder::new().build().unwrap();
    ///     let mut client = ChapaClient::from_config(config).unwrap();
    ///     // Get all transfers without filters
    ///     let all = client.get_all_transfers(None, None, None, None).await.unwrap();
    ///     // Filter by date range and status
    ///     let filtered = client.get_all_transfers(
    ///         Some("2025-03-01"),
    ///         Some("2025-03-10"),
    ///         Some("ETB"),
    ///         Some("success"),
    ///     ).await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    pub async fn get_all_transfers(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        currency: Option<&str>,
        status: Option<&str>,
    ) -> Result<GetAllTransfersResponse> {
        let mut endpoint = "transfers".to_string();
        // Build query parameters
        let mut query_params = Vec::new();
        if let Some(d) = from_date {
            query_params.push(("from_date", d));
        }
        if let Some(d) = to_date {
            query_params.push(("to_date", d));
        }
        if let Some(c) = currency {
            query_params.push(("currency", c));
        }
        if let Some(s) = status {
            query_params.push(("status", s));
        }
        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            endpoint.push('?');
            endpoint.push_str(&query_string);
        }
        let respose = self
            .make_request::<GetAllTransfersResponse, ()>(
                endpoint.as_str(),
                "GET",
                RequestBody::None,
            )
            .await?;
        Ok(respose)
    }

    /// Gets balances for all currencies.
    ///
    /// This balance API allows you to retrieve your current account balance information from Chapa
    /// useful for checking available funds before initiating transfers or for reconciliation purposes
    ///
    /// This function sends a `GET` request to `/balances`
    /// The response contains current account balance information from Chapa.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::client::ChapaClient;
    /// use chapa_rust::config::ChapaConfigBuilder;
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let  get_balances= client.get_balances().await;
    ///     match get_balances{
    ///         Ok(balances ) => println!("{:#?}", balances),
    ///         Err(e) => eprintln!("{:#?}", e),
    ///     }
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn get_balances(&self) -> Result<GetBalanceResponse> {
        let response = self
            .make_request::<GetBalanceResponse, ()>("balances", "GET", RequestBody::None)
            .await?;

        Ok(response)
    }

    /// Gets balances by currencies.
    ///
    /// This balance by filter API allows you to retrieve balance information for a
    /// specific currency by appending the currency code to the endpoint
    /// useful for checking available funds before initiating transfers or for reconciliation purposes
    ///
    /// This function sends a `GET` request to `/balances/{usd||etb}`
    ///  filter balance information for a specific currency by appending the currency code to the endpoint
    /// The response contains current account balance information from Chapa.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///    use chapa_rust::client::ChapaClient;
    ///    use chapa_rust::config::ChapaConfigBuilder;
    ///    dotenvy::dotenv().ok();
    ///    let config = ChapaConfigBuilder::new().build().unwrap();
    ///    let mut client = ChapaClient::from_config(config).unwrap();
    ///    let  get_balances= client.get_balances().await;
    ///        match result {
    ///            Ok(balances ) => println!("{:#?}", balances),
    ///            Err(e) => eprintln!("{:#?}", e),
    ///        }
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn get_balance_by_currency(
        &self,
        currency: Currency,
    ) -> Result<GetSingleCurrencyBalanceResponse> {
        let endpoint = format!("balances/{}", currency);
        let response = self
            .make_request::<GetSingleCurrencyBalanceResponse, ()>(
                endpoint.to_lowercase().as_str(),
                "GET",
                RequestBody::None,
            )
            .await?;

        Ok(response)
    }

    /// Swap API for converting USD to ETB
    ///
    /// ## Important Notes
    /// - The minimum amount for conversion is 1 USD
    /// - Current exchange rate is applied at the time of the swap
    /// - Swaps are processed immediately and cannot be reversed
    /// - The maximum allowed amount for swap is $10,000
    /// - The exchanged amount will be added to the business ETB balance
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     use chapa_rust::{
    ///         client::ChapaClient,
    ///         config::ChapaConfigBuilder,
    ///         models::{balances::SwapOptions, bank::Currency},
    ///     };
    ///     dotenvy::dotenv().ok();
    ///     let config = ChapaConfigBuilder::new().build().unwrap();
    ///     let mut client = ChapaClient::from_config(config).unwrap();
    ///     let option = SwapOptions {
    ///         amount: 100.0,
    ///         from: Currency::USD,
    ///         to: Currency::ETB,
    ///     };
    ///     let swap = client.swap_currency(option).await;
    ///     match swap {
    ///         Ok(swap) => println!("{:#?}", swap),
    ///         Err(e) => eprintln!("{:#?}", e),
    ///     }
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn swap_currency(&self, options: SwapOptions) -> Result<GetSwapResponse> {
        let response = self
            .make_request::<GetSwapResponse, SwapOptions>(
                "swap",
                "POST",
                RequestBody::Json(options),
            )
            .await?;

        Ok(response)
    }
}
