//! `Instruction`s available for the generic TMCM module.

use instructions::DirectInstruction;
use instructions::Instruction;

pub use instructions::{
    MoveOperation, ReferenceSearchAction, CALC, GIO, MST, MVP, RFS, ROL, ROR, SIO,
};

/// SAP - Set Axis Parameter
///
/// Most parameters of a TMCM module can be adjusted individually for each axis.
/// Although  these parameters vary widely in their formats (1 to 24 bits, signed or unsigned)
/// and physical locations (TMC428, TMC453, controller RAM, controller EEPROM),
/// they all can be set by this function.
#[derive(Debug, PartialEq)]
pub struct SAP {
    motor_number: u8,
    parameter_number: u8,
    operand: [u8; 4],
}
impl SAP {
    pub fn new(motor_number: u8, parameter_number: u8, operand: [u8; 4]) -> SAP {
        SAP {
            motor_number,
            parameter_number,
            operand,
        }
    }
}
impl Instruction for SAP {
    const INSTRUCTION_NUMBER: u8 = 5;

    fn operand(&self) -> [u8; 4] {
        self.operand
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for SAP {
    type Return = ();
}

/// GAP - Get Axis Parameter
///
/// Most parameters of a TMCM module can be adjusted individually for each axis.
/// Although  these parameters vary widely in their formats (1 to 24 bits, signed or unsigned)
/// and physical locations (TMC428, TMC453, controller RAM, controller EEPROM),
/// they all can be read by this function.
#[derive(Debug, PartialEq)]
pub struct GAP {
    motor_number: u8,
    parameter_number: u8,
}
impl GAP {
    pub fn new(motor_number: u8, parameter_number: u8) -> GAP {
        GAP {
            motor_number,
            parameter_number,
        }
    }
}
impl Instruction for GAP {
    const INSTRUCTION_NUMBER: u8 = 6;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for GAP {
    type Return = [u8; 4];
}

/// STAP - Store Axis Parameter
///
/// Axis parameters are located in RAM memory, so modifications are lost at power down.
/// This instruction enables permanent storing.
#[derive(Debug, PartialEq)]
pub struct STAP {
    motor_number: u8,
    parameter_number: u8,
}
impl STAP {
    pub fn new(motor_number: u8, parameter_number: u8) -> STAP {
        STAP {
            motor_number,
            parameter_number,
        }
    }
}
impl Instruction for STAP {
    const INSTRUCTION_NUMBER: u8 = 7;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for STAP {
    type Return = ();
}

/// RSAP - Restore Axis Parameter
///
/// For all configuration-related axis parameters, non-volatile memory locations are provided.
/// By default, most parameters are automatically restored after power up (see axis parameter list in
/// chapter 4). A single parameter that has been changed before can be reset by this instruction.
#[derive(Debug, PartialEq)]
pub struct RSAP {
    motor_number: u8,
    parameter_number: u8,
}
impl RSAP {
    pub fn new(motor_number: u8, parameter_number: u8) -> STAP {
        STAP {
            motor_number,
            parameter_number,
        }
    }
}
impl Instruction for RSAP {
    const INSTRUCTION_NUMBER: u8 = 8;

    fn operand(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for RSAP {
    type Return = ();
}
