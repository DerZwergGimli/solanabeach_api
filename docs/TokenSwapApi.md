# \TokenSwapApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_token_swap**](TokenSwapApi.md#fetch_token_swap) | **GET** /token-swap/{pubkey} | Fetch token swap
[**fetch_token_swaps**](TokenSwapApi.md#fetch_token_swaps) | **GET** /token-swaps | Fetch token swaps
[**fetch_token_swaps_by_mint**](TokenSwapApi.md#fetch_token_swaps_by_mint) | **GET** /token-swaps/{mint} | Fetch token swaps by mint



## fetch_token_swap

> crate::models::TokenSwap fetch_token_swap(pubkey)
Fetch token swap

Fetch token swap by pubkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Token swap address | [required] |

### Return type

[**crate::models::TokenSwap**](TokenSwap.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_token_swaps

> crate::models::InlineResponse2006 fetch_token_swaps(limit, offset, sort, dir)
Fetch token swaps

Fetch token swaps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**sort** | Option<**String**> | Sort by param |  |
**dir** | Option<**String**> | Sort direction param |  |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_token_swaps_by_mint

> Vec<crate::models::TokenSwap> fetch_token_swaps_by_mint(mint, limit, offset)
Fetch token swaps by mint

Fetch token swaps by mint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint** | **String** | Mint address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::TokenSwap>**](TokenSwap.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

