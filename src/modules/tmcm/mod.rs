//! A `TMCM` type useable with TMCM modules other than TMCM-100 and Monopack 2.

use lib::marker::PhantomData;
use lib::ops::Deref;

pub mod axis_parameters;
pub mod instructions;

use interior_mut::InteriorMut;

use instructions::DirectInstruction;
use AxisParameter;
use Command;
use Error;
use Instruction;
use Interface;
use ReadableAxisParameter;
use Return;
use Status;
use WriteableAxisParameter;

/// This type represennts a TMCM module other than TMCM-100 and Monopack 2.
#[derive(Debug)]
pub struct TmcmModule<
    'a,
    IF: Interface + 'a,
    Cell: InteriorMut<'a, IF>,
    T: Deref<Target = Cell> + 'a,
> {
    /// The module address
    address: u8,
    interface: T,
    pd1: PhantomData<&'a IF>,
    pd2: PhantomData<&'a T>,
}

impl<'a, IF: Interface, Cell: InteriorMut<'a, IF>, T: Deref<Target = Cell>>
    TmcmModule<'a, IF, Cell, T>
{
    /// Create a new module
    pub fn new(interface: T, address: u8) -> Self {
        TmcmModule {
            address,
            interface,
            pd1: PhantomData {},
            pd2: PhantomData {},
        }
    }

    /// Synchronously write a command and wait for the Reply
    pub fn write_command<Instruction: TmcmInstruction + DirectInstruction>(
        &'a self,
        instruction: Instruction,
    ) -> Result<Instruction::Return, Error<IF::Error>> {
        let mut interface = self
            .interface
            .borrow_int_mut()
            .or(Err(Error::InterfaceUnavailable))?;
        interface
            .transmit_command(&Command::new(self.address, instruction))
            .map_err(|e| Error::InterfaceError(e))?;
        let reply = interface
            .receive_reply()
            .map_err(|e| Error::InterfaceError(e))?;
        match reply.status() {
            Status::Ok(_) => Ok(<Instruction::Return as Return>::from_operand(
                reply.operand(),
            )),
            Status::Err(e) => Err(e.into()),
        }
    }
}

/// An `AxisParameter` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait TmcmInstruction: Instruction {}

/// An `AxisParameter` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait TmcmAxisParameter: AxisParameter {}

/// A `ReadableAxisParameter` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait ReadableTmcmAxisParameter: ReadableAxisParameter {}

/// A `WriteableAxisParamtere` useable with all TMCM modules other than TMCM-100 and Monopack 2.
pub trait WriteableTmcmAxisParameter: WriteableAxisParameter {}
