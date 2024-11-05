// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Profile {
    #[allow(missing_docs)] // documentation missing in model
    pub arn: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub identity_details: ::std::option::Option<crate::types::IdentityDetails>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_name: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub reference_tracker_configuration: crate::types::ReferenceTrackerConfiguration,
    #[allow(missing_docs)] // documentation missing in model
    pub kms_key_arn: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub active_functionalities: ::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>>,
    #[allow(missing_docs)] // documentation missing in model
    pub status: ::std::option::Option<crate::types::ProfileStatus>,
    #[allow(missing_docs)] // documentation missing in model
    pub error_details: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub resource_policy: ::std::option::Option<crate::types::ResourcePolicy>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_type: ::std::option::Option<crate::types::ProfileType>,
}
impl Profile {
    #[allow(missing_docs)] // documentation missing in model
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn identity_details(&self) -> ::std::option::Option<&crate::types::IdentityDetails> {
        self.identity_details.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_name(&self) -> &str {
        use std::ops::Deref;
        self.profile_name.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn reference_tracker_configuration(&self) -> &crate::types::ReferenceTrackerConfiguration {
        &self.reference_tracker_configuration
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.kms_key_arn.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.active_functionalities.is_none()`.
    pub fn active_functionalities(&self) -> &[crate::types::FunctionalityName] {
        self.active_functionalities.as_deref().unwrap_or_default()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> ::std::option::Option<&crate::types::ProfileStatus> {
        self.status.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn error_details(&self) -> ::std::option::Option<&str> {
        self.error_details.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_policy(&self) -> ::std::option::Option<&crate::types::ResourcePolicy> {
        self.resource_policy.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_type(&self) -> ::std::option::Option<&crate::types::ProfileType> {
        self.profile_type.as_ref()
    }
}
impl Profile {
    /// Creates a new builder-style object to manufacture [`Profile`](crate::types::Profile).
    pub fn builder() -> crate::types::builders::ProfileBuilder {
        crate::types::builders::ProfileBuilder::default()
    }
}

/// A builder for [`Profile`](crate::types::Profile).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ProfileBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) identity_details: ::std::option::Option<crate::types::IdentityDetails>,
    pub(crate) profile_name: ::std::option::Option<::std::string::String>,
    pub(crate) reference_tracker_configuration: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    pub(crate) kms_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) active_functionalities: ::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>>,
    pub(crate) status: ::std::option::Option<crate::types::ProfileStatus>,
    pub(crate) error_details: ::std::option::Option<::std::string::String>,
    pub(crate) resource_policy: ::std::option::Option<crate::types::ResourcePolicy>,
    pub(crate) profile_type: ::std::option::Option<crate::types::ProfileType>,
}
impl ProfileBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn identity_details(mut self, input: crate::types::IdentityDetails) -> Self {
        self.identity_details = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_identity_details(mut self, input: ::std::option::Option<crate::types::IdentityDetails>) -> Self {
        self.identity_details = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_identity_details(&self) -> &::std::option::Option<crate::types::IdentityDetails> {
        &self.identity_details
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_name = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_name = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.profile_name
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn reference_tracker_configuration(mut self, input: crate::types::ReferenceTrackerConfiguration) -> Self {
        self.reference_tracker_configuration = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_reference_tracker_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    ) -> Self {
        self.reference_tracker_configuration = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_reference_tracker_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::ReferenceTrackerConfiguration> {
        &self.reference_tracker_configuration
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_arn
    }

    /// Appends an item to `active_functionalities`.
    ///
    /// To override the contents of this collection use
    /// [`set_active_functionalities`](Self::set_active_functionalities).
    pub fn active_functionalities(mut self, input: crate::types::FunctionalityName) -> Self {
        let mut v = self.active_functionalities.unwrap_or_default();
        v.push(input);
        self.active_functionalities = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_active_functionalities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>>,
    ) -> Self {
        self.active_functionalities = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_active_functionalities(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>> {
        &self.active_functionalities
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn status(mut self, input: crate::types::ProfileStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ProfileStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ProfileStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn error_details(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_details = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_error_details(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_details = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_error_details(&self) -> &::std::option::Option<::std::string::String> {
        &self.error_details
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_policy(mut self, input: crate::types::ResourcePolicy) -> Self {
        self.resource_policy = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_resource_policy(mut self, input: ::std::option::Option<crate::types::ResourcePolicy>) -> Self {
        self.resource_policy = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_resource_policy(&self) -> &::std::option::Option<crate::types::ResourcePolicy> {
        &self.resource_policy
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_type(mut self, input: crate::types::ProfileType) -> Self {
        self.profile_type = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_type(mut self, input: ::std::option::Option<crate::types::ProfileType>) -> Self {
        self.profile_type = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_type(&self) -> &::std::option::Option<crate::types::ProfileType> {
        &self.profile_type
    }

    /// Consumes the builder and constructs a [`Profile`](crate::types::Profile).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::types::builders::ProfileBuilder::arn)
    /// - [`profile_name`](crate::types::builders::ProfileBuilder::profile_name)
    /// - [`reference_tracker_configuration`](crate::types::builders::ProfileBuilder::reference_tracker_configuration)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::Profile, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Profile {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building Profile",
                )
            })?,
            identity_details: self.identity_details,
            profile_name: self.profile_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "profile_name",
                    "profile_name was not specified but it is required when building Profile",
                )
            })?,
            reference_tracker_configuration: self.reference_tracker_configuration.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "reference_tracker_configuration",
                    "reference_tracker_configuration was not specified but it is required when building Profile",
                )
            })?,
            kms_key_arn: self.kms_key_arn,
            active_functionalities: self.active_functionalities,
            status: self.status,
            error_details: self.error_details,
            resource_policy: self.resource_policy,
            profile_type: self.profile_type,
        })
    }
}