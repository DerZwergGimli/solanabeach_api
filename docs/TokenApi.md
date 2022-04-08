# \TokenApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_token**](TokenApi.md#fetch_token) | **GET** /token/{pubkey} | Fetch single token
[**fetch_token_accounts**](TokenApi.md#fetch_token_accounts) | **GET** /token-accounts/{owner} | Fetch token accounts owned by owner
[**fetch_token_holders**](TokenApi.md#fetch_token_holders) | **GET** /token/{mint}/holders | Fetch token holders
[**fetch_token_transfers**](TokenApi.md#fetch_token_transfers) | **GET** /token/{mint}/transfers | Fetch token transfers
[**fetch_token_transfers_by_owner**](TokenApi.md#fetch_token_transfers_by_owner) | **GET** /token-transfers/{owner} | Fetch token transfers by owner
[**fetch_tokens**](TokenApi.md#fetch_tokens) | **GET** /tokens | Fetch tokens



## fetch_token

> crate::models::Token fetch_token(pubkey)
Fetch single token

Fetch token by pubkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Token address | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_token_accounts

> Vec<crate::models::TokenHolder> fetch_token_accounts(owner, limit, offset)
Fetch token accounts owned by owner

Fetch token accounts owned by owner ordered by amount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | Owner address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::TokenHolder>**](TokenHolder.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_token_holders

> Vec<crate::models::TokenHolder> fetch_token_holders(mint, limit, offset)
Fetch token holders

Fetch token holders ordered by amount. Please note that this endpoint only returns appropriate holders for known tokens (the ones in the official Solana token list).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint** | **String** | Mint address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::TokenHolder>**](TokenHolder.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_token_transfers

> Vec<crate::models::TokenTransfer> fetch_token_transfers(mint, limit, offset, cursor)
Fetch token transfers

Fetch token transfers ordered by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint** | **String** | Mint address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Token transfers cursor (blocknumber,txindex) |  |

### Return type

[**Vec<crate::models::TokenTransfer>**](TokenTransfer.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_token_transfers_by_owner

> Vec<crate::models::TokenTransfer> fetch_token_transfers_by_owner(owner, limit, offset, cursor)
Fetch token transfers by owner

Fetch token transfers by owner ordered by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | Owner address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Token transfers cursor (blocknumber,txindex) |  |

### Return type

[**Vec<crate::models::TokenTransfer>**](TokenTransfer.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_tokens

> crate::models::InlineResponse20017 fetch_tokens(limit, offset, sort, dir)
Fetch tokens

Fetch tokens ordered by holders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**sort** | Option<**String**> | Sort by param |  |
**dir** | Option<**String**> | Sort direction param |  |

### Return type

[**crate::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

