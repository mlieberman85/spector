//! This file is generated by typify through Spector. Do not edit it directly.
//! Exceptions to this rule are for cases where typify doesn't genrate the correct code.
#![allow(clippy::all)]
#![allow(warnings)]
use serde::{Deserialize, Serialize};
///A structure representing the build definition of the SLSA Provenance v1 Predicate.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct BuildDefinition {
    #[serde(rename = "buildType")]
    pub build_type: String,
    #[serde(rename = "externalParameters")]
    pub external_parameters: std::collections::HashMap<String, serde_json::Value>,
    ///Unordered collection of artifacts needed at build time. Completeness is best effort, at least through SLSA Build L3. For example, if the build script fetches and executes “example.com/foo.sh”, which in turn fetches “example.com/bar.tar.gz”, then both “foo.sh” and “bar.tar.gz” SHOULD be listed here.
    #[serde(
        rename = "internalParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_parameters: Option<
        std::collections::HashMap<String, serde_json::Value>,
    >,
    ///Unordered collection of artifacts needed at build time. Completeness is best effort, at least through SLSA Build L3. For example, if the build script fetches and executes “example.com/foo.sh”, which in turn fetches “example.com/bar.tar.gz”, then both “foo.sh” and “bar.tar.gz” SHOULD be listed here.
    #[serde(
        rename = "resolvedDependencies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolved_dependencies: Option<Vec<ResourceDescriptor>>,
}
impl From<&BuildDefinition> for BuildDefinition {
    fn from(value: &BuildDefinition) -> Self {
        value.clone()
    }
}
impl BuildDefinition {
    pub fn builder() -> builder::BuildDefinition {
        builder::BuildDefinition::default()
    }
}
///A structure representing the metadata of the SLSA Provenance v1 Predicate.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct BuildMetadata {
    ///The timestamp of when the build completed.
    #[serde(rename = "finishedOn", default, skip_serializing_if = "Option::is_none")]
    pub finished_on: Option<chrono::DateTime<chrono::offset::Utc>>,
    ///Identifies this particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The exact meaning and format is defined by builder.id; by default it is treated as opaque and case-sensitive. The value SHOULD be globally unique.
    #[serde(rename = "invocationId", default, skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    ///The timestamp of when the build started.
    #[serde(rename = "startedOn", default, skip_serializing_if = "Option::is_none")]
    pub started_on: Option<chrono::DateTime<chrono::offset::Utc>>,
}
impl From<&BuildMetadata> for BuildMetadata {
    fn from(value: &BuildMetadata) -> Self {
        value.clone()
    }
}
impl BuildMetadata {
    pub fn builder() -> builder::BuildMetadata {
        builder::BuildMetadata::default()
    }
}
///A structure representing the builder information of the SLSA Provenance v1 Predicate.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Builder {
    #[serde(
        rename = "builderDependencies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub builder_dependencies: Option<Vec<ResourceDescriptor>>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&Builder> for Builder {
    fn from(value: &Builder) -> Self {
        value.clone()
    }
}
impl Builder {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct DigestSet(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for DigestSet {
    type Target = std::collections::HashMap<String, String>;
    fn deref(&self) -> &std::collections::HashMap<String, String> {
        &self.0
    }
}
impl From<DigestSet> for std::collections::HashMap<String, String> {
    fn from(value: DigestSet) -> Self {
        value.0
    }
}
impl From<&DigestSet> for DigestSet {
    fn from(value: &DigestSet) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for DigestSet {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self(value)
    }
}
///Represents an In-Toto v1 statement.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InTotoStatementV1 {
    pub predicate: Predicate,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub subject: Vec<Subject>,
    #[serde(rename = "_type")]
    pub type_: String,
}
impl From<&InTotoStatementV1> for InTotoStatementV1 {
    fn from(value: &InTotoStatementV1) -> Self {
        value.clone()
    }
}
impl InTotoStatementV1 {
    pub fn builder() -> builder::InTotoStatementV1 {
        builder::InTotoStatementV1::default()
    }
}
/**An enum representing different predicate types.

Known predicate types have their own variants, while unknown types are represented by the `Other` variant, which stores the raw JSON value.

TODO(mlieberman85): Support (de)serializing the predicates based on the predicateType URL in the statement.*/
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Predicate {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<SlsaProvenanceV1Predicate>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<serde_json::Value>,
}
impl From<&Predicate> for Predicate {
    fn from(value: &Predicate) -> Self {
        value.clone()
    }
}
impl Predicate {
    pub fn builder() -> builder::Predicate {
        builder::Predicate::default()
    }
}
///A size-efficient description of any software artifact or resource (mutable or immutable).
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ResourceDescriptor {
    ///This field MAY be used to provide additional information or metadata about the resource or artifact that may be useful to the consumer when evaluating the attestation against a policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, serde_json::Value>>,
    ///The contents of the resource or artifact. This field is REQUIRED unless either uri or digest is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    ///A set of cryptographic digests of the contents of the resource or artifact. This field is REQUIRED unless either uri or content is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<std::collections::HashMap<String, String>>,
    ///The location of the described resource or artifact, if different from the uri.
    #[serde(
        rename = "downloadLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_location: Option<String>,
    ///The MIME Type (i.e., media type) of the described resource or artifact.
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    ///Machine-readable identifier for distinguishing between descriptors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A URI used to identify the resource or artifact globally. This field is REQUIRED unless either digest or content is set.
    pub uri: String,
}
impl From<&ResourceDescriptor> for ResourceDescriptor {
    fn from(value: &ResourceDescriptor) -> Self {
        value.clone()
    }
}
impl ResourceDescriptor {
    pub fn builder() -> builder::ResourceDescriptor {
        builder::ResourceDescriptor::default()
    }
}
///A structure representing the run details of the SLSA Provenance v1 Predicate.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct RunDetails {
    ///Identifies the build platform that executed the invocation, which is trusted to have correctly performed the operation and populated this provenance.
    pub builder: Builder,
    ///Additional artifacts generated during the build that are not considered the “output” of the build but that might be needed during debugging or incident response. For example, this might reference logs generated during the build and/or a digest of the fully evaluated build configuration.\nIn most cases, this SHOULD NOT contain all intermediate files generated during the build. Instead, this SHOULD only contain files that are likely to be useful later and that cannot be easily reproduced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byproducts: Option<Vec<ResourceDescriptor>>,
    ///metadata about this particular execution of the build.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BuildMetadata>,
}
impl From<&RunDetails> for RunDetails {
    fn from(value: &RunDetails) -> Self {
        value.clone()
    }
}
impl RunDetails {
    pub fn builder() -> builder::RunDetails {
        builder::RunDetails::default()
    }
}
///A structure representing the SLSA Provenance v1 Predicate.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SlsaProvenanceV1Predicate {
    #[serde(rename = "buildDefinition")]
    pub build_definition: BuildDefinition,
    #[serde(rename = "runDetails")]
    pub run_details: RunDetails,
}
impl From<&SlsaProvenanceV1Predicate> for SlsaProvenanceV1Predicate {
    fn from(value: &SlsaProvenanceV1Predicate) -> Self {
        value.clone()
    }
}
impl SlsaProvenanceV1Predicate {
    pub fn builder() -> builder::SlsaProvenanceV1Predicate {
        builder::SlsaProvenanceV1Predicate::default()
    }
}
///Represents a subject in an In-Toto v1 statement.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Subject {
    pub digest: DigestSet,
    pub name: String,
}
impl From<&Subject> for Subject {
    fn from(value: &Subject) -> Self {
        value.clone()
    }
}
impl Subject {
    pub fn builder() -> builder::Subject {
        builder::Subject::default()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BuildDefinition {
        build_type: Result<String, String>,
        external_parameters: Result<
            std::collections::HashMap<String, serde_json::Value>,
            String,
        >,
        internal_parameters: Result<
            Option<std::collections::HashMap<String, serde_json::Value>>,
            String,
        >,
        resolved_dependencies: Result<Option<Vec<super::ResourceDescriptor>>, String>,
    }
    impl Default for BuildDefinition {
        fn default() -> Self {
            Self {
                build_type: Err("no value supplied for build_type".to_string()),
                external_parameters: Err(
                    "no value supplied for external_parameters".to_string(),
                ),
                internal_parameters: Ok(Default::default()),
                resolved_dependencies: Ok(Default::default()),
            }
        }
    }
    impl BuildDefinition {
        pub fn build_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .build_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for build_type: {}", e)
                });
            self
        }
        pub fn external_parameters<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, serde_json::Value>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .external_parameters = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for external_parameters: {}", e
                    )
                });
            self
        }
        pub fn internal_parameters<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<std::collections::HashMap<String, serde_json::Value>>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .internal_parameters = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for internal_parameters: {}", e
                    )
                });
            self
        }
        pub fn resolved_dependencies<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::ResourceDescriptor>>>,
            T::Error: std::fmt::Display,
        {
            self
                .resolved_dependencies = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for resolved_dependencies: {}",
                        e
                    )
                });
            self
        }
    }
    impl std::convert::TryFrom<BuildDefinition> for super::BuildDefinition {
        type Error = String;
        fn try_from(value: BuildDefinition) -> Result<Self, String> {
            Ok(Self {
                build_type: value.build_type?,
                external_parameters: value.external_parameters?,
                internal_parameters: value.internal_parameters?,
                resolved_dependencies: value.resolved_dependencies?,
            })
        }
    }
    impl From<super::BuildDefinition> for BuildDefinition {
        fn from(value: super::BuildDefinition) -> Self {
            Self {
                build_type: Ok(value.build_type),
                external_parameters: Ok(value.external_parameters),
                internal_parameters: Ok(value.internal_parameters),
                resolved_dependencies: Ok(value.resolved_dependencies),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BuildMetadata {
        finished_on: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        invocation_id: Result<Option<String>, String>,
        started_on: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
    }
    impl Default for BuildMetadata {
        fn default() -> Self {
            Self {
                finished_on: Ok(Default::default()),
                invocation_id: Ok(Default::default()),
                started_on: Ok(Default::default()),
            }
        }
    }
    impl BuildMetadata {
        pub fn finished_on<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self
                .finished_on = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for finished_on: {}", e)
                });
            self
        }
        pub fn invocation_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .invocation_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for invocation_id: {}", e)
                });
            self
        }
        pub fn started_on<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self
                .started_on = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for started_on: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<BuildMetadata> for super::BuildMetadata {
        type Error = String;
        fn try_from(value: BuildMetadata) -> Result<Self, String> {
            Ok(Self {
                finished_on: value.finished_on?,
                invocation_id: value.invocation_id?,
                started_on: value.started_on?,
            })
        }
    }
    impl From<super::BuildMetadata> for BuildMetadata {
        fn from(value: super::BuildMetadata) -> Self {
            Self {
                finished_on: Ok(value.finished_on),
                invocation_id: Ok(value.invocation_id),
                started_on: Ok(value.started_on),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Builder {
        builder_dependencies: Result<Option<Vec<super::ResourceDescriptor>>, String>,
        id: Result<String, String>,
        version: Result<Option<String>, String>,
    }
    impl Default for Builder {
        fn default() -> Self {
            Self {
                builder_dependencies: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl Builder {
        pub fn builder_dependencies<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::ResourceDescriptor>>>,
            T::Error: std::fmt::Display,
        {
            self
                .builder_dependencies = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for builder_dependencies: {}", e
                    )
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Builder> for super::Builder {
        type Error = String;
        fn try_from(value: Builder) -> Result<Self, String> {
            Ok(Self {
                builder_dependencies: value.builder_dependencies?,
                id: value.id?,
                version: value.version?,
            })
        }
    }
    impl From<super::Builder> for Builder {
        fn from(value: super::Builder) -> Self {
            Self {
                builder_dependencies: Ok(value.builder_dependencies),
                id: Ok(value.id),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InTotoStatementV1 {
        predicate: Result<super::Predicate, String>,
        predicate_type: Result<String, String>,
        subject: Result<Vec<super::Subject>, String>,
        type_: Result<String, String>,
    }
    impl Default for InTotoStatementV1 {
        fn default() -> Self {
            Self {
                predicate: Err("no value supplied for predicate".to_string()),
                predicate_type: Err("no value supplied for predicate_type".to_string()),
                subject: Err("no value supplied for subject".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl InTotoStatementV1 {
        pub fn predicate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Predicate>,
            T::Error: std::fmt::Display,
        {
            self
                .predicate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for predicate: {}", e)
                });
            self
        }
        pub fn predicate_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .predicate_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for predicate_type: {}", e)
                });
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Subject>>,
            T::Error: std::fmt::Display,
        {
            self
                .subject = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subject: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<InTotoStatementV1> for super::InTotoStatementV1 {
        type Error = String;
        fn try_from(value: InTotoStatementV1) -> Result<Self, String> {
            Ok(Self {
                predicate: value.predicate?,
                predicate_type: value.predicate_type?,
                subject: value.subject?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::InTotoStatementV1> for InTotoStatementV1 {
        fn from(value: super::InTotoStatementV1) -> Self {
            Self {
                predicate: Ok(value.predicate),
                predicate_type: Ok(value.predicate_type),
                subject: Ok(value.subject),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Predicate {
        subtype_0: Result<Option<super::SlsaProvenanceV1Predicate>, String>,
        subtype_1: Result<Option<serde_json::Value>, String>,
    }
    impl Default for Predicate {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl Predicate {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SlsaProvenanceV1Predicate>>,
            T::Error: std::fmt::Display,
        {
            self
                .subtype_0 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_0: {}", e)
                });
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self
                .subtype_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subtype_1: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Predicate> for super::Predicate {
        type Error = String;
        fn try_from(value: Predicate) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::Predicate> for Predicate {
        fn from(value: super::Predicate) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResourceDescriptor {
        annotations: Result<
            Option<std::collections::HashMap<String, serde_json::Value>>,
            String,
        >,
        content: Result<Option<String>, String>,
        digest: Result<Option<std::collections::HashMap<String, String>>, String>,
        download_location: Result<Option<String>, String>,
        media_type: Result<Option<String>, String>,
        name: Result<Option<String>, String>,
        uri: Result<String, String>,
    }
    impl Default for ResourceDescriptor {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                content: Ok(Default::default()),
                digest: Ok(Default::default()),
                download_location: Ok(Default::default()),
                media_type: Ok(Default::default()),
                name: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl ResourceDescriptor {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<std::collections::HashMap<String, serde_json::Value>>,
            >,
            T::Error: std::fmt::Display,
        {
            self
                .annotations = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for annotations: {}", e)
                });
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .content = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content: {}", e)
                });
            self
        }
        pub fn digest<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<std::collections::HashMap<String, String>>>,
            T::Error: std::fmt::Display,
        {
            self
                .digest = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for digest: {}", e)
                });
            self
        }
        pub fn download_location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .download_location = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for download_location: {}", e
                    )
                });
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .media_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for media_type: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ResourceDescriptor> for super::ResourceDescriptor {
        type Error = String;
        fn try_from(value: ResourceDescriptor) -> Result<Self, String> {
            Ok(Self {
                annotations: value.annotations?,
                content: value.content?,
                digest: value.digest?,
                download_location: value.download_location?,
                media_type: value.media_type?,
                name: value.name?,
                uri: value.uri?,
            })
        }
    }
    impl From<super::ResourceDescriptor> for ResourceDescriptor {
        fn from(value: super::ResourceDescriptor) -> Self {
            Self {
                annotations: Ok(value.annotations),
                content: Ok(value.content),
                digest: Ok(value.digest),
                download_location: Ok(value.download_location),
                media_type: Ok(value.media_type),
                name: Ok(value.name),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunDetails {
        builder: Result<super::Builder, String>,
        byproducts: Result<Option<Vec<super::ResourceDescriptor>>, String>,
        metadata: Result<Option<super::BuildMetadata>, String>,
    }
    impl Default for RunDetails {
        fn default() -> Self {
            Self {
                builder: Err("no value supplied for builder".to_string()),
                byproducts: Ok(Default::default()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl RunDetails {
        pub fn builder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Builder>,
            T::Error: std::fmt::Display,
        {
            self
                .builder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for builder: {}", e)
                });
            self
        }
        pub fn byproducts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::ResourceDescriptor>>>,
            T::Error: std::fmt::Display,
        {
            self
                .byproducts = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for byproducts: {}", e)
                });
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BuildMetadata>>,
            T::Error: std::fmt::Display,
        {
            self
                .metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for metadata: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<RunDetails> for super::RunDetails {
        type Error = String;
        fn try_from(value: RunDetails) -> Result<Self, String> {
            Ok(Self {
                builder: value.builder?,
                byproducts: value.byproducts?,
                metadata: value.metadata?,
            })
        }
    }
    impl From<super::RunDetails> for RunDetails {
        fn from(value: super::RunDetails) -> Self {
            Self {
                builder: Ok(value.builder),
                byproducts: Ok(value.byproducts),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SlsaProvenanceV1Predicate {
        build_definition: Result<super::BuildDefinition, String>,
        run_details: Result<super::RunDetails, String>,
    }
    impl Default for SlsaProvenanceV1Predicate {
        fn default() -> Self {
            Self {
                build_definition: Err(
                    "no value supplied for build_definition".to_string(),
                ),
                run_details: Err("no value supplied for run_details".to_string()),
            }
        }
    }
    impl SlsaProvenanceV1Predicate {
        pub fn build_definition<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BuildDefinition>,
            T::Error: std::fmt::Display,
        {
            self
                .build_definition = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for build_definition: {}", e
                    )
                });
            self
        }
        pub fn run_details<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RunDetails>,
            T::Error: std::fmt::Display,
        {
            self
                .run_details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for run_details: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<SlsaProvenanceV1Predicate>
    for super::SlsaProvenanceV1Predicate {
        type Error = String;
        fn try_from(value: SlsaProvenanceV1Predicate) -> Result<Self, String> {
            Ok(Self {
                build_definition: value.build_definition?,
                run_details: value.run_details?,
            })
        }
    }
    impl From<super::SlsaProvenanceV1Predicate> for SlsaProvenanceV1Predicate {
        fn from(value: super::SlsaProvenanceV1Predicate) -> Self {
            Self {
                build_definition: Ok(value.build_definition),
                run_details: Ok(value.run_details),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Subject {
        digest: Result<super::DigestSet, String>,
        name: Result<String, String>,
    }
    impl Default for Subject {
        fn default() -> Self {
            Self {
                digest: Err("no value supplied for digest".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Subject {
        pub fn digest<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DigestSet>,
            T::Error: std::fmt::Display,
        {
            self
                .digest = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for digest: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self
                .name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Subject> for super::Subject {
        type Error = String;
        fn try_from(value: Subject) -> Result<Self, String> {
            Ok(Self {
                digest: value.digest?,
                name: value.name?,
            })
        }
    }
    impl From<super::Subject> for Subject {
        fn from(value: super::Subject) -> Self {
            Self {
                digest: Ok(value.digest),
                name: Ok(value.name),
            }
        }
    }
}

