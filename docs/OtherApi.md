# \OtherApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_alias**](OtherApi.md#fetch_alias) | **GET** /alias | Fetch alias
[**fetch_cluster_time**](OtherApi.md#fetch_cluster_time) | **GET** /cluster-time | Fetch current cluster time
[**fetch_epoch_history**](OtherApi.md#fetch_epoch_history) | **GET** /epoch-history | Fetch epoch history
[**fetch_health**](OtherApi.md#fetch_health) | **GET** /health | Fetch performance info
[**fetch_market_chard_data**](OtherApi.md#fetch_market_chard_data) | **GET** /market-chart-data | Fetch market chart data
[**fetch_network_status**](OtherApi.md#fetch_network_status) | **GET** /network-status | Fetch network status
[**fetch_non_validators**](OtherApi.md#fetch_non_validators) | **GET** /non-validators | Fetch non validator nodes
[**fetch_stake_history**](OtherApi.md#fetch_stake_history) | **GET** /stake-history | Fetch stake history
[**fetch_staking_apy**](OtherApi.md#fetch_staking_apy) | **GET** /staking-apy | Fetch staking APY



## fetch_alias

> crate::models::InlineResponse20014 fetch_alias()
Fetch alias

Fetch alias data. Aliases represent an address to metadata mapping that includes human readable address names and token metadata (such as ticker, website, ...). It includes all tokens from the official Solana token list, most of the known programs and DEX markets.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_cluster_time

> i32 fetch_cluster_time()
Fetch current cluster time

Fetch current cluster time

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_epoch_history

> crate::models::InlineResponse20013 fetch_epoch_history()
Fetch epoch history

Fetch epoch history

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_health

> crate::models::InlineResponse20010 fetch_health()
Fetch performance info

Fetch performance info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_market_chard_data

> Vec<crate::models::InlineResponse20016> fetch_market_chard_data()
Fetch market chart data

Fetch market chart data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20016>**](inline_response_200_16.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_network_status

> crate::models::InlineResponse20011 fetch_network_status()
Fetch network status

Fetch network status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_non_validators

> Vec<crate::models::InlineResponse20015> fetch_non_validators()
Fetch non validator nodes

Fetch non validator nodes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20015>**](inline_response_200_15.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_stake_history

> Vec<crate::models::InlineResponse2009> fetch_stake_history(limit, offset)
Fetch stake history

Fetch stake history by epochs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 512) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::InlineResponse2009>**](inline_response_200_9.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_staking_apy

> crate::models::InlineResponse20012 fetch_staking_apy()
Fetch staking APY

Fetch staking APY

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

