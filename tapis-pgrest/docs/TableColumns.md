# TableColumns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**null** | Option<**bool**> | Whether the column can contain null values. | [optional]
**data_type** | **String** | The Postgres type of the column, such as \"integer\", \"varchar\", etc. | 
**char_len** | Option<**i32**> | For columns of type \"varchar\", the max length of the column. | [optional]
**comments** | Option<**String**> | Text area to describe column. Shown in manage endpoints. | [optional]
**unique** | Option<**bool**> | Whether the column can contain null values. | [optional]
**default** | Option<**bool**> | Whether the column can contain null values. | [optional]
**primary_key** | Option<**String**> | Allows user to specify which column to act as a primary_key rather than defaulting to {table_name}_id. | [optional]
**foreign_key** | Option<**bool**> | Whether this column should reference a foreign key in another table. | [optional]
**reference_table** | Option<**String**> | Only if foreign_key, sets which table to reference. | [optional]
**reference_column** | Option<**String**> | Only if foreign_key, sets when table column to reference. | [optional]
**on_event** | Option<**OnEvent**> | Only if foreign_key, sets whether to use ON DELETE or ON UPDATE postgres definition. (enum: ON DELETE, ON UPDATE) | [optional]
**event_action** | Option<**EventAction**> | Only if foreign_key, sets which event action to call when on_event event occurs. (enum: CASCADE, SET NULL, SET DEFAULT, RESTRICT, NO ACTION) | [optional]
**serial_start** | Option<**i32**> | Only if data_type is serial. Delegates what number the data_type will start at. | [optional]
**serial_increment** | Option<**i32**> | Only if data_type is serial. Delegates what the serial increment between rows will be. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


