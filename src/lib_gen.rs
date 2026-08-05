#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///`AcceptInvitationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "invitation_id": {
    ///      "type": "string"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "AcceptInvitationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AcceptInvitationRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invitation_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AcceptInvitationRequest {
        fn default() -> Self {
            Self {
                invitation_id: Default::default(),
                token: Default::default(),
            }
        }
    }
    ///`AccountBalanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "balance": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "AccountBalanceResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountBalanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for AccountBalanceResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                balance: Default::default(),
            }
        }
    }
    ///`AccountListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "memberships": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountMembershipResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "AccountListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub memberships: ::std::vec::Vec<AccountMembershipResponse>,
    }
    impl ::std::default::Default for AccountListResponse {
        fn default() -> Self {
            Self {
                memberships: Default::default(),
            }
        }
    }
    ///`AccountMemberListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "members": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountMemberResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "AccountMemberListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountMemberListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub members: ::std::vec::Vec<AccountMemberResponse>,
    }
    impl ::std::default::Default for AccountMemberListResponse {
        fn default() -> Self {
            Self {
                members: Default::default(),
            }
        }
    }
    ///`AccountMemberResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at_unix": {
    ///      "type": "integer"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "role": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "AccountMemberResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountMemberResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at_unix: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AccountMemberResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                created_at_unix: Default::default(),
                email: Default::default(),
                role: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    ///`AccountMembershipResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account": {
    ///      "$ref": "#/components/schemas/AccountResponse"
    ///    },
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at_unix": {
    ///      "type": "integer"
    ///    },
    ///    "role": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "AccountMembershipResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountMembershipResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account: ::std::option::Option<AccountResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at_unix: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AccountMembershipResponse {
        fn default() -> Self {
            Self {
                account: Default::default(),
                account_id: Default::default(),
                created_at_unix: Default::default(),
                role: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    ///`AccountResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at_unix": {
    ///      "type": "integer"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_address": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_city": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_company_name": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_country": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_customer_type": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_invoice_email": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_legal_name": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_postcode": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_profile_completed": {
    ///      "type": "boolean"
    ///    },
    ///    "fiscal_region": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_tax_id": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_vat_validated": {
    ///      "type": "boolean"
    ///    },
    ///    "holded_contact_id": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "AccountResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at_unix: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_city: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_company_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_customer_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_invoice_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_legal_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_postcode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_profile_completed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_tax_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_vat_validated: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub holded_contact_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AccountResponse {
        fn default() -> Self {
            Self {
                created_at_unix: Default::default(),
                description: Default::default(),
                fiscal_address: Default::default(),
                fiscal_city: Default::default(),
                fiscal_company_name: Default::default(),
                fiscal_country: Default::default(),
                fiscal_customer_type: Default::default(),
                fiscal_invoice_email: Default::default(),
                fiscal_legal_name: Default::default(),
                fiscal_postcode: Default::default(),
                fiscal_profile_completed: Default::default(),
                fiscal_region: Default::default(),
                fiscal_tax_id: Default::default(),
                fiscal_vat_validated: Default::default(),
                holded_contact_id: Default::default(),
                id: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`AddAccountMemberRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "role": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "AddAccountMemberRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AddAccountMemberRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AddAccountMemberRequest {
        fn default() -> Self {
            Self {
                email: Default::default(),
                role: Default::default(),
            }
        }
    }
    ///`ApiKeyListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "api_keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ApiKeyResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "ApiKeyListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiKeyListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub api_keys: ::std::vec::Vec<ApiKeyResponse>,
    }
    impl ::std::default::Default for ApiKeyListResponse {
        fn default() -> Self {
            Self {
                api_keys: Default::default(),
            }
        }
    }
    ///`ApiKeyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "key_prefix": {
    ///      "type": "string"
    ///    },
    ///    "plaintext_key": {
    ///      "type": "string"
    ///    },
    ///    "revoked_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ApiKeyResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiKeyResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key_prefix: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub plaintext_key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub revoked_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ApiKeyResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                id: Default::default(),
                key_prefix: Default::default(),
                plaintext_key: Default::default(),
                revoked_at: Default::default(),
            }
        }
    }
    ///`AudioSpeechRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "input": {
    ///      "type": "string"
    ///    },
    ///    "instructions": {
    ///      "type": "string"
    ///    },
    ///    "language": {
    ///      "type": "string"
    ///    },
    ///    "model": {
    ///      "type": "string"
    ///    },
    ///    "response_format": {
    ///      "type": "string"
    ///    },
    ///    "speed": {
    ///      "type": "number"
    ///    },
    ///    "task_type": {
    ///      "type": "string"
    ///    },
    ///    "voice": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "AudioSpeechRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AudioSpeechRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub input: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instructions: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub response_format: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub speed: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub task_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub voice: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for AudioSpeechRequest {
        fn default() -> Self {
            Self {
                input: Default::default(),
                instructions: Default::default(),
                language: Default::default(),
                model: Default::default(),
                response_format: Default::default(),
                speed: Default::default(),
                task_type: Default::default(),
                voice: Default::default(),
            }
        }
    }
    ///`BeginPasskeyRegistrationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "challenge_id": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "BeginPasskeyRegistrationResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BeginPasskeyRegistrationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub challenge_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub public_key: ::std::vec::Vec<i64>,
    }
    impl ::std::default::Default for BeginPasskeyRegistrationResponse {
        fn default() -> Self {
            Self {
                challenge_id: Default::default(),
                public_key: Default::default(),
            }
        }
    }
    ///`BeginPasskeyloginRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email"
    ///  ],
    ///  "properties": {
    ///    "cf_turnstile_response": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "BeginPasskeyloginRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BeginPasskeyloginRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cf_turnstile_response: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
    }
    ///`BeginPasskeyloginResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "challenge_id": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "BeginPasskeyloginResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BeginPasskeyloginResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub challenge_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub public_key: ::std::vec::Vec<i64>,
    }
    impl ::std::default::Default for BeginPasskeyloginResponse {
        fn default() -> Self {
            Self {
                challenge_id: Default::default(),
                public_key: Default::default(),
            }
        }
    }
    ///`BeginTotpEnrollmentResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "otpauth_uri": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "BeginTotpEnrollmentResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct BeginTotpEnrollmentResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub otpauth_uri: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub secret: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for BeginTotpEnrollmentResponse {
        fn default() -> Self {
            Self {
                otpauth_uri: Default::default(),
                secret: Default::default(),
            }
        }
    }
    ///`CatalogGpuResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "availability_count": {
    ///      "type": "integer"
    ///    },
    ///    "availability_total": {
    ///      "type": "integer"
    ///    },
    ///    "capacity_tier": {
    ///      "type": "string"
    ///    },
    ///    "city": {
    ///      "type": "string"
    ///    },
    ///    "compute_tflops": {
    ///      "type": "number"
    ///    },
    ///    "continent": {
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "type": "string"
    ///    },
    ///    "cpu_cores": {
    ///      "type": "integer"
    ///    },
    ///    "cpu_name": {
    ///      "type": "string"
    ///    },
    ///    "cpu_ram_gb": {
    ///      "type": "integer"
    ///    },
    ///    "cuda_max_good": {
    ///      "type": "number"
    ///    },
    ///    "disk_bandwidth_mbps": {
    ///      "type": "number"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "gpu_count": {
    ///      "type": "integer"
    ///    },
    ///    "internet_down_mbps": {
    ///      "type": "number"
    ///    },
    ///    "internet_up_mbps": {
    ///      "type": "number"
    ///    },
    ///    "market_type": {
    ///      "type": "string"
    ///    },
    ///    "price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "provider_codename": {
    ///      "type": "string"
    ///    },
    ///    "public_gpu_id": {
    ///      "type": "string"
    ///    },
    ///    "reliability": {
    ///      "type": "number"
    ///    },
    ///    "stock_status": {
    ///      "type": "string"
    ///    },
    ///    "vram_gb": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "CatalogGPUResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CatalogGpuResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub availability_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub availability_total: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub capacity_tier: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub city: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub compute_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub continent: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu_cores: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu_ram_gb: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cuda_max_good: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub disk_bandwidth_mbps: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub internet_down_mbps: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub internet_up_mbps: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub market_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub provider_codename: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_gpu_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reliability: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stock_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub vram_gb: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for CatalogGpuResponse {
        fn default() -> Self {
            Self {
                availability_count: Default::default(),
                availability_total: Default::default(),
                capacity_tier: Default::default(),
                city: Default::default(),
                compute_tflops: Default::default(),
                continent: Default::default(),
                country: Default::default(),
                cpu_cores: Default::default(),
                cpu_name: Default::default(),
                cpu_ram_gb: Default::default(),
                cuda_max_good: Default::default(),
                disk_bandwidth_mbps: Default::default(),
                display_name: Default::default(),
                enabled: Default::default(),
                gpu_count: Default::default(),
                internet_down_mbps: Default::default(),
                internet_up_mbps: Default::default(),
                market_type: Default::default(),
                price_per_hour_eur: Default::default(),
                provider_codename: Default::default(),
                public_gpu_id: Default::default(),
                reliability: Default::default(),
                stock_status: Default::default(),
                vram_gb: Default::default(),
            }
        }
    }
    ///`ChangePasswordRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "current_password",
    ///    "new_password"
    ///  ],
    ///  "properties": {
    ///    "current_password": {
    ///      "type": "string"
    ///    },
    ///    "new_password": {
    ///      "type": "string",
    ///      "minLength": 8
    ///    }
    ///  },
    ///  "x-go-name": "ChangePasswordRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChangePasswordRequest {
        pub current_password: ::std::string::String,
        pub new_password: ChangePasswordRequestNewPassword,
    }
    ///`ChangePasswordRequestNewPassword`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 8
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ChangePasswordRequestNewPassword(::std::string::String);
    impl ::std::ops::Deref for ChangePasswordRequestNewPassword {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ChangePasswordRequestNewPassword>
    for ::std::string::String {
        fn from(value: ChangePasswordRequestNewPassword) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ChangePasswordRequestNewPassword {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 8usize {
                return Err("shorter than 8 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ChangePasswordRequestNewPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ChangePasswordRequestNewPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ChangePasswordRequestNewPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChangePasswordRequestNewPassword {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`ChatMessageEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "content": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "role": {
    ///      "type": "string"
    ///    },
    ///    "tool_call_id": {
    ///      "type": "string"
    ///    },
    ///    "tool_calls": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ChatToolCallEnvelope"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "ChatMessageEnvelope"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChatMessageEnvelope {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub content: ::std::vec::Vec<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tool_call_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tool_calls: ::std::vec::Vec<ChatToolCallEnvelope>,
    }
    impl ::std::default::Default for ChatMessageEnvelope {
        fn default() -> Self {
            Self {
                content: Default::default(),
                role: Default::default(),
                tool_call_id: Default::default(),
                tool_calls: Default::default(),
            }
        }
    }
    ///`ChatRequestEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "firewall": {
    ///      "type": "string"
    ///    },
    ///    "max_tokens": {
    ///      "type": "integer"
    ///    },
    ///    "messages": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ChatMessageEnvelope"
    ///      }
    ///    },
    ///    "model": {
    ///      "type": "string"
    ///    },
    ///    "stream": {
    ///      "type": "boolean"
    ///    },
    ///    "temperature": {
    ///      "type": "number"
    ///    },
    ///    "top_p": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "ChatRequestEnvelope"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChatRequestEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firewall: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_tokens: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub messages: ::std::vec::Vec<ChatMessageEnvelope>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stream: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub temperature: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub top_p: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for ChatRequestEnvelope {
        fn default() -> Self {
            Self {
                firewall: Default::default(),
                max_tokens: Default::default(),
                messages: Default::default(),
                model: Default::default(),
                stream: Default::default(),
                temperature: Default::default(),
                top_p: Default::default(),
            }
        }
    }
    ///`ChatToolCallEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "function": {
    ///      "$ref": "#/components/schemas/ChatToolCallFunctionEnvelope"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "index": {
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ChatToolCallEnvelope"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChatToolCallEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub function: ::std::option::Option<ChatToolCallFunctionEnvelope>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub index: ::std::option::Option<i64>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ChatToolCallEnvelope {
        fn default() -> Self {
            Self {
                function: Default::default(),
                id: Default::default(),
                index: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ChatToolCallFunctionEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "arguments": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ChatToolCallFunctionEnvelope"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChatToolCallFunctionEnvelope {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub arguments: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ChatToolCallFunctionEnvelope {
        fn default() -> Self {
            Self {
                arguments: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`CheckoutQuoteRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "amount_cents": {
    ///      "type": "number"
    ///    },
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "discount_code": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CheckoutQuoteRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CheckoutQuoteRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_cents: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discount_code: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CheckoutQuoteRequest {
        fn default() -> Self {
            Self {
                amount_cents: Default::default(),
                currency: Default::default(),
                discount_code: Default::default(),
            }
        }
    }
    ///`CheckoutQuoteResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "discount_amount_minor": {
    ///      "type": "integer"
    ///    },
    ///    "gross_total_minor": {
    ///      "type": "integer"
    ///    },
    ///    "original_amount_minor": {
    ///      "type": "integer"
    ///    },
    ///    "policy_version": {
    ///      "type": "string"
    ///    },
    ///    "quote_hash": {
    ///      "type": "string"
    ///    },
    ///    "quote_id": {
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "type": "string"
    ///    },
    ///    "reverse_charge": {
    ///      "type": "boolean"
    ///    },
    ///    "tax_included": {
    ///      "type": "boolean"
    ///    },
    ///    "tax_minor": {
    ///      "type": "integer"
    ///    },
    ///    "tax_rate_basis_points": {
    ///      "type": "integer"
    ///    },
    ///    "taxable_base_minor": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "CheckoutQuoteResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CheckoutQuoteResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discount_amount_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gross_total_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub original_amount_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub policy_version: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote_hash: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reason: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reverse_charge: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_included: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tax_rate_basis_points: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub taxable_base_minor: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for CheckoutQuoteResponse {
        fn default() -> Self {
            Self {
                currency: Default::default(),
                discount_amount_minor: Default::default(),
                gross_total_minor: Default::default(),
                original_amount_minor: Default::default(),
                policy_version: Default::default(),
                quote_hash: Default::default(),
                quote_id: Default::default(),
                reason: Default::default(),
                reverse_charge: Default::default(),
                tax_included: Default::default(),
                tax_minor: Default::default(),
                tax_rate_basis_points: Default::default(),
                taxable_base_minor: Default::default(),
            }
        }
    }
    ///`CheckoutSessionResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "checkout_url": {
    ///      "type": "string"
    ///    },
    ///    "session_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CheckoutSessionResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CheckoutSessionResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub checkout_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub session_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CheckoutSessionResponse {
        fn default() -> Self {
            Self {
                checkout_url: Default::default(),
                session_id: Default::default(),
            }
        }
    }
    ///`ConfirmTotpEnrollmentRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ConfirmTotpEnrollmentRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ConfirmTotpEnrollmentRequest {
        pub code: ::std::string::String,
        pub secret: ::std::string::String,
    }
    ///`CreateAccountRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateAccountRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateAccountRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateAccountRequest {
        fn default() -> Self {
            Self { name: Default::default() }
        }
    }
    ///`CreateCheckoutSessionRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "amount_cents": {
    ///      "type": "number"
    ///    },
    ///    "cancel_url": {
    ///      "type": "string"
    ///    },
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "discount_code": {
    ///      "type": "string"
    ///    },
    ///    "success_url": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateCheckoutSessionRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateCheckoutSessionRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_cents: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cancel_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discount_code: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateCheckoutSessionRequest {
        fn default() -> Self {
            Self {
                amount_cents: Default::default(),
                cancel_url: Default::default(),
                currency: Default::default(),
                discount_code: Default::default(),
                success_url: Default::default(),
            }
        }
    }
    ///`CreateFirewallRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "evaluator_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "mode": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule_slugs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateFirewallRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateFirewallRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub evaluator_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rule_slugs: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateFirewallRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                evaluator_serving_name: Default::default(),
                mode: Default::default(),
                name: Default::default(),
                rule_slugs: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`CreateFirewallRuleRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "default_severity": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "prompt": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateFirewallRuleRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateFirewallRuleRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_severity: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prompt: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateFirewallRuleRequest {
        fn default() -> Self {
            Self {
                category: Default::default(),
                default_severity: Default::default(),
                description: Default::default(),
                name: Default::default(),
                prompt: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`CreateInstanceRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "download_size_bytes": {
    ///      "type": "integer"
    ///    },
    ///    "gguf_model_path": {
    ///      "type": "string"
    ///    },
    ///    "gpu_preferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PublicGPUPreferenceJSON"
    ///      }
    ///    },
    ///    "huggingface_repo_id": {
    ///      "type": "string"
    ///    },
    ///    "idle_timeout_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "replica_count": {
    ///      "type": "integer"
    ///    },
    ///    "runtime_config": {
    ///      "$ref": "#/components/schemas/RuntimeConfigRequest"
    ///    },
    ///    "runtime_preset": {
    ///      "type": "string"
    ///    },
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "serverless_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "serving_name": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_max_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_region": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "smart_selection_label": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateInstanceRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateInstanceRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub download_size_bytes: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gguf_model_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gpu_preferences: ::std::vec::Vec<PublicGpuPreferenceJson>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub huggingface_repo_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub idle_timeout_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub replica_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_config: ::std::option::Option<RuntimeConfigRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_preset: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serverless_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_selection_label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateInstanceRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                download_size_bytes: Default::default(),
                gguf_model_path: Default::default(),
                gpu_preferences: Default::default(),
                huggingface_repo_id: Default::default(),
                idle_timeout_seconds: Default::default(),
                name: Default::default(),
                replica_count: Default::default(),
                runtime_config: Default::default(),
                runtime_preset: Default::default(),
                scheduling_mode: Default::default(),
                serverless_enabled: Default::default(),
                serving_name: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_max_price_per_hour_eur: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_region: Default::default(),
                smart_regions: Default::default(),
                smart_selection_label: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`CreateInvitationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "role"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "role": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateInvitationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateInvitationRequest {
        pub email: ::std::string::String,
        pub role: ::std::string::String,
    }
    ///`CreateServingEndpointRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/ServingTargetRequest"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateServingEndpointRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateServingEndpointRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target: ::std::option::Option<ServingTargetRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateServingEndpointRequest {
        fn default() -> Self {
            Self {
                display_name: Default::default(),
                name: Default::default(),
                target: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`CreateSmartBalancerRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "router_model": {
    ///      "$ref": "#/components/schemas/SmartBalancerRouterModelRequest"
    ///    },
    ///    "routes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SmartBalancerRouteRequest"
    ///      }
    ///    },
    ///    "routing_mode": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateSmartBalancerRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateSmartBalancerRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub router_model: ::std::option::Option<SmartBalancerRouterModelRequest>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub routes: ::std::vec::Vec<SmartBalancerRouteRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub routing_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateSmartBalancerRequest {
        fn default() -> Self {
            Self {
                display_name: Default::default(),
                enabled: Default::default(),
                name: Default::default(),
                router_model: Default::default(),
                routes: Default::default(),
                routing_mode: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`CreateStorageVolumeRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateStorageVolumeRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateStorageVolumeRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateStorageVolumeRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`CreateTargetGroupRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "members": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TargetGroupMemberRequest"
    ///      }
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "selection_policy": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateTargetGroupRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateTargetGroupRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub members: ::std::vec::Vec<TargetGroupMemberRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub selection_policy: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateTargetGroupRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                display_name: Default::default(),
                enabled: Default::default(),
                members: Default::default(),
                name: Default::default(),
                selection_policy: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`CreateVectorDatabaseRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "embedding_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "CreateVectorDatabaseRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CreateVectorDatabaseRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub embedding_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for CreateVectorDatabaseRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                embedding_serving_name: Default::default(),
                name: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`DebitBalanceRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "amount_cents": {
    ///      "type": "number"
    ///    },
    ///    "reference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "DebitBalanceRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebitBalanceRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_cents: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for DebitBalanceRequest {
        fn default() -> Self {
            Self {
                amount_cents: Default::default(),
                reference: Default::default(),
            }
        }
    }
    ///`DebitBalanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "new_balance": {
    ///      "type": "number"
    ///    },
    ///    "success": {
    ///      "type": "boolean"
    ///    }
    ///  },
    ///  "x-go-name": "DebitBalanceResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DebitBalanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub new_balance: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub success: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for DebitBalanceResponse {
        fn default() -> Self {
            Self {
                new_balance: Default::default(),
                success: Default::default(),
            }
        }
    }
    ///`EmbeddingsRequestEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "input": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "model": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "EmbeddingsRequestEnvelope"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct EmbeddingsRequestEnvelope {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub input: ::std::vec::Vec<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for EmbeddingsRequestEnvelope {
        fn default() -> Self {
            Self {
                input: Default::default(),
                model: Default::default(),
            }
        }
    }
    ///`ErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "error": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ErrorResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ErrorResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ErrorResponse {
        fn default() -> Self {
            Self { error: Default::default() }
        }
    }
    ///`FinishPasskeyRegistrationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "challenge_id",
    ///    "credential"
    ///  ],
    ///  "properties": {
    ///    "challenge_id": {
    ///      "type": "string"
    ///    },
    ///    "credential": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "FinishPasskeyRegistrationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FinishPasskeyRegistrationRequest {
        pub challenge_id: ::std::string::String,
        pub credential: ::std::vec::Vec<i64>,
    }
    ///`FinishPasskeyloginRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "challenge_id",
    ///    "credential",
    ///    "email"
    ///  ],
    ///  "properties": {
    ///    "cf_turnstile_response": {
    ///      "type": "string"
    ///    },
    ///    "challenge_id": {
    ///      "type": "string"
    ///    },
    ///    "credential": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FinishPasskeyloginRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FinishPasskeyloginRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cf_turnstile_response: ::std::option::Option<::std::string::String>,
        pub challenge_id: ::std::string::String,
        pub credential: ::std::vec::Vec<i64>,
        pub email: ::std::string::String,
    }
    ///`FinishPasskeyloginResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "access_token_expiry": {
    ///      "type": "integer"
    ///    },
    ///    "refresh_token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FinishPasskeyloginResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FinishPasskeyloginResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token_expiry: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub refresh_token: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FinishPasskeyloginResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expiry: Default::default(),
                refresh_token: Default::default(),
            }
        }
    }
    ///`FirewallEvaluationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "input": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FirewallEvaluationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FirewallEvaluationRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub input: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FirewallEvaluationRequest {
        fn default() -> Self {
            Self { input: Default::default() }
        }
    }
    ///`FirewallEvaluationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "evaluator_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "firewall_slug": {
    ///      "type": "string"
    ///    },
    ///    "matched_rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "mode": {
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "type": "string"
    ///    },
    ///    "severity": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FirewallEvaluationResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FirewallEvaluationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub evaluator_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub firewall_slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub matched_rules: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reason: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub severity: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FirewallEvaluationResponse {
        fn default() -> Self {
            Self {
                allowed: Default::default(),
                evaluator_serving_name: Default::default(),
                firewall_slug: Default::default(),
                matched_rules: Default::default(),
                mode: Default::default(),
                reason: Default::default(),
                severity: Default::default(),
            }
        }
    }
    ///`FirewallResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "evaluator_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "mode": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule_slugs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FirewallResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FirewallResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub evaluator_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rule_slugs: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FirewallResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                description: Default::default(),
                evaluator_serving_name: Default::default(),
                mode: Default::default(),
                name: Default::default(),
                rule_slugs: Default::default(),
                slug: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`FirewallRuleEvaluationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "evaluator_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "input": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FirewallRuleEvaluationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FirewallRuleEvaluationRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub evaluator_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub input: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FirewallRuleEvaluationRequest {
        fn default() -> Self {
            Self {
                evaluator_serving_name: Default::default(),
                input: Default::default(),
            }
        }
    }
    ///`FirewallRuleResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "default_severity": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "prompt": {
    ///      "type": "string"
    ///    },
    ///    "recommended": {
    ///      "type": "boolean"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "FirewallRuleResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FirewallRuleResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_severity: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prompt: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommended: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for FirewallRuleResponse {
        fn default() -> Self {
            Self {
                category: Default::default(),
                created_at: Default::default(),
                default_severity: Default::default(),
                description: Default::default(),
                name: Default::default(),
                prompt: Default::default(),
                recommended: Default::default(),
                slug: Default::default(),
                source: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`ForgotPasswordRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email"
    ///  ],
    ///  "properties": {
    ///    "cf_turnstile_response": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ForgotPasswordRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ForgotPasswordRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cf_turnstile_response: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
    }
    ///`ForgotPasswordResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ForgotPasswordResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ForgotPasswordResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ForgotPasswordResponse {
        fn default() -> Self {
            Self {
                message: Default::default(),
            }
        }
    }
    ///`GetInstanceUsageRange`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "24h",
    ///    "7d",
    ///    "30d"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum GetInstanceUsageRange {
        #[serde(rename = "24h")]
        X24h,
        #[serde(rename = "7d")]
        X7d,
        #[serde(rename = "30d")]
        X30d,
    }
    impl ::std::fmt::Display for GetInstanceUsageRange {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X24h => f.write_str("24h"),
                Self::X7d => f.write_str("7d"),
                Self::X30d => f.write_str("30d"),
            }
        }
    }
    impl ::std::str::FromStr for GetInstanceUsageRange {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "24h" => Ok(Self::X24h),
                "7d" => Ok(Self::X7d),
                "30d" => Ok(Self::X30d),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetInstanceUsageRange {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetInstanceUsageRange {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetInstanceUsageRange {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GetTemplateBySlugResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "template": {
    ///      "$ref": "#/components/schemas/TemplateWithVariants"
    ///    }
    ///  },
    ///  "x-go-name": "GetTemplateBySlugResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetTemplateBySlugResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template: ::std::option::Option<TemplateWithVariants>,
    }
    impl ::std::default::Default for GetTemplateBySlugResponse {
        fn default() -> Self {
            Self {
                template: Default::default(),
            }
        }
    }
    ///`GetUserVariantResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "variant": {
    ///      "$ref": "#/components/schemas/VariantWithRelations"
    ///    }
    ///  },
    ///  "x-go-name": "GetUserVariantResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetUserVariantResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub variant: ::std::option::Option<VariantWithRelations>,
    }
    impl ::std::default::Default for GetUserVariantResponse {
        fn default() -> Self {
            Self {
                variant: Default::default(),
            }
        }
    }
    ///`GpuListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "gpus": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CatalogGPUResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "GpuListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GpuListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gpus: ::std::vec::Vec<CatalogGpuResponse>,
    }
    impl ::std::default::Default for GpuListResponse {
        fn default() -> Self {
            Self { gpus: Default::default() }
        }
    }
    ///`GpuPreferenceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "type": "integer"
    ///    },
    ///    "provider_codename": {
    ///      "type": "string"
    ///    },
    ///    "public_gpu_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "GpuPreferenceResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GpuPreferenceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub provider_codename: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_gpu_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for GpuPreferenceResponse {
        fn default() -> Self {
            Self {
                count: Default::default(),
                provider_codename: Default::default(),
                public_gpu_id: Default::default(),
            }
        }
    }
    ///`HuggingFaceGgufFileResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "filename": {
    ///      "type": "string"
    ///    },
    ///    "quantization": {
    ///      "type": "string"
    ///    },
    ///    "size_bytes": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "HuggingFaceGGUFFileResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HuggingFaceGgufFileResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub filename: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantization: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size_bytes: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for HuggingFaceGgufFileResponse {
        fn default() -> Self {
            Self {
                filename: Default::default(),
                quantization: Default::default(),
                size_bytes: Default::default(),
            }
        }
    }
    ///`HuggingFaceModelResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "downloads": {
    ///      "type": "integer"
    ///    },
    ///    "gguf_files": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/HuggingFaceGGUFFileResponse"
    ///      }
    ///    },
    ///    "has_gguf": {
    ///      "type": "boolean"
    ///    },
    ///    "has_safetensors": {
    ///      "type": "boolean"
    ///    },
    ///    "likes": {
    ///      "type": "integer"
    ///    },
    ///    "pipeline_tag": {
    ///      "type": "string"
    ///    },
    ///    "repo_id": {
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "warnings": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "HuggingFaceModelResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HuggingFaceModelResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub downloads: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gguf_files: ::std::vec::Vec<HuggingFaceGgufFileResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub has_gguf: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub has_safetensors: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub likes: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline_tag: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repo_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub warnings: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for HuggingFaceModelResponse {
        fn default() -> Self {
            Self {
                downloads: Default::default(),
                gguf_files: Default::default(),
                has_gguf: Default::default(),
                has_safetensors: Default::default(),
                likes: Default::default(),
                pipeline_tag: Default::default(),
                repo_id: Default::default(),
                tags: Default::default(),
                warnings: Default::default(),
            }
        }
    }
    ///`HuggingFaceModelsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "models": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/HuggingFaceModelResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "HuggingFaceModelsResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HuggingFaceModelsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub models: ::std::vec::Vec<HuggingFaceModelResponse>,
    }
    impl ::std::default::Default for HuggingFaceModelsResponse {
        fn default() -> Self {
            Self { models: Default::default() }
        }
    }
    ///`HuggingFaceOrgInfoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "avatar_url": {
    ///      "type": "string"
    ///    },
    ///    "fullname": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "HuggingFaceOrgInfoResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct HuggingFaceOrgInfoResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fullname: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for HuggingFaceOrgInfoResponse {
        fn default() -> Self {
            Self {
                avatar_url: Default::default(),
                fullname: Default::default(),
            }
        }
    }
    ///`ImageGenerationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "guidance_scale": {
    ///      "type": "number"
    ///    },
    ///    "height": {
    ///      "type": "integer"
    ///    },
    ///    "model": {
    ///      "type": "string"
    ///    },
    ///    "n": {
    ///      "type": "integer"
    ///    },
    ///    "num_inference_steps": {
    ///      "type": "integer"
    ///    },
    ///    "prompt": {
    ///      "type": "string"
    ///    },
    ///    "quality": {
    ///      "type": "string"
    ///    },
    ///    "response_format": {
    ///      "type": "string"
    ///    },
    ///    "seed": {
    ///      "type": "integer"
    ///    },
    ///    "size": {
    ///      "type": "string"
    ///    },
    ///    "style": {
    ///      "type": "string"
    ///    },
    ///    "user": {
    ///      "type": "string"
    ///    },
    ///    "width": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "ImageGenerationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ImageGenerationRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub guidance_scale: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub height: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub n: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub num_inference_steps: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prompt: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quality: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub response_format: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub seed: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub style: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub width: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ImageGenerationRequest {
        fn default() -> Self {
            Self {
                guidance_scale: Default::default(),
                height: Default::default(),
                model: Default::default(),
                n: Default::default(),
                num_inference_steps: Default::default(),
                prompt: Default::default(),
                quality: Default::default(),
                response_format: Default::default(),
                seed: Default::default(),
                size: Default::default(),
                style: Default::default(),
                user: Default::default(),
                width: Default::default(),
            }
        }
    }
    ///`IndexDocumentRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "content_image_url": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/IndexDocumentSourceRequest"
    ///    }
    ///  },
    ///  "x-go-name": "IndexDocumentRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct IndexDocumentRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content_image_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub metadata: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<IndexDocumentSourceRequest>,
    }
    impl ::std::default::Default for IndexDocumentRequest {
        fn default() -> Self {
            Self {
                content: Default::default(),
                content_image_url: Default::default(),
                id: Default::default(),
                metadata: Default::default(),
                source: Default::default(),
            }
        }
    }
    ///`IndexDocumentSourceRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "image_url": {
    ///      "type": "string"
    ///    },
    ///    "text": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "IndexDocumentSourceRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct IndexDocumentSourceRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for IndexDocumentSourceRequest {
        fn default() -> Self {
            Self {
                image_url: Default::default(),
                text: Default::default(),
            }
        }
    }
    ///`IndexDocumentsRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "documents": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IndexDocumentRequest"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "IndexDocumentsRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct IndexDocumentsRequest {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub documents: ::std::vec::Vec<IndexDocumentRequest>,
    }
    impl ::std::default::Default for IndexDocumentsRequest {
        fn default() -> Self {
            Self {
                documents: Default::default(),
            }
        }
    }
    ///`IndexJobResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "completed_at": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "database_id": {
    ///      "type": "string"
    ///    },
    ///    "error_message": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "kind": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "IndexJobResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct IndexJobResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub completed_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub database_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for IndexJobResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                completed_at: Default::default(),
                created_at: Default::default(),
                database_id: Default::default(),
                error_message: Default::default(),
                id: Default::default(),
                kind: Default::default(),
                status: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`InstanceListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "instances": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "InstanceListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InstanceListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub instances: ::std::vec::Vec<InstanceResponse>,
    }
    impl ::std::default::Default for InstanceListResponse {
        fn default() -> Self {
            Self {
                instances: Default::default(),
            }
        }
    }
    ///`InstanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "capacity_tier": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "estimated_required_vram_gb": {
    ///      "type": "number"
    ///    },
    ///    "failure_reason": {
    ///      "type": "string"
    ///    },
    ///    "gpu_count": {
    ///      "type": "integer"
    ///    },
    ///    "gpu_preferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GpuPreferenceResponse"
    ///      }
    ///    },
    ///    "huggingface_repo_id": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "idle_timeout_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "last_request_at": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "public_gpu_id": {
    ///      "type": "string"
    ///    },
    ///    "public_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "replica_set_id": {
    ///      "type": "string"
    ///    },
    ///    "runtime_config_snapshot": {
    ///      "type": "string"
    ///    },
    ///    "runtime_preset": {
    ///      "type": "string"
    ///    },
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "serverless_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "serving_name": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_max_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_region": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "smart_selection_label": {
    ///      "type": "string"
    ///    },
    ///    "started_at": {
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "type": "string"
    ///    },
    ///    "stopped_at": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "InstanceResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InstanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub capacity_tier: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub estimated_required_vram_gb: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub failure_reason: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gpu_preferences: ::std::vec::Vec<GpuPreferenceResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub huggingface_repo_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub idle_timeout_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_request_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_gpu_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub replica_set_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_config_snapshot: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_preset: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serverless_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_selection_label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub started_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stopped_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for InstanceResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                capacity_tier: Default::default(),
                created_at: Default::default(),
                description: Default::default(),
                estimated_required_vram_gb: Default::default(),
                failure_reason: Default::default(),
                gpu_count: Default::default(),
                gpu_preferences: Default::default(),
                huggingface_repo_id: Default::default(),
                id: Default::default(),
                idle_timeout_seconds: Default::default(),
                last_request_at: Default::default(),
                name: Default::default(),
                public_gpu_id: Default::default(),
                public_price_per_hour_eur: Default::default(),
                replica_set_id: Default::default(),
                runtime_config_snapshot: Default::default(),
                runtime_preset: Default::default(),
                scheduling_mode: Default::default(),
                serverless_enabled: Default::default(),
                serving_name: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_max_price_per_hour_eur: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_region: Default::default(),
                smart_regions: Default::default(),
                smart_selection_label: Default::default(),
                started_at: Default::default(),
                state: Default::default(),
                stopped_at: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`InstanceUsageBucketResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "billable_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "cost_eur": {
    ///      "type": "number"
    ///    },
    ///    "end_at": {
    ///      "type": "string"
    ///    },
    ///    "start_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "InstanceUsageBucketResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InstanceUsageBucketResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub billable_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cost_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for InstanceUsageBucketResponse {
        fn default() -> Self {
            Self {
                billable_seconds: Default::default(),
                cost_eur: Default::default(),
                end_at: Default::default(),
                start_at: Default::default(),
            }
        }
    }
    ///`InstanceUsageResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "bucket_unit": {
    ///      "type": "string"
    ///    },
    ///    "range": {
    ///      "type": "string"
    ///    },
    ///    "resource_metrics": {
    ///      "$ref": "#/components/schemas/ResourceMetricsResponse"
    ///    },
    ///    "series": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceUsageBucketResponse"
    ///      }
    ///    },
    ///    "summary": {
    ///      "$ref": "#/components/schemas/InstanceUsageSummaryResponse"
    ///    }
    ///  },
    ///  "x-go-name": "InstanceUsageResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InstanceUsageResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bucket_unit: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub range: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resource_metrics: ::std::option::Option<ResourceMetricsResponse>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub series: ::std::vec::Vec<InstanceUsageBucketResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub summary: ::std::option::Option<InstanceUsageSummaryResponse>,
    }
    impl ::std::default::Default for InstanceUsageResponse {
        fn default() -> Self {
            Self {
                bucket_unit: Default::default(),
                range: Default::default(),
                resource_metrics: Default::default(),
                series: Default::default(),
                summary: Default::default(),
            }
        }
    }
    ///`InstanceUsageSummaryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "month_billable_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "month_daily_average_cost_eur": {
    ///      "type": "number"
    ///    },
    ///    "month_total_cost_eur": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "InstanceUsageSummaryResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InstanceUsageSummaryResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub month_billable_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub month_daily_average_cost_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub month_total_cost_eur: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for InstanceUsageSummaryResponse {
        fn default() -> Self {
            Self {
                month_billable_seconds: Default::default(),
                month_daily_average_cost_eur: Default::default(),
                month_total_cost_eur: Default::default(),
            }
        }
    }
    ///`InvitationListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "invitations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InvitationResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "InvitationListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InvitationListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub invitations: ::std::vec::Vec<InvitationResponse>,
    }
    impl ::std::default::Default for InvitationListResponse {
        fn default() -> Self {
            Self {
                invitations: Default::default(),
            }
        }
    }
    ///`InvitationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accepted_at_unix": {
    ///      "type": "integer"
    ///    },
    ///    "accepted_by": {
    ///      "type": "string"
    ///    },
    ///    "account_description": {
    ///      "type": "string"
    ///    },
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "account_name": {
    ///      "type": "string"
    ///    },
    ///    "created_at_unix": {
    ///      "type": "integer"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "expires_at_unix": {
    ///      "type": "integer"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "invited_by": {
    ///      "type": "string"
    ///    },
    ///    "role": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "InvitationResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InvitationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub accepted_at_unix: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub accepted_by: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at_unix: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires_at_unix: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invited_by: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for InvitationResponse {
        fn default() -> Self {
            Self {
                accepted_at_unix: Default::default(),
                accepted_by: Default::default(),
                account_description: Default::default(),
                account_id: Default::default(),
                account_name: Default::default(),
                created_at_unix: Default::default(),
                email: Default::default(),
                expires_at_unix: Default::default(),
                id: Default::default(),
                invited_by: Default::default(),
                role: Default::default(),
                status: Default::default(),
                token: Default::default(),
            }
        }
    }
    ///`LedgerEntryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "amount": {
    ///      "type": "number"
    ///    },
    ///    "balance": {
    ///      "type": "number"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "event_type": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "reference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "LedgerEntryResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LedgerEntryResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub balance: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub event_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for LedgerEntryResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                amount: Default::default(),
                balance: Default::default(),
                created_at: Default::default(),
                event_type: Default::default(),
                id: Default::default(),
                reference: Default::default(),
            }
        }
    }
    ///`LedgerHistoryResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "entries": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/LedgerEntryResponse"
    ///      }
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "LedgerHistoryResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LedgerHistoryResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub entries: ::std::vec::Vec<LedgerEntryResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for LedgerHistoryResponse {
        fn default() -> Self {
            Self {
                entries: Default::default(),
                total: Default::default(),
            }
        }
    }
    ///`ListDocumentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "documents": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ListedDocumentResponse"
    ///      }
    ///    },
    ///    "next_cursor": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ListDocumentsResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListDocumentsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub documents: ::std::vec::Vec<ListedDocumentResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_cursor: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ListDocumentsResponse {
        fn default() -> Self {
            Self {
                documents: Default::default(),
                next_cursor: Default::default(),
            }
        }
    }
    ///`ListUserVariantsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "items": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VariantWithRelations"
    ///      }
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "ListUserVariantsResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListUserVariantsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub items: ::std::vec::Vec<VariantWithRelations>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ListUserVariantsResponse {
        fn default() -> Self {
            Self {
                items: Default::default(),
                total: Default::default(),
            }
        }
    }
    ///`ListedDocumentResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/SearchHitSourceResponse"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ListedDocumentResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ListedDocumentResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub metadata: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<SearchHitSourceResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ListedDocumentResponse {
        fn default() -> Self {
            Self {
                content: Default::default(),
                id: Default::default(),
                metadata: Default::default(),
                source: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`LoginRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "password"
    ///  ],
    ///  "properties": {
    ///    "cf_turnstile_response": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "password": {
    ///      "type": "string"
    ///    },
    ///    "totp_code": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "LoginRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoginRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cf_turnstile_response: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub password: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub totp_code: ::std::option::Option<::std::string::String>,
    }
    ///`LoginResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "access_token_expiry": {
    ///      "type": "integer"
    ///    },
    ///    "next_step": {
    ///      "type": "string"
    ///    },
    ///    "refresh_token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "LoginResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LoginResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token_expiry: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_step: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub refresh_token: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for LoginResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expiry: Default::default(),
                next_step: Default::default(),
                refresh_token: Default::default(),
            }
        }
    }
    ///`Model`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "downloads": {
    ///      "type": "integer"
    ///    },
    ///    "likes": {
    ///      "type": "integer"
    ///    },
    ///    "organization": {
    ///      "type": "string"
    ///    },
    ///    "organization_logo_url": {
    ///      "type": "string"
    ///    },
    ///    "pipeline_tag": {
    ///      "type": "string"
    ///    },
    ///    "price_eur_per_hour": {
    ///      "type": "number"
    ///    },
    ///    "pricing_status": {
    ///      "$ref": "#/components/schemas/PricingStatus"
    ///    },
    ///    "recommended_gpu": {
    ///      "$ref": "#/components/schemas/RecommendedGPU"
    ///    },
    ///    "repo_id": {
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "Model"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Model {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub downloads: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub likes: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub organization: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub organization_logo_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline_tag: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price_eur_per_hour: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pricing_status: ::std::option::Option<PricingStatus>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommended_gpu: ::std::option::Option<RecommendedGpu>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repo_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for Model {
        fn default() -> Self {
            Self {
                display_name: Default::default(),
                downloads: Default::default(),
                likes: Default::default(),
                organization: Default::default(),
                organization_logo_url: Default::default(),
                pipeline_tag: Default::default(),
                price_eur_per_hour: Default::default(),
                pricing_status: Default::default(),
                recommended_gpu: Default::default(),
                repo_id: Default::default(),
                tags: Default::default(),
            }
        }
    }
    ///`ModelPricesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "models": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Model"
    ///      }
    ///    },
    ///    "query": {
    ///      "type": "string"
    ///    },
    ///    "services": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PricingService"
    ///      }
    ///    },
    ///    "source": {
    ///      "type": "string"
    ///    },
    ///    "source_status": {
    ///      "type": "string"
    ///    },
    ///    "stale": {
    ///      "type": "boolean"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "ModelPricesResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ModelPricesResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub models: ::std::vec::Vec<Model>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub services: ::std::vec::Vec<PricingService>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stale: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ModelPricesResponse {
        fn default() -> Self {
            Self {
                models: Default::default(),
                query: Default::default(),
                services: Default::default(),
                source: Default::default(),
                source_status: Default::default(),
                stale: Default::default(),
                updated_at: Default::default(),
                version: Default::default(),
            }
        }
    }
    ///`PasskeyListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "passkeys": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PasskeyResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "PasskeyListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PasskeyListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub passkeys: ::std::vec::Vec<PasskeyResponse>,
    }
    impl ::std::default::Default for PasskeyListResponse {
        fn default() -> Self {
            Self {
                passkeys: Default::default(),
            }
        }
    }
    ///`PasskeyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "credential_id": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "string"
    ///    },
    ///    "sign_count": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "PasskeyResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PasskeyResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub credential_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sign_count: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for PasskeyResponse {
        fn default() -> Self {
            Self {
                credential_id: Default::default(),
                public_key: Default::default(),
                sign_count: Default::default(),
            }
        }
    }
    ///`PaymentResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "amount_credit_base_minor": {
    ///      "type": "integer"
    ///    },
    ///    "amount_discount_minor": {
    ///      "type": "integer"
    ///    },
    ///    "amount_subtotal_minor": {
    ///      "type": "integer"
    ///    },
    ///    "amount_tax_minor": {
    ///      "type": "integer"
    ///    },
    ///    "amount_total_minor": {
    ///      "type": "integer"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "credited_at": {
    ///      "type": "string"
    ///    },
    ///    "credited_eur_cents": {
    ///      "type": "integer"
    ///    },
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "discount_code_snapshot": {
    ///      "type": "string"
    ///    },
    ///    "discount_ticket_id": {
    ///      "type": "string"
    ///    },
    ///    "holded_contact_id": {
    ///      "type": "string"
    ///    },
    ///    "holded_invoice_id": {
    ///      "type": "string"
    ///    },
    ///    "holded_invoice_number": {
    ///      "type": "string"
    ///    },
    ///    "holded_invoice_status": {
    ///      "type": "string"
    ///    },
    ///    "holded_invoice_url": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "invoice_state": {
    ///      "type": "string"
    ///    },
    ///    "ledger_entry_id": {
    ///      "type": "string"
    ///    },
    ///    "paid_at": {
    ///      "type": "string"
    ///    },
    ///    "provider": {
    ///      "type": "string"
    ///    },
    ///    "reference": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "stripe_customer_id": {
    ///      "type": "string"
    ///    },
    ///    "stripe_event_id": {
    ///      "type": "string"
    ///    },
    ///    "stripe_payment_intent_id": {
    ///      "type": "string"
    ///    },
    ///    "stripe_session_id": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "PaymentResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PaymentResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_credit_base_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_discount_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_subtotal_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_tax_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_total_minor: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub credited_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub credited_eur_cents: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discount_code_snapshot: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discount_ticket_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub holded_contact_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub holded_invoice_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub holded_invoice_number: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub holded_invoice_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub holded_invoice_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub invoice_state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ledger_entry_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub paid_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stripe_customer_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stripe_event_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stripe_payment_intent_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stripe_session_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PaymentResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                amount_credit_base_minor: Default::default(),
                amount_discount_minor: Default::default(),
                amount_subtotal_minor: Default::default(),
                amount_tax_minor: Default::default(),
                amount_total_minor: Default::default(),
                created_at: Default::default(),
                credited_at: Default::default(),
                credited_eur_cents: Default::default(),
                currency: Default::default(),
                discount_code_snapshot: Default::default(),
                discount_ticket_id: Default::default(),
                holded_contact_id: Default::default(),
                holded_invoice_id: Default::default(),
                holded_invoice_number: Default::default(),
                holded_invoice_status: Default::default(),
                holded_invoice_url: Default::default(),
                id: Default::default(),
                invoice_state: Default::default(),
                ledger_entry_id: Default::default(),
                paid_at: Default::default(),
                provider: Default::default(),
                reference: Default::default(),
                status: Default::default(),
                stripe_customer_id: Default::default(),
                stripe_event_id: Default::default(),
                stripe_payment_intent_id: Default::default(),
                stripe_session_id: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`PaymentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "items": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PaymentResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "PaymentsResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PaymentsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub items: ::std::vec::Vec<PaymentResponse>,
    }
    impl ::std::default::Default for PaymentsResponse {
        fn default() -> Self {
            Self { items: Default::default() }
        }
    }
    ///`PreviewInstanceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "capacity_tier": {
    ///      "type": "string"
    ///    },
    ///    "compatible_gpus": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CatalogGPUResponse"
    ///      }
    ///    },
    ///    "estimate": {
    ///      "$ref": "#/components/schemas/RuntimeMemoryEstimateResponse"
    ///    },
    ///    "huggingface_repo_id": {
    ///      "type": "string"
    ///    },
    ///    "max_context_size": {
    ///      "type": "integer"
    ///    },
    ///    "recommended_gpu": {
    ///      "$ref": "#/components/schemas/CatalogGPUResponse"
    ///    },
    ///    "runtime_config": {
    ///      "$ref": "#/components/schemas/RuntimeConfigResponse"
    ///    },
    ///    "runtime_preset": {
    ///      "type": "string"
    ///    },
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_current_gpu": {
    ///      "$ref": "#/components/schemas/CatalogGPUResponse"
    ///    },
    ///    "smart_current_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_max_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_recommended_max_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_region": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "smart_selection_label": {
    ///      "type": "string"
    ///    },
    ///    "warnings": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "PreviewInstanceResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PreviewInstanceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub capacity_tier: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub compatible_gpus: ::std::vec::Vec<CatalogGpuResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub estimate: ::std::option::Option<RuntimeMemoryEstimateResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub huggingface_repo_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_context_size: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommended_gpu: ::std::option::Option<CatalogGpuResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_config: ::std::option::Option<RuntimeConfigResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_preset: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_current_gpu: ::std::option::Option<CatalogGpuResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_current_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_recommended_max_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_selection_label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub warnings: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PreviewInstanceResponse {
        fn default() -> Self {
            Self {
                capacity_tier: Default::default(),
                compatible_gpus: Default::default(),
                estimate: Default::default(),
                huggingface_repo_id: Default::default(),
                max_context_size: Default::default(),
                recommended_gpu: Default::default(),
                runtime_config: Default::default(),
                runtime_preset: Default::default(),
                scheduling_mode: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_current_gpu: Default::default(),
                smart_current_price_per_hour_eur: Default::default(),
                smart_max_price_per_hour_eur: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_recommended_max_price_per_hour_eur: Default::default(),
                smart_region: Default::default(),
                smart_regions: Default::default(),
                smart_selection_label: Default::default(),
                warnings: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`PreviewInstanceUpdateRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_max_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_region": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "smart_selection_label": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "PreviewInstanceUpdateRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PreviewInstanceUpdateRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_selection_label: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PreviewInstanceUpdateRequest {
        fn default() -> Self {
            Self {
                scheduling_mode: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_max_price_per_hour_eur: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_region: Default::default(),
                smart_regions: Default::default(),
                smart_selection_label: Default::default(),
            }
        }
    }
    ///`PricingService`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "price_eur_per_gb_hour": {
    ///      "type": "number"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "PricingService"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PricingService {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price_eur_per_gb_hour: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PricingService {
        fn default() -> Self {
            Self {
                currency: Default::default(),
                id: Default::default(),
                price_eur_per_gb_hour: Default::default(),
                status: Default::default(),
            }
        }
    }
    ///`PricingStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "estimated",
    ///    "unavailable"
    ///  ],
    ///  "x-enum-varnames": [
    ///    "PricingEstimated",
    ///    "PricingUnavailable"
    ///  ],
    ///  "x-go-name": "PricingStatus"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum PricingStatus {
        #[serde(rename = "estimated")]
        Estimated,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    impl ::std::fmt::Display for PricingStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Estimated => f.write_str("estimated"),
                Self::Unavailable => f.write_str("unavailable"),
            }
        }
    }
    impl ::std::str::FromStr for PricingStatus {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "estimated" => Ok(Self::Estimated),
                "unavailable" => Ok(Self::Unavailable),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PricingStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PricingStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PricingStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`PublicGpuPreferenceJson`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "type": "integer"
    ///    },
    ///    "public_gpu_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "PublicGPUPreferenceJSON"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PublicGpuPreferenceJson {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_gpu_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PublicGpuPreferenceJson {
        fn default() -> Self {
            Self {
                count: Default::default(),
                public_gpu_id: Default::default(),
            }
        }
    }
    ///`PublicPricingAvailability`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "type": "integer"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "PublicPricingAvailability"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PublicPricingAvailability {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for PublicPricingAvailability {
        fn default() -> Self {
            Self {
                count: Default::default(),
                status: Default::default(),
                total: Default::default(),
            }
        }
    }
    ///`PublicPricingGpu`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "availability": {
    ///      "$ref": "#/components/schemas/PublicPricingAvailability"
    ///    },
    ///    "capacity_tier": {
    ///      "type": "string"
    ///    },
    ///    "city": {
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "gpu_count": {
    ///      "type": "integer"
    ///    },
    ///    "market_type": {
    ///      "type": "string"
    ///    },
    ///    "price_eur_per_hour": {
    ///      "type": "number"
    ///    },
    ///    "provider_codename": {
    ///      "type": "string"
    ///    },
    ///    "public_gpu_id": {
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "vram_gb": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "PublicPricingGPU"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PublicPricingGpu {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub availability: ::std::option::Option<PublicPricingAvailability>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub capacity_tier: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub city: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub market_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price_eur_per_hour: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub provider_codename: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_gpu_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub vram_gb: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for PublicPricingGpu {
        fn default() -> Self {
            Self {
                availability: Default::default(),
                capacity_tier: Default::default(),
                city: Default::default(),
                country: Default::default(),
                display_name: Default::default(),
                gpu_count: Default::default(),
                market_type: Default::default(),
                price_eur_per_hour: Default::default(),
                provider_codename: Default::default(),
                public_gpu_id: Default::default(),
                region: Default::default(),
                vram_gb: Default::default(),
            }
        }
    }
    ///`PublicPricingResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "services": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PublicPricingService"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "PublicPricingResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PublicPricingResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub services: ::std::vec::Vec<PublicPricingService>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for PublicPricingResponse {
        fn default() -> Self {
            Self {
                services: Default::default(),
                updated_at: Default::default(),
                version: Default::default(),
            }
        }
    }
    ///`PublicPricingService`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "billing_model": {
    ///      "type": "string"
    ///    },
    ///    "currency": {
    ///      "type": "string"
    ///    },
    ///    "gpus": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PublicPricingGPU"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "price_eur_per_gb_hour": {
    ///      "type": "number"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "unit": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "PublicPricingService"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PublicPricingService {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub billing_model: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub currency: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gpus: ::std::vec::Vec<PublicPricingGpu>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price_eur_per_gb_hour: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub unit: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for PublicPricingService {
        fn default() -> Self {
            Self {
                billing_model: Default::default(),
                currency: Default::default(),
                gpus: Default::default(),
                id: Default::default(),
                name: Default::default(),
                price_eur_per_gb_hour: Default::default(),
                status: Default::default(),
                unit: Default::default(),
            }
        }
    }
    ///`RecommendRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "recommend_token": {
    ///      "type": "string"
    ///    },
    ///    "top_k": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "RecommendRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RecommendRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommend_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub top_k: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for RecommendRequest {
        fn default() -> Self {
            Self {
                recommend_token: Default::default(),
                top_k: Default::default(),
            }
        }
    }
    ///`RecommendedGpu`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "availability": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "gpu_count": {
    ///      "type": "integer"
    ///    },
    ///    "market_type": {
    ///      "type": "string"
    ///    },
    ///    "vram_gb": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "RecommendedGPU"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RecommendedGpu {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub availability: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub market_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub vram_gb: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for RecommendedGpu {
        fn default() -> Self {
            Self {
                availability: Default::default(),
                display_name: Default::default(),
                gpu_count: Default::default(),
                market_type: Default::default(),
                vram_gb: Default::default(),
            }
        }
    }
    ///`RefreshRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "refresh_token"
    ///  ],
    ///  "properties": {
    ///    "refresh_token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "RefreshRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RefreshRequest {
        pub refresh_token: ::std::string::String,
    }
    ///`RefreshResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "access_token_expiry": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "RefreshResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RefreshResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token_expiry: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for RefreshResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expiry: Default::default(),
            }
        }
    }
    ///`RegisterPasskeyRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "credential_id",
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "credential_id": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "string"
    ///    },
    ///    "sign_count": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "RegisterPasskeyRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RegisterPasskeyRequest {
        pub credential_id: ::std::string::String,
        pub public_key: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sign_count: ::std::option::Option<i64>,
    }
    ///`RegisterRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "password"
    ///  ],
    ///  "properties": {
    ///    "cf_turnstile_response": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "password": {
    ///      "type": "string",
    ///      "minLength": 8
    ///    },
    ///    "resend": {
    ///      "type": "boolean"
    ///    }
    ///  },
    ///  "x-go-name": "RegisterRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RegisterRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cf_turnstile_response: ::std::option::Option<::std::string::String>,
        pub email: ::std::string::String,
        pub password: RegisterRequestPassword,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resend: ::std::option::Option<bool>,
    }
    ///`RegisterRequestPassword`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 8
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RegisterRequestPassword(::std::string::String);
    impl ::std::ops::Deref for RegisterRequestPassword {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<RegisterRequestPassword> for ::std::string::String {
        fn from(value: RegisterRequestPassword) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for RegisterRequestPassword {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 8usize {
                return Err("shorter than 8 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for RegisterRequestPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RegisterRequestPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RegisterRequestPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RegisterRequestPassword {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`RegisterResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "access_token_expiry": {
    ///      "type": "integer"
    ///    },
    ///    "refresh_token": {
    ///      "type": "string"
    ///    },
    ///    "requires_verification": {
    ///      "type": "boolean"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "RegisterResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RegisterResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token_expiry: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub refresh_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requires_verification: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for RegisterResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expiry: Default::default(),
                refresh_token: Default::default(),
                requires_verification: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    ///`ReplicaSetListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "replica_sets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReplicaSetResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "ReplicaSetListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ReplicaSetListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub replica_sets: ::std::vec::Vec<ReplicaSetResponse>,
    }
    impl ::std::default::Default for ReplicaSetListResponse {
        fn default() -> Self {
            Self {
                replica_sets: Default::default(),
            }
        }
    }
    ///`ReplicaSetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "desired_replicas": {
    ///      "type": "integer"
    ///    },
    ///    "gguf_model_path": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "min_replicas": {
    ///      "type": "integer"
    ///    },
    ///    "model_ref": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "serving_endpoint_id": {
    ///      "type": "string"
    ///    },
    ///    "serving_name": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_max_price_per_hour_usd": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_region": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "smart_selection_label": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "target_group_id": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ReplicaSetResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ReplicaSetResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub desired_replicas: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gguf_model_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub min_replicas: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_ref: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_endpoint_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_usd: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_selection_label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_group_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ReplicaSetResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                created_at: Default::default(),
                desired_replicas: Default::default(),
                gguf_model_path: Default::default(),
                id: Default::default(),
                min_replicas: Default::default(),
                model_ref: Default::default(),
                name: Default::default(),
                scheduling_mode: Default::default(),
                serving_endpoint_id: Default::default(),
                serving_name: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_max_price_per_hour_usd: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_region: Default::default(),
                smart_regions: Default::default(),
                smart_selection_label: Default::default(),
                status: Default::default(),
                target_group_id: Default::default(),
                updated_at: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`ResetPasswordRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "new_password",
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "cf_turnstile_response": {
    ///      "type": "string"
    ///    },
    ///    "new_password": {
    ///      "type": "string",
    ///      "minLength": 8
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ResetPasswordRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResetPasswordRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cf_turnstile_response: ::std::option::Option<::std::string::String>,
        pub new_password: ResetPasswordRequestNewPassword,
        pub token: ::std::string::String,
    }
    ///`ResetPasswordRequestNewPassword`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "minLength": 8
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ResetPasswordRequestNewPassword(::std::string::String);
    impl ::std::ops::Deref for ResetPasswordRequestNewPassword {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ResetPasswordRequestNewPassword>
    for ::std::string::String {
        fn from(value: ResetPasswordRequestNewPassword) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ResetPasswordRequestNewPassword {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 8usize {
                return Err("shorter than 8 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ResetPasswordRequestNewPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ResetPasswordRequestNewPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ResetPasswordRequestNewPassword {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ResetPasswordRequestNewPassword {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`ResetPasswordResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "access_token_expiry": {
    ///      "type": "integer"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "refresh_token": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ResetPasswordResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResetPasswordResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token_expiry: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub refresh_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ResetPasswordResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expiry: Default::default(),
                email: Default::default(),
                refresh_token: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    ///`ResourceMetricSnapshot`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cpu_utilization_percent": {
    ///      "type": "number"
    ///    },
    ///    "disk_utilization_percent": {
    ///      "type": "number"
    ///    },
    ///    "gpu_memory_utilization_percent": {
    ///      "type": "number"
    ///    },
    ///    "gpu_power_watts": {
    ///      "type": "number"
    ///    },
    ///    "gpu_temperature_celsius": {
    ///      "type": "number"
    ///    },
    ///    "gpu_utilization_percent": {
    ///      "type": "number"
    ///    },
    ///    "memory_utilization_percent": {
    ///      "type": "number"
    ///    },
    ///    "sampled_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ResourceMetricSnapshot"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResourceMetricSnapshot {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu_utilization_percent: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub disk_utilization_percent: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_memory_utilization_percent: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_power_watts: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_temperature_celsius: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_utilization_percent: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub memory_utilization_percent: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sampled_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ResourceMetricSnapshot {
        fn default() -> Self {
            Self {
                cpu_utilization_percent: Default::default(),
                disk_utilization_percent: Default::default(),
                gpu_memory_utilization_percent: Default::default(),
                gpu_power_watts: Default::default(),
                gpu_temperature_celsius: Default::default(),
                gpu_utilization_percent: Default::default(),
                memory_utilization_percent: Default::default(),
                sampled_at: Default::default(),
            }
        }
    }
    ///`ResourceMetricsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "latest": {
    ///      "$ref": "#/components/schemas/ResourceMetricSnapshot"
    ///    },
    ///    "retention_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "sample_interval_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "series": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResourceMetricSnapshot"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "ResourceMetricsResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResourceMetricsResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latest: ::std::option::Option<ResourceMetricSnapshot>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub retention_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sample_interval_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub series: ::std::vec::Vec<ResourceMetricSnapshot>,
    }
    impl ::std::default::Default for ResourceMetricsResponse {
        fn default() -> Self {
            Self {
                latest: Default::default(),
                retention_seconds: Default::default(),
                sample_interval_seconds: Default::default(),
                series: Default::default(),
            }
        }
    }
    ///`ResponsesRequestEnvelope`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "input": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "model": {
    ///      "type": "string"
    ///    },
    ///    "previous_response_id": {
    ///      "type": "string"
    ///    },
    ///    "reasoning": {
    ///      "type": "object",
    ///      "properties": {
    ///        "effort": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "reasoning_effort": {
    ///      "type": "string"
    ///    },
    ///    "store": {
    ///      "type": "boolean"
    ///    },
    ///    "stream": {
    ///      "type": "boolean"
    ///    },
    ///    "thinking_token_budget": {
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ResponsesRequestEnvelope"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResponsesRequestEnvelope {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub input: ::std::vec::Vec<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub previous_response_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reasoning: ::std::option::Option<ResponsesRequestEnvelopeReasoning>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reasoning_effort: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub store: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stream: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub thinking_token_budget: ::std::option::Option<i64>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ResponsesRequestEnvelope {
        fn default() -> Self {
            Self {
                input: Default::default(),
                model: Default::default(),
                previous_response_id: Default::default(),
                reasoning: Default::default(),
                reasoning_effort: Default::default(),
                store: Default::default(),
                stream: Default::default(),
                thinking_token_budget: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ResponsesRequestEnvelopeReasoning`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "effort": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ResponsesRequestEnvelopeReasoning {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub effort: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ResponsesRequestEnvelopeReasoning {
        fn default() -> Self {
            Self { effort: Default::default() }
        }
    }
    ///`RuntimeConfigRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "context_size": {
    ///      "type": "integer"
    ///    },
    ///    "dtype": {
    ///      "type": "string"
    ///    },
    ///    "gpu_memory_utilization": {
    ///      "type": "number"
    ///    },
    ///    "kv_cache_dtype": {
    ///      "type": "string"
    ///    },
    ///    "max_num_batched_tokens": {
    ///      "type": "integer"
    ///    },
    ///    "profile": {
    ///      "type": "string"
    ///    },
    ///    "quantization": {
    ///      "type": "string"
    ///    },
    ///    "storage_mounts": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "mount_path": {
    ///            "type": "string"
    ///          },
    ///          "set_hf_home": {
    ///            "type": "boolean"
    ///          },
    ///          "subpath": {
    ///            "type": "string"
    ///          },
    ///          "volume_id": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "tensor_parallel_size": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "RuntimeConfigRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RuntimeConfigRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub context_size: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dtype: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_memory_utilization: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kv_cache_dtype: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_num_batched_tokens: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub profile: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantization: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub storage_mounts: ::std::vec::Vec<RuntimeConfigRequestStorageMountsItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tensor_parallel_size: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for RuntimeConfigRequest {
        fn default() -> Self {
            Self {
                context_size: Default::default(),
                dtype: Default::default(),
                gpu_memory_utilization: Default::default(),
                kv_cache_dtype: Default::default(),
                max_num_batched_tokens: Default::default(),
                profile: Default::default(),
                quantization: Default::default(),
                storage_mounts: Default::default(),
                tensor_parallel_size: Default::default(),
            }
        }
    }
    ///`RuntimeConfigRequestStorageMountsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "mount_path": {
    ///      "type": "string"
    ///    },
    ///    "set_hf_home": {
    ///      "type": "boolean"
    ///    },
    ///    "subpath": {
    ///      "type": "string"
    ///    },
    ///    "volume_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RuntimeConfigRequestStorageMountsItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mount_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub set_hf_home: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub subpath: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for RuntimeConfigRequestStorageMountsItem {
        fn default() -> Self {
            Self {
                mount_path: Default::default(),
                set_hf_home: Default::default(),
                subpath: Default::default(),
                volume_id: Default::default(),
            }
        }
    }
    ///`RuntimeConfigResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "context_size": {
    ///      "type": "integer"
    ///    },
    ///    "dtype": {
    ///      "type": "string"
    ///    },
    ///    "gpu_memory_utilization": {
    ///      "type": "number"
    ///    },
    ///    "kv_cache_dtype": {
    ///      "type": "string"
    ///    },
    ///    "max_num_batched_tokens": {
    ///      "type": "integer"
    ///    },
    ///    "profile": {
    ///      "type": "string"
    ///    },
    ///    "quantization": {
    ///      "type": "string"
    ///    },
    ///    "storage_mounts": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "mount_path": {
    ///            "type": "string"
    ///          },
    ///          "set_hf_home": {
    ///            "type": "boolean"
    ///          },
    ///          "subpath": {
    ///            "type": "string"
    ///          },
    ///          "volume_id": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "tensor_parallel_size": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "RuntimeConfigResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RuntimeConfigResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub context_size: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dtype: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_memory_utilization: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kv_cache_dtype: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_num_batched_tokens: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub profile: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantization: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub storage_mounts: ::std::vec::Vec<RuntimeConfigResponseStorageMountsItem>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tensor_parallel_size: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for RuntimeConfigResponse {
        fn default() -> Self {
            Self {
                context_size: Default::default(),
                dtype: Default::default(),
                gpu_memory_utilization: Default::default(),
                kv_cache_dtype: Default::default(),
                max_num_batched_tokens: Default::default(),
                profile: Default::default(),
                quantization: Default::default(),
                storage_mounts: Default::default(),
                tensor_parallel_size: Default::default(),
            }
        }
    }
    ///`RuntimeConfigResponseStorageMountsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "mount_path": {
    ///      "type": "string"
    ///    },
    ///    "set_hf_home": {
    ///      "type": "boolean"
    ///    },
    ///    "subpath": {
    ///      "type": "string"
    ///    },
    ///    "volume_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RuntimeConfigResponseStorageMountsItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mount_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub set_hf_home: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub subpath: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for RuntimeConfigResponseStorageMountsItem {
        fn default() -> Self {
            Self {
                mount_path: Default::default(),
                set_hf_home: Default::default(),
                subpath: Default::default(),
                volume_id: Default::default(),
            }
        }
    }
    ///`RuntimeMemoryEstimateResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "active_parameter_count": {
    ///      "type": "integer"
    ///    },
    ///    "download_size_bytes": {
    ///      "type": "integer"
    ///    },
    ///    "gpu_memory_utilization": {
    ///      "type": "number"
    ///    },
    ///    "kv_cache_vram_gb": {
    ///      "type": "number"
    ///    },
    ///    "model_vram_gb": {
    ///      "type": "number"
    ///    },
    ///    "parameter_count": {
    ///      "type": "integer"
    ///    },
    ///    "per_gpu_required_vram_gb": {
    ///      "type": "number"
    ///    },
    ///    "required_vram_gb": {
    ///      "type": "number"
    ///    },
    ///    "tensor_parallel_size": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "RuntimeMemoryEstimateResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RuntimeMemoryEstimateResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub active_parameter_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub download_size_bytes: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu_memory_utilization: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kv_cache_vram_gb: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_vram_gb: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parameter_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub per_gpu_required_vram_gb: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub required_vram_gb: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tensor_parallel_size: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for RuntimeMemoryEstimateResponse {
        fn default() -> Self {
            Self {
                active_parameter_count: Default::default(),
                download_size_bytes: Default::default(),
                gpu_memory_utilization: Default::default(),
                kv_cache_vram_gb: Default::default(),
                model_vram_gb: Default::default(),
                parameter_count: Default::default(),
                per_gpu_required_vram_gb: Default::default(),
                required_vram_gb: Default::default(),
                tensor_parallel_size: Default::default(),
            }
        }
    }
    ///`ScaleReplicaSetRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "desired_replicas": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "ScaleReplicaSetRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ScaleReplicaSetRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub desired_replicas: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ScaleReplicaSetRequest {
        fn default() -> Self {
            Self {
                desired_replicas: Default::default(),
            }
        }
    }
    ///`ScheduleRuleListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "schedule_rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScheduleRuleResponse"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "ScheduleRuleListResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ScheduleRuleListResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub schedule_rules: ::std::vec::Vec<ScheduleRuleResponse>,
    }
    impl ::std::default::Default for ScheduleRuleListResponse {
        fn default() -> Self {
            Self {
                schedule_rules: Default::default(),
            }
        }
    }
    ///`ScheduleRuleRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "action": {
    ///      "type": "string"
    ///    },
    ///    "cron": {
    ///      "type": "string"
    ///    },
    ///    "desired_replicas": {
    ///      "type": "integer"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "replica_set_id": {
    ///      "type": "string"
    ///    },
    ///    "timezone": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ScheduleRuleRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ScheduleRuleRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cron: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub desired_replicas: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub replica_set_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timezone: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ScheduleRuleRequest {
        fn default() -> Self {
            Self {
                action: Default::default(),
                cron: Default::default(),
                desired_replicas: Default::default(),
                enabled: Default::default(),
                replica_set_id: Default::default(),
                timezone: Default::default(),
            }
        }
    }
    ///`ScheduleRuleResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "action": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "cron": {
    ///      "type": "string"
    ///    },
    ///    "desired_replicas": {
    ///      "type": "integer"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "instance_id": {
    ///      "type": "string"
    ///    },
    ///    "last_error": {
    ///      "type": "string"
    ///    },
    ///    "last_run_at": {
    ///      "type": "string"
    ///    },
    ///    "next_run_at": {
    ///      "type": "string"
    ///    },
    ///    "replica_set_id": {
    ///      "type": "string"
    ///    },
    ///    "timezone": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ScheduleRuleResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ScheduleRuleResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cron: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub desired_replicas: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_run_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_run_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub replica_set_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timezone: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ScheduleRuleResponse {
        fn default() -> Self {
            Self {
                action: Default::default(),
                created_at: Default::default(),
                cron: Default::default(),
                desired_replicas: Default::default(),
                enabled: Default::default(),
                id: Default::default(),
                instance_id: Default::default(),
                last_error: Default::default(),
                last_run_at: Default::default(),
                next_run_at: Default::default(),
                replica_set_id: Default::default(),
                timezone: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`SearchHitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "score": {
    ///      "type": "number"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/SearchHitSourceResponse"
    ///    }
    ///  },
    ///  "x-go-name": "SearchHitResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchHitResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub metadata: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<SearchHitSourceResponse>,
    }
    impl ::std::default::Default for SearchHitResponse {
        fn default() -> Self {
            Self {
                content: Default::default(),
                id: Default::default(),
                metadata: Default::default(),
                score: Default::default(),
                source: Default::default(),
            }
        }
    }
    ///`SearchHitSourceResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "image_url": {
    ///      "type": "string"
    ///    },
    ///    "text": {
    ///      "type": "string"
    ///    },
    ///    "truncate": {
    ///      "type": "boolean"
    ///    }
    ///  },
    ///  "x-go-name": "SearchHitSourceResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchHitSourceResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub truncate: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for SearchHitSourceResponse {
        fn default() -> Self {
            Self {
                image_url: Default::default(),
                text: Default::default(),
                truncate: Default::default(),
            }
        }
    }
    ///`SearchRecommendRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "bias": {
    ///      "type": "number"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SearchRecommendRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchRecommendRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bias: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SearchRecommendRequest {
        fn default() -> Self {
            Self {
                bias: Default::default(),
                token: Default::default(),
            }
        }
    }
    ///`SearchRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "image_url": {
    ///      "type": "string"
    ///    },
    ///    "query": {
    ///      "type": "string"
    ///    },
    ///    "recommend": {
    ///      "$ref": "#/components/schemas/SearchRecommendRequest"
    ///    },
    ///    "top_k": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "SearchRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SearchRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub query: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommend: ::std::option::Option<SearchRecommendRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub top_k: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for SearchRequest {
        fn default() -> Self {
            Self {
                image_url: Default::default(),
                query: Default::default(),
                recommend: Default::default(),
                top_k: Default::default(),
            }
        }
    }
    ///`ServingEndpointResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "endpoint_type": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "providers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ServingProviderResponse"
    ///      }
    ///    },
    ///    "public_input_per_million_eur": {
    ///      "type": "number"
    ///    },
    ///    "public_output_per_million_eur": {
    ///      "type": "number"
    ///    },
    ///    "state": {
    ///      "type": "string"
    ///    },
    ///    "supports_thinking": {
    ///      "type": "boolean"
    ///    },
    ///    "targets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ServingTargetResponse"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ServingEndpointResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServingEndpointResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub endpoint_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub providers: ::std::vec::Vec<ServingProviderResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_input_per_million_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_output_per_million_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub supports_thinking: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub targets: ::std::vec::Vec<ServingTargetResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ServingEndpointResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                display_name: Default::default(),
                enabled: Default::default(),
                endpoint_type: Default::default(),
                id: Default::default(),
                name: Default::default(),
                providers: Default::default(),
                public_input_per_million_eur: Default::default(),
                public_output_per_million_eur: Default::default(),
                state: Default::default(),
                supports_thinking: Default::default(),
                targets: Default::default(),
                updated_at: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`ServingProviderResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "model_id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ServingProviderResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServingProviderResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ServingProviderResponse {
        fn default() -> Self {
            Self {
                model_id: Default::default(),
                name: Default::default(),
                region: Default::default(),
            }
        }
    }
    ///`ServingTargetRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "instance_id": {
    ///      "type": "string"
    ///    },
    ///    "target_ref_id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "ServingTargetRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServingTargetRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_ref_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for ServingTargetRequest {
        fn default() -> Self {
            Self {
                instance_id: Default::default(),
                target_ref_id: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`ServingTargetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "instance_id": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer"
    ///    },
    ///    "target_ref_id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "weight": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "ServingTargetResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ServingTargetResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_ref_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub weight: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for ServingTargetResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                enabled: Default::default(),
                id: Default::default(),
                instance_id: Default::default(),
                priority: Default::default(),
                target_ref_id: Default::default(),
                type_: Default::default(),
                updated_at: Default::default(),
                weight: Default::default(),
            }
        }
    }
    ///`SmartBalancerRouteDestinationRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ref": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerRouteDestinationRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerRouteDestinationRequest {
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ref_: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SmartBalancerRouteDestinationRequest {
        fn default() -> Self {
            Self {
                ref_: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`SmartBalancerRouteDestinationResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "label": {
    ///      "type": "string"
    ///    },
    ///    "ref": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerRouteDestinationResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerRouteDestinationResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub label: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ref_: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SmartBalancerRouteDestinationResponse {
        fn default() -> Self {
            Self {
                label: Default::default(),
                ref_: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`SmartBalancerRouteRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "destination": {
    ///      "$ref": "#/components/schemas/SmartBalancerRouteDestinationRequest"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "intent": {
    ///      "type": "string"
    ///    },
    ///    "is_default": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerRouteRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerRouteRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub destination: ::std::option::Option<SmartBalancerRouteDestinationRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub intent: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_default: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for SmartBalancerRouteRequest {
        fn default() -> Self {
            Self {
                destination: Default::default(),
                enabled: Default::default(),
                intent: Default::default(),
                is_default: Default::default(),
                name: Default::default(),
                priority: Default::default(),
            }
        }
    }
    ///`SmartBalancerRouteResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "destination": {
    ///      "$ref": "#/components/schemas/SmartBalancerRouteDestinationResponse"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "intent": {
    ///      "type": "string"
    ///    },
    ///    "is_default": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerRouteResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerRouteResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub destination: ::std::option::Option<SmartBalancerRouteDestinationResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub intent: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_default: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SmartBalancerRouteResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                destination: Default::default(),
                enabled: Default::default(),
                id: Default::default(),
                intent: Default::default(),
                is_default: Default::default(),
                name: Default::default(),
                priority: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`SmartBalancerRouterModelRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ref": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerRouterModelRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerRouterModelRequest {
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ref_: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SmartBalancerRouterModelRequest {
        fn default() -> Self {
            Self {
                ref_: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`SmartBalancerRouterModelResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ref": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerRouterModelResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerRouterModelResponse {
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ref_: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SmartBalancerRouterModelResponse {
        fn default() -> Self {
            Self {
                ref_: Default::default(),
                type_: Default::default(),
            }
        }
    }
    ///`SmartBalancerViewResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "default_route_id": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "router_model": {
    ///      "$ref": "#/components/schemas/SmartBalancerRouterModelResponse"
    ///    },
    ///    "routes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SmartBalancerRouteResponse"
    ///      }
    ///    },
    ///    "routing_mode": {
    ///      "type": "string"
    ///    },
    ///    "serving_endpoint_id": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SmartBalancerViewResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SmartBalancerViewResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_route_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub router_model: ::std::option::Option<SmartBalancerRouterModelResponse>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub routes: ::std::vec::Vec<SmartBalancerRouteResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub routing_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_endpoint_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SmartBalancerViewResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                default_route_id: Default::default(),
                display_name: Default::default(),
                enabled: Default::default(),
                id: Default::default(),
                name: Default::default(),
                router_model: Default::default(),
                routes: Default::default(),
                routing_mode: Default::default(),
                serving_endpoint_id: Default::default(),
                updated_at: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`StorageVolumeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "bucket": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "error_message": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "prefix": {
    ///      "type": "string"
    ///    },
    ///    "provider": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "StorageVolumeResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct StorageVolumeResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prefix: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for StorageVolumeResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                bucket: Default::default(),
                created_at: Default::default(),
                description: Default::default(),
                error_message: Default::default(),
                id: Default::default(),
                name: Default::default(),
                prefix: Default::default(),
                provider: Default::default(),
                status: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`SystemToolOutput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "configuration_schema": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "managed_by": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "visibility": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "SystemToolOutput"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SystemToolOutput {
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub configuration_schema: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub managed_by: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visibility: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for SystemToolOutput {
        fn default() -> Self {
            Self {
                configuration_schema: Default::default(),
                description: Default::default(),
                managed_by: Default::default(),
                type_: Default::default(),
                visibility: Default::default(),
            }
        }
    }
    ///`TargetGroupMemberRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "instance_id": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer"
    ///    },
    ///    "weight": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "TargetGroupMemberRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TargetGroupMemberRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub weight: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for TargetGroupMemberRequest {
        fn default() -> Self {
            Self {
                enabled: Default::default(),
                instance_id: Default::default(),
                priority: Default::default(),
                weight: Default::default(),
            }
        }
    }
    ///`TargetGroupMemberResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "instance_id": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "weight": {
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "TargetGroupMemberResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TargetGroupMemberResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub priority: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub weight: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for TargetGroupMemberResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                enabled: Default::default(),
                id: Default::default(),
                instance_id: Default::default(),
                priority: Default::default(),
                updated_at: Default::default(),
                weight: Default::default(),
            }
        }
    }
    ///`TargetGroupResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "members": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TargetGroupMemberResponse"
    ///      }
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "selection_policy": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "TargetGroupResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TargetGroupResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub members: ::std::vec::Vec<TargetGroupMemberResponse>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub selection_policy: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for TargetGroupResponse {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                description: Default::default(),
                display_name: Default::default(),
                enabled: Default::default(),
                id: Default::default(),
                members: Default::default(),
                name: Default::default(),
                selection_policy: Default::default(),
                updated_at: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`Template`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "$ref": "#/components/schemas/Timestamp"
    ///    },
    ///    "created_by": {
    ///      "type": "string"
    ///    },
    ///    "health_checked_at": {
    ///      "$ref": "#/components/schemas/Timestamp"
    ///    },
    ///    "health_error": {
    ///      "type": "string"
    ///    },
    ///    "health_status": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "is_active": {
    ///      "type": "boolean"
    ///    },
    ///    "is_featured": {
    ///      "type": "boolean"
    ///    },
    ///    "ordering": {
    ///      "type": "integer"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "translations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Translation"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "$ref": "#/components/schemas/Timestamp"
    ///    },
    ///    "updated_by": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "Template"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Template {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<Timestamp>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_by: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub health_checked_at: ::std::option::Option<Timestamp>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub health_error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub health_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_active: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_featured: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ordering: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub translations: ::std::vec::Vec<Translation>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<Timestamp>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_by: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Template {
        fn default() -> Self {
            Self {
                category: Default::default(),
                created_at: Default::default(),
                created_by: Default::default(),
                health_checked_at: Default::default(),
                health_error: Default::default(),
                health_status: Default::default(),
                id: Default::default(),
                is_active: Default::default(),
                is_featured: Default::default(),
                ordering: Default::default(),
                slug: Default::default(),
                tags: Default::default(),
                translations: Default::default(),
                updated_at: Default::default(),
                updated_by: Default::default(),
            }
        }
    }
    ///`TemplateWithVariants`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "template": {
    ///      "$ref": "#/components/schemas/Template"
    ///    },
    ///    "variants": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VariantWithRelations"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "TemplateWithVariants"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TemplateWithVariants {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template: ::std::option::Option<Template>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub variants: ::std::vec::Vec<VariantWithRelations>,
    }
    impl ::std::default::Default for TemplateWithVariants {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
                template: Default::default(),
                variants: Default::default(),
            }
        }
    }
    ///`Timestamp`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "nanos": {
    ///      "description": "Non-negative fractions of a second at nanosecond resolution. This field is\nthe nanosecond portion of the duration, not an alternative to seconds.\nNegative second values with fractions must still have non-negative nanos\nvalues that count forward in time. Must be between 0 and 999,999,999\ninclusive.",
    ///      "type": "integer"
    ///    },
    ///    "seconds": {
    ///      "description": "Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must\nbe between -315576000000 and 315576000000 inclusive (which corresponds to\n0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z).",
    ///      "type": "integer"
    ///    }
    ///  },
    ///  "x-go-name": "Timestamp"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Timestamp {
        /**Non-negative fractions of a second at nanosecond resolution. This field is
the nanosecond portion of the duration, not an alternative to seconds.
Negative second values with fractions must still have non-negative nanos
values that count forward in time. Must be between 0 and 999,999,999
inclusive.*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub nanos: ::std::option::Option<i64>,
        /**Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must
be between -315576000000 and 315576000000 inclusive (which corresponds to
0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z).*/
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub seconds: ::std::option::Option<i64>,
    }
    impl ::std::default::Default for Timestamp {
        fn default() -> Self {
            Self {
                nanos: Default::default(),
                seconds: Default::default(),
            }
        }
    }
    ///`Translation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "Translation"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Translation {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locale: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for Translation {
        fn default() -> Self {
            Self {
                description: Default::default(),
                locale: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`UpdateAccountMemberRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "role": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateAccountMemberRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateAccountMemberRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateAccountMemberRequest {
        fn default() -> Self {
            Self { role: Default::default() }
        }
    }
    ///`UpdateAccountRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_address": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_city": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_company_name": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_country": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_customer_type": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_invoice_email": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_legal_name": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_postcode": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_profile_completed": {
    ///      "type": "boolean"
    ///    },
    ///    "fiscal_region": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_tax_id": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_vat_validated": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateAccountRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateAccountRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_city: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_company_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_customer_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_invoice_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_legal_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_postcode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_profile_completed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_tax_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_vat_validated: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateAccountRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                fiscal_address: Default::default(),
                fiscal_city: Default::default(),
                fiscal_company_name: Default::default(),
                fiscal_country: Default::default(),
                fiscal_customer_type: Default::default(),
                fiscal_invoice_email: Default::default(),
                fiscal_legal_name: Default::default(),
                fiscal_postcode: Default::default(),
                fiscal_profile_completed: Default::default(),
                fiscal_region: Default::default(),
                fiscal_tax_id: Default::default(),
                fiscal_vat_validated: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`UpdateFirewallRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "evaluator_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "mode": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule_slugs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateFirewallRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateFirewallRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub evaluator_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rule_slugs: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateFirewallRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                evaluator_serving_name: Default::default(),
                mode: Default::default(),
                name: Default::default(),
                rule_slugs: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`UpdateFirewallRuleRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "default_severity": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "prompt": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateFirewallRuleRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateFirewallRuleRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_severity: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prompt: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateFirewallRuleRequest {
        fn default() -> Self {
            Self {
                category: Default::default(),
                default_severity: Default::default(),
                description: Default::default(),
                name: Default::default(),
                prompt: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`UpdateFiscalProfileRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "fiscal_address": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_city": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_company_name": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_country": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_customer_type": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_invoice_email": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_legal_name": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_postcode": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_profile_completed": {
    ///      "type": "boolean"
    ///    },
    ///    "fiscal_region": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_tax_id": {
    ///      "type": "string"
    ///    },
    ///    "fiscal_vat_validated": {
    ///      "type": "boolean"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateFiscalProfileRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateFiscalProfileRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_city: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_company_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_country: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_customer_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_invoice_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_legal_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_postcode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_profile_completed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_tax_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fiscal_vat_validated: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for UpdateFiscalProfileRequest {
        fn default() -> Self {
            Self {
                fiscal_address: Default::default(),
                fiscal_city: Default::default(),
                fiscal_company_name: Default::default(),
                fiscal_country: Default::default(),
                fiscal_customer_type: Default::default(),
                fiscal_invoice_email: Default::default(),
                fiscal_legal_name: Default::default(),
                fiscal_postcode: Default::default(),
                fiscal_profile_completed: Default::default(),
                fiscal_region: Default::default(),
                fiscal_tax_id: Default::default(),
                fiscal_vat_validated: Default::default(),
            }
        }
    }
    ///`UpdateInstanceRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "gpu_preferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PublicGPUPreferenceJSON"
    ///      }
    ///    },
    ///    "idle_timeout_seconds": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "runtime_config": {
    ///      "$ref": "#/components/schemas/RuntimeConfigRequest"
    ///    },
    ///    "runtime_preset": {
    ///      "type": "string"
    ///    },
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "serverless_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "serving_name": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_max_price_per_hour_eur": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_region": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "smart_selection_label": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateInstanceRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateInstanceRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gpu_preferences: ::std::vec::Vec<PublicGpuPreferenceJson>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub idle_timeout_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_config: ::std::option::Option<RuntimeConfigRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runtime_preset: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serverless_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_eur: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_region: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_selection_label: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateInstanceRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                gpu_preferences: Default::default(),
                idle_timeout_seconds: Default::default(),
                name: Default::default(),
                runtime_config: Default::default(),
                runtime_preset: Default::default(),
                scheduling_mode: Default::default(),
                serverless_enabled: Default::default(),
                serving_name: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_max_price_per_hour_eur: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_region: Default::default(),
                smart_regions: Default::default(),
                smart_selection_label: Default::default(),
            }
        }
    }
    ///`UpdatePreferencesRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "email_2fa_enabled": {
    ///      "type": "boolean"
    ///    }
    ///  },
    ///  "x-go-name": "UpdatePreferencesRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdatePreferencesRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_2fa_enabled: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for UpdatePreferencesRequest {
        fn default() -> Self {
            Self {
                email_2fa_enabled: Default::default(),
            }
        }
    }
    ///`UpdateReplicaSetRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "desired_replicas": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "scheduling_mode": {
    ///      "type": "string"
    ///    },
    ///    "serving_name": {
    ///      "type": "string"
    ///    },
    ///    "smart_allow_community": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_allow_spot": {
    ///      "type": "boolean"
    ///    },
    ///    "smart_max_price_per_hour_usd": {
    ///      "type": "number"
    ///    },
    ///    "smart_min_gpu_class": {
    ///      "type": "string"
    ///    },
    ///    "smart_min_total_tflops": {
    ///      "type": "number"
    ///    },
    ///    "smart_provider_filter_mode": {
    ///      "type": "string"
    ///    },
    ///    "smart_provider_preference": {
    ///      "type": "string"
    ///    },
    ///    "smart_regions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  },
    ///  "x-go-name": "UpdateReplicaSetRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateReplicaSetRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub desired_replicas: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scheduling_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_community: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_allow_spot: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_max_price_per_hour_usd: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_gpu_class: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_min_total_tflops: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_filter_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub smart_provider_preference: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub smart_regions: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for UpdateReplicaSetRequest {
        fn default() -> Self {
            Self {
                desired_replicas: Default::default(),
                name: Default::default(),
                scheduling_mode: Default::default(),
                serving_name: Default::default(),
                smart_allow_community: Default::default(),
                smart_allow_spot: Default::default(),
                smart_max_price_per_hour_usd: Default::default(),
                smart_min_gpu_class: Default::default(),
                smart_min_total_tflops: Default::default(),
                smart_provider_filter_mode: Default::default(),
                smart_provider_preference: Default::default(),
                smart_regions: Default::default(),
            }
        }
    }
    ///`UpdateSmartBalancerRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "router_model": {
    ///      "$ref": "#/components/schemas/SmartBalancerRouterModelRequest"
    ///    },
    ///    "routes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SmartBalancerRouteRequest"
    ///      }
    ///    },
    ///    "routing_mode": {
    ///      "type": "string"
    ///    },
    ///    "workload_kind": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateSmartBalancerRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateSmartBalancerRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub router_model: ::std::option::Option<SmartBalancerRouterModelRequest>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub routes: ::std::vec::Vec<SmartBalancerRouteRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub routing_mode: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub workload_kind: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateSmartBalancerRequest {
        fn default() -> Self {
            Self {
                display_name: Default::default(),
                enabled: Default::default(),
                name: Default::default(),
                router_model: Default::default(),
                routes: Default::default(),
                routing_mode: Default::default(),
                workload_kind: Default::default(),
            }
        }
    }
    ///`UpdateTargetGroupRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "display_name": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "members": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TargetGroupMemberRequest"
    ///      }
    ///    },
    ///    "selection_policy": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateTargetGroupRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateTargetGroupRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub members: ::std::vec::Vec<TargetGroupMemberRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub selection_policy: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateTargetGroupRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                display_name: Default::default(),
                enabled: Default::default(),
                members: Default::default(),
                selection_policy: Default::default(),
            }
        }
    }
    ///`UpdateVectorDatabaseRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UpdateVectorDatabaseRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UpdateVectorDatabaseRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UpdateVectorDatabaseRequest {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
                slug: Default::default(),
            }
        }
    }
    ///`UserInfoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "UserInfoResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UserInfoResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for UserInfoResponse {
        fn default() -> Self {
            Self {
                email: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    ///`UserPreferencesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "email_2fa_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "totp_enabled": {
    ///      "type": "boolean"
    ///    }
    ///  },
    ///  "x-go-name": "UserPreferencesResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UserPreferencesResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email_2fa_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub totp_enabled: ::std::option::Option<bool>,
    }
    impl ::std::default::Default for UserPreferencesResponse {
        fn default() -> Self {
            Self {
                email_2fa_enabled: Default::default(),
                totp_enabled: Default::default(),
            }
        }
    }
    ///`Variant`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "artifact_type": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "$ref": "#/components/schemas/Timestamp"
    ///    },
    ///    "gguf_files": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "health_checked_at": {
    ///      "$ref": "#/components/schemas/Timestamp"
    ///    },
    ///    "health_error": {
    ///      "type": "string"
    ///    },
    ///    "health_status": {
    ///      "type": "string"
    ///    },
    ///    "icon_url": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "is_active": {
    ///      "type": "boolean"
    ///    },
    ///    "max_context_tokens": {
    ///      "type": "integer"
    ///    },
    ///    "model_id": {
    ///      "type": "string"
    ///    },
    ///    "ordering": {
    ///      "type": "integer"
    ///    },
    ///    "quantization": {
    ///      "type": "string"
    ///    },
    ///    "recommended_preset": {
    ///      "type": "string"
    ///    },
    ///    "revision": {
    ///      "type": "string"
    ///    },
    ///    "stop_sequences": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "system_prompt": {
    ///      "type": "string"
    ///    },
    ///    "temperature": {
    ///      "type": "number"
    ///    },
    ///    "template_id": {
    ///      "type": "string"
    ///    },
    ///    "translations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VariantTranslation"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "$ref": "#/components/schemas/Timestamp"
    ///    }
    ///  },
    ///  "x-go-name": "Variant"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Variant {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artifact_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<Timestamp>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub gguf_files: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub health_checked_at: ::std::option::Option<Timestamp>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub health_error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub health_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub is_active: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_context_tokens: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ordering: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantization: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommended_preset: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub revision: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub stop_sequences: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub system_prompt: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub temperature: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub translations: ::std::vec::Vec<VariantTranslation>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<Timestamp>,
    }
    impl ::std::default::Default for Variant {
        fn default() -> Self {
            Self {
                artifact_type: Default::default(),
                created_at: Default::default(),
                gguf_files: Default::default(),
                health_checked_at: Default::default(),
                health_error: Default::default(),
                health_status: Default::default(),
                icon_url: Default::default(),
                id: Default::default(),
                is_active: Default::default(),
                max_context_tokens: Default::default(),
                model_id: Default::default(),
                ordering: Default::default(),
                quantization: Default::default(),
                recommended_preset: Default::default(),
                revision: Default::default(),
                stop_sequences: Default::default(),
                system_prompt: Default::default(),
                temperature: Default::default(),
                template_id: Default::default(),
                translations: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`VariantTranslation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "VariantTranslation"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct VariantTranslation {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locale: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for VariantTranslation {
        fn default() -> Self {
            Self {
                description: Default::default(),
                locale: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`VariantWithRelations`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "template_category": {
    ///      "type": "string"
    ///    },
    ///    "template_is_active": {
    ///      "type": "boolean"
    ///    },
    ///    "template_slug": {
    ///      "type": "string"
    ///    },
    ///    "template_tags": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "variant": {
    ///      "$ref": "#/components/schemas/Variant"
    ///    }
    ///  },
    ///  "x-go-name": "VariantWithRelations"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct VariantWithRelations {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template_category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template_is_active: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub template_slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub template_tags: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub variant: ::std::option::Option<Variant>,
    }
    impl ::std::default::Default for VariantWithRelations {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
                template_category: Default::default(),
                template_is_active: Default::default(),
                template_slug: Default::default(),
                template_tags: Default::default(),
                variant: Default::default(),
            }
        }
    }
    ///`VectorDatabaseResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "embedding_model_ref": {
    ///      "type": "string"
    ///    },
    ///    "embedding_serving_name": {
    ///      "type": "string"
    ///    },
    ///    "error_message": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "opensearch_index_name": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "VectorDatabaseResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct VectorDatabaseResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub embedding_model_ref: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub embedding_serving_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub opensearch_index_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for VectorDatabaseResponse {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                created_at: Default::default(),
                description: Default::default(),
                embedding_model_ref: Default::default(),
                embedding_serving_name: Default::default(),
                error_message: Default::default(),
                id: Default::default(),
                name: Default::default(),
                opensearch_index_name: Default::default(),
                slug: Default::default(),
                status: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    ///`VerifyEmailResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "access_token_expiry": {
    ///      "type": "integer"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "refresh_token": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "x-go-name": "VerifyEmailResponse"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct VerifyEmailResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_token_expiry: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub refresh_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::default::Default for VerifyEmailResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                access_token_expiry: Default::default(),
                email: Default::default(),
                refresh_token: Default::default(),
                user_id: Default::default(),
            }
        }
    }
    ///`ViewElementRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "document_id": {
    ///      "type": "string"
    ///    },
    ///    "recommend_token": {
    ///      "type": "string"
    ///    },
    ///    "weight": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "x-go-name": "ViewElementRequest"
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ViewElementRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub document_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub recommend_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub weight: ::std::option::Option<f64>,
    }
    impl ::std::default::Default for ViewElementRequest {
        fn default() -> Self {
            Self {
                document_id: Default::default(),
                recommend_token: Default::default(),
                weight: Default::default(),
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Tokenfactory API Gateway

REST gateway for authentication, account, and billing operations.

Version: v1beta*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "v1beta"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**List user accounts

Returns the accounts and memberships visible to the authenticated user.

Sends a `GET` request to `/accounts`

*/
    pub async fn get_accounts<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::AccountListResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/accounts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_accounts",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create an account

Creates a new account owned by the authenticated user.

Sends a `POST` request to `/accounts`

*/
    pub async fn post_accounts<'a>(
        &'a self,
        body: &'a types::CreateAccountRequest,
    ) -> Result<
        ResponseValue<types::AccountMembershipResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/accounts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_accounts",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the selected account

Returns the account resolved from the active account context.

Sends a `GET` request to `/accounts/current`

*/
    pub async fn get_accounts_current<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::AccountResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/accounts/current", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_accounts_current",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete the selected account

Deletes the active account context.

Sends a `DELETE` request to `/accounts/current`

*/
    pub async fn delete_accounts_current<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!("{}/accounts/current", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_accounts_current",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update the selected account

Updates editable metadata for the active account context.

Sends a `PATCH` request to `/accounts/current`

*/
    pub async fn patch_accounts_current<'a>(
        &'a self,
        body: &'a types::UpdateAccountRequest,
    ) -> Result<ResponseValue<types::AccountResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/accounts/current", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_accounts_current",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update the current account fiscal profile

Updates only the fiscal-profile fields of the active account context, preserving the existing account name and description.

Sends a `PATCH` request to `/accounts/current/fiscal-profile`

*/
    pub async fn patch_accounts_current_fiscal_profile<'a>(
        &'a self,
        body: &'a types::UpdateFiscalProfileRequest,
    ) -> Result<ResponseValue<types::AccountResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/accounts/current/fiscal-profile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_accounts_current_fiscal_profile",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create an invitation for the selected account

Invites a user by email to join the active account context with a given role.

Sends a `POST` request to `/accounts/current/invitations`

*/
    pub async fn post_accounts_current_invitations<'a>(
        &'a self,
        body: &'a types::CreateInvitationRequest,
    ) -> Result<ResponseValue<types::InvitationResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/accounts/current/invitations", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_accounts_current_invitations",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            412u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Cancel an invitation for the selected account

Cancels a pending invitation issued by the active account context.

Sends a `DELETE` request to `/accounts/current/invitations/{invitationID}`

Arguments:
- `invitation_id`: Invitation ID
*/
    pub async fn delete_accounts_current_invitations_by_invitation_id<'a>(
        &'a self,
        invitation_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/accounts/current/invitations/{}", self.baseurl, encode_path(&
            invitation_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_accounts_current_invitations_by_invitation_id",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List selected account members

Returns the members and roles for the active account context.

Sends a `GET` request to `/accounts/current/members`

*/
    pub async fn get_accounts_current_members<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::AccountMemberListResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/accounts/current/members", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_accounts_current_members",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add a member to the selected account

Adds a registered user to the active account context or updates their role if they already belong to it.

Sends a `POST` request to `/accounts/current/members`

*/
    pub async fn post_accounts_current_members<'a>(
        &'a self,
        body: &'a types::AddAccountMemberRequest,
    ) -> Result<
        ResponseValue<types::AccountMemberResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/accounts/current/members", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_accounts_current_members",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            412u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a selected account member role

Updates the role of an existing user within the active account context.

Sends a `PATCH` request to `/accounts/current/members/{userID}`

Arguments:
- `user_id`: User ID
- `body`
*/
    pub async fn patch_accounts_current_members_by_user_id<'a>(
        &'a self,
        user_id: &'a str,
        body: &'a types::UpdateAccountMemberRequest,
    ) -> Result<
        ResponseValue<types::AccountMemberResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/accounts/current/members/{}", self.baseurl, encode_path(& user_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_accounts_current_members_by_user_id",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            412u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List API keys for the selected account

Returns the API keys issued for the active account context.

Sends a `GET` request to `/api-keys`

*/
    pub async fn get_api_keys<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ApiKeyListResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/api-keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_api_keys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create an API key for the selected account

Issues a new API key for the active account context and returns the plaintext key once.

Sends a `POST` request to `/api-keys`

*/
    pub async fn post_api_keys<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ApiKeyResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/api-keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_api_keys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Revoke an API key

Revokes an API key issued for the active account context.

Sends a `POST` request to `/api-keys/{keyID}/revoke`

Arguments:
- `key_id`: API key ID
*/
    pub async fn post_api_keys_key_id_revoke<'a>(
        &'a self,
        key_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api-keys/{}/revoke", self.baseurl, encode_path(& key_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_api_keys_key_id_revoke",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Request a password reset email

Always returns 200 with a generic message to avoid user enumeration.

Sends a `POST` request to `/auth/forgot-password`

*/
    pub async fn post_auth_forgot_password<'a>(
        &'a self,
        body: &'a types::ForgotPasswordRequest,
    ) -> Result<
        ResponseValue<types::ForgotPasswordResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/forgot-password", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_forgot_password",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Authenticate a user

Exchanges email and password credentials for access and refresh tokens.

Sends a `POST` request to `/auth/login`

*/
    pub async fn post_auth_login<'a>(
        &'a self,
        body: &'a types::LoginRequest,
    ) -> Result<ResponseValue<types::LoginResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/login", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_login",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Return the current authenticated user

Resolves the user attached to the bearer token.

Sends a `GET` request to `/auth/me`

*/
    pub async fn get_auth_me<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::UserInfoResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/me", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_auth_me",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List passkeys

Returns the current user's passkey credentials.

Sends a `GET` request to `/auth/passkeys`

*/
    pub async fn get_auth_passkeys<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::PasskeyListResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/passkeys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_auth_passkeys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Register a passkey

Stores a passkey credential for the authenticated user.

Sends a `POST` request to `/auth/passkeys`

*/
    pub async fn post_auth_passkeys<'a>(
        &'a self,
        body: &'a types::RegisterPasskeyRequest,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/passkeys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_passkeys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Begin passkey login

Creates a passkey login challenge for the provided email.

Sends a `POST` request to `/auth/passkeys/begin-login`

*/
    pub async fn post_auth_passkeys_begin_login<'a>(
        &'a self,
        body: &'a types::BeginPasskeyloginRequest,
    ) -> Result<
        ResponseValue<types::BeginPasskeyloginResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/passkeys/begin-login", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_passkeys_begin_login",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            503u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Begin passkey registration

Creates a passkey registration challenge for the authenticated user.

Sends a `POST` request to `/auth/passkeys/begin-registration`

*/
    pub async fn post_auth_passkeys_begin_registration<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::BeginPasskeyRegistrationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/passkeys/begin-registration", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_passkeys_begin_registration",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Finish passkey login

Consumes a passkey login challenge and returns a session.

Sends a `POST` request to `/auth/passkeys/finish-login`

*/
    pub async fn post_auth_passkeys_finish_login<'a>(
        &'a self,
        body: &'a types::FinishPasskeyloginRequest,
    ) -> Result<
        ResponseValue<types::FinishPasskeyloginResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/passkeys/finish-login", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_passkeys_finish_login",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Finish passkey registration

Consumes a passkey registration challenge and stores the credential.

Sends a `POST` request to `/auth/passkeys/finish-registration`

*/
    pub async fn post_auth_passkeys_finish_registration<'a>(
        &'a self,
        body: &'a types::FinishPasskeyRegistrationRequest,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/passkeys/finish-registration", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_passkeys_finish_registration",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Remove a passkey

Deletes a passkey credential from the authenticated user.

Sends a `DELETE` request to `/auth/passkeys/{credentialID}`

*/
    pub async fn delete_auth_passkeys<'a>(
        &'a self,
        credential_id: &'a str,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/auth/passkeys/{}", self.baseurl, encode_path(& credential_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_auth_passkeys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Change the current user's password

Requires the authenticated user to provide the current password and a new password.

Sends a `PATCH` request to `/auth/password`

*/
    pub async fn patch_auth_password<'a>(
        &'a self,
        body: &'a types::ChangePasswordRequest,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/password", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_auth_password",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the current user's security preferences

Returns the per-user security toggles for the authenticated user.

Sends a `GET` request to `/auth/preferences`

*/
    pub async fn get_auth_preferences<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::UserPreferencesResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/preferences", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_auth_preferences",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update the current user's security preferences

Partial update of the per-user security toggles. Only fields present in the body are applied.

Sends a `PATCH` request to `/auth/preferences`

*/
    pub async fn patch_auth_preferences<'a>(
        &'a self,
        body: &'a types::UpdatePreferencesRequest,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/preferences", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_auth_preferences",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Refresh an access token

Exchanges a refresh token for a new access token.

Sends a `POST` request to `/auth/refresh`

*/
    pub async fn post_auth_refresh<'a>(
        &'a self,
        body: &'a types::RefreshRequest,
    ) -> Result<ResponseValue<types::RefreshResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/refresh", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_refresh",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Register a new user

Creates a new account using email and password credentials.

Sends a `POST` request to `/auth/register`

*/
    pub async fn post_auth_register<'a>(
        &'a self,
        body: &'a types::RegisterRequest,
    ) -> Result<ResponseValue<types::RegisterResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/register", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_register",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Reset a password using a one-time token

Consumes the reset token, updates the password, and returns a fresh session.

Sends a `POST` request to `/auth/reset-password`

*/
    pub async fn post_auth_reset_password<'a>(
        &'a self,
        body: &'a types::ResetPasswordRequest,
    ) -> Result<
        ResponseValue<types::ResetPasswordResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/reset-password", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_reset_password",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Disable TOTP

Removes the current user's TOTP credential.

Sends a `DELETE` request to `/auth/totp`

*/
    pub async fn delete_auth_totp<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/totp", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_auth_totp",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Start TOTP enrollment

Returns a new TOTP secret and otpauth URI for the authenticated user.

Sends a `POST` request to `/auth/totp/begin`

*/
    pub async fn post_auth_totp_begin<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::BeginTotpEnrollmentResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/auth/totp/begin", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_totp_begin",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Confirm TOTP enrollment

Stores a TOTP secret for the authenticated user after verifying the code.

Sends a `POST` request to `/auth/totp/confirm`

*/
    pub async fn post_auth_totp_confirm<'a>(
        &'a self,
        body: &'a types::ConfirmTotpEnrollmentRequest,
    ) -> Result<ResponseValue<::std::string::String>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/totp/confirm", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_auth_totp_confirm",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Verify an email activation link

Consumes the activation token and returns a session.

Sends a `GET` request to `/auth/verify-email`

Arguments:
- `token`: Activation token from the email
*/
    pub async fn get_auth_verify_email<'a>(
        &'a self,
        token: &'a str,
    ) -> Result<ResponseValue<types::VerifyEmailResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/auth/verify-email", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("token", &token))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_auth_verify_email",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the current account balance

Returns the billing balance of the authenticated account.

Sends a `GET` request to `/billing/balance`

*/
    pub async fn get_billing_balance<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::AccountBalanceResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/billing/balance", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_billing_balance",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a checkout session

Creates a billing checkout session for the authenticated account.

Sends a `POST` request to `/billing/checkout`

Arguments:
- `idempotency_key`: Checkout idempotency key
- `body`
*/
    pub async fn post_billing_checkout<'a>(
        &'a self,
        idempotency_key: &'a str,
        body: &'a types::CreateCheckoutSessionRequest,
    ) -> Result<
        ResponseValue<types::CheckoutSessionResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/billing/checkout", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        header_map.append("Idempotency-Key", idempotency_key.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_billing_checkout",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Debit the current account balance

Debits a positive amount from the authenticated account.

Sends a `POST` request to `/billing/debit`

*/
    pub async fn post_billing_debit<'a>(
        &'a self,
        body: &'a types::DebitBalanceRequest,
    ) -> Result<
        ResponseValue<types::DebitBalanceResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/billing/debit", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_billing_debit",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List ledger entries

Returns ledger movements for the authenticated account.

Sends a `GET` request to `/billing/ledger`

Arguments:
- `limit`: Maximum number of entries
- `offset`: Number of entries to skip
*/
    pub async fn get_billing_ledger<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<
        ResponseValue<types::LedgerHistoryResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/billing/ledger", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_billing_ledger",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List payments for the authenticated account

Returns the payment history for the active account context.

Sends a `GET` request to `/billing/payments`

*/
    pub async fn get_billing_payments<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::PaymentsResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/billing/payments", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_billing_payments",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Download a payment invoice PDF

Returns the invoice PDF for a given payment of the active account context.

Sends a `GET` request to `/billing/payments/{paymentID}/invoice.pdf`

Arguments:
- `payment_id`: Payment ID
*/
    pub async fn get_billing_payments_payment_id_invoice_pdf<'a>(
        &'a self,
        payment_id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/billing/payments/{}/invoice.pdf", self.baseurl, encode_path(& payment_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "get_billing_payments_payment_id_invoice_pdf",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Quote a checkout total

Resolves the authenticated account's application-side tax quote without creating Stripe or checkout state.

Sends a `POST` request to `/billing/quote`

*/
    pub async fn post_billing_quote<'a>(
        &'a self,
        body: &'a types::CheckoutQuoteRequest,
    ) -> Result<
        ResponseValue<types::CheckoutQuoteResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/billing/quote", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_billing_quote",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            503u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List firewall rules

Returns all firewall rules for the authenticated account.

Sends a `GET` request to `/firewall-rules`

*/
    pub async fn get_firewall_rules<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::FirewallRuleResponse>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/firewall-rules", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewall_rules",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a firewall rule

Creates a new firewall rule for the authenticated account.

Sends a `POST` request to `/firewall-rules`

*/
    pub async fn post_firewall_rule<'a>(
        &'a self,
        body: &'a types::CreateFirewallRuleRequest,
    ) -> Result<
        ResponseValue<types::FirewallRuleResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/firewall-rules", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_firewall_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a firewall rule

Returns a single firewall rule by slug.

Sends a `GET` request to `/firewall-rules/{ruleSlug}`

Arguments:
- `rule_slug`: Firewall rule slug
*/
    pub async fn get_firewall_rule<'a>(
        &'a self,
        rule_slug: &'a str,
    ) -> Result<
        ResponseValue<types::FirewallRuleResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/firewall-rules/{}", self.baseurl, encode_path(& rule_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewall_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a firewall rule

Removes a firewall rule by slug.

Sends a `DELETE` request to `/firewall-rules/{ruleSlug}`

Arguments:
- `rule_slug`: Firewall rule slug
*/
    pub async fn delete_firewall_rule<'a>(
        &'a self,
        rule_slug: &'a str,
    ) -> Result<
        ResponseValue<::std::collections::HashMap<::std::string::String, bool>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/firewall-rules/{}", self.baseurl, encode_path(& rule_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_firewall_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a firewall rule

Partially updates a firewall rule by slug.

Sends a `PATCH` request to `/firewall-rules/{ruleSlug}`

Arguments:
- `rule_slug`: Firewall rule slug
- `body`
*/
    pub async fn patch_firewall_rule<'a>(
        &'a self,
        rule_slug: &'a str,
        body: &'a types::UpdateFirewallRuleRequest,
    ) -> Result<
        ResponseValue<types::FirewallRuleResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/firewall-rules/{}", self.baseurl, encode_path(& rule_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_firewall_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Evaluate a firewall rule

Runs input against a single firewall rule and returns the evaluation result.

Sends a `POST` request to `/firewall-rules/{ruleSlug}:evaluate`

Arguments:
- `rule_slug`: Firewall rule slug
- `body`
*/
    pub async fn post_firewall_rule_evaluate<'a>(
        &'a self,
        rule_slug: &'a str,
        body: &'a types::FirewallRuleEvaluationRequest,
    ) -> Result<
        ResponseValue<types::FirewallEvaluationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/firewall-rules/{}:evaluate", self.baseurl, encode_path(& rule_slug
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_firewall_rule_evaluate",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List firewalls

Returns all firewalls for the authenticated account.

Sends a `GET` request to `/firewalls`

*/
    pub async fn get_firewalls<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::FirewallResponse>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/firewalls", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewalls",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a firewall

Creates a new firewall for the authenticated account.

Sends a `POST` request to `/firewalls`

*/
    pub async fn post_firewall<'a>(
        &'a self,
        body: &'a types::CreateFirewallRequest,
    ) -> Result<ResponseValue<types::FirewallResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/firewalls", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_firewall",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a firewall

Returns a single firewall by slug.

Sends a `GET` request to `/firewalls/{firewallSlug}`

Arguments:
- `firewall_slug`: Firewall slug
*/
    pub async fn get_firewall<'a>(
        &'a self,
        firewall_slug: &'a str,
    ) -> Result<ResponseValue<types::FirewallResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/firewalls/{}", self.baseurl, encode_path(& firewall_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewall",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a firewall

Removes a firewall by slug.

Sends a `DELETE` request to `/firewalls/{firewallSlug}`

Arguments:
- `firewall_slug`: Firewall slug
*/
    pub async fn delete_firewall<'a>(
        &'a self,
        firewall_slug: &'a str,
    ) -> Result<
        ResponseValue<::std::collections::HashMap<::std::string::String, bool>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/firewalls/{}", self.baseurl, encode_path(& firewall_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_firewall",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a firewall

Partially updates a firewall by slug.

Sends a `PATCH` request to `/firewalls/{firewallSlug}`

Arguments:
- `firewall_slug`: Firewall slug
- `body`
*/
    pub async fn patch_firewall<'a>(
        &'a self,
        firewall_slug: &'a str,
        body: &'a types::UpdateFirewallRequest,
    ) -> Result<ResponseValue<types::FirewallResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/firewalls/{}", self.baseurl, encode_path(& firewall_slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_firewall",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Evaluate a firewall

Runs input against a firewall and returns the evaluation result.

Sends a `POST` request to `/firewalls/{firewallSlug}:evaluate`

Arguments:
- `firewall_slug`: Firewall slug
- `body`
*/
    pub async fn post_firewall_evaluate<'a>(
        &'a self,
        firewall_slug: &'a str,
        body: &'a types::FirewallEvaluationRequest,
    ) -> Result<
        ResponseValue<types::FirewallEvaluationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/firewalls/{}:evaluate", self.baseurl, encode_path(& firewall_slug
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_firewall_evaluate",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List vector databases

Returns the vector databases owned by the authenticated account.

Sends a `GET` request to `/flexible-vector-databases`

*/
    pub async fn get_vector_databases<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::VectorDatabaseResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/flexible-vector-databases", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vector_databases",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a vector database

Creates a new vector database for the authenticated account.

Sends a `POST` request to `/flexible-vector-databases`

*/
    pub async fn post_vector_databases<'a>(
        &'a self,
        body: &'a types::CreateVectorDatabaseRequest,
    ) -> Result<
        ResponseValue<types::VectorDatabaseResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/flexible-vector-databases", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_vector_databases",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a vector database

Returns a single vector database owned by the authenticated account.

Sends a `GET` request to `/flexible-vector-databases/{databaseID}`

Arguments:
- `database_id`: Vector database ID
*/
    pub async fn get_vector_database<'a>(
        &'a self,
        database_id: &'a str,
    ) -> Result<
        ResponseValue<types::VectorDatabaseResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}", self.baseurl, encode_path(& database_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vector_database",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a vector database

Removes a vector database owned by the authenticated account.

Sends a `DELETE` request to `/flexible-vector-databases/{databaseID}`

Arguments:
- `database_id`: Vector database ID
*/
    pub async fn delete_vector_database<'a>(
        &'a self,
        database_id: &'a str,
    ) -> Result<
        ResponseValue<::std::collections::HashMap<::std::string::String, bool>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}", self.baseurl, encode_path(& database_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_vector_database",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a vector database

Updates the slug, name, or description of an existing vector database.

Sends a `PATCH` request to `/flexible-vector-databases/{databaseID}`

Arguments:
- `database_id`: Vector database ID
- `body`
*/
    pub async fn patch_vector_database<'a>(
        &'a self,
        database_id: &'a str,
        body: &'a types::UpdateVectorDatabaseRequest,
    ) -> Result<
        ResponseValue<types::VectorDatabaseResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}", self.baseurl, encode_path(& database_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_vector_database",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List documents in a vector database

Returns documents stored in a vector database with optional pagination.

Sends a `GET` request to `/flexible-vector-databases/{databaseID}/documents`

Arguments:
- `database_id`: Vector database ID
- `cursor`: Pagination cursor from a previous response
- `limit`: Maximum number of documents to return
*/
    pub async fn get_vector_database_documents<'a>(
        &'a self,
        database_id: &'a str,
        cursor: Option<&'a str>,
        limit: Option<i64>,
    ) -> Result<
        ResponseValue<types::ListDocumentsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}/documents", self.baseurl, encode_path(&
            database_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("cursor", &cursor))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vector_database_documents",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Index documents directly

Indexes documents into a vector database synchronously.

Sends a `POST` request to `/flexible-vector-databases/{databaseID}/index-direct`

Arguments:
- `database_id`: Vector database ID
- `body`
*/
    pub async fn post_vector_database_index_direct<'a>(
        &'a self,
        database_id: &'a str,
        body: &'a types::IndexDocumentsRequest,
    ) -> Result<
        ResponseValue<::std::collections::HashMap<::std::string::String, i32>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}/index-direct", self.baseurl, encode_path(&
            database_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_vector_database_index_direct",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Enqueue an index job

Enqueues documents for asynchronous indexing into a vector database.

Sends a `POST` request to `/flexible-vector-databases/{databaseID}/index-jobs`

Arguments:
- `database_id`: Vector database ID
- `body`
*/
    pub async fn post_vector_database_index_jobs<'a>(
        &'a self,
        database_id: &'a str,
        body: &'a types::IndexDocumentsRequest,
    ) -> Result<
        ResponseValue<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}/index-jobs", self.baseurl, encode_path(&
            database_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_vector_database_index_jobs",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an index job

Returns the status of an asynchronous index job.

Sends a `GET` request to `/flexible-vector-databases/{databaseID}/index-jobs/{jobID}`

Arguments:
- `database_id`: Vector database ID
- `job_id`: Index job ID
*/
    pub async fn get_vector_database_index_job<'a>(
        &'a self,
        database_id: &'a str,
        job_id: &'a str,
    ) -> Result<ResponseValue<types::IndexJobResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/flexible-vector-databases/{}/index-jobs/{}", self.baseurl, encode_path(&
            database_id.to_string()), encode_path(& job_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vector_database_index_job",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Recommend documents

Returns recommended documents for a recommend token.

Sends a `POST` request to `/flexible-vector-databases/{databaseID}/recommend`

Arguments:
- `database_id`: Vector database ID
- `body`
*/
    pub async fn post_vector_database_recommend<'a>(
        &'a self,
        database_id: &'a str,
        body: &'a types::RecommendRequest,
    ) -> Result<ResponseValue<types::SearchHitResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/flexible-vector-databases/{}/recommend", self.baseurl, encode_path(&
            database_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_vector_database_recommend",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Search a vector database

Searches a vector database by query text and/or image URL.

Sends a `POST` request to `/flexible-vector-databases/{databaseID}/search`

Arguments:
- `database_id`: Vector database ID
- `body`
*/
    pub async fn post_vector_database_search<'a>(
        &'a self,
        database_id: &'a str,
        body: &'a types::SearchRequest,
    ) -> Result<ResponseValue<types::SearchHitResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/flexible-vector-databases/{}/search", self.baseurl, encode_path(&
            database_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_vector_database_search",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**View an element

Records a view for a document and returns a recommend token.

Sends a `POST` request to `/flexible-vector-databases/{databaseID}/view-element`

Arguments:
- `database_id`: Vector database ID
- `body`
*/
    pub async fn post_vector_database_view_element<'a>(
        &'a self,
        database_id: &'a str,
        body: &'a types::ViewElementRequest,
    ) -> Result<
        ResponseValue<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/flexible-vector-databases/{}/view-element", self.baseurl, encode_path(&
            database_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_vector_database_view_element",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Health check

Returns service health

Sends a `GET` request to `/health`

*/
    pub async fn x<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::string::String>, Error<()>> {
        let url = format!("{}/health", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo { operation_id: "x" };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Search embedding models

Searches Hugging Face for embedding models.

Sends a `GET` request to `/huggingface/embeddings`

Arguments:
- `limit`: Number of models (default 50, max 100)
- `search`: Search query
*/
    pub async fn get_huggingface_embeddings<'a>(
        &'a self,
        limit: Option<i64>,
        search: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::HuggingFaceModelsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/huggingface/embeddings", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("search", &search))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_huggingface_embeddings",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Hugging Face file sizes

Returns GGUF file sizes and quantization for a Hugging Face repository.

Sends a `GET` request to `/huggingface/files`

Arguments:
- `repo`: Hugging Face repository ID (e.g. owner/model)
*/
    pub async fn get_huggingface_files<'a>(
        &'a self,
        repo: &'a str,
    ) -> Result<
        ResponseValue<types::HuggingFaceGgufFileResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/huggingface/files", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("repo", &repo))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_huggingface_files",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Search Hugging Face models

Searches Hugging Face for compatible models matching the query.

Sends a `GET` request to `/huggingface/models`

Arguments:
- `q`: Search query
*/
    pub async fn get_huggingface_models<'a>(
        &'a self,
        q: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::HuggingFaceModelsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/huggingface/models", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("q", &q))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_huggingface_models",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Hugging Face organization info

Returns profile information for a Hugging Face organization.

Sends a `GET` request to `/huggingface/orgs/{org}`

Arguments:
- `org`: Hugging Face organization name
*/
    pub async fn get_huggingface_orgs<'a>(
        &'a self,
        org: &'a str,
    ) -> Result<
        ResponseValue<types::HuggingFaceOrgInfoResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/huggingface/orgs/{}", self.baseurl, encode_path(& org.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_huggingface_orgs",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Search Hugging Face models by tag

Searches Hugging Face for models matching a query and pipeline tag.

Sends a `GET` request to `/huggingface/search-tag`

Arguments:
- `limit`: Number of models (default 50, max 100)
- `pipeline_tag`: Pipeline tag (e.g. text-generation)
- `q`: Search query
*/
    pub async fn get_huggingface_search_tag<'a>(
        &'a self,
        limit: Option<i64>,
        pipeline_tag: &'a str,
        q: &'a str,
    ) -> Result<
        ResponseValue<types::HuggingFaceModelsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/huggingface/search-tag", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("pipeline_tag", &pipeline_tag))
            .query(&progenitor_client::QueryParam::new("q", &q))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_huggingface_search_tag",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Trending Hugging Face models

Returns trending Hugging Face models.

Sends a `GET` request to `/huggingface/trending`

Arguments:
- `limit`: Number of models (default 20, max 100)
*/
    pub async fn get_huggingface_trending<'a>(
        &'a self,
        limit: Option<i64>,
    ) -> Result<
        ResponseValue<types::HuggingFaceModelsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/huggingface/trending", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_huggingface_trending",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List available GPUs

Returns the GPU catalog with pricing, regions, and availability for the authenticated account.

Sends a `GET` request to `/instance-config/gpus`

*/
    pub async fn get_instance_config_gpus<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GpuListResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/instance-config/gpus", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_instance_config_gpus",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List inference instances

Sends a `GET` request to `/instances`

Arguments:
- `state`: Optional state filter (non_bootstrap, pending, running, stopped, failed)
*/
    pub async fn get_instances<'a>(
        &'a self,
        state: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::InstanceListResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/instances", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("state", &state))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_instances",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create an inference instance

Sends a `POST` request to `/instances`

Arguments:
- `idempotency_key`: Optional idempotency key
- `body`
*/
    pub async fn post_instances<'a>(
        &'a self,
        idempotency_key: Option<&'a str>,
        body: &'a types::CreateInstanceRequest,
    ) -> Result<ResponseValue<types::InstanceResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/instances", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = idempotency_key {
            header_map.append("Idempotency-Key", value.to_string().try_into()?);
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_instances",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            402u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an inference instance

Sends a `GET` request to `/instances/{instanceID}`

Arguments:
- `instance_id`: Instance ID
*/
    pub async fn get_instance_by_id<'a>(
        &'a self,
        instance_id: &'a str,
    ) -> Result<ResponseValue<types::InstanceResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/instances/{}", self.baseurl, encode_path(& instance_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_instance_by_id",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete an inference instance

Sends a `DELETE` request to `/instances/{instanceID}`

Arguments:
- `instance_id`: Instance ID
*/
    pub async fn delete_instance<'a>(
        &'a self,
        instance_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/instances/{}", self.baseurl, encode_path(& instance_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_instance",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update an inference instance

Sends a `PATCH` request to `/instances/{instanceID}`

Arguments:
- `instance_id`: Instance ID
- `body`
*/
    pub async fn patch_instance<'a>(
        &'a self,
        instance_id: &'a str,
        body: &'a types::UpdateInstanceRequest,
    ) -> Result<ResponseValue<types::InstanceResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/instances/{}", self.baseurl, encode_path(& instance_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_instance",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Preview smart selection for an existing instance update

Previews the compatibility and smart selection for an existing instance while applying smart-setting overrides from the request body. Lets the instance settings page fetch the same backend-filtered GPU list used at launch time instead of reimplementing compatibility filtering in the UI.

Sends a `POST` request to `/instances/{instanceID}/preview`

Arguments:
- `instance_id`: Instance ID
- `body`
*/
    pub async fn post_instance_preview_update<'a>(
        &'a self,
        instance_id: &'a str,
        body: &'a types::PreviewInstanceUpdateRequest,
    ) -> Result<
        ResponseValue<types::PreviewInstanceResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/instances/{}/preview", self.baseurl, encode_path(& instance_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_instance_preview_update",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List schedule rules for an instance

Sends a `GET` request to `/instances/{instanceID}/schedule-rules`

Arguments:
- `instance_id`: Instance ID
*/
    pub async fn get_instance_schedule_rules<'a>(
        &'a self,
        instance_id: &'a str,
    ) -> Result<
        ResponseValue<types::ScheduleRuleListResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/instances/{}/schedule-rules", self.baseurl, encode_path(& instance_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_instance_schedule_rules",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a schedule rule for an instance

Sends a `POST` request to `/instances/{instanceID}/schedule-rules`

Arguments:
- `instance_id`: Instance ID
- `body`
*/
    pub async fn post_instance_schedule_rule<'a>(
        &'a self,
        instance_id: &'a str,
        body: &'a types::ScheduleRuleRequest,
    ) -> Result<
        ResponseValue<types::ScheduleRuleResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/instances/{}/schedule-rules", self.baseurl, encode_path(& instance_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_instance_schedule_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a schedule rule for an instance

Sends a `DELETE` request to `/instances/{instanceID}/schedule-rules/{ruleID}`

Arguments:
- `instance_id`: Instance ID
- `rule_id`: Schedule rule ID
*/
    pub async fn delete_instance_schedule_rule<'a>(
        &'a self,
        instance_id: &'a str,
        rule_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/instances/{}/schedule-rules/{}", self.baseurl, encode_path(& instance_id
            .to_string()), encode_path(& rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_instance_schedule_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a schedule rule for an instance

Sends a `PATCH` request to `/instances/{instanceID}/schedule-rules/{ruleID}`

Arguments:
- `instance_id`: Instance ID
- `rule_id`: Schedule rule ID
- `body`
*/
    pub async fn patch_instance_schedule_rule<'a>(
        &'a self,
        instance_id: &'a str,
        rule_id: &'a str,
        body: &'a types::ScheduleRuleRequest,
    ) -> Result<
        ResponseValue<types::ScheduleRuleResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/instances/{}/schedule-rules/{}", self.baseurl, encode_path(& instance_id
            .to_string()), encode_path(& rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_instance_schedule_rule",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Start an inference instance

Sends a `POST` request to `/instances/{instanceID}/start`

Arguments:
- `instance_id`: Instance ID
*/
    pub async fn post_instance_start<'a>(
        &'a self,
        instance_id: &'a str,
    ) -> Result<ResponseValue<types::InstanceResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/instances/{}/start", self.baseurl, encode_path(& instance_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_instance_start",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Stop an inference instance

Sends a `POST` request to `/instances/{instanceID}/stop`

Arguments:
- `instance_id`: Instance ID
*/
    pub async fn post_instance_stop<'a>(
        &'a self,
        instance_id: &'a str,
    ) -> Result<ResponseValue<types::InstanceResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/instances/{}/stop", self.baseurl, encode_path(& instance_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_instance_stop",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get an inference instance usage and cost breakdown

Sends a `GET` request to `/instances/{instanceID}/usage`

Arguments:
- `instance_id`: Instance ID
- `range`: Time range (24h, 7d, 30d)
*/
    pub async fn get_instance_usage<'a>(
        &'a self,
        instance_id: &'a str,
        range: Option<types::GetInstanceUsageRange>,
    ) -> Result<
        ResponseValue<types::InstanceUsageResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/instances/{}/usage", self.baseurl, encode_path(& instance_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("range", &range))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_instance_usage",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List pending invitations for the authenticated user

Returns invitations pending acceptance that were sent to the authenticated user's email.

Sends a `GET` request to `/invitations`

*/
    pub async fn get_invitations<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::InvitationListResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/invitations", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_invitations",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Accept an invitation

Accepts a pending invitation by token or invitation ID and attaches the authenticated user to the account.

Sends a `POST` request to `/invitations/accept`

*/
    pub async fn post_invitations_accept<'a>(
        &'a self,
        body: &'a types::AcceptInvitationRequest,
    ) -> Result<
        ResponseValue<types::AccountMembershipResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/invitations/accept", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_invitations_accept",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            410u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List fixed models

Returns the catalog of fixed (curated) models supported by the platform.

Sends a `GET` request to `/models/fixed`

*/
    pub async fn get_models_fixed<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::HuggingFaceModelsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/models/fixed", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_models_fixed",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List replica sets

Sends a `GET` request to `/replica-sets`

*/
    pub async fn get_replica_sets<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::ReplicaSetListResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/replica-sets", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_replica_sets",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a replica set

Sends a `GET` request to `/replica-sets/{replicaSetID}`

Arguments:
- `replica_set_id`: Replica set ID
*/
    pub async fn get_replica_set<'a>(
        &'a self,
        replica_set_id: &'a str,
    ) -> Result<ResponseValue<types::ReplicaSetResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/replica-sets/{}", self.baseurl, encode_path(& replica_set_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_replica_set",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a replica set

Sends a `DELETE` request to `/replica-sets/{replicaSetID}`

Arguments:
- `replica_set_id`: Replica set ID
*/
    pub async fn delete_replica_set<'a>(
        &'a self,
        replica_set_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/replica-sets/{}", self.baseurl, encode_path(& replica_set_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_replica_set",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a replica set

Sends a `PATCH` request to `/replica-sets/{replicaSetID}`

Arguments:
- `replica_set_id`: Replica set ID
- `body`
*/
    pub async fn patch_replica_set<'a>(
        &'a self,
        replica_set_id: &'a str,
        body: &'a types::UpdateReplicaSetRequest,
    ) -> Result<ResponseValue<types::ReplicaSetResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/replica-sets/{}", self.baseurl, encode_path(& replica_set_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_replica_set",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Scale a replica set

Sends a `POST` request to `/replica-sets/{replicaSetID}/scale`

Arguments:
- `replica_set_id`: Replica set ID
- `body`
*/
    pub async fn post_replica_set_scale<'a>(
        &'a self,
        replica_set_id: &'a str,
        body: &'a types::ScaleReplicaSetRequest,
    ) -> Result<ResponseValue<types::ReplicaSetResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/replica-sets/{}/scale", self.baseurl, encode_path(& replica_set_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_replica_set_scale",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Report a runtime pod crash

Handles crash notifications from runtime pods. The request must include a valid runtime JWT in the Authorization header (not a user Bearer token).

Sends a `POST` request to `/runtime/notify/crash`

*/
    pub async fn post_runtime_notify_crash<'a>(
        &'a self,
        body: &'a ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!("{}/runtime/notify/crash", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_runtime_notify_crash",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List serving endpoints

Sends a `GET` request to `/serving-endpoints`

*/
    pub async fn get_serving_endpoints<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<types::ServingEndpointResponse>,
            >,
        >,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/serving-endpoints", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_serving_endpoints",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a serving endpoint

Sends a `POST` request to `/serving-endpoints`

*/
    pub async fn post_serving_endpoint<'a>(
        &'a self,
        body: &'a types::CreateServingEndpointRequest,
    ) -> Result<
        ResponseValue<types::ServingEndpointResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/serving-endpoints", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_serving_endpoint",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List smart balancers

Sends a `GET` request to `/smart-balancers`

*/
    pub async fn get_smart_balancers<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<types::SmartBalancerViewResponse>,
            >,
        >,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/smart-balancers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_smart_balancers",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a smart balancer

Sends a `POST` request to `/smart-balancers`

*/
    pub async fn post_smart_balancer<'a>(
        &'a self,
        body: &'a types::CreateSmartBalancerRequest,
    ) -> Result<
        ResponseValue<types::SmartBalancerViewResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/smart-balancers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_smart_balancer",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a smart balancer

Sends a `DELETE` request to `/smart-balancers/{smartBalancerID}`

Arguments:
- `smart_balancer_id`: Smart balancer ID
*/
    pub async fn delete_smart_balancer<'a>(
        &'a self,
        smart_balancer_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/smart-balancers/{}", self.baseurl, encode_path(& smart_balancer_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_smart_balancer",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a smart balancer

Sends a `PATCH` request to `/smart-balancers/{smartBalancerID}`

Arguments:
- `smart_balancer_id`: Smart balancer ID
- `body`
*/
    pub async fn patch_smart_balancer<'a>(
        &'a self,
        smart_balancer_id: &'a str,
        body: &'a types::UpdateSmartBalancerRequest,
    ) -> Result<
        ResponseValue<types::SmartBalancerViewResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/smart-balancers/{}", self.baseurl, encode_path(& smart_balancer_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_smart_balancer",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List storage volumes

Lists all persistent storage volumes owned by the authenticated account.

Sends a `GET` request to `/storage/volumes`

*/
    pub async fn get_storage_volumes<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::StorageVolumeResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/storage/volumes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_storage_volumes",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a storage volume

Creates a persistent storage volume for model checkpoints and caches.

Sends a `POST` request to `/storage/volumes`

*/
    pub async fn post_storage_volumes<'a>(
        &'a self,
        body: &'a types::CreateStorageVolumeRequest,
    ) -> Result<
        ResponseValue<types::StorageVolumeResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/storage/volumes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_storage_volumes",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a storage volume

Returns details for a specific persistent storage volume.

Sends a `GET` request to `/storage/volumes/{volumeID}`

Arguments:
- `volume_id`: Volume ID
*/
    pub async fn get_storage_volume<'a>(
        &'a self,
        volume_id: &'a str,
    ) -> Result<
        ResponseValue<types::StorageVolumeResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/storage/volumes/{}", self.baseurl, encode_path(& volume_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_storage_volume",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a storage volume

Deletes a persistent storage volume and its contents.

Sends a `DELETE` request to `/storage/volumes/{volumeID}`

Arguments:
- `volume_id`: Volume ID
*/
    pub async fn delete_storage_volume<'a>(
        &'a self,
        volume_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/storage/volumes/{}", self.baseurl, encode_path(& volume_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_storage_volume",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List system tools

Lists available system tools (code interpreter, web search, etc.) for the authenticated account.

Sends a `GET` request to `/system-tools`

*/
    pub async fn get_system_tools<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SystemToolOutput>, Error<types::ErrorResponse>> {
        let url = format!("{}/system-tools", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_system_tools",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List target groups

Sends a `GET` request to `/target-groups`

*/
    pub async fn get_target_groups<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<types::TargetGroupResponse>,
            >,
        >,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/target-groups", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_target_groups",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a target group

Sends a `POST` request to `/target-groups`

*/
    pub async fn post_target_group<'a>(
        &'a self,
        body: &'a types::CreateTargetGroupRequest,
    ) -> Result<ResponseValue<types::TargetGroupResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/target-groups", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_target_group",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a target group

Sends a `GET` request to `/target-groups/{targetGroupID}`

Arguments:
- `target_group_id`: Target group ID
*/
    pub async fn get_target_group<'a>(
        &'a self,
        target_group_id: &'a str,
    ) -> Result<ResponseValue<types::TargetGroupResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/target-groups/{}", self.baseurl, encode_path(& target_group_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_target_group",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a target group

Sends a `DELETE` request to `/target-groups/{targetGroupID}`

Arguments:
- `target_group_id`: Target group ID
*/
    pub async fn delete_target_group<'a>(
        &'a self,
        target_group_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/target-groups/{}", self.baseurl, encode_path(& target_group_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_target_group",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a target group

Sends a `PATCH` request to `/target-groups/{targetGroupID}`

Arguments:
- `target_group_id`: Target group ID
- `body`
*/
    pub async fn patch_target_group<'a>(
        &'a self,
        target_group_id: &'a str,
        body: &'a types::UpdateTargetGroupRequest,
    ) -> Result<ResponseValue<types::TargetGroupResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/target-groups/{}", self.baseurl, encode_path(& target_group_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_target_group",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List API keys for the authenticated user

Returns the API keys issued for the authenticated user identity.

Sends a `GET` request to `/users/me/api-keys`

*/
    pub async fn get_user_api_keys<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ApiKeyListResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/users/me/api-keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_user_api_keys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create an API key for the authenticated user

Issues a new API key for the authenticated user identity and returns the plaintext key once.

Sends a `POST` request to `/users/me/api-keys`

*/
    pub async fn post_user_api_keys<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ApiKeyResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/users/me/api-keys", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_user_api_keys",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Revoke a user API key

Revokes an API key issued for the authenticated user identity.

Sends a `POST` request to `/users/me/api-keys/{keyID}/revoke`

Arguments:
- `key_id`: API key ID
*/
    pub async fn post_user_api_keys_key_id_revoke<'a>(
        &'a self,
        key_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/users/me/api-keys/{}/revoke", self.baseurl, encode_path(& key_id
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_user_api_keys_key_id_revoke",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**OpenAI-compatible audio speech (TTS)

Generates audio from text. Accepts Bearer access tokens or API keys.

Sends a `POST` request to `/v1/audio/speech`

*/
    pub async fn post_v1_audio_speech<'a>(
        &'a self,
        body: &'a types::AudioSpeechRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/audio/speech", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "post_v1_audio_speech",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List available audio voices for a model

Lists available TTS voices and supported languages for a model. Accepts Bearer access tokens or API keys.

Sends a `GET` request to `/v1/audio/voices`

Arguments:
- `model`: Model ID
*/
    pub async fn get_v1_audio_voices<'a>(
        &'a self,
        model: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/audio/voices", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("model", &model))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_audio_voices",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**OpenAI-compatible chat completions

Executes a chat completion using a managed instance or public model. Accepts Bearer access tokens or API keys. Supports streaming (SSE) when "stream": true.

Sends a `POST` request to `/v1/chat/completions`

*/
    pub async fn post_v1_chat_completions<'a>(
        &'a self,
        body: &'a types::ChatRequestEnvelope,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/chat/completions", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_v1_chat_completions",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            403u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**OpenAI-compatible embeddings

Generates embeddings for a text or multimodal input. Accepts Bearer access tokens or API keys.

Sends a `POST` request to `/v1/embeddings`

*/
    pub async fn post_v1_embeddings<'a>(
        &'a self,
        body: &'a types::EmbeddingsRequestEnvelope,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/embeddings", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_v1_embeddings",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**OpenAI-compatible image generations

Generates images from a prompt. Accepts Bearer access tokens or API keys.

Sends a `POST` request to `/v1/images/generations`

*/
    pub async fn post_v1_image_generations<'a>(
        &'a self,
        body: &'a types::ImageGenerationRequest,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/images/generations", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_v1_image_generations",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Public model prices

Returns provider model pricing for cost comparison. No authentication required.

Sends a `GET` request to `/v1/public/model-prices`

Arguments:
- `query`: Search query for model prices
*/
    pub async fn get_v1_public_model_prices<'a>(
        &'a self,
        query: Option<&'a str>,
    ) -> Result<ResponseValue<types::ModelPricesResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/public/model-prices", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("query", &query))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_public_model_prices",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            504u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Public pricing catalog

Returns the public GPU and services pricing catalog. No authentication required.

Sends a `GET` request to `/v1/public/pricing`

*/
    pub async fn get_v1_public_pricing<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::PublicPricingResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/public/pricing", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_public_pricing",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**OpenAI-compatible Responses API over WebSocket

WebSocket transport for the Responses API. Accepts Bearer access tokens or API keys. Upgrades the HTTP request to a WebSocket session that exchanges Responses payloads.

Sends a `GET` request to `/v1/responses`

*/
    pub async fn get_v1_responses_web_socket<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ErrorResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/responses", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_responses_web_socket",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            101u16 => ResponseValue::from_response(response).await,
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**OpenAI-compatible Responses API

Executes a Responses API request using a managed instance or public model. Accepts Bearer access tokens or API keys. Supports streaming (SSE) when "stream": true.

Sends a `POST` request to `/v1/responses`

*/
    pub async fn post_v1_responses<'a>(
        &'a self,
        body: &'a types::ResponsesRequestEnvelope,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/responses", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_v1_responses",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Compact a Responses conversation

Compresses a conversation history into a compact representation for context management. Accepts Bearer access tokens or API keys.

Sends a `POST` request to `/v1/responses/compact`

*/
    pub async fn post_v1_responses_compact<'a>(
        &'a self,
        body: &'a types::ResponsesRequestEnvelope,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/responses/compact", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_v1_responses_compact",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            401u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            502u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List template variants

Lists pre-configured deployment template variants visible to the authenticated user.

Sends a `GET` request to `/v1/templates`

Arguments:
- `category`: Category filter
- `featured_only`: Only featured templates
- `limit`: Page size (default 50, max 100)
- `locale`: Locale (default en)
- `offset`: Page offset
- `tag`: Tag filter
*/
    pub async fn get_v1_templates<'a>(
        &'a self,
        category: Option<&'a str>,
        featured_only: Option<bool>,
        limit: Option<i64>,
        locale: Option<&'a str>,
        offset: Option<i64>,
        tag: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::ListUserVariantsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/v1/templates", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("category", &category))
            .query(&progenitor_client::QueryParam::new("featured_only", &featured_only))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("locale", &locale))
            .query(&progenitor_client::QueryParam::new("offset", &offset))
            .query(&progenitor_client::QueryParam::new("tag", &tag))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_templates",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            503u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a template by slug

Returns a template variant by its URL slug for the authenticated user.

Sends a `GET` request to `/v1/templates/by-slug/{slug}`

Arguments:
- `slug`: Template slug
- `locale`: Locale (default en)
*/
    pub async fn get_v1_template_by_slug<'a>(
        &'a self,
        slug: &'a str,
        locale: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::GetTemplateBySlugResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/v1/templates/by-slug/{}", self.baseurl, encode_path(& slug.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("locale", &locale))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_template_by_slug",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            503u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a template variant

Returns a single template variant by ID for the authenticated user.

Sends a `GET` request to `/v1/templates/{id}`

Arguments:
- `id`: Template variant ID
- `locale`: Locale (default en)
*/
    pub async fn get_v1_template<'a>(
        &'a self,
        id: &'a str,
        locale: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::GetUserVariantResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/v1/templates/{}", self.baseurl, encode_path(& id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("locale", &locale))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_v1_template",
        };
        match (crate::auth::inject)(&mut request).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom(e.to_string())),
        }
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            503u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
