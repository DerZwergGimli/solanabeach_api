# \TransactionApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_latest_transactions**](TransactionApi.md#fetch_latest_transactions) | **GET** /latest-transactions | Fetch 50 latest transactions
[**fetch_transaction_by_hash**](TransactionApi.md#fetch_transaction_by_hash) | **GET** /transaction/{hash} | Fetch transaction by transaction hash
[**fetch_transaction_hashes_by_address**](TransactionApi.md#fetch_transaction_hashes_by_address) | **GET** /transaction-hashes/{address} | Fetch latest transaction hashes
[**fetch_transactions_by_address**](TransactionApi.md#fetch_transactions_by_address) | **GET** /transactions/{address} | Fetch latest transactions
[**fetch_transactions_by_block_number**](TransactionApi.md#fetch_transactions_by_block_number) | **GET** /block-transactions/{number} | Fetch latest transactions in a block



## fetch_latest_transactions

> Vec<crate::models::Transaction> fetch_latest_transactions(limit, cursor)
Fetch 50 latest transactions

Fetch 50 latest transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 100) |  |
**cursor** | Option<**String**> | Transaction cursor (blocknumber,index) |  |

### Return type

[**Vec<crate::models::Transaction>**](Transaction.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_transaction_by_hash

> crate::models::Transaction fetch_transaction_by_hash(hash)
Fetch transaction by transaction hash

Fetch transaction by transaction hash if the given transaction exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | Transaction hash | [required] |

### Return type

[**crate::models::Transaction**](Transaction.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_transaction_hashes_by_address

> Vec<crate::models::InlineResponse2001> fetch_transaction_hashes_by_address(address, limit, offset, cursor)
Fetch latest transaction hashes

Fetch latest transaction hashes for the provided address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Transaction cursor (blocknumber,index) |  |

### Return type

[**Vec<crate::models::InlineResponse2001>**](inline_response_200_1.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_transactions_by_address

> Vec<crate::models::Transaction> fetch_transactions_by_address(address, limit, offset, cursor)
Fetch latest transactions

Fetch latest transactions for the provided address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Transaction cursor (blocknumber,index) |  |

### Return type

[**Vec<crate::models::Transaction>**](Transaction.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_transactions_by_block_number

> crate::models::InlineResponse2002 fetch_transactions_by_block_number(number, limit, offset, cursor)
Fetch latest transactions in a block

Fetch latest transactions in the block with provided block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**number** | **i32** | Block number | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Transaction cursor (index) |  |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

