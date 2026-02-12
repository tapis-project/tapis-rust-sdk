# PostIt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**postit_id** | Option<**uuid::Uuid**> | The unique ID of the PostIt. | [optional]
**system_id** | Option<**String**> | The ID of the system where the file pointed to by the PostIt resides. | [optional]
**owner** | Option<**String**> | The owner of the PostIt. | [optional]
**tenant_id** | Option<**String**> | the tenant that tthe PostIt belongs to. | [optional]
**path** | Option<**String**> | Path relative to the system *rootDir* | [optional]
**allowed_uses** | Option<**i32**> | The number of times the PostIt may be redeemed.  This number minus *uses* is the number of uses remaining. | [optional]
**times_used** | Option<**i32**> | The number of times the PostIt has already been retrieved. | [optional]
**jwt_user** | Option<**String**> | Authenticated user from the JWT (may be different than OBO user). | [optional]
**jwt_tenant_id** | Option<**String**> | Tenant of authenticated user from the JWT (may be different than OBO user's tenant). | [optional]
**redeem_url** | Option<**String**> | The url to use to retrieve the file pointed to by the PostIt. | [optional]
**expiration** | Option<**String**> | The expiration date/time of the PostIt. | [optional]
**created** | Option<**String**> | Creation timestamp in UTC | [optional]
**updated** | Option<**String**> | Last update timestamp in UTC | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


