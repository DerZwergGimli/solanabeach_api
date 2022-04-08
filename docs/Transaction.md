# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_hash** | Option<**String**> |  | [optional]
**block_number** | Option<**i64**> |  | [optional]
**index** | Option<**i64**> |  | [optional]
**accounts** | Option<[**Vec<crate::models::TransactionAccounts>**](Transaction_accounts.md)> |  | [optional]
**header** | Option<[**crate::models::TransactionHeader**](Transaction_header.md)> |  | [optional]
**instructions** | Option<[**Vec<crate::models::TransactionInstructions>**](Transaction_instructions.md)> |  | [optional]
**recent_blockhash** | Option<**String**> |  | [optional]
**signatures** | Option<**Vec<String>**> |  | [optional]
**meta** | Option<[**crate::models::TransactionMeta**](Transaction_meta.md)> |  | [optional]
**valid** | Option<**bool**> |  | [optional]
**blocktime** | Option<[**crate::models::Timestamp**](Timestamp.md)> |  | [optional]
**most_important_instruction** | Option<[**crate::models::TransactionMostImportantInstruction**](Transaction_mostImportantInstruction.md)> |  | [optional]
**smart** | Option<[**Vec<crate::models::TransactionSmart>**](Transaction_smart.md)> |  | [optional]
**ondemand** | Option<**bool**> |  | [optional]

[[Back to Model list]](../solanabeach_api.wiki/Home.md#documentation-for-models) [[Back to API list]](../solanabeach_api.wiki/Home.md#documentation-for-api-endpoints) [[Back to README]](../solanabeach_api.wiki/Home.md)


