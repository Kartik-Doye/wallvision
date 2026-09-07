//! IEEE 802.11bf-2025 WLAN sensing â€” forward-compatibility protocol model
//! (ADR-153, amending ADR-152 Â§2.4).
//!
//! # Why this exists
//!
//! IEEE 802.11bf-2025 ("WLAN Sensing") was **published 2025-09-26** (verified
//! against the IEEE SA record â€” ADR-152 Â§1.1 F4, evidence grade MEASURED).
//! Sensing standardization is complete for sub-7 GHz and >45 GHz (DMG) bands,
//! with formal sensing measurement setup, measurement instance,
//! feedback/reporting, and sensing-by-proxy (SBP) procedures.
//!
//! **No commodity silicon â€” ESP32 parts included â€” implements the standard
//! yet.** ADR-152 Â§2.4 originally decided "track silicon; no code now";
//! ADR-153 amends that clause: build the typed protocol surface now, so
//! RuView can adopt standardized sensing the day any chipset exposes it.
//! This layer is simulation-tested forward compatibility â€” the OTA binding
//! lands when silicon does. Today's opportunistic CSI extraction (ADR-018 /
//! ADR-028) remains the backend, mapped onto the standardized report path by
//! [`transport::OpportunisticCsiBridge`].
//!
//! > This module is not a certified 802.11bf implementation. It models the
//! > public procedure shape needed by RuView and RuvSense, while intentionally
//! > avoiding OTA frame binding until chipset support and vendor APIs exist.
//!
//! # Layout
//!
//! - [`types`] â€” typed structures for the sensing procedures (setup, roles,
//!   measurement instances, CSI-variant reports, SBP, termination), plus the
//!   ADR-153 future-proofing surfaces: [`types::SpecProfile`] version gates,
//!   [`types::SensingCapabilities`] negotiation, and required
//!   [`types::ConsentMode`] governance metadata on every setup.
//! - [`messages`] â€” the procedure message types (setup request/response,
//!   measurement instance, CSI-variant report, SBP exchange, termination).
//! - [`session`] â€” deterministic event-driven session FSM:
//!   `Idle â†’ SetupNegotiating â†’ Active â†’ Terminating â†’ Idle`, with explicit
//!   rejection paths, timeout handling, single-role enforcement, and the
//!   first-class SBP proxy mode. No async, no clocks.
//! - [`events`] â€” the FSM I/O types ([`events::SessionEvent`],
//!   [`events::Action`], close reasons, configuration), re-exported via
//!   [`session`].
//! - [`table`] â€” responder-side setup registry (setup-ID collision and
//!   capacity rejection paths, for direct setups and SBP alike).
//! - [`transport`] â€” the [`transport::SensingTransport`] seam, the
//!   [`transport::SimTransport`] test double, and the ESP32 bridge.

pub mod events;
pub mod messages;
pub mod session;
pub mod table;
pub mod transport;
pub mod types;

pub use messages::{
    CsiReportPayload, DmgSensingSetupStub, DmgSensingType, SbpRequest, SbpResponse, SbpStatus,
    SensingMeasurementInstance, SensingMeasurementReport, SensingMeasurementSetupRequest,
    SensingMeasurementSetupResponse, SensingSessionTermination, TerminationReason,
};
pub use session::{Action, CloseReason, SensingSession, SessionConfig, SessionEvent, SessionState};
pub use table::SessionTable;
pub use transport::{
    action_to_frame, frame_to_event, OpportunisticCsiBridge, SensingFrame, SensingTransport,
    SimTransport, TransportError,
};
pub use types::{
    bandwidth_mhz, BfError, ConsentMode, MeasurementInstanceId, MeasurementSetupId,
    MeasurementSetupParams, ReportingConfig, SensingCapabilities, SensingRole, SetupStatus,
    SpecProfile, ThresholdParams, TransceiverRole, MAX_BURST_INSTANCES, MAX_PERIOD_MS,
    MAX_REPORT_SUBCARRIERS, MAX_SETUP_ID, MIN_PERIOD_MS,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_fsm;
#[cfg(test)]
mod tests_sbp;
#[cfg(test)]
mod testutil;

