# \ValidatorApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_validator_by_votepubkey**](ValidatorApi.md#fetch_validator_by_votepubkey) | **GET** /validator/{votepubkey} | Fetch validator by vote pubkey
[**fetch_validator_delegators**](ValidatorApi.md#fetch_validator_delegators) | **GET** /validator/{votepubkey}/delegators | Fetch validator delegators
[**fetch_validator_history**](ValidatorApi.md#fetch_validator_history) | **GET** /validator/{votepubkey}/history | Fetch validator history
[**fetch_validator_slots**](ValidatorApi.md#fetch_validator_slots) | **GET** /validator/{nodepubkey}/slots | Fetch validator slots



## fetch_validator_by_votepubkey

> crate::models::Validator fetch_validator_by_votepubkey(votepubkey)
Fetch validator by vote pubkey

Fetch validator by vote pubkey if the given transaction exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**votepubkey** | **String** | Validator pubkey | [required] |

### Return type

[**crate::models::Validator**](Validator.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_validator_delegators

> Vec<crate::models::StakeAccount> fetch_validator_delegators(votepubkey, limit, offset)
Fetch validator delegators

Fetch validator deleagators

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**votepubkey** | **String** | Validator's vote pubkey | [required] |
**limit** | Option<**i32**> | Result limit (max 1000) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::StakeAccount>**](StakeAccount.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_validator_history

> crate::models::InlineResponse2003 fetch_validator_history(votepubkey)
Fetch validator history

Fetch validator's stake and delegator history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**votepubkey** | **String** | Validator's vote pubkey | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_validator_slots

> Vec<crate::models::InlineResponse2004> fetch_validator_slots(nodepubkey)
Fetch validator slots

Fetch validator slots in the current epoch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nodepubkey** | **String** | Validator's node pubkey | [required] |

### Return type

[**Vec<crate::models::InlineResponse2004>**](inline_response_200_4.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

