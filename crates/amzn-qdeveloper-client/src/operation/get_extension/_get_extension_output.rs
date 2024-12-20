// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GetExtensionOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub extension_provider: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub extension_id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub extension_credential: ::std::option::Option<crate::types::ExtensionCredential>,
    #[allow(missing_docs)] // documentation missing in model
    pub extension_properties:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    #[allow(missing_docs)] // documentation missing in model
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetExtensionOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_provider(&self) -> &str {
        use std::ops::Deref;
        self.extension_provider.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_id(&self) -> &str {
        use std::ops::Deref;
        self.extension_id.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_credential(&self) -> ::std::option::Option<&crate::types::ExtensionCredential> {
        self.extension_credential.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_properties(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.extension_properties.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
}
impl ::std::fmt::Debug for GetExtensionOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetExtensionOutput");
        formatter.field("extension_provider", &self.extension_provider);
        formatter.field("extension_id", &self.extension_id);
        formatter.field("extension_credential", &"*** Sensitive Data Redacted ***");
        formatter.field("extension_properties", &self.extension_properties);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for GetExtensionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetExtensionOutput {
    /// Creates a new builder-style object to manufacture
    /// [`GetExtensionOutput`](crate::operation::get_extension::GetExtensionOutput).
    pub fn builder() -> crate::operation::get_extension::builders::GetExtensionOutputBuilder {
        crate::operation::get_extension::builders::GetExtensionOutputBuilder::default()
    }
}

/// A builder for [`GetExtensionOutput`](crate::operation::get_extension::GetExtensionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct GetExtensionOutputBuilder {
    pub(crate) extension_provider: ::std::option::Option<::std::string::String>,
    pub(crate) extension_id: ::std::option::Option<::std::string::String>,
    pub(crate) extension_credential: ::std::option::Option<crate::types::ExtensionCredential>,
    pub(crate) extension_properties:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetExtensionOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn extension_provider(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.extension_provider = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_provider(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.extension_provider = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_provider(&self) -> &::std::option::Option<::std::string::String> {
        &self.extension_provider
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn extension_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.extension_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.extension_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.extension_id
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_credential(mut self, input: crate::types::ExtensionCredential) -> Self {
        self.extension_credential = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_credential(mut self, input: ::std::option::Option<crate::types::ExtensionCredential>) -> Self {
        self.extension_credential = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_credential(&self) -> &::std::option::Option<crate::types::ExtensionCredential> {
        &self.extension_credential
    }

    /// Adds a key-value pair to `extension_properties`.
    ///
    /// To override the contents of this collection use
    /// [`set_extension_properties`](Self::set_extension_properties).
    pub fn extension_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.extension_properties.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.extension_properties = ::std::option::Option::Some(hash_map);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_properties(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.extension_properties = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_properties(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.extension_properties
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`GetExtensionOutput`](crate::operation::get_extension::GetExtensionOutput). This method
    /// will fail if any of the following fields are not set:
    /// - [`extension_provider`](crate::operation::get_extension::builders::GetExtensionOutputBuilder::extension_provider)
    /// - [`extension_id`](crate::operation::get_extension::builders::GetExtensionOutputBuilder::extension_id)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_extension::GetExtensionOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_extension::GetExtensionOutput {
            extension_provider: self.extension_provider.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "extension_provider",
                    "extension_provider was not specified but it is required when building GetExtensionOutput",
                )
            })?,
            extension_id: self.extension_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "extension_id",
                    "extension_id was not specified but it is required when building GetExtensionOutput",
                )
            })?,
            extension_credential: self.extension_credential,
            extension_properties: self.extension_properties,
            creation_time: self.creation_time,
            _request_id: self._request_id,
        })
    }
}
impl ::std::fmt::Debug for GetExtensionOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetExtensionOutputBuilder");
        formatter.field("extension_provider", &self.extension_provider);
        formatter.field("extension_id", &self.extension_id);
        formatter.field("extension_credential", &"*** Sensitive Data Redacted ***");
        formatter.field("extension_properties", &self.extension_properties);
        formatter.field("creation_time", &self.creation_time);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
