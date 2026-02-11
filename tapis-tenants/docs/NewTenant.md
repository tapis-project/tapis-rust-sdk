# NewTenant

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
**admin_user** | **String** | The username of the user that is automatically assigned the tenant_admin role by the Security Kernel. | 
**token_gen_services** | **Vec<String>** | The list of services that are automatically granted the token_generator role for this tenant by the Security Kernel. | 
**service_ldap_connection_id** | Option<**String**> | The unique identifier for the LDAP object for service accounts in the tenant. | [optional]
**user_ldap_connection_id** | Option<**String**> | The unique identifier for the LDAP object for user accounts in the tenant. | [optional]
**status** | Option<**String**> | The status of the tenant; Tenants can be created in \"draft\" status without a public key; Tenants in both \"inactive\" and \"draft\" status are by default not returned in the tenants listing. | [optional]
**public_key** | Option<**String**> | The public key associated with the private key used for signing tokens in this tenant. Note that the public key is optional in that tenants can be created without the public key but only in \"draft\" status. | [optional]
**description** | Option<**String**> | A description of the tenant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


