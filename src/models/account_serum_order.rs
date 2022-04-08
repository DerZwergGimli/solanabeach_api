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
pub struct AccountSerumOrder {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "blocknumber", skip_serializing_if = "Option::is_none")]
    pub blocknumber: Option<i32>,
    #[serde(rename = "transactionhash", skip_serializing_if = "Option::is_none")]
    pub transactionhash: Option<String>,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<Box<crate::models::AccountSerumOrderMarket>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::Address>>,
    #[serde(rename = "openorders", skip_serializing_if = "Option::is_none")]
    pub openorders: Option<Box<crate::models::Address>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "pricelimit", skip_serializing_if = "Option::is_none")]
    pub pricelimit: Option<i32>,
    #[serde(rename = "maxquantity", skip_serializing_if = "Option::is_none")]
    pub maxquantity: Option<i32>,
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "processedat", skip_serializing_if = "Option::is_none")]
    pub processedat: Option<i32>,
    #[serde(rename = "txindex", skip_serializing_if = "Option::is_none")]
    pub txindex: Option<i32>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Box<crate::models::Timestamp>>,
    #[serde(rename = "ondemand", skip_serializing_if = "Option::is_none")]
    pub ondemand: Option<bool>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    #[serde(rename = "qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<f32>,
}

impl AccountSerumOrder {
    pub fn new() -> AccountSerumOrder {
        AccountSerumOrder {
            id: None,
            blocknumber: None,
            transactionhash: None,
            market: None,
            owner: None,
            openorders: None,
            side: None,
            _type: None,
            pricelimit: None,
            maxquantity: None,
            valid: None,
            status: None,
            processedat: None,
            txindex: None,
            timestamp: None,
            ondemand: None,
            price: None,
            qty: None,
        }
    }
}

