/// Message describing the payload of service health logs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLog {
    /// Brief description for the event.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Free-form, human-readable description.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Identifies the category of the event.
    #[prost(enumeration = "event_log::EventCategory", tag = "3")]
    pub category: i32,
    /// The current state of the event.
    #[prost(enumeration = "event_log::State", tag = "4")]
    pub state: i32,
    /// The current detailed state of the event.
    #[prost(enumeration = "event_log::DetailedState", tag = "14")]
    pub detailed_state: i32,
    /// Google Cloud products known to be affected by the event, in JSON serialized
    /// format. List of all supported [Google Cloud
    /// products](../resources/products-locations.md).
    ///
    /// Example: "`['Google Cloud SQL', 'Cloud Compute Engine']`".
    #[prost(string, tag = "15")]
    pub impacted_products: ::prost::alloc::string::String,
    /// Locations known to be impacted by the event, in JSON serialized format. See
    /// possible \[values\](products-locations.md), which are subject to where the
    /// service is running.
    ///
    /// Example: "`['us-central1', 'us-west1']`".
    #[prost(string, tag = "6")]
    pub impacted_locations: ::prost::alloc::string::String,
    /// Communicates why a given event is deemed relevant in the context of a given
    /// project.
    #[prost(enumeration = "event_log::Relevance", tag = "7")]
    pub relevance: i32,
    /// When `detailed_state`=`MERGED`, `parent_event` contains the name of the
    /// parent event. All further updates will be published to the parent event.
    #[prost(string, tag = "8")]
    pub parent_event: ::prost::alloc::string::String,
    /// The time when the event was last modified.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The start time of the event, if applicable.
    #[prost(message, optional, tag = "11")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of the event, if applicable.
    #[prost(message, optional, tag = "12")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Incident-only field. The time when the next update can be expected.
    #[prost(message, optional, tag = "13")]
    pub next_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `EventLog`.
pub mod event_log {
    /// The category of the event. This enum lists all possible categories of
    /// event.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EventCategory {
        /// Unspecified category.
        Unspecified = 0,
        /// Event category for service outage or degradation.
        Incident = 2,
    }
    impl EventCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventCategory::Unspecified => "EVENT_CATEGORY_UNSPECIFIED",
                EventCategory::Incident => "INCIDENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
                "INCIDENT" => Some(Self::Incident),
                _ => None,
            }
        }
    }
    /// The state of the event. This enum lists all possible states of event.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state
        Unspecified = 0,
        /// Event is actively affecting a Google Cloud service and will continue to
        /// receive updates.
        Active = 1,
        /// Event is no longer affecting the Google Cloud service or has been merged
        /// with another event.
        Closed = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
    /// The detailed state of the event. This enum lists all possible detail states
    /// of event.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DetailedState {
        /// Unspecified detail state.
        Unspecified = 0,
        /// Google engineers are actively investigating the event to determine the
        /// impact.
        Emerging = 1,
        /// The event is confirmed and impacting at least one Google Cloud service.
        /// Ongoing status updates will be provided until it is resolved.
        Confirmed = 2,
        /// The event is no longer affecting any Google Cloud service, and there will
        /// be no further updates.
        Resolved = 3,
        /// Event was merged into a parent event. All further updates will be
        /// published to the parent only. The `parent_event` field contains the name
        /// of the parent.
        Merged = 4,
    }
    impl DetailedState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DetailedState::Unspecified => "DETAILED_STATE_UNSPECIFIED",
                DetailedState::Emerging => "EMERGING",
                DetailedState::Confirmed => "CONFIRMED",
                DetailedState::Resolved => "RESOLVED",
                DetailedState::Merged => "MERGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DETAILED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "EMERGING" => Some(Self::Emerging),
                "CONFIRMED" => Some(Self::Confirmed),
                "RESOLVED" => Some(Self::Resolved),
                "MERGED" => Some(Self::Merged),
                _ => None,
            }
        }
    }
    /// Communicates why a given event is deemed relevant in the context of a given
    /// project. This enum lists all possible detailed states of relevance.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Relevance {
        /// Unspecified relevance.
        Unspecified = 0,
        /// The relevance of the event to the project is unknown.
        Unknown = 2,
        /// The event does not impact the project.
        NotImpacted = 6,
        /// We determined that the event is linked to a product that is used by
        /// the project, but we have no information (either positive
        /// or negative) whether the project is affected.
        PartiallyRelated = 7,
        /// The event has a connection to your project and it may be impacted
        Related = 8,
        /// The event is impacting your project
        Impacted = 9,
    }
    impl Relevance {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Relevance::Unspecified => "RELEVANCE_UNSPECIFIED",
                Relevance::Unknown => "UNKNOWN",
                Relevance::NotImpacted => "NOT_IMPACTED",
                Relevance::PartiallyRelated => "PARTIALLY_RELATED",
                Relevance::Related => "RELATED",
                Relevance::Impacted => "IMPACTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RELEVANCE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "NOT_IMPACTED" => Some(Self::NotImpacted),
                "PARTIALLY_RELATED" => Some(Self::PartiallyRelated),
                "RELATED" => Some(Self::Related),
                "IMPACTED" => Some(Self::Impacted),
                _ => None,
            }
        }
    }
}
