//! HAP service type and characteristic enum catalogues.
//!
//! Mirrors the HAP-1.1 service/characteristic namespace used by Apple Home
//! and the `hap` crate (https://crates.io/crates/hap). Keeping these as
//! plain Rust enums in P1 avoids the heavy `hap` dep until P2.

use serde::{Deserialize, Serialize};

/// HAP service types exposed by the RuView bridge.
///
/// Derived from HomeKit Accessory Protocol Specification Â§8 (service
/// definitions) and cross-checked against HA's `homekit` integration
/// service catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HapAccessoryType {
    /// HAP `Lightbulb` service â€” maps `light.*` entities.
    Lightbulb,
    /// HAP `Switch` service â€” maps generic boolean `switch.*` entities.
    Switch,
    /// HAP `OccupancySensor` â€” maps presence / occupancy binary sensors.
    OccupancySensor,
    /// HAP `MotionSensor` â€” maps motion binary sensors + RuView motion.
    MotionSensor,
    /// HAP `TemperatureSensor` â€” maps `sensor.*temperature*` entities.
    TemperatureSensor,
    /// HAP `HumiditySensor` â€” maps `sensor.*humidity*` entities.
    HumiditySensor,
    /// HAP `LeakSensor` â€” maps abnormal event sensors; used for fall detection
    /// following HA's homekit_controller convention (HAP Â§11.42).
    LeakSensor,
    /// HAP `ContactSensor` â€” maps door / window binary sensors.
    ContactSensor,
    /// HAP `Door` service â€” maps `cover.*door*` entities.
    Door,
    /// HAP `LockMechanism` service â€” maps `lock.*` entities.
    Lock,
    /// HAP `SecuritySystem` service â€” maps alarm / security panel entities.
    SecuritySystem,
}

impl HapAccessoryType {
    /// All defined variants â€” used in tests and for UI enumeration.
    pub const ALL: &'static [HapAccessoryType] = &[
        HapAccessoryType::Lightbulb,
        HapAccessoryType::Switch,
        HapAccessoryType::OccupancySensor,
        HapAccessoryType::MotionSensor,
        HapAccessoryType::TemperatureSensor,
        HapAccessoryType::HumiditySensor,
        HapAccessoryType::LeakSensor,
        HapAccessoryType::ContactSensor,
        HapAccessoryType::Door,
        HapAccessoryType::Lock,
        HapAccessoryType::SecuritySystem,
    ];
}

/// HAP characteristic identifiers that the bridge reads or writes.
///
/// Each variant corresponds to one HAP characteristic UUID as specified in
/// HomeKit Accessory Protocol Specification Â§9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HapCharacteristic {
    /// `On` (bool) â€” Lightbulb / Switch power state.
    On,
    /// `Brightness` (uint8, 0â€“100) â€” Lightbulb brightness percentage.
    Brightness,
    /// `CurrentTemperature` (float, Â°C) â€” TemperatureSensor reading.
    CurrentTemperature,
    /// `CurrentRelativeHumidity` (float, %) â€” HumiditySensor reading.
    CurrentRelativeHumidity,
    /// `OccupancyDetected` (uint8, 0=not detected, 1=detected).
    OccupancyDetected,
    /// `MotionDetected` (bool).
    MotionDetected,
    /// `LeakDetected` (uint8, 0=no leak, 1=leak detected). Re-used for falls.
    LeakDetected,
    /// `ContactSensorState` (uint8, 0=in contact, 1=not in contact).
    ContactSensorState,
    /// `CurrentDoorState` (uint8, HAP Â§9.30).
    CurrentDoorState,
    /// `LockCurrentState` (uint8, HAP Â§9.56).
    LockCurrentState,
    /// `SecuritySystemCurrentState` (uint8, HAP Â§9.97).
    SecuritySystemCurrentState,
}

/// Typed value carried by a HAP characteristic update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HapCharacteristicValue {
    Bool(bool),
    UInt8(u8),
    Float(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_11_accessory_types_defined() {
        assert_eq!(HapAccessoryType::ALL.len(), 11);
        // Spot-check each variant is present.
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::Lightbulb));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::Switch));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::OccupancySensor));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::MotionSensor));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::TemperatureSensor));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::HumiditySensor));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::LeakSensor));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::ContactSensor));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::Door));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::Lock));
        assert!(HapAccessoryType::ALL.contains(&HapAccessoryType::SecuritySystem));
    }

    #[test]
    fn characteristic_value_roundtrip_serde() {
        let v = HapCharacteristicValue::Float(22.5);
        let json = serde_json::to_string(&v).unwrap();
        let back: HapCharacteristicValue = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }
}

