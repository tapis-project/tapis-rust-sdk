# UpdateTable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table_name** | Option<**String**> | Operation to change the table_name currently associated with the table. | [optional]
**root_url** | Option<**String**> | Operation to change the root_url currently associated with the table. | [optional]
**comments** | Option<**String**> | Operation to change the comments currently associated with the table (Overwrites existing). | [optional]
**endpoints** | Option<**Vec<String>**> | Operation to change the endpoints a table currently has available. | [optional]
**column_type** | Option<**String**> | Operation to change the column_type of a particular column in table (\"col_name, new_type\" format). | [optional]
**add_column** | Option<[**models::TableColumns**](TableColumns.md)> |  | [optional]
**drop_column** | Option<**String**> | Operation to drop a column in a table (Not reversible). | [optional]
**set_default** | Option<**String**> | Operation to set a new default on a column in a table (\"col_name, new_default\" format). | [optional]
**drop_default** | Option<**String**> | Operation to drop a default currently set on a column in a table. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


