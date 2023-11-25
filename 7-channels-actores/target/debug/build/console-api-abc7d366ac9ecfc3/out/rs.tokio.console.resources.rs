/// A resource state update.
///
/// Each `ResourceUpdate` contains any resource data that has changed since the last
/// update. This includes:
/// - any new resources that were created since the last update
/// - the current stats for any resource whose stats changed since the last update
/// - any new poll ops that have been incoked on a resource
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUpdate {
    /// A list of new resources that were created since the last `ResourceUpdate` was
    /// sent.
    #[prost(message, repeated, tag="1")]
    pub new_resources: ::prost::alloc::vec::Vec<Resource>,
    /// Any resource stats that have changed since the last update.
    #[prost(map="uint64, message", tag="2")]
    pub stats_update: ::std::collections::HashMap<u64, Stats>,
    /// A list of all new poll ops that have been invoked on resources since the last update.
    #[prost(message, repeated, tag="3")]
    pub new_poll_ops: ::prost::alloc::vec::Vec<PollOp>,
}
/// Static data recorded when a new resource is created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The resources's ID.
    ///
    /// This uniquely identifies this resource across all *currently live*
    /// resources. This is also the primary way any operations on a resource
    /// are associated with it
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::common::Id>,
    /// The numeric ID of the resources's `Metadata`.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::common::MetaId>,
    /// The resources's concrete rust type.
    #[prost(string, tag="3")]
    pub concrete_type: ::prost::alloc::string::String,
    /// The kind of resource (e.g timer, mutex)
    #[prost(message, optional, tag="4")]
    pub kind: ::core::option::Option<resource::Kind>,
    /// The location in code where the resource was created.
    #[prost(message, optional, tag="5")]
    pub location: ::core::option::Option<super::common::Location>,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Kind {
        #[prost(oneof="kind::Kind", tags="1, 2")]
        pub kind: ::core::option::Option<kind::Kind>,
    }
    /// Nested message and enum types in `Kind`.
    pub mod kind {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Known {
            Timer = 0,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(enumeration="Known", tag="1")]
            Known(i32),
            #[prost(string, tag="2")]
            Other(::prost::alloc::string::String),
        }
    }
}
/// Task runtime stats of a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    /// Timestamp of when the resource was created.
    #[prost(message, optional, tag="1")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp of when the resource was dropped.
    #[prost(message, optional, tag="2")]
    pub dropped_at: ::core::option::Option<::prost_types::Timestamp>,
    /// State attributes of the resource. These are dependent on the type of the resource.
    /// For example, a timer resource will have a duration while a semaphore resource may
    /// have permits as an attribute. These values may change over time as the state of
    /// the resource changes. Therefore, they live in the runtime stats rather than the
    /// static data describing the resource.
    #[prost(message, repeated, tag="3")]
    pub attributes: ::prost::alloc::vec::Vec<stats::Attribute>,
}
/// Nested message and enum types in `Stats`.
pub mod stats {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribute {
        #[prost(message, optional, tag="1")]
        pub field: ::core::option::Option<super::super::common::Field>,
        #[prost(string, optional, tag="2")]
        pub unit: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PollOp {
    /// The numeric ID of the op's `Metadata`.
    ///
    /// This identifies the `Metadata` that describes the `tracing` span
    /// corresponding to this op. The metadata for this ID will have been sent
    /// in a prior `RegisterMetadata` message.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::common::MetaId>,
    /// The resources's ID.
    #[prost(message, optional, tag="3")]
    pub resource_id: ::core::option::Option<super::common::Id>,
    /// the name of this op (e.g. poll_elapsed, new_timeout, reset, etc.)
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Identifies the task context that this poll op has been called from.
    #[prost(message, optional, tag="5")]
    pub task_id: ::core::option::Option<super::common::Id>,
    /// Identifies the async op ID that this poll op is part of.
    #[prost(message, optional, tag="6")]
    pub async_op_id: ::core::option::Option<super::common::Id>,
    /// Whether this poll op has returned with ready or pending.
    #[prost(bool, tag="7")]
    pub is_ready: bool,
}
