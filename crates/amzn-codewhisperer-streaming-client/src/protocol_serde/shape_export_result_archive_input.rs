// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_result_archive_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::export_result_archive::ExportResultArchiveInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.export_id {
        object.key("exportId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.export_intent {
        object.key("exportIntent").string(var_2.as_str());
    }
    if let Some(var_3) = &input.export_context {
        #[allow(unused_mut)]
        let mut object_4 = object.key("exportContext").start_object();
        crate::protocol_serde::shape_export_context::ser_export_context(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}