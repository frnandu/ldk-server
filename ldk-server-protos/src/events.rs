// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

/// EventEnvelope wraps different event types in a single message to be used by EventPublisher.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEnvelope {
	#[prost(oneof = "event_envelope::Event", tags = "2, 3, 4, 6, 8, 9")]
	pub event: ::core::option::Option<event_envelope::Event>,
}
/// Nested message and enum types in `EventEnvelope`.
pub mod event_envelope {
	#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
	#[allow(clippy::derive_partial_eq_without_eq)]
	#[derive(Clone, PartialEq, ::prost::Oneof)]
	pub enum Event {
		#[prost(message, tag = "2")]
		PaymentReceived(super::PaymentReceived),
		#[prost(message, tag = "3")]
		PaymentSuccessful(super::PaymentSuccessful),
		#[prost(message, tag = "4")]
		PaymentFailed(super::PaymentFailed),
		#[prost(message, tag = "6")]
		PaymentForwarded(super::PaymentForwarded),
		#[prost(message, tag = "8")]
		ChannelStateChange(super::ChannelStateChange),
		#[prost(message, tag = "9")]
		ChannelClosed(super::ChannelClosed),
	}
}
/// PaymentReceived indicates a payment has been received.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentReceived {
	/// The payment details for the payment in event.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentSuccessful indicates a sent payment was successful.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentSuccessful {
	/// The payment details for the payment in event.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentFailed indicates a sent payment has failed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentFailed {
	/// The payment details for the payment in event.
	#[prost(message, optional, tag = "1")]
	pub payment: ::core::option::Option<super::types::Payment>,
}
/// PaymentForwarded indicates a payment was forwarded through the node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentForwarded {
	#[prost(message, optional, tag = "1")]
	pub forwarded_payment: ::core::option::Option<super::types::ForwardedPayment>,
}
/// ChannelStateChange indicates a channel state has changed (pending, ready, closed, etc.)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelStateChange {
	/// The channel details.
	#[prost(message, optional, tag = "1")]
	pub channel: ::core::option::Option<super::types::Channel>,
	/// The state the channel has transitioned to.
	#[prost(enumeration = "ChannelState", tag = "2")]
	pub state: i32,
}
/// ChannelClosed indicates a channel has been closed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelClosed {
	/// The channel ID that was closed.
	#[prost(string, tag = "1")]
	pub channel_id: ::prost::alloc::string::String,
	/// The user channel ID.
	#[prost(string, tag = "2")]
	pub user_channel_id: ::prost::alloc::string::String,
	/// The counterparty node ID.
	#[prost(string, tag = "3")]
	pub counterparty_node_id: ::prost::alloc::string::String,
	/// Whether this was a force close.
	#[prost(bool, tag = "4")]
	pub is_force_close: bool,
	/// Who initiated the close.
	#[prost(enumeration = "CloseInitiator", tag = "5")]
	pub initiator: i32,
	/// The detailed closure reason.
	#[prost(message, optional, tag = "6")]
	pub closure_reason: ::core::option::Option<ClosureReason>,
	/// Human-readable description of why the channel was closed.
	#[prost(string, tag = "7")]
	pub reason_description: ::prost::alloc::string::String,
	/// Whether this was an open failure (channel never became ready).
	#[prost(bool, tag = "8")]
	pub is_open_failure: bool,
}
/// Closure reason details.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosureReason {
	#[prost(oneof = "closure_reason::Reason", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
	pub reason: ::core::option::Option<closure_reason::Reason>,
}
/// Nested message and enum types in `ClosureReason`.
pub mod closure_reason {
	#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
	#[allow(clippy::derive_partial_eq_without_eq)]
	#[derive(Clone, PartialEq, ::prost::Oneof)]
	pub enum Reason {
		/// Counterparty force-closed the channel.
		#[prost(message, tag = "1")]
		CounterpartyForceClosed(super::CounterpartyForceClosed),
		/// We force-closed the channel.
		#[prost(message, tag = "2")]
		HolderForceClosed(super::HolderForceClosed),
		/// Channel was cooperatively closed.
		#[prost(message, tag = "3")]
		CooperativeClosure(super::CooperativeClosure),
		/// Commitment transaction was confirmed on-chain.
		#[prost(message, tag = "4")]
		CommitmentTxConfirmed(super::CommitmentTxConfirmed),
		/// Funding transaction failed to confirm in time.
		#[prost(message, tag = "5")]
		FundingTimedOut(super::FundingTimedOut),
		/// Error during processing (HTLC forward/relay/reception).
		#[prost(message, tag = "6")]
		ProcessingError(super::ProcessingError),
		/// Peer disconnected before funding completed.
		#[prost(message, tag = "7")]
		DisconnectedPeer(super::DisconnectedPeer),
		/// ChannelManager was outdated compared to ChannelMonitor.
		#[prost(message, tag = "8")]
		OutdatedChannelManager(super::OutdatedChannelManager),
		/// Counterparty closed unfunded channel cooperatively.
		#[prost(message, tag = "9")]
		CounterpartyCoopClosedUnfunded(super::CounterpartyCoopClosedUnfunded),
		/// We closed unfunded channel cooperatively.
		#[prost(message, tag = "10")]
		LocallyCoopClosedUnfunded(super::LocallyCoopClosedUnfunded),
		/// Funding batch closure.
		#[prost(message, tag = "11")]
		FundingBatchClosure(super::FundingBatchClosure),
		/// HTLCs timed out causing force close.
		#[prost(message, tag = "12")]
		HtlcsTimedOut(super::HtlcsTimedOut),
		/// Peer provided feerate that was too low.
		#[prost(message, tag = "13")]
		PeerFeerateTooLow(super::PeerFeerateTooLow),
	}
}
/// Counterparty force-closed the channel.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterpartyForceClosed {
	/// The error message from the peer (use with caution).
	#[prost(string, tag = "1")]
	pub peer_message: ::prost::alloc::string::String,
}
/// We force-closed the channel.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HolderForceClosed {
	/// Whether the latest transaction was broadcast.
	#[prost(bool, tag = "1")]
	pub broadcasted_latest_txn: bool,
	/// The error message provided when force-closing.
	#[prost(string, tag = "2")]
	pub message: ::prost::alloc::string::String,
}
/// Channel was cooperatively closed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CooperativeClosure {
	/// Who initiated the cooperative close.
	#[prost(enumeration = "CloseInitiator", tag = "1")]
	pub initiator: i32,
}
/// Commitment transaction was confirmed on-chain.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentTxConfirmed {}
/// Funding transaction failed to confirm in time.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingTimedOut {}
/// Error during processing.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingError {
	/// The error message.
	#[prost(string, tag = "1")]
	pub error: ::prost::alloc::string::String,
}
/// Peer disconnected before funding completed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectedPeer {}
/// ChannelManager was outdated.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutdatedChannelManager {}
/// Counterparty closed unfunded channel.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterpartyCoopClosedUnfunded {}
/// We closed unfunded channel.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocallyCoopClosedUnfunded {}
/// Funding batch closure.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingBatchClosure {}
/// HTLCs timed out.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtlcsTimedOut {
	/// The payment hash that timed out.
	#[prost(string, tag = "1")]
	pub payment_hash: ::prost::alloc::string::String,
}
/// Peer feerate was too low.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerFeerateTooLow {
	/// The peer's feerate (sat/kw).
	#[prost(uint32, tag = "1")]
	pub peer_feerate_sat_per_kw: u32,
	/// The required feerate (sat/kw).
	#[prost(uint32, tag = "2")]
	pub required_feerate_sat_per_kw: u32,
}
/// Enum representing possible channel states.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelState {
	/// Channel is pending and awaiting funding transaction confirmation.
	Pending = 0,
	/// Channel is ready and can be used for payments.
	Ready = 1,
	/// Channel is closed.
	Closed = 2,
}
impl ChannelState {
	/// String value of the enum field names used in the ProtoBuf definition.
	///
	/// The values are not transformed in any way and thus are considered stable
	/// (if the ProtoBuf definition does not change) and safe for programmatic use.
	pub fn as_str_name(&self) -> &'static str {
		match self {
			ChannelState::Pending => "PENDING",
			ChannelState::Ready => "READY",
			ChannelState::Closed => "CLOSED",
		}
	}
	/// Creates an enum from field names used in the ProtoBuf definition.
	pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
		match value {
			"PENDING" => Some(Self::Pending),
			"READY" => Some(Self::Ready),
			"CLOSED" => Some(Self::Closed),
			_ => None,
		}
	}
}
/// Who initiated the channel close.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CloseInitiator {
	/// Initiator unknown or not specified.
	Unknown = 0,
	/// The counterparty (remote node) initiated the close.
	Counterparty = 1,
	/// We (the local node) initiated the close.
	Local = 2,
}
impl CloseInitiator {
	/// String value of the enum field names used in the ProtoBuf definition.
	///
	/// The values are not transformed in any way and thus are considered stable
	/// (if the ProtoBuf definition does not change) and safe for programmatic use.
	pub fn as_str_name(&self) -> &'static str {
		match self {
			CloseInitiator::Unknown => "UNKNOWN",
			CloseInitiator::Counterparty => "COUNTERPARTY",
			CloseInitiator::Local => "LOCAL",
		}
	}
	/// Creates an enum from field names used in the ProtoBuf definition.
	pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
		match value {
			"UNKNOWN" => Some(Self::Unknown),
			"COUNTERPARTY" => Some(Self::Counterparty),
			"LOCAL" => Some(Self::Local),
			_ => None,
		}
	}
}
