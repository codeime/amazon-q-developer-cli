// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `TransformationDotNetRuntimeEnv`, it is important to
/// ensure your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let transformationdotnetruntimeenv = unimplemented!();
/// match transformationdotnetruntimeenv {
///     TransformationDotNetRuntimeEnv::Net50 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::Net60 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::Net70 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::Net80 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp10 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp11 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp20 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp21 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp22 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp30 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetCoreApp31 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV35 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV40 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV45 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV451 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV452 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV46 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV461 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV462 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV47 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV471 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV472 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV48 => { /* ... */ },
///     TransformationDotNetRuntimeEnv::NetFrameworkV481 => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `transformationdotnetruntimeenv` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `TransformationDotNetRuntimeEnv::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `TransformationDotNetRuntimeEnv::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `TransformationDotNetRuntimeEnv::NewFeature` is defined.
/// Specifically, when `transformationdotnetruntimeenv` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `TransformationDotNetRuntimeEnv::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum TransformationDotNetRuntimeEnv {
    #[allow(missing_docs)] // documentation missing in model
    Net50,
    #[allow(missing_docs)] // documentation missing in model
    Net60,
    #[allow(missing_docs)] // documentation missing in model
    Net70,
    #[allow(missing_docs)] // documentation missing in model
    Net80,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp10,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp11,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp20,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp21,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp22,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp30,
    #[allow(missing_docs)] // documentation missing in model
    NetCoreApp31,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV35,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV40,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV45,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV451,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV452,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV46,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV461,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV462,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV47,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV471,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV472,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV48,
    #[allow(missing_docs)] // documentation missing in model
    NetFrameworkV481,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(
        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
    )]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for TransformationDotNetRuntimeEnv {
    fn from(s: &str) -> Self {
        match s {
            "NET_5_0" => TransformationDotNetRuntimeEnv::Net50,
            "NET_6_0" => TransformationDotNetRuntimeEnv::Net60,
            "NET_7_0" => TransformationDotNetRuntimeEnv::Net70,
            "NET_8_0" => TransformationDotNetRuntimeEnv::Net80,
            "NET_CORE_APP_1_0" => TransformationDotNetRuntimeEnv::NetCoreApp10,
            "NET_CORE_APP_1_1" => TransformationDotNetRuntimeEnv::NetCoreApp11,
            "NET_CORE_APP_2_0" => TransformationDotNetRuntimeEnv::NetCoreApp20,
            "NET_CORE_APP_2_1" => TransformationDotNetRuntimeEnv::NetCoreApp21,
            "NET_CORE_APP_2_2" => TransformationDotNetRuntimeEnv::NetCoreApp22,
            "NET_CORE_APP_3_0" => TransformationDotNetRuntimeEnv::NetCoreApp30,
            "NET_CORE_APP_3_1" => TransformationDotNetRuntimeEnv::NetCoreApp31,
            "NET_FRAMEWORK_V_3_5" => TransformationDotNetRuntimeEnv::NetFrameworkV35,
            "NET_FRAMEWORK_V_4_0" => TransformationDotNetRuntimeEnv::NetFrameworkV40,
            "NET_FRAMEWORK_V_4_5" => TransformationDotNetRuntimeEnv::NetFrameworkV45,
            "NET_FRAMEWORK_V_4_5_1" => TransformationDotNetRuntimeEnv::NetFrameworkV451,
            "NET_FRAMEWORK_V_4_5_2" => TransformationDotNetRuntimeEnv::NetFrameworkV452,
            "NET_FRAMEWORK_V_4_6" => TransformationDotNetRuntimeEnv::NetFrameworkV46,
            "NET_FRAMEWORK_V_4_6_1" => TransformationDotNetRuntimeEnv::NetFrameworkV461,
            "NET_FRAMEWORK_V_4_6_2" => TransformationDotNetRuntimeEnv::NetFrameworkV462,
            "NET_FRAMEWORK_V_4_7" => TransformationDotNetRuntimeEnv::NetFrameworkV47,
            "NET_FRAMEWORK_V_4_7_1" => TransformationDotNetRuntimeEnv::NetFrameworkV471,
            "NET_FRAMEWORK_V_4_7_2" => TransformationDotNetRuntimeEnv::NetFrameworkV472,
            "NET_FRAMEWORK_V_4_8" => TransformationDotNetRuntimeEnv::NetFrameworkV48,
            "NET_FRAMEWORK_V_4_8_1" => TransformationDotNetRuntimeEnv::NetFrameworkV481,
            other => TransformationDotNetRuntimeEnv::Unknown(
                crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned()),
            ),
        }
    }
}
impl ::std::str::FromStr for TransformationDotNetRuntimeEnv {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(TransformationDotNetRuntimeEnv::from(s))
    }
}
impl TransformationDotNetRuntimeEnv {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            TransformationDotNetRuntimeEnv::Net50 => "NET_5_0",
            TransformationDotNetRuntimeEnv::Net60 => "NET_6_0",
            TransformationDotNetRuntimeEnv::Net70 => "NET_7_0",
            TransformationDotNetRuntimeEnv::Net80 => "NET_8_0",
            TransformationDotNetRuntimeEnv::NetCoreApp10 => "NET_CORE_APP_1_0",
            TransformationDotNetRuntimeEnv::NetCoreApp11 => "NET_CORE_APP_1_1",
            TransformationDotNetRuntimeEnv::NetCoreApp20 => "NET_CORE_APP_2_0",
            TransformationDotNetRuntimeEnv::NetCoreApp21 => "NET_CORE_APP_2_1",
            TransformationDotNetRuntimeEnv::NetCoreApp22 => "NET_CORE_APP_2_2",
            TransformationDotNetRuntimeEnv::NetCoreApp30 => "NET_CORE_APP_3_0",
            TransformationDotNetRuntimeEnv::NetCoreApp31 => "NET_CORE_APP_3_1",
            TransformationDotNetRuntimeEnv::NetFrameworkV35 => "NET_FRAMEWORK_V_3_5",
            TransformationDotNetRuntimeEnv::NetFrameworkV40 => "NET_FRAMEWORK_V_4_0",
            TransformationDotNetRuntimeEnv::NetFrameworkV45 => "NET_FRAMEWORK_V_4_5",
            TransformationDotNetRuntimeEnv::NetFrameworkV451 => "NET_FRAMEWORK_V_4_5_1",
            TransformationDotNetRuntimeEnv::NetFrameworkV452 => "NET_FRAMEWORK_V_4_5_2",
            TransformationDotNetRuntimeEnv::NetFrameworkV46 => "NET_FRAMEWORK_V_4_6",
            TransformationDotNetRuntimeEnv::NetFrameworkV461 => "NET_FRAMEWORK_V_4_6_1",
            TransformationDotNetRuntimeEnv::NetFrameworkV462 => "NET_FRAMEWORK_V_4_6_2",
            TransformationDotNetRuntimeEnv::NetFrameworkV47 => "NET_FRAMEWORK_V_4_7",
            TransformationDotNetRuntimeEnv::NetFrameworkV471 => "NET_FRAMEWORK_V_4_7_1",
            TransformationDotNetRuntimeEnv::NetFrameworkV472 => "NET_FRAMEWORK_V_4_7_2",
            TransformationDotNetRuntimeEnv::NetFrameworkV48 => "NET_FRAMEWORK_V_4_8",
            TransformationDotNetRuntimeEnv::NetFrameworkV481 => "NET_FRAMEWORK_V_4_8_1",
            TransformationDotNetRuntimeEnv::Unknown(value) => value.as_str(),
        }
    }

    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "NET_5_0",
            "NET_6_0",
            "NET_7_0",
            "NET_8_0",
            "NET_CORE_APP_1_0",
            "NET_CORE_APP_1_1",
            "NET_CORE_APP_2_0",
            "NET_CORE_APP_2_1",
            "NET_CORE_APP_2_2",
            "NET_CORE_APP_3_0",
            "NET_CORE_APP_3_1",
            "NET_FRAMEWORK_V_3_5",
            "NET_FRAMEWORK_V_4_0",
            "NET_FRAMEWORK_V_4_5",
            "NET_FRAMEWORK_V_4_5_1",
            "NET_FRAMEWORK_V_4_5_2",
            "NET_FRAMEWORK_V_4_6",
            "NET_FRAMEWORK_V_4_6_1",
            "NET_FRAMEWORK_V_4_6_2",
            "NET_FRAMEWORK_V_4_7",
            "NET_FRAMEWORK_V_4_7_1",
            "NET_FRAMEWORK_V_4_7_2",
            "NET_FRAMEWORK_V_4_8",
            "NET_FRAMEWORK_V_4_8_1",
        ]
    }
}
impl ::std::convert::AsRef<str> for TransformationDotNetRuntimeEnv {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl TransformationDotNetRuntimeEnv {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
impl ::std::fmt::Display for TransformationDotNetRuntimeEnv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TransformationDotNetRuntimeEnv::Net50 => write!(f, "NET_5_0"),
            TransformationDotNetRuntimeEnv::Net60 => write!(f, "NET_6_0"),
            TransformationDotNetRuntimeEnv::Net70 => write!(f, "NET_7_0"),
            TransformationDotNetRuntimeEnv::Net80 => write!(f, "NET_8_0"),
            TransformationDotNetRuntimeEnv::NetCoreApp10 => write!(f, "NET_CORE_APP_1_0"),
            TransformationDotNetRuntimeEnv::NetCoreApp11 => write!(f, "NET_CORE_APP_1_1"),
            TransformationDotNetRuntimeEnv::NetCoreApp20 => write!(f, "NET_CORE_APP_2_0"),
            TransformationDotNetRuntimeEnv::NetCoreApp21 => write!(f, "NET_CORE_APP_2_1"),
            TransformationDotNetRuntimeEnv::NetCoreApp22 => write!(f, "NET_CORE_APP_2_2"),
            TransformationDotNetRuntimeEnv::NetCoreApp30 => write!(f, "NET_CORE_APP_3_0"),
            TransformationDotNetRuntimeEnv::NetCoreApp31 => write!(f, "NET_CORE_APP_3_1"),
            TransformationDotNetRuntimeEnv::NetFrameworkV35 => write!(f, "NET_FRAMEWORK_V_3_5"),
            TransformationDotNetRuntimeEnv::NetFrameworkV40 => write!(f, "NET_FRAMEWORK_V_4_0"),
            TransformationDotNetRuntimeEnv::NetFrameworkV45 => write!(f, "NET_FRAMEWORK_V_4_5"),
            TransformationDotNetRuntimeEnv::NetFrameworkV451 => write!(f, "NET_FRAMEWORK_V_4_5_1"),
            TransformationDotNetRuntimeEnv::NetFrameworkV452 => write!(f, "NET_FRAMEWORK_V_4_5_2"),
            TransformationDotNetRuntimeEnv::NetFrameworkV46 => write!(f, "NET_FRAMEWORK_V_4_6"),
            TransformationDotNetRuntimeEnv::NetFrameworkV461 => write!(f, "NET_FRAMEWORK_V_4_6_1"),
            TransformationDotNetRuntimeEnv::NetFrameworkV462 => write!(f, "NET_FRAMEWORK_V_4_6_2"),
            TransformationDotNetRuntimeEnv::NetFrameworkV47 => write!(f, "NET_FRAMEWORK_V_4_7"),
            TransformationDotNetRuntimeEnv::NetFrameworkV471 => write!(f, "NET_FRAMEWORK_V_4_7_1"),
            TransformationDotNetRuntimeEnv::NetFrameworkV472 => write!(f, "NET_FRAMEWORK_V_4_7_2"),
            TransformationDotNetRuntimeEnv::NetFrameworkV48 => write!(f, "NET_FRAMEWORK_V_4_8"),
            TransformationDotNetRuntimeEnv::NetFrameworkV481 => write!(f, "NET_FRAMEWORK_V_4_8_1"),
            TransformationDotNetRuntimeEnv::Unknown(value) => write!(f, "{}", value),
        }
    }
}
