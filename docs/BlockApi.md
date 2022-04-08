# \BlockApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_block_by_block_hash**](BlockApi.md#fetch_block_by_block_hash) | **GET** /block-hash/{hash} | Fetch block by block hash
[**fetch_block_by_block_number**](BlockApi.md#fetch_block_by_block_number) | **GET** /block/{number} | Fetch block by block number
[**fetch_latest_blocks**](BlockApi.md#fetch_latest_blocks) | **GET** /latest-blocks | Fetch 50 latest blocks
[**fetch_top_programs**](BlockApi.md#fetch_top_programs) | **GET** /top-programs | Fetch top program stats for last 1000 blocks



## fetch_block_by_block_hash

> crate::models::Block fetch_block_by_block_hash(hash)
Fetch block by block hash

Fetch block by block hash if the given block exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | Block hash | [required] |

### Return type

[**crate::models::Block**](Block.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_block_by_block_number

> crate::models::Block fetch_block_by_block_number(number)
Fetch block by block number

Fetch block by block number if the given block exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**number** | **String** | Block number | [required] |

### Return type

[**crate::models::Block**](Block.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_latest_blocks

> Vec<crate::models::Block> fetch_latest_blocks(limit, cursor)
Fetch 50 latest blocks

Fetch 50 latest blocks ordered by block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 100) |  |
**cursor** | Option<**String**> | Block cursor (blocknumber) |  |

### Return type

[**Vec<crate::models::Block>**](Block.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_top_programs

> Vec<crate::models::InlineResponse200> fetch_top_programs()
Fetch top program stats for last 1000 blocks

Fetch top program stats for last 1000 blocks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse200>**](inline_response_200.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

