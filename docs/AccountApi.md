# \AccountApi

All URIs are relative to *https://api.solanabeach.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_account**](AccountApi.md#fetch_account) | **GET** /account/{pubkey} | Fetch account data
[**fetch_account_serum_instructions**](AccountApi.md#fetch_account_serum_instructions) | **GET** /account/{pubkey}/serum-instructions | Fetch account serum instructions
[**fetch_account_serum_orders**](AccountApi.md#fetch_account_serum_orders) | **GET** /account/{pubkey}/serum-orders | Fetch account serum orders
[**fetch_account_swap_instructions**](AccountApi.md#fetch_account_swap_instructions) | **GET** /account/{pubkey}/swap-instructions | Fetch account swap instructions
[**fetch_account_token_transfers**](AccountApi.md#fetch_account_token_transfers) | **GET** /account/{pubkey}/token-transfers | Fetch account token transfers
[**fetch_account_tokens**](AccountApi.md#fetch_account_tokens) | **GET** /account/{pubkey}/tokens | Fetch account tokens
[**fetch_account_transactions**](AccountApi.md#fetch_account_transactions) | **GET** /account/{pubkey}/transactions | Fetch account transactions
[**fetch_accounts**](AccountApi.md#fetch_accounts) | **GET** /accounts | Fetch accounts
[**fetch_stake_account_rewards**](AccountApi.md#fetch_stake_account_rewards) | **GET** /account/{stake_pubkey}/stake-rewards | Fetch stake account rewards
[**fetch_stake_accounts**](AccountApi.md#fetch_stake_accounts) | **GET** /account/{pubkey}/stakes | Fetch stake accounts
[**fetch_wealth_metrics**](AccountApi.md#fetch_wealth_metrics) | **GET** /wealth | Fetch wealth distribution metrics



## fetch_account

> crate::models::Account fetch_account(pubkey)
Fetch account data

Fetch account data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_account_serum_instructions

> Vec<crate::models::AccountSerumInstruction> fetch_account_serum_instructions(pubkey, limit, offset, cursor)
Fetch account serum instructions

Fetch account serum instructions ordered by block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Serum instruction cursor (blocknumber,txindex,index) |  |

### Return type

[**Vec<crate::models::AccountSerumInstruction>**](AccountSerumInstruction.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_account_serum_orders

> Vec<crate::models::AccountSerumOrder> fetch_account_serum_orders(pubkey, limit, offset, cursor)
Fetch account serum orders

Fetch account serum orders ordered by block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Serum orders cursor (blocknumber,txindex) |  |

### Return type

[**Vec<crate::models::AccountSerumOrder>**](AccountSerumOrder.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_account_swap_instructions

> Vec<crate::models::AccountSwapInstruction> fetch_account_swap_instructions(pubkey, limit, offset, cursor)
Fetch account swap instructions

Fetch account swap instructions ordered by block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Swap instruction cursor (blocknumber,txindex,index) |  |

### Return type

[**Vec<crate::models::AccountSwapInstruction>**](AccountSwapInstruction.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_account_token_transfers

> Vec<crate::models::AccountTokenTransfer> fetch_account_token_transfers(pubkey, limit, offset, cursor, inner)
Fetch account token transfers

Fetch account token transfers ordered by block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Token transfers cursor (blocknumber,txindex) |  |
**inner** | Option<**bool**> | Filter inner instructions |  |

### Return type

[**Vec<crate::models::AccountTokenTransfer>**](AccountTokenTransfer.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_account_tokens

> Vec<crate::models::AccountToken> fetch_account_tokens(pubkey, limit, offset)
Fetch account tokens

Fetch account tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 100) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::AccountToken>**](AccountToken.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_account_transactions

> Vec<crate::models::Transaction> fetch_account_transactions(pubkey, limit, offset, cursor)
Fetch account transactions

Fetch account transactions ordered by block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
**limit** | Option<**i32**> | Result limit (max 1000) |  |
**offset** | Option<**i32**> | Result offset |  |
**cursor** | Option<**String**> | Transaction cursor (blocknumber,txindex) |  |

### Return type

[**Vec<crate::models::Transaction>**](Transaction.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_accounts

> Vec<crate::models::SimpleAccount> fetch_accounts(limit, offset)
Fetch accounts

Fetch accounts ordered by balance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Result limit (max 1000) |  |
**offset** | Option<**i32**> | Result offset |  |

### Return type

[**Vec<crate::models::SimpleAccount>**](SimpleAccount.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_stake_account_rewards

> Vec<crate::models::StakeAccountReward> fetch_stake_account_rewards(stake_pubkey, cursor)
Fetch stake account rewards

Fetch stake account rewards by the stake account pubkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stake_pubkey** | **String** | Stake account address | [required] |
**cursor** | Option<**i32**> | Epoch cursor |  |

### Return type

[**Vec<crate::models::StakeAccountReward>**](StakeAccountReward.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)


## fetch_stake_accounts

> Vec<crate::models::StakeAccount> fetch_stake_accounts(pubkey, limit, offset)
Fetch stake accounts

Fetch stake accounts owned by the pubkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Account address | [required] |
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


## fetch_wealth_metrics

> crate::models::Wealth fetch_wealth_metrics()
Fetch wealth distribution metrics

Fetch wealth distribution metrics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Wealth**](Wealth.md)

### Authorization

[apiAuth](../solanabeach_api.wiki/Home.md#apiAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to README]](../solanabeach_api.wiki/Home.md)

