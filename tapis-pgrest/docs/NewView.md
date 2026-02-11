# NewView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**view_name** | **String** | The name of the view to create. | 
**select_query** | Option<**String**> | Query to run on 'from_table' to create a view from. | [optional]
**from_table** | Option<**String**> | Table to run select query on and create view from. | [optional]
**comments** | Option<**String**> | Text area to describe view. Returned when calling manage endpoints. | [optional]
**permission_rules** | Option<**Vec<String>**> | Roles users who access this view must have in security kernel to have permission to see this view. | [optional]
**raw_sql** | Option<**String**> | Admin only. Full support of SQL when creating complex views. | [optional]
**materialized_view_raw_sql** | Option<**String**> | Admin only. Full support of SQL when creating complex materialized views. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


