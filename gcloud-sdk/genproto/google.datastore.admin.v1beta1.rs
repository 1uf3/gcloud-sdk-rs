///  Metadata common to all Datastore Admin operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonMetadata {
    ///  The time that work began on the operation.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The time the operation ended, either successfully or otherwise.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The type of the operation. Can be used as a filter in
    ///  ListOperationsRequest.
    #[prost(enumeration="OperationType", tag="3")]
    pub operation_type: i32,
    ///  The client-assigned labels which were provided when the operation was
    ///  created. May also include additional labels.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The current state of the Operation.
    #[prost(enumeration="common_metadata::State", tag="5")]
    pub state: i32,
}
/// Nested message and enum types in `CommonMetadata`.
pub mod common_metadata {
    ///  The various possible states for an ongoing Operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Unspecified.
        Unspecified = 0,
        ///  Request is being prepared for processing.
        Initializing = 1,
        ///  Request is actively being processed.
        Processing = 2,
        ///  Request is in the process of being cancelled after user called
        ///  google.longrunning.Operations.CancelOperation on the operation.
        Cancelling = 3,
        ///  Request has been processed and is in its finalization stage.
        Finalizing = 4,
        ///  Request has completed successfully.
        Successful = 5,
        ///  Request has finished being processed, but encountered an error.
        Failed = 6,
        ///  Request has finished being cancelled after user called
        ///  google.longrunning.Operations.CancelOperation.
        Cancelled = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Initializing => "INITIALIZING",
                State::Processing => "PROCESSING",
                State::Cancelling => "CANCELLING",
                State::Finalizing => "FINALIZING",
                State::Successful => "SUCCESSFUL",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
            }
        }
    }
}
///  Measures the progress of a particular metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    ///  The amount of work that has been completed. Note that this may be greater
    ///  than work_estimated.
    #[prost(int64, tag="1")]
    pub work_completed: i64,
    ///  An estimate of how much work needs to be performed. May be zero if the
    ///  work estimate is unavailable.
    #[prost(int64, tag="2")]
    pub work_estimated: i64,
}
///  The request for
///  \[google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesRequest {
    ///  Project ID against which to make the request.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    ///  Client-assigned labels.
    #[prost(map="string, string", tag="2")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Description of what data from the project is included in the export.
    #[prost(message, optional, tag="3")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    ///  Location for the export metadata and data files.
    ///
    ///  The full resource URL of the external storage location. Currently, only
    ///  Google Cloud Storage is supported. So output_url_prefix should be of the
    ///  form: `gs://BUCKET_NAME\[/NAMESPACE_PATH\]`, where `BUCKET_NAME` is the
    ///  name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud
    ///  Storage namespace path (this is not a Cloud Datastore namespace). For more
    ///  information about Cloud Storage namespace paths, see
    ///  [Object name
    ///  considerations](<https://cloud.google.com/storage/docs/naming#object-considerations>).
    ///
    ///  The resulting files will be nested deeper than the specified URL prefix.
    ///  The final output URL will be provided in the
    ///  \[google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url\]
    ///  field. That value should be used for subsequent ImportEntities operations.
    ///
    ///  By nesting the data files deeper, the same Cloud Storage bucket can be used
    ///  in multiple ExportEntities operations without conflict.
    #[prost(string, tag="4")]
    pub output_url_prefix: ::prost::alloc::string::String,
}
///  The request for
///  \[google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesRequest {
    ///  Project ID against which to make the request.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    ///  Client-assigned labels.
    #[prost(map="string, string", tag="2")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The full resource URL of the external storage location. Currently, only
    ///  Google Cloud Storage is supported. So input_url should be of the form:
    ///  `gs://BUCKET_NAME\[/NAMESPACE_PATH\]/OVERALL_EXPORT_METADATA_FILE`, where
    ///  `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is
    ///  an optional Cloud Storage namespace path (this is not a Cloud Datastore
    ///  namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written
    ///  by the ExportEntities operation. For more information about Cloud Storage
    ///  namespace paths, see
    ///  [Object name
    ///  considerations](<https://cloud.google.com/storage/docs/naming#object-considerations>).
    ///
    ///  For more information, see
    ///  \[google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url\].
    #[prost(string, tag="3")]
    pub input_url: ::prost::alloc::string::String,
    ///  Optionally specify which kinds/namespaces are to be imported. If provided,
    ///  the list must be a subset of the EntityFilter used in creating the export,
    ///  otherwise a FAILED_PRECONDITION error will be returned. If no filter is
    ///  specified then all entities from the export are imported.
    #[prost(message, optional, tag="4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
}
///  The response for
///  \[google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesResponse {
    ///  Location of the output metadata file. This can be used to begin an import
    ///  into Cloud Datastore (this project or another project). See
    ///  \[google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url][google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url\].
    ///  Only present if the operation completed successfully.
    #[prost(string, tag="1")]
    pub output_url: ::prost::alloc::string::String,
}
///  Metadata for ExportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesMetadata {
    ///  Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag="1")]
    pub common: ::core::option::Option<CommonMetadata>,
    ///  An estimate of the number of entities processed.
    #[prost(message, optional, tag="2")]
    pub progress_entities: ::core::option::Option<Progress>,
    ///  An estimate of the number of bytes processed.
    #[prost(message, optional, tag="3")]
    pub progress_bytes: ::core::option::Option<Progress>,
    ///  Description of which entities are being exported.
    #[prost(message, optional, tag="4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    ///  Location for the export metadata and data files. This will be the same
    ///  value as the
    ///  \[google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix][google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix\]
    ///  field. The final output location is provided in
    ///  \[google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url\].
    #[prost(string, tag="5")]
    pub output_url_prefix: ::prost::alloc::string::String,
}
///  Metadata for ImportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesMetadata {
    ///  Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag="1")]
    pub common: ::core::option::Option<CommonMetadata>,
    ///  An estimate of the number of entities processed.
    #[prost(message, optional, tag="2")]
    pub progress_entities: ::core::option::Option<Progress>,
    ///  An estimate of the number of bytes processed.
    #[prost(message, optional, tag="3")]
    pub progress_bytes: ::core::option::Option<Progress>,
    ///  Description of which entities are being imported.
    #[prost(message, optional, tag="4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    ///  The location of the import metadata file. This will be the same value as
    ///  the
    ///  \[google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url\]
    ///  field.
    #[prost(string, tag="5")]
    pub input_url: ::prost::alloc::string::String,
}
///  Identifies a subset of entities in a project. This is specified as
///  combinations of kinds and namespaces (either or both of which may be all, as
///  described in the following examples).
///  Example usage:
///
///  Entire project:
///    kinds=[], namespace_ids=[]
///
///  Kinds Foo and Bar in all namespaces:
///    kinds=['Foo', 'Bar'], namespace_ids=[]
///
///  Kinds Foo and Bar only in the default namespace:
///    kinds=['Foo', 'Bar'], namespace_ids=\[''\]
///
///  Kinds Foo and Bar in both the default and Baz namespaces:
///    kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz']
///
///  The entire Baz namespace:
///    kinds=[], namespace_ids=\['Baz'\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityFilter {
    ///  If empty, then this represents all kinds.
    #[prost(string, repeated, tag="1")]
    pub kinds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  An empty list represents all namespaces. This is the preferred
    ///  usage for projects that don't use namespaces.
    ///
    ///  An empty string element represents the default namespace. This should be
    ///  used if the project has data in non-default namespaces, but doesn't want to
    ///  include them.
    ///  Each namespace in this list must be unique.
    #[prost(string, repeated, tag="2")]
    pub namespace_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Operation types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    ///  Unspecified.
    Unspecified = 0,
    ///  ExportEntities.
    ExportEntities = 1,
    ///  ImportEntities.
    ImportEntities = 2,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
            OperationType::ExportEntities => "EXPORT_ENTITIES",
            OperationType::ImportEntities => "IMPORT_ENTITIES",
        }
    }
}
/// Generated client implementations.
pub mod datastore_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Cloud Datastore Admin API
    ///
    /// The Datastore Admin API provides several admin services for Cloud Datastore.
    ///
    /// -----------------------------------------------------------------------------
    /// ## Concepts
    ///
    /// Project, namespace, kind, and entity as defined in the Google Cloud Datastore
    /// API.
    ///
    /// Operation: An Operation represents work being performed in the background.
    ///
    /// EntityFilter: Allows specifying a subset of entities in a project. This is
    /// specified as a combination of kinds and namespaces (either or both of which
    /// may be all).
    ///
    /// -----------------------------------------------------------------------------
    /// ## Services
    ///
    /// # Export/Import
    ///
    /// The Export/Import service provides the ability to copy all or a subset of
    /// entities to/from Google Cloud Storage.
    ///
    /// Exported data may be imported into Cloud Datastore for any Google Cloud
    /// Platform project. It is not restricted to the export source project. It is
    /// possible to export from one project and then import into another.
    ///
    /// Exported data can also be loaded into Google BigQuery for analysis.
    ///
    /// Exports and imports are performed asynchronously. An Operation resource is
    /// created for each export/import. The state (including any errors encountered)
    /// of the export/import may be queried via the Operation resource.
    ///
    /// # Operation
    ///
    /// The Operations collection provides a record of actions performed for the
    /// specified project (including any operations in progress). Operations are not
    /// created directly but through calls on other collections or resources.
    ///
    /// An operation that is not yet done may be cancelled. The request to cancel is
    /// asynchronous and the operation may continue to run for some time after the
    /// request to cancel is made.
    ///
    /// An operation that is done may be deleted so that it is no longer listed as
    /// part of the Operation collection.
    ///
    /// ListOperations returns all pending operations, but not completed operations.
    ///
    /// Operations are created by service DatastoreAdmin,
    /// but are accessed via service google.longrunning.Operations.
    #[derive(Debug, Clone)]
    pub struct DatastoreAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatastoreAdminClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatastoreAdminClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DatastoreAdminClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DatastoreAdminClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Exports a copy of all or a subset of entities from Google Cloud Datastore
        /// to another storage system, such as Google Cloud Storage. Recent updates to
        /// entities may not be reflected in the export. The export occurs in the
        /// background and its progress can be monitored and managed via the
        /// Operation resource that is created. The output of an export may only be
        /// used once the associated operation is done. If an export operation is
        /// cancelled before completion it may leave partial data behind in Google
        /// Cloud Storage.
        pub async fn export_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ExportEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Imports entities into Google Cloud Datastore. Existing entities with the
        /// same key are overwritten. The import occurs in the background and its
        /// progress can be monitored and managed via the Operation resource that is
        /// created. If an ImportEntities operation is cancelled, it is possible
        /// that a subset of the data has already been imported to Cloud Datastore.
        pub async fn import_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ImportEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}