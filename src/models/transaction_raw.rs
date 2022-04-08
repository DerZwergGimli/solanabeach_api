/*
 * Solanabeach Backend API
 *
 * Solanabeach Backend REST API documentation.  ## Rate limiting Current API rate limit per IP is 100 requests per second.    ## Pagination Most of the endpoints returning array data support pagination. The API uses two types of pagination, depending on the returned data. Some endpoints support both, some are limited to just one type.    ## Supported pagination types ### Offset / limit Offset / limit pagination is the simplest form of pagination supported by the API. Offset parameter represents the number of results to skip before returning the data, and the limit parameter limits the number of returned results.   To prevent overloading the API, all limit params have a max value. Each API endpoint has its own max value.  ### Cursor Cursor pagination is more complex than the offset / limit one, but, it comes naturally for some blockchain data (such as blocks, transactions, token transfers, etc). Cursors contain data like blocknumber, transaction index, ... and they're described on their respective API endpoints. Limit parameter works exactly the same way as it does in the offset / limit pagination.  ## Authentication The public API uses a Bearer OAuth authentication method, and the API key should be provided in the `Authorization` header in each request. API keys are issued on request. 
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: andrej@vgng.io
 * Generated by: https://openapi-generator.tech
 */

/// TransactionRaw : Available if the instruction can't be parsed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionRaw {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<i32>>,
    #[serde(rename = "programIdIndex", skip_serializing_if = "Option::is_none")]
    pub program_id_index: Option<i32>,
}

impl TransactionRaw {
    /// Available if the instruction can't be parsed
    pub fn new() -> TransactionRaw {
        TransactionRaw {
            data: None,
            accounts: None,
            program_id_index: None,
        }
    }
}


