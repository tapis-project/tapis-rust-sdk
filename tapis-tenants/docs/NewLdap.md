# NewLdap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ldap_id** | **String** | Unique id for the LDAP object. | 
**url** | **String** | url to the LDAP | 
**port** | **i32** | port for the LDAP | 
**use_ssl** | **bool** | Whether to use SSL with the LDAP | 
**user_dn** | **String** | base DN for users | 
**bind_dn** | **String** | DN used for binding to the LDAP. | 
**bind_credential** | **String** | Pointed to a Tapis credential for binding to the LDAP. | 
**account_type** | **AccountType** | Whether this LDAP is used for service accounts or user accounts. (enum: service, user) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


