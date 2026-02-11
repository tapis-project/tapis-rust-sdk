# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the role. | [optional]
**tenant** | Option<**String**> | Tenant the role is in. | [optional]
**description** | Option<**String**> | Description of the role defined at role creation. | [optional]
**owner** | Option<**String**> | The owner of the role. | [optional]
**owner_tenant** | Option<**String**> | The tenant of the owner of the role. | [optional]
**created** | Option<**String**> | When the role was created. | [optional]
**createdby** | Option<**String**> | Who created the role. | [optional]
**createdby_tenant** | Option<**String**> | The tenant of the creator of the role. | [optional]
**updated** | Option<**String**> | When the role was last updated. | [optional]
**updatedby** | Option<**String**> | Who last updated the role. | [optional]
**updatedby_tenant** | Option<**String**> | The tenant of the user who last updated the role. | [optional]
**users_in_role** | Option<**Vec<String>**> | Users who currently have this role granted to them. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


