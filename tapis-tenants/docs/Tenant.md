# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant_id** | **String** | The unique identifier for the tenant. | 
**base_url** | **String** | The base URL for the tenant. | 
**site_id** | **String** | The site for which the tenant belongs. | 
**token_service** | **String** | The resolvable location of the token service for the tenant. | 
**security_kernel** | **String** | The resolvable location of the security kernel for the tenant. | 
**authenticator** | **String** | The resolvable location of the authenticator for the tenant. | 
**owner** | **String** | The email address of the primary owner and contact for the tenant. | 
**admin_user** | Option<**String**> | The username of the user that is automatically assigned the tenant_admin role by the Security Kernel. | [optional]
**token_gen_services** | Option<**Vec<String>**> | The list of services that are automatically granted the token_generator role for this tenant by the Security Kernel. | [optional]
**service_ldap_connection_id** | Option<**String**> | The unique identifier for the LDAP object for service accounts in the tenant. | [optional]
**user_ldap_connection_id** | Option<**String**> | The unique identifier for the LDAP object for user accounts in the tenant. | [optional]
**public_key** | Option<**String**> | The public key that should be used to validate the signatures of JWTs associated with the tenant. | [optional]
**description** | Option<**String**> | A description of the tenant. | [optional]
**created_by** | Option<**String**> | The API identity who created this tenant. | [optional]
**last_updated_by** | Option<**String**> | The API identity that last updated this tenant. | [optional]
**create_time** | Option<**String**> | The time the client was created. | [optional]
**last_update_time** | Option<**String**> | The time the client was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


