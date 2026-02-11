# UpdateTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token_service** | Option<**String**> | The resolvable location of the token service for the tenant. | [optional]
**security_kernel** | Option<**String**> | The resolvable location of the security kernel for the tenant. | [optional]
**authenticator** | Option<**String**> | The resolvable location of the authenticator for the tenant. | [optional]
**owner** | Option<**String**> | The email address of the primary owner and contact for the tenant. | [optional]
**admin_user** | Option<**String**> | The username of the user that is automatically assigned the tenant_admin role by the Security Kernel. | [optional]
**token_gen_services** | Option<**Vec<String>**> | The list of services that are automatically granted the token_generator role for this tenant by the Security Kernel. | [optional]
**service_ldap_connection_id** | Option<**String**> | The unique identifier for the LDAP object for service accounts in the tenant. | [optional]
**user_ldap_connection_id** | Option<**String**> | The unique identifier for the LDAP object for user accounts in the tenant. | [optional]
**public_key** | Option<**String**> | The public key that should be used to validate the signatures of JWTs associated with the tenant. | [optional]
**status** | Option<**String**> | The status of the tenant; Tenants can be created in \"draft\" status without a public key; Tenants in both \"inactive\" and \"draft\" status are by default not returned in the tenants listing. | [optional]
**description** | Option<**String**> | A description of the tenant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


