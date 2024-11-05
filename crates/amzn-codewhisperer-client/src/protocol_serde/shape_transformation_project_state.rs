// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transformation_project_state(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TransformationProjectState,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.language {
        object.key("language").string(var_1.as_str());
    }
    if let Some(var_2) = &input.runtime_env {
        #[allow(unused_mut)]
        let mut object_3 = object.key("runtimeEnv").start_object();
        crate::protocol_serde::shape_transformation_runtime_env::ser_transformation_runtime_env(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.platform_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("platformConfig").start_object();
        crate::protocol_serde::shape_transformation_platform_config::ser_transformation_platform_config(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_transformation_project_state<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::TransformationProjectState>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TransformationProjectStateBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "language" => {
                                builder = builder.set_language(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| {
                                            s.to_unescaped()
                                                .map(|u| crate::types::TransformationLanguage::from(u.as_ref()))
                                        })
                                        .transpose()?,
                                );
                            },
                            "runtimeEnv" => {
                                builder = builder.set_runtime_env(
                                crate::protocol_serde::shape_transformation_runtime_env::de_transformation_runtime_env(tokens)?,
                            );
                            },
                            "platformConfig" => {
                                builder = builder.set_platform_config(
                                crate::protocol_serde::shape_transformation_platform_config::de_transformation_platform_config(tokens)?,
                            );
                            },
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            format!("expected object key or end object, found: {:?}", other),
                        ));
                    },
                }
            }
            Ok(Some(builder.build()))
        },
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}