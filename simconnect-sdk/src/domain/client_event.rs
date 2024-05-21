use std::os::raw::c_char;

use crate::{bindings, SimConnectError};

// System Events start from 0 so we have to stagger the values to avoid collisions.
pub(crate) const CLIENT_EVENT_DISCRIMINANT_START: u32 = 1024;

/// SimConnect Client Event Request.
///
/// Defined by <https://www.prepar3d.com/SDKv5/sdk/references/variables/event_ids.html>.
/// Extended by <https://docs.flightsimulator.com/html/Programming_Tools/Event_IDs/Event_IDs.htm>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClientEventRequest {
    // ---------------
    // Aircraft Engine
    // ---------------
    /// Set throttle 1 exactly (0 to 16383).
    Throttle1Set = CLIENT_EVENT_DISCRIMINANT_START,
    /// Set throttle 2 exactly (0 to 16383).
    Throttle2Set,
    /// Set throttle 3 exactly (0 to 16383).
    Throttle3Set,
    /// Set throttle 4 exactly (0 to 16383).
    Throttle4Set,
    // ---------------
    // Aircraft Flight Controls
    // ---------------
    /// Sets elevator position (-16383 - +16383).
    AxisElevatorSet,
    // ---------------
    // Aircraft Miscellaneous Systems
    // ---------------
    /// Increment brake pressure. Note: These are simulated spring-loaded toe brakes, which will bleed back to zero over time.
    Brakes,
    /// Increments left brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesLeft,
    /// Increments right brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesRight,
    /// Sets left brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisLeftBrakeSet,
    /// Sets right brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisRightBrakeSet,
    /// Toggles parking brake on/off.
    ParkingBrakes,
    // ---------------
    // Failure
    // ---------------
    /// Toggles engine 1 failure on/off.
    ToggleEngine1Failure
}

impl ClientEventRequest {
    pub(crate) fn into_c_char(self) -> *const c_char {
        match self {
            // Aircraft Engine
            Self::Throttle1Set => "THROTTLE1_SET\0".as_ptr() as *const c_char,
            Self::Throttle2Set => "THROTTLE2_SET\0".as_ptr() as *const c_char,
            Self::Throttle3Set => "THROTTLE3_SET\0".as_ptr() as *const c_char,
            Self::Throttle4Set => "THROTTLE4_SET\0".as_ptr() as *const c_char,
            // Aircraft Flight Controls
            Self::AxisElevatorSet => "AXIS_ELEVATOR_SET\0".as_ptr() as *const c_char,
            // Aircraft Miscellaneous Systems
            Self::Brakes => "BRAKES\0".as_ptr() as *const c_char,
            Self::BrakesLeft => "BRAKES_LEFT\0".as_ptr() as *const c_char,
            Self::BrakesRight => "BRAKES_RIGHT\0".as_ptr() as *const c_char,
            Self::AxisLeftBrakeSet => "AXIS_LEFT_BRAKE_SET\0".as_ptr() as *const c_char,
            Self::AxisRightBrakeSet => "AXIS_RIGHT_BRAKE_SET\0".as_ptr() as *const c_char,
            Self::ParkingBrakes => "PARKING_BRAKES\0".as_ptr() as *const c_char,
            Self::ToggleEngine1Failure => "TOGGLE_ENGINE1_FAILURE\0".as_ptr() as *const c_char,
        }
    }
}

/// SimConnect Client Event.
///
/// Defined by <https://www.prepar3d.com/SDKv5/sdk/references/variables/event_ids.html>.
/// Extended by <https://docs.flightsimulator.com/html/Programming_Tools/Event_IDs/Event_IDs.htm>.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClientEvent {
    // ---------------
    // Aircraft Engine
    // ---------------
    /// Set throttle 1 exactly (0 to 16383).
    Throttle1Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    /// Set throttle 2 exactly (0 to 16383).
    Throttle2Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    /// Set throttle 3 exactly (0 to 16383).
    Throttle3Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    /// Set throttle 4 exactly (0 to 16383).
    Throttle4Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    // ---------------
    // Aircraft Flight Controls
    // ---------------
    /// Sets elevator position (-16383 - +16383).
    AxisElevatorSet {
        /// -16383 (full down) to +16383 (full up).
        value: i32,
    },
    // ---------------
    // Aircraft Miscellaneous Systems
    // ---------------
    /// Increment brake pressure. Note: These are simulated spring-loaded toe brakes, which will bleed back to zero over time.
    Brakes,
    /// Increments left brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesLeft,
    /// Increments right brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesRight,
    /// Sets left brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisLeftBrakeSet {
        /// -16383 (0 brakes) to +16383 (max brakes).
        value: i32,
    },
    /// Sets right brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisRightBrakeSet {
        /// -16383 (0 brakes) to +16383 (max brakes).
        value: i32,
    },
    /// Toggles parking brake on/off.
    ParkingBrakes,
    // ---------------
    // Failure
    // ---------------
    ToggleEngine1Failure {
        /// 1 - Engine 1 failure on, 0 - Engine 1 failure off.
        value: i32,
    },
}

impl TryFrom<&bindings::SIMCONNECT_RECV_EVENT> for ClientEvent {
    type Error = SimConnectError;

    fn try_from(event: &bindings::SIMCONNECT_RECV_EVENT) -> Result<Self, Self::Error> {
        let request = ClientEventRequest::try_from(event.uEventID)
            .map_err(|_| SimConnectError::UnimplementedEventType(event.uEventID))?;

        match request {
            // Aircraft Engine
            ClientEventRequest::Throttle1Set => Ok(Self::Throttle1Set {
                value: event.dwData as i32,
            }),
            ClientEventRequest::Throttle2Set => Ok(Self::Throttle2Set {
                value: event.dwData as i32,
            }),
            ClientEventRequest::Throttle3Set => Ok(Self::Throttle3Set {
                value: event.dwData as i32,
            }),
            ClientEventRequest::Throttle4Set => Ok(Self::Throttle4Set {
                value: event.dwData as i32,
            }),
            // Aircraft Flight Controls
            ClientEventRequest::AxisElevatorSet => Ok(Self::AxisElevatorSet {
                value: event.dwData as i32,
            }),
            // Aircraft Miscellaneous Systems
            ClientEventRequest::Brakes => Ok(Self::Brakes),
            ClientEventRequest::BrakesLeft => Ok(Self::BrakesLeft),
            ClientEventRequest::BrakesRight => Ok(Self::BrakesRight),
            ClientEventRequest::AxisLeftBrakeSet => Ok(Self::AxisLeftBrakeSet {
                value: event.dwData as i32,
            }),
            ClientEventRequest::AxisRightBrakeSet => Ok(Self::AxisRightBrakeSet {
                value: event.dwData as i32,
            }),
            ClientEventRequest::ParkingBrakes => Ok(Self::ParkingBrakes),
            // Failure
            ClientEventRequest::ToggleEngine1Failure => Ok(Self::ToggleEngine1Failure {
                value: event.dwData as i32,
            }),
        }
    }
}

impl From<ClientEvent> for (ClientEventRequest, i32) {
    fn from(event: ClientEvent) -> Self {
        match event {
            // Aircraft Engine
            ClientEvent::Throttle1Set { value } => (ClientEventRequest::Throttle1Set, value),
            ClientEvent::Throttle2Set { value } => (ClientEventRequest::Throttle2Set, value),
            ClientEvent::Throttle3Set { value } => (ClientEventRequest::Throttle3Set, value),
            ClientEvent::Throttle4Set { value } => (ClientEventRequest::Throttle4Set, value),
            // Aircraft Flight Controls
            ClientEvent::AxisElevatorSet { value } => (ClientEventRequest::AxisElevatorSet, value),
            // Aircraft Miscellaneous Systems
            ClientEvent::Brakes => (ClientEventRequest::Brakes, 0),
            ClientEvent::BrakesLeft => (ClientEventRequest::BrakesLeft, 0),
            ClientEvent::BrakesRight => (ClientEventRequest::BrakesRight, 0),
            ClientEvent::AxisLeftBrakeSet { value } => {
                (ClientEventRequest::AxisLeftBrakeSet, value)
            }
            ClientEvent::AxisRightBrakeSet { value } => {
                (ClientEventRequest::AxisRightBrakeSet, value)
            }
            ClientEvent::ParkingBrakes => (ClientEventRequest::ParkingBrakes, 0),
            // Failure
            ClientEvent::ToggleEngine1Failure { value } => {
                (ClientEventRequest::ToggleEngine1Failure, value)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventFlag {
    Default,
    FastRepeatTimer,
    SlowRepeatTimer,
    GroupIdIsPriority,
}

impl From<EventFlag> for u32 {
    fn from(flag: EventFlag) -> Self {
        match flag {
            EventFlag::Default => bindings::SIMCONNECT_EVENT_FLAG_DEFAULT,
            EventFlag::FastRepeatTimer => bindings::SIMCONNECT_EVENT_FLAG_FAST_REPEAT_TIMER,
            EventFlag::SlowRepeatTimer => bindings::SIMCONNECT_EVENT_FLAG_SLOW_REPEAT_TIMER,
            EventFlag::GroupIdIsPriority => bindings::SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
        }
    }
}