#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
//// A Rust source code location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The file path
    #[prost(string, optional, tag="1")]
    pub file: ::core::option::Option<::prost::alloc::string::String>,
    /// The Rust module path
    #[prost(string, optional, tag="2")]
    pub module_path: ::core::option::Option<::prost::alloc::string::String>,
    /// The line number in the source code file.
    #[prost(uint32, optional, tag="3")]
    pub line: ::core::option::Option<u32>,
    /// The character in `line`.
    #[prost(uint32, optional, tag="4")]
    pub column: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaId {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpanId {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// A message representing a key-value pair of data associated with a `Span`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(message, optional, tag="8")]
    pub metadata_id: ::core::option::Option<MetaId>,
    #[prost(oneof="field::Name", tags="1, 2")]
    pub name: ::core::option::Option<field::Name>,
    #[prost(oneof="field::Value", tags="3, 4, 5, 6, 7")]
    pub value: ::core::option::Option<field::Value>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Name {
        /// The string representation of the name.
        #[prost(string, tag="1")]
        StrName(::prost::alloc::string::String),
        /// An index position into the `Metadata.field_names`.
        #[prost(uint64, tag="2")]
        NameIdx(u64),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="3")]
        DebugVal(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        StrVal(::prost::alloc::string::String),
        #[prost(uint64, tag="5")]
        U64Val(u64),
        #[prost(sint64, tag="6")]
        I64Val(i64),
        #[prost(bool, tag="7")]
        BoolVal(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Span {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<SpanId>,
    #[prost(message, optional, tag="2")]
    pub metadata_id: ::core::option::Option<MetaId>,
    #[prost(message, repeated, tag="3")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    #[prost(message, optional, tag="4")]
    pub at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterMetadata {
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<register_metadata::NewMetadata>,
}
/// Nested message and enum types in `RegisterMetadata`.
pub mod register_metadata {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NewMetadata {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::MetaId>,
        #[prost(message, optional, tag="2")]
        pub metadata: ::core::option::Option<super::Metadata>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub module_path: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub location: ::core::option::Option<Location>,
    #[prost(enumeration="metadata::Kind", tag="5")]
    pub kind: i32,
    #[prost(enumeration="metadata::Level", tag="6")]
    pub level: i32,
    /// The names of the key-value fields attached to the
    /// span or event this metadata is associated with.
    #[prost(string, repeated, tag="7")]
    pub field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Span = 0,
        Event = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        Error = 0,
        Warn = 1,
        Info = 2,
        Debug = 3,
        Trace = 4,
    }
}
/// Contains stats about objects that can be polled. Currently these can be:
/// - tasks that have been spawned
/// - async operations on resources that are performed within the context of a task
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PollStats {
    /// The total number of times this object has been polled.
    #[prost(uint64, tag="1")]
    pub polls: u64,
    /// The timestamp of the first time this object was polled.
    ///
    /// If this is `None`, the object has not yet been polled.
    ///
    /// Subtracting this timestamp from `created_at` can be used to calculate the
    /// time to first poll for this object, a measurement of executor latency.
    #[prost(message, optional, tag="3")]
    pub first_poll: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp of the most recent time this objects's poll method was invoked.
    ///
    /// If this is `None`, the object has not yet been polled.
    ///
    /// If the object has only been polled a single time, then this value may be
    /// equal to the `first_poll` timestamp.
    ///
    #[prost(message, optional, tag="4")]
    pub last_poll_started: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp of the most recent time this objects's poll method finished execution.
    ///
    /// If this is `None`, the object has not yet been polled or is currently being polled.
    ///
    /// If the object does not exist anymore, then this is the time the final invocation of
    /// its poll method has completed.
    #[prost(message, optional, tag="5")]
    pub last_poll_ended: ::core::option::Option<::prost_types::Timestamp>,
    /// The total duration this object was being *actively polled*, summed across
    /// all polls. Note that this includes only polls that have completed and is
    /// not reflecting any inprogress polls. Subtracting `busy_time` from the
    /// total lifetime of the polled object results in the amount of time it
    /// has spent *waiting* to be polled.
    #[prost(message, optional, tag="6")]
    pub busy_time: ::core::option::Option<::prost_types::Duration>,
}
