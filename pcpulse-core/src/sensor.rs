//! One sensor trait for every health/collector identity.
//!
//! The trait keeps collector registration platform-specific behind `#[cfg(...)]`
//! while every target OS still sees the same trait surface.

pub trait Sensor {
    fn id(&self) -> crate::model::StatId;
    fn label(&self) -> String;
    fn supported(&self) -> bool;
}

pub struct UptimeSensor;
pub struct CpuUsageSensor;
pub struct CpuFreqSensor;
pub struct CpuTempSensor;
pub struct RamSensor;
pub struct SwapSensor;
pub struct DiskUsageSensor;
pub struct NetIoSensor;
pub struct TopProcessesSensor;
pub struct GpuTempSensor;
pub struct GpuUtilSensor;
pub struct GpuVramSensor;
pub struct FanSpeedSensor;
pub struct PowerDrawSensor;
pub struct SsdSmartSensor;

impl Sensor for UptimeSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::Uptime
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for CpuUsageSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::CpuUsage
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for CpuFreqSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::CpuFreq
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for CpuTempSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::CpuTemp
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for RamSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::Ram
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for SwapSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::Swap
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for DiskUsageSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::DiskUsage
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for NetIoSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::NetIo
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for TopProcessesSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::TopProcesses
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        true
    }
}

impl Sensor for GpuTempSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::GpuTemp
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        false
    }
}

impl Sensor for GpuUtilSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::GpuUtil
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        false
    }
}

impl Sensor for GpuVramSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::GpuVram
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        false
    }
}

impl Sensor for FanSpeedSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::FanSpeed
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        false
    }
}

impl Sensor for PowerDrawSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::PowerDraw
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        false
    }
}

impl Sensor for SsdSmartSensor {
    fn id(&self) -> crate::model::StatId {
        crate::model::StatId::SsdSmart
    }

    fn label(&self) -> String {
        self.id().label()
    }

    fn supported(&self) -> bool {
        false
    }
}
