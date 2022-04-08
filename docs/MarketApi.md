# \MarketApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_market**](MarketApi.md#fetch_market) | **GET** /market/{pubkey} | Fetch market
[**fetch_markets**](MarketApi.md#fetch_markets) | **GET** /markets | Fetch markets
[**fetch_markets_by_base_mint**](MarketApi.md#fetch_markets_by_base_mint) | **GET** /markets/base/{basemint} | Fetch markets by base mint
[**fetch_markets_by_quote_mint**](MarketApi.md#fetch_markets_by_quote_mint) | **GET** /markets/quote/{quotemint} | Fetch markets by quote mint



## fetch_market

> crate::models::Market fetch_market(pubkey)
Fetch market

Fetch market by pubkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Market address | [required] |

### Return type

[**crate::models::Market**](Market.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_markets

> crate::models::InlineResponse2005 fetch_markets(limit, offset, sort, dir)
Fetch markets

Fetch markets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**sort** | Option<**String**> | Sort by param |  |
**dir** | Option<**String**> | Sort direction param |  |

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_markets_by_base_mint

> Vec<crate::models::Market> fetch_markets_by_base_mint(basemint, limit, offset)
Fetch markets by base mint

Fetch markets by base mint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**basemint** | **String** | Base mint address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::Market>**](Market.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_markets_by_quote_mint

> Vec<crate::models::Market> fetch_markets_by_quote_mint(quotemint, limit, offset)
Fetch markets by quote mint

Fetch markets by quote mint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quotemint** | **String** | Quote mint address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::Market>**](Market.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

