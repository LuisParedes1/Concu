/// An AsyncOp state update.
///
/// An async operation is a an operation that is associated with a resource
/// This could, for example, be a a read or write on a TCP stream or a receive operation on
/// a channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncOpUpdate {
    /// A list of new async operations that were created since the last `AsyncOpUpdate`
    /// was sent. Note that the fact that an async operation has been created
    /// does not mean that is has been polled or is being polled. This information
    /// is reflected in the Stats of the operation.
    #[prost(message, repeated, tag="1")]
    pub new_async_ops: ::prost::alloc::vec::Vec<AsyncOp>,
    /// Any async op stats that have changed since the last update.
    #[prost(map="uint64, message", tag="2")]
    pub stats_update: ::std::collections::HashMap<u64, Stats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncOp {
    /// The async op's ID.
    ///
    /// This uniquely identifies this op across all *currently live*
    /// ones.
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<super::common::Id>,
    /// The numeric ID of the op's `Metadata`.
    ///
    /// This identifies the `Metadata` that describes the `tracing` span
    /// corresponding to this async op. The metadata for this ID will have been sent
    /// in a prior `RegisterMetadata` message.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::common::MetaId>,
    /// The source of this async operation. Most commonly this should be the name
    /// of the method where the instantiation of this op has happened.
    #[prost(string, tag="3")]
    pub source: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    /// Timestamp of when the async op has been created.
    #[prost(message, optional, tag="1")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp of when the async op was dropped.
    #[prost(message, optional, tag="2")]
    pub dropped_at: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource Id this `AsyncOp` is associated with. Note that both
    /// `resource_id` and `task_id` can be None if this async op has not been polled yet
    #[prost(message, optional, tag="3")]
    pub resource_id: ::core::option::Option<super::common::Id>,
    /// The Id of the task that is awaiting on this op.
    #[prost(message, optional, tag="4")]
    pub task_id: ::core::option::Option<super::common::Id>,
    /// Contains the operation poll stats.
    #[prost(message, optional, tag="5")]
    pub poll_stats: ::core::option::Option<super::common::PollStats>,
}
