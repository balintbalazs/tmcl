//! All axis parameters useable with TMCM modules other than TMCM-100 and Monopack 2.
//!
//! # Mnemonics for use in macros:
//! - AP - ActualPosition (1)
//! - AS - ActualSpeed (3)
//! - MPS - MaximumPositioningSpeed (4)
//! - AMC - AbolsuteMaxCurrent (6)
//! - SBC - StandbyCurrent (7)
//! - RLSD - RightLimitSwitchDisable (12)
//! - LLSD - LeftLimitSwitchDisable (13)
//! - MSR - MicrostepResolution (140)

use AxisParameter;
use ReadableAxisParameter;
use Return;
use WriteableAxisParameter;

use modules::tmcm::{ReadableTmcmAxisParameter, TmcmAxisParameter, WriteableTmcmAxisParameter};

axis_param_rw!(
    /// The desired target position in position mode
    TargetPosition,
    i32,
    0
);
impl TargetPosition {
    pub fn new(position: i32) -> Self {
        TargetPosition(position)
    }
}
impl ReadableTmcmAxisParameter for TargetPosition {}
impl WriteableTmcmAxisParameter for TargetPosition {}

axis_param_rw!(
    /// The actual position of the motor.
    ///
    /// Stop the motor before overwriting it. Should normally only be
    /// overwritten for reference position setting.
    ActualPosition,
    i32,
    1
);
impl ActualPosition {
    pub fn new(position: i32) -> Self {
        ActualPosition(position)
    }
}
impl ReadableTmcmAxisParameter for ActualPosition {}
impl WriteableTmcmAxisParameter for ActualPosition {}

axis_param_rw!(
    /// The target rotation speed.
    ///
    /// The desired speed in velocity mode. Not valid in position mode.
    TargetSpeed,
    i32, // PD42-1240 supports a range of -7999744..7999744
    2
);
impl TargetSpeed {
    pub fn new(speed: i32) -> Self {
        TargetSpeed(speed)
    }
}
impl ReadableTmcmAxisParameter for TargetSpeed {}
impl WriteableTmcmAxisParameter for TargetSpeed {}

axis_param_r!(
    /// The current rotation speed.
    ///
    /// Should never be overwritten.
    ActualSpeed,
    i32,
    3
);
impl ReadableTmcmAxisParameter for ActualSpeed {}

axis_param_rw!(
    /// The maximum positioning speed.
    ///
    /// Should not exceed the physically highest possible value. Adjust the pulse divisor (no. 154),
    /// if the speed value is very  low  (<50)  or  above  the  upper  limit.
    /// See TMC 428 datasheet (p.24) for calculation of physical units.
    MaximumPositioningSpeed,
    u32,
    4
);
impl MaximumPositioningSpeed {
    pub fn new(speed: u32) -> Self {
        MaximumPositioningSpeed(speed)
    }
}
impl TmcmAxisParameter for MaximumPositioningSpeed {}
impl ReadableTmcmAxisParameter for MaximumPositioningSpeed {}
impl WriteableTmcmAxisParameter for MaximumPositioningSpeed {}

axis_param_rw!(
    /// The maximum acceleration.
    ///
    /// Maximum acceleration during ramp-up and maximum deceleration during ramp-down.
    MaximumAcceleration,
    u32,
    5
);
impl MaximumAcceleration {
    pub fn new(acc: u32) -> Self {
        MaximumAcceleration(acc)
    }
}
impl TmcmAxisParameter for MaximumAcceleration {}
impl ReadableTmcmAxisParameter for MaximumAcceleration {}
impl WriteableTmcmAxisParameter for MaximumAcceleration {}

axis_param_rw!(
    /// The absolute maximum current
    ///
    /// The most important motor setting, since too high values might cause motor damage!
    ///
    /// Note  that  on  the  TMCM-300 the phase current can not be reduced down to zero due
    /// to the Allegro A3972 driver hardware. On the TMCM-300, 303, 310, 110, 610, 611 and 612
    /// the maximum value is 1500 (which means 1.5A).
    /// On all other modules the maximum value is 255 (which means 100% of the maximum current of the module).
    AbsoluteMaxCurrent,
    u16,
    6
);
impl AbsoluteMaxCurrent {
    pub fn new(current: u16) -> Self {
        AbsoluteMaxCurrent(current)
    }
}
impl TmcmAxisParameter for AbsoluteMaxCurrent {}
impl ReadableTmcmAxisParameter for AbsoluteMaxCurrent {}
impl WriteableTmcmAxisParameter for AbsoluteMaxCurrent {}

axis_param_rw!(
    /// The current used when the motor is not running.
    ///
    /// Note  that  on  the  TMCM-300 the phase current can not be reduced down to zero due
    /// to the Allegro A3972 driver hardware. On the TMCM-300, 303, 310, 110, 610, 611 and 612
    /// the maximum value is 1500 (which means 1.5A).
    /// On all other modules the maximum value is 255 (which means 100% of the maximum current
    /// of the module).
    ///
    /// This value should be as low as possible so that the motor can cool down when it is
    /// not moving. Please see also parameter 214 (PowerDownDelay).
    StandbyCurrent,
    u16,
    7
);
impl StandbyCurrent {
    pub fn new(current: u16) -> Self {
        StandbyCurrent(current)
    }
}
impl TmcmAxisParameter for StandbyCurrent {}
impl ReadableTmcmAxisParameter for StandbyCurrent {}
impl WriteableTmcmAxisParameter for StandbyCurrent {}

axis_param_r!(
    /// Position reached flag
    /// 
    /// This flag is always set when target position and actual position are equal.
    PositionReachedFlag,
    bool,
    8
);
impl TmcmAxisParameter for PositionReachedFlag {}
impl ReadableTmcmAxisParameter for PositionReachedFlag {}

axis_param_r!(
    /// Home switch state
    /// 
    /// The logical state of the home switch input.
    HomeSwitchState,
    bool,
    9
);
impl TmcmAxisParameter for HomeSwitchState {}
impl ReadableTmcmAxisParameter for HomeSwitchState {}

axis_param_r!(
    /// Right limit switch state
    /// 
    /// The logical state of the right limit switch input.
    RightLimitSwitchState,
    bool,
    10
);
impl TmcmAxisParameter for RightLimitSwitchState {}
impl ReadableTmcmAxisParameter for RightLimitSwitchState {}

axis_param_r!(
    /// Left limit switch state
    /// 
    /// The logical state of the left limit switch input.
    LeftLimitSwitchState,
    bool,
    11
);
impl TmcmAxisParameter for LeftLimitSwitchState {}
impl ReadableTmcmAxisParameter for LeftLimitSwitchState {}

axis_param_rw!(
    /// If set, deactivates the stop function of the right switch
    RightLimitSwitchDisable,
    bool,
    12
);
impl RightLimitSwitchDisable {
    pub fn disabled() -> Self {
        RightLimitSwitchDisable(true)
    }
    pub fn enabled() -> Self {
        RightLimitSwitchDisable(false)
    }
}
impl TmcmAxisParameter for RightLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for RightLimitSwitchDisable {}
impl WriteableTmcmAxisParameter for RightLimitSwitchDisable {}

axis_param_rw!(
    /// Deactivates the stop function of the left switch resp. reference switch if set.
    LeftLimitSwitchDisable,
    bool,
    13
);
impl LeftLimitSwitchDisable {
    pub fn disabled() -> Self {
        LeftLimitSwitchDisable(true)
    }
    pub fn enabled() -> Self {
        LeftLimitSwitchDisable(false)
    }
}
impl TmcmAxisParameter for LeftLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl WriteableTmcmAxisParameter for LeftLimitSwitchDisable {}

axis_param_rw!(
    /// The maximum deceleration.
    ///
    /// Maximum deceleration in positioning ramps. Used to decelerate from
    /// maximum positiong speed (axis parameter 4) to velocity V1.
    ///
    /// Not available on TMCL-1140
    MaximumDeceleration,
    u32,
    17
);
impl MaximumDeceleration {
    pub fn new(acc: u32) -> Self {
        MaximumDeceleration(acc)
    }
}
impl TmcmAxisParameter for MaximumDeceleration {}
impl ReadableTmcmAxisParameter for MaximumDeceleration {}
impl WriteableTmcmAxisParameter for MaximumDeceleration {}

/// Microstep Resolution
///
/// Note that modifying this parameter will affect the rotation speed in the same relation.
/// Even if the module is specified for 16 microsteps only, switching to 32 or 64 microsteps still
/// brings an enhancement in resolution and smoothness. The position counter will use the full
/// resolution, but, however, the motor will resolve a maximum of 24 different microsteps only
/// for the 32 or 64 microstep units.
///
/// *) Please note that the fullstep setting as well as the half step setting are not optimized for
/// use without an adapted microstepping table. These settings just step through the microstep table
/// in steps of 64 respectively 32. To get real full stepping use axis parameter 211 or load an
/// adapted microstepping table.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MicrostepResolution {
    /// Fullstep
    Full = 0,
    /// Halfstep
    Half = 1,
    /// 4 microsteps
    Micro4 = 2,
    /// 8 microsteps
    Micro8 = 3,
    /// 16 microsteps
    Micro16 = 4,
    /// 32 microsteps
    Micro32 = 5,
    /// 64 microsteps
    Micro64 = 6,
    /// 128 microsteps
    Micro128 = 7,
    /// 256 microsteps
    Micro256 = 8,
}
impl MicrostepResolution {
    pub fn try_from_u8(v: u8) -> Result<Self, ()> {
        match v {
            0 => Ok(MicrostepResolution::Full),
            1 => Ok(MicrostepResolution::Half),
            2 => Ok(MicrostepResolution::Micro4),
            3 => Ok(MicrostepResolution::Micro8),
            4 => Ok(MicrostepResolution::Micro16),
            5 => Ok(MicrostepResolution::Micro32),
            6 => Ok(MicrostepResolution::Micro64),
            7 => Ok(MicrostepResolution::Micro128),
            8 => Ok(MicrostepResolution::Micro256),
            _ => Err(()),
        }
    }
    
    pub fn try_from_scaled(v: u16) -> Result<Self, ()> {
        match v {
            1 => Ok(MicrostepResolution::Full),
            2 => Ok(MicrostepResolution::Half),
            4 => Ok(MicrostepResolution::Micro4),
            8 => Ok(MicrostepResolution::Micro8),
            16 => Ok(MicrostepResolution::Micro16),
            32 => Ok(MicrostepResolution::Micro32),
            64 => Ok(MicrostepResolution::Micro64),
            128 => Ok(MicrostepResolution::Micro128),
            256 => Ok(MicrostepResolution::Micro256),
            _ => Err(()),
        }
    }
}
impl AxisParameter for MicrostepResolution {
    const NUMBER: u8 = 140;
}
impl Return for MicrostepResolution {
    fn from_operand(array: [u8; 4]) -> Self {
        MicrostepResolution::try_from_u8(array[0]).unwrap()
    }
}
impl TmcmAxisParameter for MicrostepResolution {}
impl ReadableAxisParameter for MicrostepResolution {}
impl ReadableTmcmAxisParameter for MicrostepResolution {}
impl WriteableAxisParameter for MicrostepResolution {
    fn operand(&self) -> [u8; 4] {
        [*self as u8, 0u8, 0u8, 0u8]
    }
}
impl WriteableTmcmAxisParameter for MicrostepResolution {}

axis_param_rw!(
    /// Ramp divisor
    ///
    /// The exponent of the scaling factor for the ramp generator. Change this
    /// parameter carefully (in steps of one) and only while the motor is not
    /// moving. Lower values lead to higher accelerations. This parameter
    /// specifes the relation between internal and real world acceleration units.
    RampDivisor,
    u8,
    153
);
impl RampDivisor {
    pub fn new(divisor: u8) -> Self {
        assert!(divisor <= 13);
        RampDivisor(divisor)
    }
}
impl TmcmAxisParameter for RampDivisor {}
impl ReadableTmcmAxisParameter for RampDivisor {}
impl WriteableTmcmAxisParameter for RampDivisor {}

axis_param_rw!(
    /// Pulse divisor
    ///
    /// The exponent of the scaling factor for the pulse (step) generator.
    /// Change this parameter carefully (in steps of one) and only while the
    /// motor is not moving. Lower values lead to higher speeds. This parameter
    /// specifes the relation between internal and real world velocity units.
    PulseDivisor,
    u8,
    154
);
impl PulseDivisor {
    pub fn new(divisor: u8) -> Self {
        assert!(divisor <= 13);
        PulseDivisor(divisor)
    }
}
impl TmcmAxisParameter for PulseDivisor {}
impl ReadableTmcmAxisParameter for PulseDivisor {}
impl WriteableTmcmAxisParameter for PulseDivisor {}

/// Reference search mode
/// 
/// 1. Search left stop switch only.
/// 2. Search right stop switch, then search left stop switch.
/// 3. Search right stop switch, then search left stop switch from both sides.
/// 4. Search left stop switch from both sides.
/// 5. Search home switch in negative direction, reverse the direction when left
///    stop switch reached.
/// 6. Search home switch in positive direction, reverse the direction when
///    right stop switch reached.
/// 7. Search home switch in positive direction, ignore end switches.
/// 8. Search home switch in negative direction, ignore end switches.
/// 
/// Additional functions
/// - Add 128 to a mode value for inverting the home switch (can be used
///   with mode 5...8).
/// - Add 64 to a mode for searching the right instead of the left reference
///   switch (can be used with mode 1...4).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReferenceSearchMode {
    LimitSwitchSearch {
        search_mode: SearchMode,
        swap_left_right: bool,
    },
    HomeSearch {
        search_mode: HomeSearchMode,
        invert_home_switch: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SearchMode {
    /// Search left stop switch only.
    LeftSwitch = 1,
    /// Search right stop switch, then search left stop switch.
    RightThenLeftSwitch = 2,
    /// Search right stop switch, then search left stop switch from both sides.
    RightThenLeftFromBothSides = 3,
    /// Search left stop switch from both sides.
    LeftFromBothSides = 4,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HomeSearchMode {
    /// Search home switch in negative direction, reverse the direction when
    /// left stop switch reached.
    NegativeThenLeft = 5,
    /// Search home switch in positive direction, reverse the direction when
    /// right stop switch reached.
    PositiveThenRight = 6,
    /// Search home switch in positive direction, ignore end switches.
    Positive = 7,
    /// Search home switch in negative direction, ignore end switches.
    Negative = 8,
}
impl ReferenceSearchMode {
    fn try_from_u8(v: u8) -> Result<Self, ()> {
        let lsb = v & 0b_0000_0111;
        if lsb <= 4 {
            Ok(ReferenceSearchMode::LimitSwitchSearch {
                search_mode: match lsb {
                    1 => SearchMode::LeftSwitch,
                    2 => SearchMode::RightThenLeftSwitch,
                    3 => SearchMode::RightThenLeftFromBothSides,
                    4 => SearchMode::LeftFromBothSides,
                    _ => return Err(()),
                },
                swap_left_right: v & 64 > 0,
            })
        } else {
            Ok(ReferenceSearchMode::HomeSearch {
                search_mode: match lsb {
                    5 => HomeSearchMode::NegativeThenLeft,
                    6 => HomeSearchMode::PositiveThenRight,
                    7 => HomeSearchMode::Positive,
                    8 => HomeSearchMode::Negative,
                    _ => return Err(()),
                },
                invert_home_switch: v & 128 > 0,
            })
        }
    }
}
impl AxisParameter for ReferenceSearchMode {
    const NUMBER: u8 = 193;
}
impl Return for ReferenceSearchMode {
    fn from_operand(array: [u8; 4]) -> Self {
        ReferenceSearchMode::try_from_u8(array[0]).unwrap()
    }
}
impl TmcmAxisParameter for ReferenceSearchMode {}
impl ReadableAxisParameter for ReferenceSearchMode {}
impl ReadableTmcmAxisParameter for ReferenceSearchMode {}
impl WriteableAxisParameter for ReferenceSearchMode {
    fn operand(&self) -> [u8; 4] {
        let value = match self {
            ReferenceSearchMode::LimitSwitchSearch {
                search_mode,
                swap_left_right,
            } => *search_mode as u8 + if *swap_left_right { 64 } else { 0 },
            ReferenceSearchMode::HomeSearch {
                search_mode,
                invert_home_switch,
            } => *search_mode as u8 + if *invert_home_switch { 128 } else { 0 },
        };
        [value, 0u8, 0u8, 0u8]
    }
}
impl WriteableTmcmAxisParameter for ReferenceSearchMode {}

axis_param_rw!(
    /// Reference search speed
    ///
    /// This value specifes the speed for roughly searching the reference switch.
    ReferenceSearchSpeed,
    u32, // PD42-1240 supports a range of 0..7999744
    194
);
impl ReferenceSearchSpeed {
    pub fn new(speed: u32) -> Self {
        ReferenceSearchSpeed(speed)
    }
}
impl TmcmAxisParameter for ReferenceSearchSpeed {}
impl ReadableTmcmAxisParameter for ReferenceSearchSpeed {}
impl WriteableTmcmAxisParameter for ReferenceSearchSpeed {}

axis_param_rw!(
    /// Reference switch speed
    ///
    /// This parameter specifes the speed for searching the switching point.
    /// It should be slower than parameter 194.
    ReferenceSwitchSpeed,
    u32, // PD42-1240 supports a range of 0..7999744
    195 
);
impl ReferenceSwitchSpeed {
    pub fn new(speed: u32) -> Self {
        ReferenceSwitchSpeed(speed)
    }
}
impl TmcmAxisParameter for ReferenceSwitchSpeed {}
impl ReadableTmcmAxisParameter for ReferenceSwitchSpeed {}
impl WriteableTmcmAxisParameter for ReferenceSwitchSpeed {}

axis_param_r!(
    /// End switch distance
    ///
    /// This parameter provides the distance between the end switches after
    /// executing the RFS command (with reference search mode 2 or 3).
    EndSwitchDistance,
    i32,
    196 
);
impl TmcmAxisParameter for EndSwitchDistance {}
impl ReadableTmcmAxisParameter for EndSwitchDistance {}

axis_param_r!(
    /// Last reference position
    ///
    /// TThis parameter contains the last position value before the position
    /// counter is set to zero during reference search.
    LastReferencePosition,
    i32,
    197
);
impl TmcmAxisParameter for LastReferencePosition {}
impl ReadableTmcmAxisParameter for LastReferencePosition {}

axis_param_rw!(
    /// Boost current
    ///
    /// Current used for acceleration and deceleration phases. If set to 0 the
    /// same current as set by axis parameter #6 will be used. Same scaling as
    /// with axis parameter #6.
    BoostCurrent,
    u8,
    200
);
impl BoostCurrent {
    pub fn new(current: u8) -> Self {
        BoostCurrent(current)
    }
}
impl TmcmAxisParameter for BoostCurrent {}
impl ReadableTmcmAxisParameter for BoostCurrent {}
impl WriteableTmcmAxisParameter for BoostCurrent {}

axis_param_rw!(
    /// Power down delay
    ///
    /// Standstill period before the motor current will be switched to standby
    /// current. The default value is 200 which means 2000ms.
    /// 
    /// Units are 10 ms
    PowerDownDelay,
    u16,
    214 
);
impl PowerDownDelay {
    pub fn new(delay: u16) -> Self {
        PowerDownDelay(delay)
    }
}
impl TmcmAxisParameter for PowerDownDelay {}
impl ReadableTmcmAxisParameter for PowerDownDelay {}
impl WriteableTmcmAxisParameter for PowerDownDelay {}
