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
pub struct ValidatorValidator {
    #[serde(rename = "activatedStake", skip_serializing_if = "Option::is_none")]
    pub activated_stake: Option<i32>,
    #[serde(rename = "stakePercentage", skip_serializing_if = "Option::is_none")]
    pub stake_percentage: Option<i32>,
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<i32>,
    #[serde(rename = "epochCredits", skip_serializing_if = "Option::is_none")]
    pub epoch_credits: Option<Vec<i32>>,
    #[serde(rename = "epochVoteAccount", skip_serializing_if = "Option::is_none")]
    pub epoch_vote_account: Option<bool>,
    #[serde(rename = "lastVote", skip_serializing_if = "Option::is_none")]
    pub last_vote: Option<i32>,
    #[serde(rename = "nodePubkey", skip_serializing_if = "Option::is_none")]
    pub node_pubkey: Option<String>,
    #[serde(rename = "rootSlot", skip_serializing_if = "Option::is_none")]
    pub root_slot: Option<i32>,
    #[serde(rename = "votePubkey", skip_serializing_if = "Option::is_none")]
    pub vote_pubkey: Option<String>,
    #[serde(rename = "blockProduction", skip_serializing_if = "Option::is_none")]
    pub block_production: Option<Box<crate::models::ValidatorValidatorBlockProduction>>,
    #[serde(rename = "delegatingStakeAccounts", skip_serializing_if = "Option::is_none")]
    pub delegating_stake_accounts: Option<Vec<crate::models::StakeAccount>>,
    #[serde(rename = "delegatorCount", skip_serializing_if = "Option::is_none")]
    pub delegator_count: Option<i32>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::models::NonvalidatorsLocation>>,
    #[serde(rename = "moniker", skip_serializing_if = "Option::is_none")]
    pub moniker: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "pictureURL", skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "asn", skip_serializing_if = "Option::is_none")]
    pub asn: Option<Box<crate::models::ValidatorValidatorAsn>>,
}

impl ValidatorValidator {
    pub fn new() -> ValidatorValidator {
        ValidatorValidator {
            activated_stake: None,
            stake_percentage: None,
            commission: None,
            epoch_credits: None,
            epoch_vote_account: None,
            last_vote: None,
            node_pubkey: None,
            root_slot: None,
            vote_pubkey: None,
            block_production: None,
            delegating_stake_accounts: None,
            delegator_count: None,
            location: None,
            moniker: None,
            website: None,
            picture_url: None,
            version: None,
            details: None,
            asn: None,
        }
    }
}

