// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundErrorBuilder,
) -> crate::types::error::builders::ResourceNotFoundErrorBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerErrorBuilder,
) -> crate::types::error::builders::InternalServerErrorBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedErrorBuilder,
) -> crate::types::error::builders::AccessDeniedErrorBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictErrorBuilder,
) -> crate::types::error::builders::ConflictErrorBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationErrorBuilder,
) -> crate::types::error::builders::ValidationErrorBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.reason.is_none() {
        builder.reason = "no value was set"
            .parse::<crate::types::ValidationExceptionReason>()
            .ok()
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingErrorBuilder,
) -> crate::types::error::builders::ThrottlingErrorBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn create_extension_output_output_correct_errors(
    mut builder: crate::operation::create_extension::builders::CreateExtensionOutputBuilder,
) -> crate::operation::create_extension::builders::CreateExtensionOutputBuilder {
    if builder.extension_id.is_none() {
        builder.extension_id = Some(Default::default())
    }
    builder
}

pub(crate) fn get_conversation_output_output_correct_errors(
    mut builder: crate::operation::get_conversation::builders::GetConversationOutputBuilder,
) -> crate::operation::get_conversation::builders::GetConversationOutputBuilder {
    if builder.conversation_id.is_none() {
        builder.conversation_id = Some(Default::default())
    }
    if builder.messages.is_none() {
        builder.messages = Some(Default::default())
    }
    builder
}

pub(crate) fn get_extension_output_output_correct_errors(
    mut builder: crate::operation::get_extension::builders::GetExtensionOutputBuilder,
) -> crate::operation::get_extension::builders::GetExtensionOutputBuilder {
    if builder.extension_provider.is_none() {
        builder.extension_provider = Some(Default::default())
    }
    if builder.extension_id.is_none() {
        builder.extension_id = Some(Default::default())
    }
    builder
}

pub(crate) fn list_conversations_output_output_correct_errors(
    mut builder: crate::operation::list_conversations::builders::ListConversationsOutputBuilder,
) -> crate::operation::list_conversations::builders::ListConversationsOutputBuilder {
    if builder.conversations.is_none() {
        builder.conversations = Some(Default::default())
    }
    builder
}

pub(crate) fn list_extension_providers_output_output_correct_errors(
    mut builder: crate::operation::list_extension_providers::builders::ListExtensionProvidersOutputBuilder,
) -> crate::operation::list_extension_providers::builders::ListExtensionProvidersOutputBuilder {
    if builder.extension_providers.is_none() {
        builder.extension_providers = Some(Default::default())
    }
    builder
}

pub(crate) fn list_extensions_output_output_correct_errors(
    mut builder: crate::operation::list_extensions::builders::ListExtensionsOutputBuilder,
) -> crate::operation::list_extensions::builders::ListExtensionsOutputBuilder {
    if builder.extensions.is_none() {
        builder.extensions = Some(Default::default())
    }
    builder
}

pub(crate) fn send_message_output_output_correct_errors(
    mut builder: crate::operation::send_message::builders::SendMessageOutputBuilder,
) -> crate::operation::send_message::builders::SendMessageOutputBuilder {
    if builder.result.is_none() {
        builder.result = {
            let builder = crate::types::builders::NellyResultBuilder::default();
            crate::serde_util::nelly_result_correct_errors(builder).build().ok()
        }
    }
    if builder.metadata.is_none() {
        builder.metadata = {
            let builder = crate::types::builders::NellyResponseMetadataBuilder::default();
            crate::serde_util::nelly_response_metadata_correct_errors(builder)
                .build()
                .ok()
        }
    }
    if builder.result_code.is_none() {
        builder.result_code = "no value was set".parse::<crate::types::ResultCode>().ok()
    }
    builder
}

pub(crate) fn start_conversation_output_output_correct_errors(
    mut builder: crate::operation::start_conversation::builders::StartConversationOutputBuilder,
) -> crate::operation::start_conversation::builders::StartConversationOutputBuilder {
    if builder.conversation_id.is_none() {
        builder.conversation_id = Some(Default::default())
    }
    builder
}

pub(crate) fn start_troubleshooting_analysis_output_output_correct_errors(
    mut builder: crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisOutputBuilder,
) -> crate::operation::start_troubleshooting_analysis::builders::StartTroubleshootingAnalysisOutputBuilder {
    if builder.session_id.is_none() {
        builder.session_id = Some(Default::default())
    }
    builder
}

pub(crate) fn nelly_result_correct_errors(
    mut builder: crate::types::builders::NellyResultBuilder,
) -> crate::types::builders::NellyResultBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::ResultType>().ok()
    }
    if builder.format.is_none() {
        builder.format = "no value was set".parse::<crate::types::ResultFormat>().ok()
    }
    if builder.content.is_none() {
        builder.content = Some(crate::types::NellyContent::Unknown)
    }
    builder
}

pub(crate) fn nelly_response_metadata_correct_errors(
    mut builder: crate::types::builders::NellyResponseMetadataBuilder,
) -> crate::types::builders::NellyResponseMetadataBuilder {
    if builder.conversation_id.is_none() {
        builder.conversation_id = Some(Default::default())
    }
    if builder.utterance_id.is_none() {
        builder.utterance_id = Some(Default::default())
    }
    builder
}

pub(crate) fn conversation_metadata_correct_errors(
    mut builder: crate::types::builders::ConversationMetadataBuilder,
) -> crate::types::builders::ConversationMetadataBuilder {
    if builder.conversation_id.is_none() {
        builder.conversation_id = Some(Default::default())
    }
    if builder.timestamp.is_none() {
        builder.timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn encrypted_tool_fas_creds_correct_errors(
    mut builder: crate::types::builders::EncryptedToolFasCredsBuilder,
) -> crate::types::builders::EncryptedToolFasCredsBuilder {
    if builder.tool_id.is_none() {
        builder.tool_id = "no value was set".parse::<crate::types::ToolId>().ok()
    }
    if builder.encrypted_tool_fas_creds.is_none() {
        builder.encrypted_tool_fas_creds = Some(Default::default())
    }
    builder
}

pub(crate) fn extension_correct_errors(
    mut builder: crate::types::builders::ExtensionBuilder,
) -> crate::types::builders::ExtensionBuilder {
    if builder.extension_provider.is_none() {
        builder.extension_provider = Some(Default::default())
    }
    if builder.extension_id.is_none() {
        builder.extension_id = Some(Default::default())
    }
    builder
}

pub(crate) fn extension_provider_metadata_correct_errors(
    mut builder: crate::types::builders::ExtensionProviderMetadataBuilder,
) -> crate::types::builders::ExtensionProviderMetadataBuilder {
    if builder.extension_provider.is_none() {
        builder.extension_provider = Some(Default::default())
    }
    builder
}

pub(crate) fn message_correct_errors(
    mut builder: crate::types::builders::MessageBuilder,
) -> crate::types::builders::MessageBuilder {
    if builder.utterance_id.is_none() {
        builder.utterance_id = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::ResultType>().ok()
    }
    if builder.format.is_none() {
        builder.format = "no value was set".parse::<crate::types::ResultFormat>().ok()
    }
    if builder.content.is_none() {
        builder.content = Some(crate::types::NellyContent::Unknown)
    }
    if builder.from.is_none() {
        builder.from = "no value was set".parse::<crate::types::MessageFromType>().ok()
    }
    if builder.timestamp.is_none() {
        builder.timestamp = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn text_content_correct_errors(
    mut builder: crate::types::builders::TextContentBuilder,
) -> crate::types::builders::TextContentBuilder {
    if builder.body.is_none() {
        builder.body = Some(Default::default())
    }
    builder
}

pub(crate) fn nelly_url_correct_errors(
    mut builder: crate::types::builders::NellyUrlBuilder,
) -> crate::types::builders::NellyUrlBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.url.is_none() {
        builder.url = Some(Default::default())
    }
    builder
}
