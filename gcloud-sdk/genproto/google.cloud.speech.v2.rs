/// Request message for the
/// \[CreateRecognizer][google.cloud.speech.v2.Speech.CreateRecognizer\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRecognizerRequest {
    /// Required. The Recognizer to create.
    #[prost(message, optional, tag="1")]
    pub recognizer: ::core::option::Option<Recognizer>,
    /// If set, validate the request and preview the Recognizer, but do not
    /// actually create it.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// The ID to use for the Recognizer, which will become the final component of
    /// the Recognizer's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag="3")]
    pub recognizer_id: ::prost::alloc::string::String,
    /// Required. The project and location where this Recognizer will be created.
    /// The expected format is `projects/{project}/locations/{location}`.
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
}
/// Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation was last updated.
    #[prost(message, optional, tag="2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub resource: ::prost::alloc::string::String,
    /// The method that triggered the operation.
    #[prost(string, tag="4")]
    pub method: ::prost::alloc::string::String,
    /// The [KMS key
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#keys>) with which
    /// the content of the Operation is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    #[prost(string, tag="6")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// The [KMS key version
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#key_versions>)
    /// with which content of the Operation is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`.
    #[prost(string, tag="7")]
    pub kms_key_version_name: ::prost::alloc::string::String,
    /// The percent progress of the Operation. Values can range from 0-100. If the
    /// value is 100, then the operation is finished.
    #[prost(int32, tag="22")]
    pub progress_percent: i32,
    /// The request that spawned the Operation.
    #[prost(oneof="operation_metadata::Request", tags="8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21")]
    pub request: ::core::option::Option<operation_metadata::Request>,
    /// Specific metadata per RPC
    #[prost(oneof="operation_metadata::Metadata", tags="23")]
    pub metadata: ::core::option::Option<operation_metadata::Metadata>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// The request that spawned the Operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// The BatchRecognizeRequest that spawned the Operation.
        #[prost(message, tag="8")]
        BatchRecognizeRequest(super::BatchRecognizeRequest),
        /// The CreateRecognizerRequest that spawned the Operation.
        #[prost(message, tag="9")]
        CreateRecognizerRequest(super::CreateRecognizerRequest),
        /// The UpdateRecognizerRequest that spawned the Operation.
        #[prost(message, tag="10")]
        UpdateRecognizerRequest(super::UpdateRecognizerRequest),
        /// The DeleteRecognizerRequest that spawned the Operation.
        #[prost(message, tag="11")]
        DeleteRecognizerRequest(super::DeleteRecognizerRequest),
        /// The UndeleteRecognizerRequest that spawned the Operation.
        #[prost(message, tag="12")]
        UndeleteRecognizerRequest(super::UndeleteRecognizerRequest),
        /// The CreateCustomClassRequest that spawned the Operation.
        #[prost(message, tag="13")]
        CreateCustomClassRequest(super::CreateCustomClassRequest),
        /// The UpdateCustomClassRequest that spawned the Operation.
        #[prost(message, tag="14")]
        UpdateCustomClassRequest(super::UpdateCustomClassRequest),
        /// The DeleteCustomClassRequest that spawned the Operation.
        #[prost(message, tag="15")]
        DeleteCustomClassRequest(super::DeleteCustomClassRequest),
        /// The UndeleteCustomClassRequest that spawned the Operation.
        #[prost(message, tag="16")]
        UndeleteCustomClassRequest(super::UndeleteCustomClassRequest),
        /// The CreatePhraseSetRequest that spawned the Operation.
        #[prost(message, tag="17")]
        CreatePhraseSetRequest(super::CreatePhraseSetRequest),
        /// The UpdatePhraseSetRequest that spawned the Operation.
        #[prost(message, tag="18")]
        UpdatePhraseSetRequest(super::UpdatePhraseSetRequest),
        /// The DeletePhraseSetRequest that spawned the Operation.
        #[prost(message, tag="19")]
        DeletePhraseSetRequest(super::DeletePhraseSetRequest),
        /// The UndeletePhraseSetRequest that spawned the Operation.
        #[prost(message, tag="20")]
        UndeletePhraseSetRequest(super::UndeletePhraseSetRequest),
        /// The UpdateConfigRequest that spawned the Operation.
        #[prost(message, tag="21")]
        UpdateConfigRequest(super::UpdateConfigRequest),
    }
    /// Specific metadata per RPC
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Metadata specific to the BatchRecognize method.
        #[prost(message, tag="23")]
        BatchRecognizeMetadata(super::BatchRecognizeMetadata),
    }
}
/// Request message for the
/// \[ListRecognizers][google.cloud.speech.v2.Speech.ListRecognizers\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecognizersRequest {
    /// Required. The project and location of Recognizers to list. The expected
    /// format is `projects/{project}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of Recognizers to return. The service may return fewer
    /// than this value. If unspecified, at most 20 Recognizers will be returned.
    /// The maximum value is 20; values above 20 will be coerced to 20.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// \[ListRecognizers][google.cloud.speech.v2.Speech.ListRecognizers\] call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// \[ListRecognizers][google.cloud.speech.v2.Speech.ListRecognizers\] must match
    /// the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether, or not, to show resources that have been deleted.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message for the
/// \[ListRecognizers][google.cloud.speech.v2.Speech.ListRecognizers\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecognizersResponse {
    /// The list of requested Recognizers.
    #[prost(message, repeated, tag="1")]
    pub recognizers: ::prost::alloc::vec::Vec<Recognizer>,
    /// A token, which can be sent as
    /// \[page_token][google.cloud.speech.v2.ListRecognizersRequest.page_token\] to
    /// retrieve the next page. If this field is omitted, there are no subsequent
    /// pages. This token expires after 72 hours.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the
/// \[GetRecognizer][google.cloud.speech.v2.Speech.GetRecognizer\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecognizerRequest {
    /// Required. The name of the Recognizer to retrieve. The expected format is
    /// `projects/{project}/locations/{location}/recognizers/{recognizer}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UpdateRecognizer][google.cloud.speech.v2.Speech.UpdateRecognizer\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRecognizerRequest {
    /// Required. The Recognizer to update.
    ///
    /// The Recognizer's `name` field is used to identify the Recognizer to update.
    /// Format: `projects/{project}/locations/{location}/recognizers/{recognizer}`.
    #[prost(message, optional, tag="1")]
    pub recognizer: ::core::option::Option<Recognizer>,
    /// The list of fields to update. If empty, all non-default valued fields are
    /// considered for update. Use `*` to update the entire Recognizer resource.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set, validate the request and preview the updated Recognizer, but do not
    /// actually update it.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Request message for the
/// \[DeleteRecognizer][google.cloud.speech.v2.Speech.DeleteRecognizer\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRecognizerRequest {
    /// Required. The name of the Recognizer to delete.
    /// Format: `projects/{project}/locations/{location}/recognizers/{recognizer}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the deleted Recognizer, but do not
    /// actually delete it.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// If set to true, and the Recognizer is not found, the request will succeed
    /// and  be a no-op (no Operation is recorded in this case).
    #[prost(bool, tag="4")]
    pub allow_missing: bool,
    /// This checksum is computed by the server based on the value of other
    /// fields. This may be sent on update, undelete, and delete requests to ensure
    /// the client has an up-to-date value before proceeding.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UndeleteRecognizer][google.cloud.speech.v2.Speech.UndeleteRecognizer\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteRecognizerRequest {
    /// Required. The name of the Recognizer to undelete.
    /// Format: `projects/{project}/locations/{location}/recognizers/{recognizer}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the undeleted Recognizer, but do
    /// not actually undelete it.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
    /// This checksum is computed by the server based on the value of other
    /// fields. This may be sent on update, undelete, and delete requests to ensure
    /// the client has an up-to-date value before proceeding.
    #[prost(string, tag="4")]
    pub etag: ::prost::alloc::string::String,
}
/// A Recognizer message. Stores recognition configuration and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recognizer {
    /// Output only. The resource name of the Recognizer.
    /// Format: `projects/{project}/locations/{location}/recognizers/{recognizer}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System-assigned unique identifier for the Recognizer.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// User-settable, human-readable name for the Recognizer. Must be 63
    /// characters or less.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Which model to use for recognition requests. Select the model
    /// best suited to your domain to get best results.
    ///
    /// Supported models:
    ///
    /// - `latest_long`
    ///
    ///    Best for long form content like media or conversation.
    ///
    /// - `latest_short`
    ///
    ///    Best for short form content like commands or single shot directed speech.
    ///    When using this model, the service will stop transcribing audio after the
    ///    first utterance is detected and completed.
    ///
    /// When using this model,
    /// \[SEPARATE_RECOGNITION_PER_CHANNEL][google.cloud.speech.v2.RecognitionFeatures.MultiChannelMode.SEPARATE_RECOGNITION_PER_CHANNEL\]
    /// is not supported; multi-channel audio is accepted, but only the first
    /// channel will be processed and transcribed.
    #[prost(string, tag="4")]
    pub model: ::prost::alloc::string::String,
    /// Required. The language of the supplied audio as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag.
    ///
    /// Supported languages:
    ///
    /// - `en-US`
    ///
    /// - `en-GB`
    ///
    /// - `fr-FR`
    ///
    /// If additional languages are provided, recognition result will contain
    /// recognition in the most likely language detected. The recognition result
    /// will include the language tag of the language detected in the audio.
    /// When you create or update a Recognizer, these values are
    /// stored in normalized BCP-47 form. For example, "en-us" is stored as
    /// "en-US".
    #[prost(string, repeated, tag="17")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Default configuration to use for requests with this Recognizer.
    /// This can be overwritten by inline configuration in the
    /// \[RecognizeRequest.config][google.cloud.speech.v2.RecognizeRequest.config\]
    /// field.
    #[prost(message, optional, tag="6")]
    pub default_recognition_config: ::core::option::Option<RecognitionConfig>,
    /// Allows users to store small amounts of arbitrary data.
    /// Both the key and the value must be 63 characters or less each.
    /// At most 100 annotations.
    #[prost(map="string, string", tag="7")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The Recognizer lifecycle state.
    #[prost(enumeration="recognizer::State", tag="8")]
    pub state: i32,
    /// Output only. Creation time.
    #[prost(message, optional, tag="9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time this Recognizer was modified.
    #[prost(message, optional, tag="10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this Recognizer was requested for deletion.
    #[prost(message, optional, tag="11")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this Recognizer will be purged.
    #[prost(message, optional, tag="14")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. This checksum is computed by the server based on the value of
    /// other fields. This may be sent on update, undelete, and delete requests to
    /// ensure the client has an up-to-date value before proceeding.
    #[prost(string, tag="12")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Whether or not this Recognizer is in the process of being
    /// updated.
    #[prost(bool, tag="13")]
    pub reconciling: bool,
    /// Output only. The [KMS key
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#keys>) with which
    /// the Recognizer is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    #[prost(string, tag="15")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Output only. The [KMS key version
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#key_versions>)
    /// with which the Recognizer is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`.
    #[prost(string, tag="16")]
    pub kms_key_version_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Recognizer`.
pub mod recognizer {
    /// Set of states that define the lifecycle of a Recognizer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// The Recognizer is active and ready for use.
        Active = 2,
        /// This Recognizer has been deleted.
        Deleted = 4,
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
                State::Deleted => "DELETED",
            }
        }
    }
}
/// Automatically detected decoding parameters.
/// Supported for the following encodings:
/// * WAV_LINEAR16: 16-bit signed little-endian PCM samples in a WAV container.
/// * WAV_MULAW: 8-bit companded mulaw samples in a WAV container.
/// * WAV_ALAW: 8-bit companded alaw samples in a WAV container.
/// * RFC4867_5_AMR: AMR frames with an rfc4867.5 header.
/// * RFC4867_5_AMRWB: AMR-WB frames with an rfc4867.5 header.
/// * FLAC: FLAC frames in the "native FLAC" container format.
/// * MP3: MPEG audio frames with optional (ignored) ID3 metadata.
/// * OGG_OPUS: Opus audio frames in an Ogg container.
/// * WEBM_OPUS: Opus audio frames in a WebM container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoDetectDecodingConfig {
}
/// Explicitly specified decoding parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitDecodingConfig {
    /// Required. Encoding of the audio data sent for recognition.
    #[prost(enumeration="explicit_decoding_config::AudioEncoding", tag="1")]
    pub encoding: i32,
    /// Sample rate in Hertz of the audio data sent for recognition. Valid
    /// values are: 8000-48000. 16000 is optimal. For best results, set the
    /// sampling rate of the audio source to 16000 Hz. If that's not possible, use
    /// the native sample rate of the audio source (instead of re-sampling).
    /// Supported for the following encodings:
    /// * LINEAR16: Headerless 16-bit signed little-endian PCM samples.
    /// * MULAW: Headerless 8-bit companded mulaw samples.
    /// * ALAW: Headerless 8-bit companded alaw samples.
    #[prost(int32, tag="2")]
    pub sample_rate_hertz: i32,
    /// Number of channels present in the audio data sent for recognition.
    /// Supported for the following encodings:
    /// * LINEAR16: Headerless 16-bit signed little-endian PCM samples.
    /// * MULAW: Headerless 8-bit companded mulaw samples.
    /// * ALAW: Headerless 8-bit companded alaw samples.
    #[prost(int32, tag="3")]
    pub audio_channel_count: i32,
}
/// Nested message and enum types in `ExplicitDecodingConfig`.
pub mod explicit_decoding_config {
    /// Supported audio data encodings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AudioEncoding {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Headerless 16-bit signed little-endian PCM samples.
        Linear16 = 1,
        /// Headerless 8-bit companded mulaw samples.
        Mulaw = 2,
        /// Headerless 8-bit companded alaw samples.
        Alaw = 3,
    }
    impl AudioEncoding {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AudioEncoding::Unspecified => "AUDIO_ENCODING_UNSPECIFIED",
                AudioEncoding::Linear16 => "LINEAR16",
                AudioEncoding::Mulaw => "MULAW",
                AudioEncoding::Alaw => "ALAW",
            }
        }
    }
}
/// Configuration to enable speaker diarization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeakerDiarizationConfig {
    /// Required. Minimum number of speakers in the conversation. This range gives
    /// you more flexibility by allowing the system to automatically determine the
    /// correct number of speakers. If not set, the default value is 2.
    ///
    /// To fix the number of speakers detected in the audio, set
    /// `min_speaker_count` = `max_speaker_count`.
    #[prost(int32, tag="2")]
    pub min_speaker_count: i32,
    /// Required. Maximum number of speakers in the conversation. Valid values are:
    /// 1-6. Must be >= `min_speaker_count`. This range gives you more flexibility
    /// by allowing the system to automatically determine the correct number of
    /// speakers.
    #[prost(int32, tag="3")]
    pub max_speaker_count: i32,
}
/// Available recognition features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionFeatures {
    /// If set to `true`, the server will attempt to filter out profanities,
    /// replacing all but the initial character in each filtered word with
    /// asterisks, for instance, "f***". If set to `false` or omitted, profanities
    /// won't be filtered out.
    #[prost(bool, tag="1")]
    pub profanity_filter: bool,
    /// If `true`, the top result includes a list of words and the start and end
    /// time offsets (timestamps) for those words. If `false`, no word-level time
    /// offset information is returned. The default is `false`.
    #[prost(bool, tag="2")]
    pub enable_word_time_offsets: bool,
    /// If `true`, the top result includes a list of words and the confidence for
    /// those words. If `false`, no word-level confidence information is returned.
    /// The default is `false`.
    #[prost(bool, tag="3")]
    pub enable_word_confidence: bool,
    /// If `true`, adds punctuation to recognition result hypotheses. This feature
    /// is only available in select languages. The default `false` value does not
    /// add punctuation to result hypotheses.
    #[prost(bool, tag="4")]
    pub enable_automatic_punctuation: bool,
    /// The spoken punctuation behavior for the call. If `true`, replaces spoken
    /// punctuation with the corresponding symbols in the request. For example,
    /// "how are you question mark" becomes "how are you?". See
    /// <https://cloud.google.com/speech-to-text/docs/spoken-punctuation> for
    /// support. If `false`, spoken punctuation is not replaced.
    #[prost(bool, tag="14")]
    pub enable_spoken_punctuation: bool,
    /// The spoken emoji behavior for the call. If `true`, adds spoken emoji
    /// formatting for the request. This will replace spoken emojis with the
    /// corresponding Unicode symbols in the final transcript. If `false`, spoken
    /// emojis are not replaced.
    #[prost(bool, tag="15")]
    pub enable_spoken_emojis: bool,
    /// Mode for recognizing multi-channel audio.
    #[prost(enumeration="recognition_features::MultiChannelMode", tag="17")]
    pub multi_channel_mode: i32,
    /// Configuration to enable speaker diarization and set additional
    /// parameters to make diarization better suited for your application.
    /// When this is enabled, we send all the words from the beginning of the
    /// audio for the top alternative in every consecutive STREAMING responses.
    /// This is done in order to improve our speaker tags as our models learn to
    /// identify the speakers in the conversation over time.
    /// For non-streaming requests, the diarization results will be provided only
    /// in the top alternative of the FINAL SpeechRecognitionResult.
    #[prost(message, optional, tag="9")]
    pub diarization_config: ::core::option::Option<SpeakerDiarizationConfig>,
    /// Maximum number of recognition hypotheses to be returned.
    /// The server may return fewer than `max_alternatives`.
    /// Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of
    /// one. If omitted, will return a maximum of one.
    #[prost(int32, tag="16")]
    pub max_alternatives: i32,
}
/// Nested message and enum types in `RecognitionFeatures`.
pub mod recognition_features {
    /// Options for how to recognize multi-channel audio.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MultiChannelMode {
        /// Default value for the multi-channel mode. If the audio contains
        /// multiple channels, only the first channel will be transcribed; other
        /// channels will be ignored.
        Unspecified = 0,
        /// If selected, each channel in the provided audio is transcribed
        /// independently. This cannot be selected if the selected
        /// \[model][google.cloud.speech.v2.Recognizer.model\] is `latest_short`.
        SeparateRecognitionPerChannel = 1,
    }
    impl MultiChannelMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MultiChannelMode::Unspecified => "MULTI_CHANNEL_MODE_UNSPECIFIED",
                MultiChannelMode::SeparateRecognitionPerChannel => "SEPARATE_RECOGNITION_PER_CHANNEL",
            }
        }
    }
}
/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results. Phrase sets can be specified as an inline resource, or a
/// reference to an existing phrase set resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechAdaptation {
    /// A list of inline or referenced phrase sets.
    #[prost(message, repeated, tag="1")]
    pub phrase_sets: ::prost::alloc::vec::Vec<speech_adaptation::AdaptationPhraseSet>,
    /// A list of inline custom classes. Existing custom class resources can be
    /// referenced directly in a phrase set.
    #[prost(message, repeated, tag="2")]
    pub custom_classes: ::prost::alloc::vec::Vec<CustomClass>,
}
/// Nested message and enum types in `SpeechAdaptation`.
pub mod speech_adaptation {
    /// A biasing phrase set, which can be either a string referencing the name of
    /// an existing phrase set resource, or an inline definition of a phrase set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdaptationPhraseSet {
        #[prost(oneof="adaptation_phrase_set::Value", tags="1, 2")]
        pub value: ::core::option::Option<adaptation_phrase_set::Value>,
    }
    /// Nested message and enum types in `AdaptationPhraseSet`.
    pub mod adaptation_phrase_set {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            /// The name of an existing phrase set resource. The user must have read
            /// access to the resource and it must not be deleted.
            #[prost(string, tag="1")]
            PhraseSet(::prost::alloc::string::String),
            /// An inline defined phrase set.
            #[prost(message, tag="2")]
            InlinePhraseSet(super::super::PhraseSet),
        }
    }
}
/// Provides information to the Recognizer that specifies how to process the
/// recognition request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionConfig {
    /// Speech recognition features to enable.
    #[prost(message, optional, tag="2")]
    pub features: ::core::option::Option<RecognitionFeatures>,
    /// Speech adaptation context that weights recognizer predictions for specific
    /// words and phrases.
    #[prost(message, optional, tag="6")]
    pub adaptation: ::core::option::Option<SpeechAdaptation>,
    /// Decoding parameters for audio being sent for recognition.
    #[prost(oneof="recognition_config::DecodingConfig", tags="7, 8")]
    pub decoding_config: ::core::option::Option<recognition_config::DecodingConfig>,
}
/// Nested message and enum types in `RecognitionConfig`.
pub mod recognition_config {
    /// Decoding parameters for audio being sent for recognition.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DecodingConfig {
        /// Automatically detect decoding parameters.
        /// Preferred for supported formats.
        #[prost(message, tag="7")]
        AutoDecodingConfig(super::AutoDetectDecodingConfig),
        /// Explicitly specified decoding parameters.
        /// Required if using headerless PCM audio (linear16, mulaw, alaw).
        #[prost(message, tag="8")]
        ExplicitDecodingConfig(super::ExplicitDecodingConfig),
    }
}
/// Request message for the
/// \[Recognize][google.cloud.speech.v2.Speech.Recognize\] method. Either
/// `content` or `uri` must be supplied. Supplying both or neither returns
/// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]. See [content
/// limits](<https://cloud.google.com/speech-to-text/quotas#content>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognizeRequest {
    /// Required. The name of the Recognizer to use during recognition. The
    /// expected format is
    /// `projects/{project}/locations/{location}/recognizers/{recognizer}`.
    #[prost(string, tag="3")]
    pub recognizer: ::prost::alloc::string::String,
    /// Features and audio metadata to use for the Automatic Speech Recognition.
    /// This field in combination with the
    /// \[config_mask][google.cloud.speech.v2.RecognizeRequest.config_mask\] field
    /// can be used to override parts of the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the Recognizer resource.
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// The list of fields in
    /// \[config][google.cloud.speech.v2.RecognizeRequest.config\] that override the
    /// values in the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the recognizer during this recognition request. If no mask is provided,
    /// all non-default valued fields in
    /// \[config][google.cloud.speech.v2.RecognizeRequest.config\] override the
    /// values in the recognizer for this recognition request. If a mask is
    /// provided, only the fields listed in the mask override the config in the
    /// recognizer for this recognition request. If a wildcard (`*`) is provided,
    /// \[config][google.cloud.speech.v2.RecognizeRequest.config\] completely
    /// overrides and replaces the config in the recognizer for this recognition
    /// request.
    #[prost(message, optional, tag="8")]
    pub config_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The audio source, which is either inline content or a Google Cloud
    /// Storage URI.
    #[prost(oneof="recognize_request::AudioSource", tags="5, 6")]
    pub audio_source: ::core::option::Option<recognize_request::AudioSource>,
}
/// Nested message and enum types in `RecognizeRequest`.
pub mod recognize_request {
    /// The audio source, which is either inline content or a Google Cloud
    /// Storage URI.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AudioSource {
        /// The audio data bytes encoded as specified in
        /// \[RecognitionConfig][google.cloud.speech.v2.RecognitionConfig\]. As
        /// with all bytes fields, proto buffers use a pure binary representation,
        /// whereas JSON representations use base64.
        #[prost(bytes, tag="5")]
        Content(::prost::alloc::vec::Vec<u8>),
        /// URI that points to a file that contains audio data bytes as specified in
        /// \[RecognitionConfig][google.cloud.speech.v2.RecognitionConfig\]. The file
        /// must not be compressed (for example, gzip). Currently, only Google Cloud
        /// Storage URIs are supported, which must be specified in the following
        /// format: `gs://bucket_name/object_name` (other URI formats return
        /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]). For more
        /// information, see [Request
        /// URIs](<https://cloud.google.com/storage/docs/reference-uris>).
        #[prost(string, tag="6")]
        Uri(::prost::alloc::string::String),
    }
}
/// Metadata about the recognition request and response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionResponseMetadata {
    /// When available, billed audio seconds for the corresponding request.
    #[prost(message, optional, tag="6")]
    pub total_billed_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Alternative hypotheses (a.k.a. n-best list).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechRecognitionAlternative {
    /// Transcript text representing the words that the user spoke.
    #[prost(string, tag="1")]
    pub transcript: ::prost::alloc::string::String,
    /// The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative of a non-streaming
    /// result or, of a streaming result where
    /// \[is_final][google.cloud.speech.v2.StreamingRecognitionResult.is_final\] is
    /// set to `true`. This field is not guaranteed to be accurate and users should
    /// not rely on it to be always provided. The default of 0.0 is a sentinel
    /// value indicating `confidence` was not set.
    #[prost(float, tag="2")]
    pub confidence: f32,
    /// A list of word-specific information for each recognized word.
    /// When
    /// \[enable_speaker_diarization][google.cloud.speech.v2.SpeakerDiarizationConfig.enable_speaker_diarization\]
    /// is true, you will see all the words from the beginning of the audio.
    #[prost(message, repeated, tag="3")]
    pub words: ::prost::alloc::vec::Vec<WordInfo>,
}
/// Word-specific information for recognized words.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordInfo {
    /// Time offset relative to the beginning of the audio,
    /// and corresponding to the start of the spoken word.
    /// This field is only set if
    /// \[enable_word_time_offsets][google.cloud.speech.v2.RecognitionFeatures.enable_word_time_offsets\]
    /// is `true` and only in the top hypothesis. This is an experimental feature
    /// and the accuracy of the time offset can vary.
    #[prost(message, optional, tag="1")]
    pub start_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio,
    /// and corresponding to the end of the spoken word.
    /// This field is only set if
    /// \[enable_word_time_offsets][google.cloud.speech.v2.RecognitionFeatures.enable_word_time_offsets\]
    /// is `true` and only in the top hypothesis. This is an experimental feature
    /// and the accuracy of the time offset can vary.
    #[prost(message, optional, tag="2")]
    pub end_offset: ::core::option::Option<::prost_types::Duration>,
    /// The word corresponding to this set of information.
    #[prost(string, tag="3")]
    pub word: ::prost::alloc::string::String,
    /// The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative of a non-streaming
    /// result or, of a streaming result where
    /// \[is_final][google.cloud.speech.v2.StreamingRecognitionResult.is_final\] is
    /// set to `true`. This field is not guaranteed to be accurate and users should
    /// not rely on it to be always provided. The default of 0.0 is a sentinel
    /// value indicating `confidence` was not set.
    #[prost(float, tag="4")]
    pub confidence: f32,
    /// A distinct label is assigned for every speaker within the audio. This field
    /// specifies which one of those speakers was detected to have spoken this
    /// word. `speaker_label` is set if
    /// \[enable_speaker_diarization][google.cloud.speech.v2.SpeakerDiarizationConfig.enable_speaker_diarization\]
    /// is `true` and only in the top alternative.
    #[prost(string, tag="6")]
    pub speaker_label: ::prost::alloc::string::String,
}
/// A speech recognition result corresponding to a portion of the audio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechRecognitionResult {
    /// May contain one or more recognition hypotheses. These alternatives are
    /// ordered in terms of accuracy, with the top (first) alternative being the
    /// most probable, as ranked by the recognizer.
    #[prost(message, repeated, tag="1")]
    pub alternatives: ::prost::alloc::vec::Vec<SpeechRecognitionAlternative>,
    /// For multi-channel audio, this is the channel number corresponding to the
    /// recognized result for the audio from that channel.
    /// For `audio_channel_count` = `N`, its output values can range from `1` to
    /// `N`.
    #[prost(int32, tag="2")]
    pub channel_tag: i32,
    /// Time offset of the end of this result relative to the beginning of the
    /// audio.
    #[prost(message, optional, tag="4")]
    pub result_end_offset: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>)
    /// language tag of the language in this result. This language code was
    /// detected to have the most likelihood of being spoken in the audio.
    #[prost(string, tag="5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for the
/// \[Recognize][google.cloud.speech.v2.Speech.Recognize\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognizeResponse {
    /// Sequential list of transcription results corresponding to sequential
    /// portions of audio.
    #[prost(message, repeated, tag="3")]
    pub results: ::prost::alloc::vec::Vec<SpeechRecognitionResult>,
    /// Metadata about the recognition.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<RecognitionResponseMetadata>,
}
/// Available recognition features specific to streaming recognition requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionFeatures {
    /// If `true`, responses with voice activity speech events will be returned as
    /// they are detected.
    #[prost(bool, tag="1")]
    pub enable_voice_activity_events: bool,
    /// Whether or not to stream interim results to the client. If set to true,
    /// interim results will be streamed to the client. Otherwise, only the final
    /// response will be streamed back.
    #[prost(bool, tag="2")]
    pub interim_results: bool,
    /// If set, the server will automatically close the stream after the specified
    /// duration has elapsed after the last VOICE_ACTIVITY speech event has been
    /// sent. The field `voice_activity_events` must also be set to true.
    #[prost(message, optional, tag="3")]
    pub voice_activity_timeout: ::core::option::Option<streaming_recognition_features::VoiceActivityTimeout>,
}
/// Nested message and enum types in `StreamingRecognitionFeatures`.
pub mod streaming_recognition_features {
    /// Events that a timeout can be set on for voice activity.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VoiceActivityTimeout {
        /// Duration to timeout the stream if no speech begins. If this is set and
        /// no speech is detected in this duration at the start of the stream, the
        /// server will close the stream.
        #[prost(message, optional, tag="1")]
        pub speech_start_timeout: ::core::option::Option<::prost_types::Duration>,
        /// Duration to timeout the stream after speech ends. If this is set and no
        /// speech is detected in this duration after speech was detected, the server
        /// will close the stream.
        #[prost(message, optional, tag="2")]
        pub speech_end_timeout: ::core::option::Option<::prost_types::Duration>,
    }
}
/// Provides configuration information for the StreamingRecognize request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionConfig {
    /// Required. Features and audio metadata to use for the Automatic Speech
    /// Recognition. This field in combination with the
    /// \[config_mask][google.cloud.speech.v2.StreamingRecognitionConfig.config_mask\]
    /// field can be used to override parts of the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the Recognizer resource.
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// The list of fields in
    /// \[config][google.cloud.speech.v2.StreamingRecognitionConfig.config\] that
    /// override the values in the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the recognizer during this recognition request. If no mask is provided,
    /// all non-default valued fields in
    /// \[config][google.cloud.speech.v2.StreamingRecognitionConfig.config\] override
    /// the values in the recognizer for this recognition request. If a mask is
    /// provided, only the fields listed in the mask override the config in the
    /// recognizer for this recognition request. If a wildcard (`*`) is provided,
    /// \[config][google.cloud.speech.v2.StreamingRecognitionConfig.config\]
    /// completely overrides and replaces the config in the recognizer for this
    /// recognition request.
    #[prost(message, optional, tag="3")]
    pub config_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Speech recognition features to enable specific to streaming audio
    /// recognition requests.
    #[prost(message, optional, tag="2")]
    pub streaming_features: ::core::option::Option<StreamingRecognitionFeatures>,
}
/// Request message for the
/// \[StreamingRecognize][google.cloud.speech.v2.Speech.StreamingRecognize\]
/// method. Multiple
/// \[StreamingRecognizeRequest][google.cloud.speech.v2.StreamingRecognizeRequest\]
/// messages are sent. The first message must contain a
/// \[recognizer][google.cloud.speech.v2.StreamingRecognizeRequest.recognizer\] and
/// optionally a
/// \[streaming_config][google.cloud.speech.v2.StreamingRecognizeRequest.streaming_config\]
/// message and must not contain
/// \[audio][google.cloud.speech.v2.StreamingRecognizeRequest.audio\]. All
/// subsequent messages must contain
/// \[audio][google.cloud.speech.v2.StreamingRecognizeRequest.audio\] and must not
/// contain a
/// \[streaming_config][google.cloud.speech.v2.StreamingRecognizeRequest.streaming_config\]
/// message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognizeRequest {
    /// Required. Streaming recognition should start with an initial request having
    /// a `recognizer`. Subsequent requests carry the audio data to be recognized.
    ///
    /// The initial request with configuration can be omitted if the Recognizer
    /// being used has a
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\].
    #[prost(string, tag="3")]
    pub recognizer: ::prost::alloc::string::String,
    #[prost(oneof="streaming_recognize_request::StreamingRequest", tags="6, 5")]
    pub streaming_request: ::core::option::Option<streaming_recognize_request::StreamingRequest>,
}
/// Nested message and enum types in `StreamingRecognizeRequest`.
pub mod streaming_recognize_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamingRequest {
        /// StreamingRecognitionConfig to be used in this recognition attempt.
        /// If provided, it will override the default RecognitionConfig stored in the
        /// Recognizer.
        #[prost(message, tag="6")]
        StreamingConfig(super::StreamingRecognitionConfig),
        /// Inline audio bytes to be Recognized.
        #[prost(bytes, tag="5")]
        Audio(::prost::alloc::vec::Vec<u8>),
    }
}
/// Request message for the
/// \[BatchRecognize][google.cloud.speech.v2.Speech.BatchRecognize\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRecognizeRequest {
    /// Required. Resource name of the recognizer to be used for ASR.
    #[prost(string, tag="1")]
    pub recognizer: ::prost::alloc::string::String,
    /// Features and audio metadata to use for the Automatic Speech Recognition.
    /// This field in combination with the
    /// \[config_mask][google.cloud.speech.v2.BatchRecognizeRequest.config_mask\]
    /// field can be used to override parts of the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the Recognizer resource.
    #[prost(message, optional, tag="4")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// The list of fields in
    /// \[config][google.cloud.speech.v2.BatchRecognizeRequest.config\] that override
    /// the values in the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the recognizer during this recognition request. If no mask is provided,
    /// all given fields in
    /// \[config][google.cloud.speech.v2.BatchRecognizeRequest.config\] override the
    /// values in the recognizer for this recognition request. If a mask is
    /// provided, only the fields listed in the mask override the config in the
    /// recognizer for this recognition request. If a wildcard (`*`) is provided,
    /// \[config][google.cloud.speech.v2.BatchRecognizeRequest.config\] completely
    /// overrides and replaces the config in the recognizer for this recognition
    /// request.
    #[prost(message, optional, tag="5")]
    pub config_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Audio files with file metadata for ASR.
    #[prost(message, repeated, tag="3")]
    pub files: ::prost::alloc::vec::Vec<BatchRecognizeFileMetadata>,
}
/// Response message for
/// \[BatchRecognize][google.cloud.speech.v2.Speech.BatchRecognize\] that is
/// packaged into a longrunning \[Operation][google.longrunning.Operation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRecognizeResponse {
    /// Map from filename to the final result for that file.
    #[prost(map="string, message", tag="1")]
    pub results: ::std::collections::HashMap<::prost::alloc::string::String, BatchRecognizeFileResult>,
}
/// Final results for a single file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRecognizeFileResult {
    /// The GCS URI to which recognition results were written.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    /// Error if one was encountered.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Metadata about transcription for a single file (for example, progress
/// percent).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRecognizeTranscriptionMetadata {
    /// How much of the file has been transcribed so far.
    #[prost(int32, tag="1")]
    pub progress_percent: i32,
    /// Error if one was encountered.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The GCS URI to which recognition results will be written.
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
}
/// Operation metadata for
/// \[BatchRecognize][google.cloud.speech.v2.Speech.BatchRecognize\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRecognizeMetadata {
    /// Map from provided filename to the transcription metadata for that file.
    #[prost(map="string, message", tag="1")]
    pub transcription_metadata: ::std::collections::HashMap<::prost::alloc::string::String, BatchRecognizeTranscriptionMetadata>,
}
/// Metadata about a single file in a batch for BatchRecognize.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRecognizeFileMetadata {
    /// Features and audio metadata to use for the Automatic Speech Recognition.
    /// This field in combination with the
    /// \[config_mask][google.cloud.speech.v2.BatchRecognizeFileMetadata.config_mask\]
    /// field can be used to override parts of the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the Recognizer resource as well as the
    /// \[config][google.cloud.speech.v2.BatchRecognizeRequest.config\] at the
    /// request level.
    #[prost(message, optional, tag="4")]
    pub config: ::core::option::Option<RecognitionConfig>,
    /// The list of fields in
    /// \[config][google.cloud.speech.v2.BatchRecognizeFileMetadata.config\] that
    /// override the values in the
    /// \[default_recognition_config][google.cloud.speech.v2.Recognizer.default_recognition_config\]
    /// of the recognizer during this recognition request. If no mask is provided,
    /// all non-default valued fields in
    /// \[config][google.cloud.speech.v2.BatchRecognizeFileMetadata.config\] override
    /// the values in the recognizer for this recognition request. If a mask is
    /// provided, only the fields listed in the mask override the config in the
    /// recognizer for this recognition request. If a wildcard (`*`) is provided,
    /// \[config][google.cloud.speech.v2.BatchRecognizeFileMetadata.config\]
    /// completely overrides and replaces the config in the recognizer for this
    /// recognition request.
    #[prost(message, optional, tag="5")]
    pub config_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The audio source, which is a Google Cloud Storage URI.
    #[prost(oneof="batch_recognize_file_metadata::AudioSource", tags="1")]
    pub audio_source: ::core::option::Option<batch_recognize_file_metadata::AudioSource>,
}
/// Nested message and enum types in `BatchRecognizeFileMetadata`.
pub mod batch_recognize_file_metadata {
    /// The audio source, which is a Google Cloud Storage URI.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AudioSource {
        /// Cloud Storage URI for the audio file.
        #[prost(string, tag="1")]
        Uri(::prost::alloc::string::String),
    }
}
/// A streaming speech recognition result corresponding to a portion of the audio
/// that is currently being processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionResult {
    /// May contain one or more recognition hypotheses. These alternatives are
    /// ordered in terms of accuracy, with the top (first) alternative being the
    /// most probable, as ranked by the recognizer.
    #[prost(message, repeated, tag="1")]
    pub alternatives: ::prost::alloc::vec::Vec<SpeechRecognitionAlternative>,
    /// If `false`, this
    /// \[StreamingRecognitionResult][google.cloud.speech.v2.StreamingRecognitionResult\]
    /// represents an interim result that may change. If `true`, this is the final
    /// time the speech service will return this particular
    /// \[StreamingRecognitionResult][google.cloud.speech.v2.StreamingRecognitionResult\],
    /// the recognizer will not return any further hypotheses for this portion of
    /// the transcript and corresponding audio.
    #[prost(bool, tag="2")]
    pub is_final: bool,
    /// An estimate of the likelihood that the recognizer will not change its guess
    /// about this interim result. Values range from 0.0 (completely unstable)
    /// to 1.0 (completely stable). This field is only provided for interim results
    /// (\[is_final][google.cloud.speech.v2.StreamingRecognitionResult.is_final\]=`false`).
    /// The default of 0.0 is a sentinel value indicating `stability` was not set.
    #[prost(float, tag="3")]
    pub stability: f32,
    /// Time offset of the end of this result relative to the beginning of the
    /// audio.
    #[prost(message, optional, tag="4")]
    pub result_end_offset: ::core::option::Option<::prost_types::Duration>,
    /// For multi-channel audio, this is the channel number corresponding to the
    /// recognized result for the audio from that channel.
    /// For
    /// `audio_channel_count` = `N`, its output values can range from `1` to `N`.
    #[prost(int32, tag="5")]
    pub channel_tag: i32,
    /// Output only. The \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>)
    /// language tag of the language in this result. This language code was
    /// detected to have the most likelihood of being spoken in the audio.
    #[prost(string, tag="6")]
    pub language_code: ::prost::alloc::string::String,
}
/// `StreamingRecognizeResponse` is the only message returned to the client by
/// `StreamingRecognize`. A series of zero or more `StreamingRecognizeResponse`
/// messages are streamed back to the client. If there is no recognizable
/// audio then no messages are streamed back to the client.
///
/// Here are some examples of `StreamingRecognizeResponse`s that might
/// be returned while processing audio:
///
/// 1. results { alternatives { transcript: "tube" } stability: 0.01 }
///
/// 2. results { alternatives { transcript: "to be a" } stability: 0.01 }
///
/// 3. results { alternatives { transcript: "to be" } stability: 0.9 }
///     results { alternatives { transcript: " or not to be" } stability: 0.01 }
///
/// 4. results { alternatives { transcript: "to be or not to be"
///                              confidence: 0.92 }
///               alternatives { transcript: "to bee or not to bee" }
///               is_final: true }
///
/// 5. results { alternatives { transcript: " that's" } stability: 0.01 }
///
/// 6. results { alternatives { transcript: " that is" } stability: 0.9 }
///     results { alternatives { transcript: " the question" } stability: 0.01 }
///
/// 7. results { alternatives { transcript: " that is the question"
///                              confidence: 0.98 }
///               alternatives { transcript: " that was the question" }
///               is_final: true }
///
/// Notes:
///
/// - Only two of the above responses #4 and #7 contain final results; they are
///    indicated by `is_final: true`. Concatenating these together generates the
///    full transcript: "to be or not to be that is the question".
///
/// - The others contain interim `results`. #3 and #6 contain two interim
///    `results`: the first portion has a high stability and is less likely to
///    change; the second portion has a low stability and is very likely to
///    change. A UI designer might choose to show only high stability `results`.
///
/// - The specific `stability` and `confidence` values shown above are only for
///    illustrative purposes. Actual values may vary.
///
/// - In each response, only one of these fields will be set:
///      `error`,
///      `speech_event_type`, or
///      one or more (repeated) `results`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognizeResponse {
    /// This repeated list contains zero or more results that
    /// correspond to consecutive portions of the audio currently being processed.
    /// It contains zero or one
    /// \[is_final][google.cloud.speech.v2.StreamingRecognitionResult.is_final\]=`true`
    /// result (the newly settled portion), followed by zero or more
    /// \[is_final][google.cloud.speech.v2.StreamingRecognitionResult.is_final\]=`false`
    /// results (the interim results).
    #[prost(message, repeated, tag="6")]
    pub results: ::prost::alloc::vec::Vec<StreamingRecognitionResult>,
    /// Indicates the type of speech event.
    #[prost(enumeration="streaming_recognize_response::SpeechEventType", tag="3")]
    pub speech_event_type: i32,
    /// Time offset between the beginning of the audio and event emission.
    #[prost(message, optional, tag="7")]
    pub speech_event_offset: ::core::option::Option<::prost_types::Duration>,
    /// Metadata about the recognition.
    #[prost(message, optional, tag="5")]
    pub metadata: ::core::option::Option<RecognitionResponseMetadata>,
}
/// Nested message and enum types in `StreamingRecognizeResponse`.
pub mod streaming_recognize_response {
    /// Indicates the type of speech event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpeechEventType {
        /// No speech event specified.
        Unspecified = 0,
        /// This event indicates that the server has detected the end of the user's
        /// speech utterance and expects no additional speech. Therefore, the server
        /// will not process additional audio and will close the gRPC bidirectional
        /// stream. This event is only sent if there was a force cutoff due to
        /// silence being detected early. This event is only available through the
        /// `latest_short` \[model][google.cloud.speech.v2.Recognizer.model\].
        EndOfSingleUtterance = 1,
        /// This event indicates that the server has detected the beginning of human
        /// voice activity in the stream. This event can be returned multiple times
        /// if speech starts and stops repeatedly throughout the stream. This event
        /// is only sent if `voice_activity_events` is set to true.
        SpeechActivityBegin = 2,
        /// This event indicates that the server has detected the end of human voice
        /// activity in the stream. This event can be returned multiple times if
        /// speech starts and stops repeatedly throughout the stream. This event is
        /// only sent if `voice_activity_events` is set to true.
        SpeechActivityEnd = 3,
    }
    impl SpeechEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SpeechEventType::Unspecified => "SPEECH_EVENT_TYPE_UNSPECIFIED",
                SpeechEventType::EndOfSingleUtterance => "END_OF_SINGLE_UTTERANCE",
                SpeechEventType::SpeechActivityBegin => "SPEECH_ACTIVITY_BEGIN",
                SpeechEventType::SpeechActivityEnd => "SPEECH_ACTIVITY_END",
            }
        }
    }
}
/// Message representing the config for the Speech-to-Text API. This includes an
/// optional [KMS key](<https://cloud.google.com/kms/docs/resource-hierarchy#keys>)
/// with which incoming data will be encrypted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Output only. The name of the config resource. There is exactly one config
    /// resource per project per location. The expected format is
    /// `projects/{project}/locations/{location}/config`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional [KMS key
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#keys>) that if
    /// present, will be used to encrypt Speech-to-Text resources at-rest. Updating
    /// this key will not encrypt existing resources using this key; only new
    /// resources will be encrypted using this key. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    #[prost(string, tag="2")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Output only. The most recent time this resource was modified.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for the
/// \[GetConfig][google.cloud.speech.v2.Speech.GetConfig\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {
    /// Required. The name of the config to retrieve. There is exactly one config
    /// resource per project per location. The expected format is
    /// `projects/{project}/locations/{location}/config`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UpdateConfig][google.cloud.speech.v2.Speech.UpdateConfig\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfigRequest {
    /// Required. The config to update.
    ///
    /// The config's `name` field is used to identify the config to be updated.
    /// The expected format is `projects/{project}/locations/{location}/config`.
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<Config>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// CustomClass for biasing in speech recognition. Used to define a set of words
/// or phrases that represents a common concept or theme likely to appear in your
/// audio, for example a list of passenger ship names.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomClass {
    /// Output only. The resource name of the CustomClass.
    /// Format:
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System-assigned unique identifier for the CustomClass.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// User-settable, human-readable name for the CustomClass. Must be 63
    /// characters or less.
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    /// A collection of class items.
    #[prost(message, repeated, tag="5")]
    pub items: ::prost::alloc::vec::Vec<custom_class::ClassItem>,
    /// Output only. The CustomClass lifecycle state.
    #[prost(enumeration="custom_class::State", tag="15")]
    pub state: i32,
    /// Output only. Creation time.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time this resource was modified.
    #[prost(message, optional, tag="7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this resource was requested for deletion.
    #[prost(message, optional, tag="8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this resource will be purged.
    #[prost(message, optional, tag="9")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows users to store small amounts of arbitrary data.
    /// Both the key and the value must be 63 characters or less each.
    /// At most 100 annotations.
    #[prost(map="string, string", tag="10")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. This checksum is computed by the server based on the value of
    /// other fields. This may be sent on update, undelete, and delete requests to
    /// ensure the client has an up-to-date value before proceeding.
    #[prost(string, tag="11")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Whether or not this CustomClass is in the process of being
    /// updated.
    #[prost(bool, tag="12")]
    pub reconciling: bool,
    /// Output only. The [KMS key
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#keys>) with which
    /// the CustomClass is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    #[prost(string, tag="13")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Output only. The [KMS key version
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#key_versions>)
    /// with which the CustomClass is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`.
    #[prost(string, tag="14")]
    pub kms_key_version_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CustomClass`.
pub mod custom_class {
    /// An item of the class.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassItem {
        /// The class item's value.
        #[prost(string, tag="1")]
        pub value: ::prost::alloc::string::String,
    }
    /// Set of states that define the lifecycle of a CustomClass.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.  This is only used/useful for distinguishing
        /// unset values.
        Unspecified = 0,
        /// The normal and active state.
        Active = 2,
        /// This CustomClass has been deleted.
        Deleted = 4,
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
                State::Deleted => "DELETED",
            }
        }
    }
}
/// PhraseSet for biasing in speech recognition. A PhraseSet is used to provide
/// "hints" to the speech recognizer to favor specific words and phrases in the
/// results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseSet {
    /// Output only. The resource name of the PhraseSet.
    /// Format: `projects/{project}/locations/{location}/phraseSets/{phrase_set}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System-assigned unique identifier for the PhraseSet.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// A list of word and phrases.
    #[prost(message, repeated, tag="3")]
    pub phrases: ::prost::alloc::vec::Vec<phrase_set::Phrase>,
    /// Hint Boost. Positive value will increase the probability that a specific
    /// phrase will be recognized over other similar sounding phrases. The higher
    /// the boost, the higher the chance of false positive recognition as well.
    /// Valid `boost` values are between 0 (exclusive) and 20. We recommend using a
    /// binary search approach to finding the optimal value for your use case.
    #[prost(float, tag="4")]
    pub boost: f32,
    /// User-settable, human-readable name for the PhraseSet. Must be 63
    /// characters or less.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The PhraseSet lifecycle state.
    #[prost(enumeration="phrase_set::State", tag="15")]
    pub state: i32,
    /// Output only. Creation time.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time this resource was modified.
    #[prost(message, optional, tag="7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this resource was requested for deletion.
    #[prost(message, optional, tag="8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this resource will be purged.
    #[prost(message, optional, tag="9")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows users to store small amounts of arbitrary data.
    /// Both the key and the value must be 63 characters or less each.
    /// At most 100 annotations.
    #[prost(map="string, string", tag="10")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. This checksum is computed by the server based on the value of
    /// other fields. This may be sent on update, undelete, and delete requests to
    /// ensure the client has an up-to-date value before proceeding.
    #[prost(string, tag="11")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Whether or not this PhraseSet is in the process of being
    /// updated.
    #[prost(bool, tag="12")]
    pub reconciling: bool,
    /// Output only. The [KMS key
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#keys>) with which
    /// the PhraseSet is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    #[prost(string, tag="13")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Output only. The [KMS key version
    /// name](<https://cloud.google.com/kms/docs/resource-hierarchy#key_versions>)
    /// with which the PhraseSet is encrypted. The expected format is
    /// `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`.
    #[prost(string, tag="14")]
    pub kms_key_version_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PhraseSet`.
pub mod phrase_set {
    /// A Phrase contains words and phrase "hints" so that the speech recognition
    /// is more likely to recognize them. This can be used to improve the accuracy
    /// for specific words and phrases, for example, if specific commands are
    /// typically spoken by the user. This can also be used to add additional words
    /// to the vocabulary of the recognizer.
    ///
    /// List items can also include CustomClass references containing groups of
    /// words that represent common concepts that occur in natural language.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Phrase {
        /// The phrase itself.
        #[prost(string, tag="1")]
        pub value: ::prost::alloc::string::String,
        /// Hint Boost. Overrides the boost set at the phrase set level.
        /// Positive value will increase the probability that a specific phrase will
        /// be recognized over other similar sounding phrases. The higher the boost,
        /// the higher the chance of false positive recognition as well. Negative
        /// boost values would correspond to anti-biasing. Anti-biasing is not
        /// enabled, so negative boost will simply be ignored. Though `boost` can
        /// accept a wide range of positive values, most use cases are best served
        /// with values between 0 and 20. We recommend using a binary search approach
        /// to finding the optimal value for your use case. Speech recognition
        /// will skip PhraseSets with a boost value of 0.
        #[prost(float, tag="2")]
        pub boost: f32,
    }
    /// Set of states that define the lifecycle of a PhraseSet.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.  This is only used/useful for distinguishing
        /// unset values.
        Unspecified = 0,
        /// The normal and active state.
        Active = 2,
        /// This PhraseSet has been deleted.
        Deleted = 4,
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
                State::Deleted => "DELETED",
            }
        }
    }
}
/// Request message for the
/// \[CreateCustomClass][google.cloud.speech.v2.Speech.CreateCustomClass\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomClassRequest {
    /// Required. The CustomClass to create.
    #[prost(message, optional, tag="1")]
    pub custom_class: ::core::option::Option<CustomClass>,
    /// If set, validate the request and preview the CustomClass, but do not
    /// actually create it.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// The ID to use for the CustomClass, which will become the final component of
    /// the CustomClass's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag="3")]
    pub custom_class_id: ::prost::alloc::string::String,
    /// Required. The project and location where this CustomClass will be created.
    /// The expected format is `projects/{project}/locations/{location}`.
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for the
/// \[ListCustomClasses][google.cloud.speech.v2.Speech.ListCustomClasses\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomClassesRequest {
    /// Required. The project and location of CustomClass resources to list. The
    /// expected format is `projects/{project}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of results per requests. A valid page_size ranges from 0 to 20
    /// inclusive. If the page_size is zero or unspecified, a page size of 5 will
    /// be chosen. If the page size exceeds 20, it will be coerced down to 20. Note
    /// that a call might return fewer results than the requested page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// \[ListCustomClasses][google.cloud.speech.v2.Speech.ListCustomClasses\] call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// \[ListCustomClasses][google.cloud.speech.v2.Speech.ListCustomClasses\] must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether, or not, to show resources that have been deleted.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message for the
/// \[ListCustomClasses][google.cloud.speech.v2.Speech.ListCustomClasses\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomClassesResponse {
    /// The list of requested CustomClasses.
    #[prost(message, repeated, tag="1")]
    pub custom_classes: ::prost::alloc::vec::Vec<CustomClass>,
    /// A token, which can be sent as
    /// \[page_token][google.cloud.speech.v2.ListCustomClassesRequest.page_token\] to
    /// retrieve the next page. If this field is omitted, there are no subsequent
    /// pages. This token expires after 72 hours.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the
/// \[GetCustomClass][google.cloud.speech.v2.Speech.GetCustomClass\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomClassRequest {
    /// Required. The name of the CustomClass to retrieve. The expected format is
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UpdateCustomClass][google.cloud.speech.v2.Speech.UpdateCustomClass\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomClassRequest {
    /// Required. The CustomClass to update.
    ///
    /// The CustomClass's `name` field is used to identify the CustomClass to
    /// update. Format:
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`.
    #[prost(message, optional, tag="1")]
    pub custom_class: ::core::option::Option<CustomClass>,
    /// The list of fields to be updated. If empty, all fields are considered for
    /// update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set, validate the request and preview the updated CustomClass, but do
    /// not actually update it.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Request message for the
/// \[DeleteCustomClass][google.cloud.speech.v2.Speech.DeleteCustomClass\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomClassRequest {
    /// Required. The name of the CustomClass to delete.
    /// Format:
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the deleted CustomClass, but do
    /// not actually delete it.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// If set to true, and the CustomClass is not found, the request will succeed
    /// and  be a no-op (no Operation is recorded in this case).
    #[prost(bool, tag="4")]
    pub allow_missing: bool,
    /// This checksum is computed by the server based on the value of other
    /// fields. This may be sent on update, undelete, and delete requests to ensure
    /// the client has an up-to-date value before proceeding.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UndeleteCustomClass][google.cloud.speech.v2.Speech.UndeleteCustomClass\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteCustomClassRequest {
    /// Required. The name of the CustomClass to undelete.
    /// Format:
    /// `projects/{project}/locations/{location}/customClasses/{custom_class}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the undeleted CustomClass, but do
    /// not actually undelete it.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
    /// This checksum is computed by the server based on the value of other
    /// fields. This may be sent on update, undelete, and delete requests to ensure
    /// the client has an up-to-date value before proceeding.
    #[prost(string, tag="4")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for the
/// \[CreatePhraseSet][google.cloud.speech.v2.Speech.CreatePhraseSet\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhraseSetRequest {
    /// Required. The PhraseSet to create.
    #[prost(message, optional, tag="1")]
    pub phrase_set: ::core::option::Option<PhraseSet>,
    /// If set, validate the request and preview the PhraseSet, but do not
    /// actually create it.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// The ID to use for the PhraseSet, which will become the final component of
    /// the PhraseSet's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag="3")]
    pub phrase_set_id: ::prost::alloc::string::String,
    /// Required. The project and location where this PhraseSet will be created.
    /// The expected format is `projects/{project}/locations/{location}`.
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for the
/// \[ListPhraseSets][google.cloud.speech.v2.Speech.ListPhraseSets\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhraseSetsRequest {
    /// Required. The project and location of PhraseSet resources to list. The
    /// expected format is `projects/{project}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of PhraseSets to return. The service may return fewer
    /// than this value. If unspecified, at most 20 PhraseSets will be returned.
    /// The maximum value is 20; values above 20 will be coerced to 20.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// \[ListPhraseSets][google.cloud.speech.v2.Speech.ListPhraseSets\] call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// \[ListPhraseSets][google.cloud.speech.v2.Speech.ListPhraseSets\] must match
    /// the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether, or not, to show resources that have been deleted.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message for the
/// \[ListPhraseSets][google.cloud.speech.v2.Speech.ListPhraseSets\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhraseSetsResponse {
    /// The list of requested PhraseSets.
    #[prost(message, repeated, tag="1")]
    pub phrase_sets: ::prost::alloc::vec::Vec<PhraseSet>,
    /// A token, which can be sent as
    /// \[page_token][google.cloud.speech.v2.ListPhraseSetsRequest.page_token\] to
    /// retrieve the next page. If this field is omitted, there are no subsequent
    /// pages. This token expires after 72 hours.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the
/// \[GetPhraseSet][google.cloud.speech.v2.Speech.GetPhraseSet\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhraseSetRequest {
    /// Required. The name of the PhraseSet to retrieve. The expected format is
    /// `projects/{project}/locations/{location}/phraseSets/{phrase_set}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UpdatePhraseSet][google.cloud.speech.v2.Speech.UpdatePhraseSet\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhraseSetRequest {
    /// Required. The PhraseSet to update.
    ///
    /// The PhraseSet's `name` field is used to identify the PhraseSet to update.
    /// Format: `projects/{project}/locations/{location}/phraseSets/{phrase_set}`.
    #[prost(message, optional, tag="1")]
    pub phrase_set: ::core::option::Option<PhraseSet>,
    /// The list of fields to update. If empty, all non-default valued fields are
    /// considered for update. Use `*` to update the entire PhraseSet resource.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set, validate the request and preview the updated PhraseSet, but do not
    /// actually update it.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Request message for the
/// \[DeletePhraseSet][google.cloud.speech.v2.Speech.DeletePhraseSet\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhraseSetRequest {
    /// Required. The name of the PhraseSet to delete.
    /// Format: `projects/{project}/locations/{location}/phraseSets/{phrase_set}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the deleted PhraseSet, but do not
    /// actually delete it.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// If set to true, and the PhraseSet is not found, the request will succeed
    /// and  be a no-op (no Operation is recorded in this case).
    #[prost(bool, tag="4")]
    pub allow_missing: bool,
    /// This checksum is computed by the server based on the value of other
    /// fields. This may be sent on update, undelete, and delete requests to ensure
    /// the client has an up-to-date value before proceeding.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for the
/// \[UndeletePhraseSet][google.cloud.speech.v2.Speech.UndeletePhraseSet\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeletePhraseSetRequest {
    /// Required. The name of the PhraseSet to undelete.
    /// Format: `projects/{project}/locations/{location}/phraseSets/{phrase_set}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the undeleted PhraseSet, but do
    /// not actually undelete it.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
    /// This checksum is computed by the server based on the value of other
    /// fields. This may be sent on update, undelete, and delete requests to ensure
    /// the client has an up-to-date value before proceeding.
    #[prost(string, tag="4")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod speech_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Enables speech transcription and resource management.
    #[derive(Debug, Clone)]
    pub struct SpeechClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SpeechClient<tonic::transport::Channel> {
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
    impl<T> SpeechClient<T>
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
        ) -> SpeechClient<InterceptedService<T, F>>
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
            SpeechClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a [Recognizer][google.cloud.speech.v2.Recognizer].
        pub async fn create_recognizer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRecognizerRequest>,
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
                "/google.cloud.speech.v2.Speech/CreateRecognizer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Recognizers.
        pub async fn list_recognizers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecognizersRequest>,
        ) -> Result<tonic::Response<super::ListRecognizersResponse>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/ListRecognizers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the requested
        /// [Recognizer][google.cloud.speech.v2.Recognizer]. Fails with
        /// [NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested recognizer doesn't
        /// exist.
        pub async fn get_recognizer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecognizerRequest>,
        ) -> Result<tonic::Response<super::Recognizer>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/GetRecognizer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the [Recognizer][google.cloud.speech.v2.Recognizer].
        pub async fn update_recognizer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRecognizerRequest>,
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
                "/google.cloud.speech.v2.Speech/UpdateRecognizer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the [Recognizer][google.cloud.speech.v2.Recognizer].
        pub async fn delete_recognizer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRecognizerRequest>,
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
                "/google.cloud.speech.v2.Speech/DeleteRecognizer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes the [Recognizer][google.cloud.speech.v2.Recognizer].
        pub async fn undelete_recognizer(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteRecognizerRequest>,
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
                "/google.cloud.speech.v2.Speech/UndeleteRecognizer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Performs synchronous Speech recognition: receive results after all audio
        /// has been sent and processed.
        pub async fn recognize(
            &mut self,
            request: impl tonic::IntoRequest<super::RecognizeRequest>,
        ) -> Result<tonic::Response<super::RecognizeResponse>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/Recognize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Performs bidirectional streaming speech recognition: receive results while
        /// sending audio. This method is only available via the gRPC API (not REST).
        pub async fn streaming_recognize(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::StreamingRecognizeRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingRecognizeResponse>>,
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
                "/google.cloud.speech.v2.Speech/StreamingRecognize",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Performs batch asynchronous speech recognition: send a request with N
        /// audio files and receive a long running operation that can be polled to see
        /// when the transcriptions are finished.
        pub async fn batch_recognize(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchRecognizeRequest>,
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
                "/google.cloud.speech.v2.Speech/BatchRecognize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the requested [Config][google.cloud.speech.v2.Config].
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::Config>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/GetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the [Config][google.cloud.speech.v2.Config].
        pub async fn update_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConfigRequest>,
        ) -> Result<tonic::Response<super::Config>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/UpdateConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a [CustomClass][google.cloud.speech.v2.CustomClass].
        pub async fn create_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomClassRequest>,
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
                "/google.cloud.speech.v2.Speech/CreateCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CustomClasses.
        pub async fn list_custom_classes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomClassesRequest>,
        ) -> Result<tonic::Response<super::ListCustomClassesResponse>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/ListCustomClasses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the requested
        /// [CustomClass][google.cloud.speech.v2.CustomClass].
        pub async fn get_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomClassRequest>,
        ) -> Result<tonic::Response<super::CustomClass>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/GetCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the [CustomClass][google.cloud.speech.v2.CustomClass].
        pub async fn update_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomClassRequest>,
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
                "/google.cloud.speech.v2.Speech/UpdateCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the [CustomClass][google.cloud.speech.v2.CustomClass].
        pub async fn delete_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomClassRequest>,
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
                "/google.cloud.speech.v2.Speech/DeleteCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes the [CustomClass][google.cloud.speech.v2.CustomClass].
        pub async fn undelete_custom_class(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteCustomClassRequest>,
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
                "/google.cloud.speech.v2.Speech/UndeleteCustomClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a [PhraseSet][google.cloud.speech.v2.PhraseSet].
        pub async fn create_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhraseSetRequest>,
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
                "/google.cloud.speech.v2.Speech/CreatePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists PhraseSets.
        pub async fn list_phrase_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhraseSetsRequest>,
        ) -> Result<tonic::Response<super::ListPhraseSetsResponse>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/ListPhraseSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the requested
        /// [PhraseSet][google.cloud.speech.v2.PhraseSet].
        pub async fn get_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhraseSetRequest>,
        ) -> Result<tonic::Response<super::PhraseSet>, tonic::Status> {
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
                "/google.cloud.speech.v2.Speech/GetPhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the [PhraseSet][google.cloud.speech.v2.PhraseSet].
        pub async fn update_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePhraseSetRequest>,
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
                "/google.cloud.speech.v2.Speech/UpdatePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the [PhraseSet][google.cloud.speech.v2.PhraseSet].
        pub async fn delete_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhraseSetRequest>,
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
                "/google.cloud.speech.v2.Speech/DeletePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes the [PhraseSet][google.cloud.speech.v2.PhraseSet].
        pub async fn undelete_phrase_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeletePhraseSetRequest>,
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
                "/google.cloud.speech.v2.Speech/UndeletePhraseSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}