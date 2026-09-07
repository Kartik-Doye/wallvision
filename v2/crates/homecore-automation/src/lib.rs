//! homecore-automation â€” ADR-129 HOMECORE-AUTO
//!
//! Automation engine, trigger evaluator, MiniJinja template evaluator, and
//! script action executor for the HOMECORE Home Assistant port.
//!
//! ## Layout
//!
//! - [`automation`] â€” `Automation` struct: id, alias, mode, triggers, conditions, actions
//! - [`trigger`] â€” `Trigger` enum + `EvaluateTrigger` trait
//! - [`condition`] â€” `Condition` enum + async `evaluate` method + `EvalContext`
//! - [`action`] â€” `Action` enum + async `execute` method + `ExecutionContext`
//! - [`template`] â€” MiniJinja environment with HA-compat globals (states, state_attr, is_state, now)
//! - [`engine`] â€” `AutomationEngine`: subscribes to event bus, drives triggerâ†’conditionâ†’action pipeline
//! - [`error`] â€” crate-wide `AutomationError`

pub mod automation;
pub mod trigger;
pub mod condition;
pub mod action;
pub mod template;
pub mod engine;
pub mod runmode;
pub mod error;

pub use automation::{Automation, RunMode};
pub use trigger::{EvaluateTrigger, Trigger, TriggerContext};
pub use condition::{Condition, EvalContext};
pub use action::{Action, ExecutionContext};
pub use template::TemplateEnvironment;
pub use engine::AutomationEngine;
pub use error::AutomationError;

