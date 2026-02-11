# NativeLinuxSetFaclRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation** | **Operation** |  (enum: ADD, REMOVE, REMOVE_DEFAULT, REMOVE_ALL) | 
**recursion_method** | Option<**RecursionMethod**> | Recursion may be set to physical (don't follow symlinks) or logical (follow symlinks), or none (don't recurse).  (enum: NONE, PHYSICAL, LOGICAL) | [optional][default to None]
**acl_string** | **String** | specifies the actual acl string to set.  Multiple acls may be separated by  commas. Examples - user:myuser:rwx,group             group:mygroup:rw             user:myuser:rwx,group,group:mygroup:rw   | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


