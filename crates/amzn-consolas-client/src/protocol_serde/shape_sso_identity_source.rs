// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sso_identity_source(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SsoIdentitySource,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("instanceArn").string(input.instance_arn.as_str());
    }
    if let Some(var_1) = &input.sso_region {
        object.key("ssoRegion").string(var_1.as_str());
    }
    Ok(())
}