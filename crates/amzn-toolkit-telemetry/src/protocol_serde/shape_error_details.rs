// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_error_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ErrorDetails,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Command").string(input.command.as_str());
    }
    {
        object.key("EpochTimestamp").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.epoch_timestamp).into()),
        );
    }
    {
        object.key("Type").string(input.r#type.as_str());
    }
    if let Some(var_1) = &input.message {
        object.key("Message").string(var_1.as_str());
    }
    {
        object.key("StackTrace").string(input.stack_trace.as_str());
    }
    Ok(())
}