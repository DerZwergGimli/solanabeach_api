/*
 * Solanabeach Backend API
 *
 * Solanabeach Backend REST API documentation.  ## Rate limiting Current API rate limit per IP is 100 requests per second.    ## Pagination Most of the endpoints returning array data support pagination. The API uses two types of pagination, depending on the returned data. Some endpoints support both, some are limited to just one type.    ## Supported pagination types ### Offset / limit Offset / limit pagination is the simplest form of pagination supported by the API. Offset parameter represents the number of results to skip before returning the data, and the limit parameter limits the number of returned results.   To prevent overloading the API, all limit params have a max value. Each API endpoint has its own max value.  ### Cursor Cursor pagination is more complex than the offset / limit one, but, it comes naturally for some blockchain data (such as blocks, transactions, token transfers, etc). Cursors contain data like blocknumber, transaction index, ... and they're described on their respective API endpoints. Limit parameter works exactly the same way as it does in the offset / limit pagination.  ## Authentication The public API uses a Bearer OAuth authentication method, and the API key should be provided in the `Authorization` header in each request. API keys are issued on request. 
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: andrej@vgng.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StakeAccountReward {
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i32>,
    #[serde(rename = "effectiveSlot", skip_serializing_if = "Option::is_none")]
    pub effective_slot: Option<i32>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "postBalance", skip_serializing_if = "Option::is_none")]
    pub post_balance: Option<String>,
    #[serde(rename = "percentChange", skip_serializing_if = "Option::is_none")]
    pub percent_change: Option<i32>,
    #[serde(rename = "apr", skip_serializing_if = "Option::is_none")]
    pub apr: Option<i32>,
}

impl StakeAccountReward {
    pub fn new() -> StakeAccountReward {
        StakeAccountReward {
            epoch: None,
            effective_slot: None,
            amount: None,
            post_balance: None,
            percent_change: None,
            apr: None,
        }
    }
}

