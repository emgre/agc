use crate::cpu::instructions::*;
use crate::cpu::registers::{AddressRegister, MemoryAddress, SequenceRegister};
use crate::memory::{ErasableStorage, FixedStorage, MemoryWord};
use crate::word::*;

mod control_pulses;
mod instructions;
mod registers;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TimePulse {
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
}

impl TimePulse {
    fn next(&self) -> Self {
        match self {
            Self::T1 => Self::T2,
            Self::T2 => Self::T3,
            Self::T3 => Self::T4,
            Self::T4 => Self::T5,
            Self::T5 => Self::T6,
            Self::T6 => Self::T7,
            Self::T7 => Self::T8,
            Self::T8 => Self::T9,
            Self::T9 => Self::T10,
            Self::T10 => Self::T11,
            Self::T11 => Self::T12,
            Self::T12 => Self::T1,
        }
    }
}

impl From<TimePulse> for usize {
    fn from(from: TimePulse) -> usize {
        match from {
            TimePulse::T1 => 1,
            TimePulse::T2 => 2,
            TimePulse::T3 => 3,
            TimePulse::T4 => 4,
            TimePulse::T5 => 5,
            TimePulse::T6 => 6,
            TimePulse::T7 => 7,
            TimePulse::T8 => 8,
            TimePulse::T9 => 9,
            TimePulse::T10 => 10,
            TimePulse::T11 => 11,
            TimePulse::T12 => 12,
        }
    }
}

pub struct Cpu {
    // These registers are visible to the programmer
    /// Accumulator
    pub a: W16,
    /// Low-order product
    pub l: W16,
    /// Return address register
    pub q: W16,
    /// Program counter
    pub z: W16,
    pub ebank: W3,
    pub fbank: W5,

    // These registers are hidden to the programmer. They are only used by control pulses
    /// Buffer register
    pub b: W16,
    /// Memory buffer register
    pub g: W16,
    /// Memory address register
    pub s: AddressRegister,
    /// Sequence register
    pub sq: SequenceRegister,
    /// Extend flip-flop
    pub ext: bool,
    /// Stage counter
    pub st: W3,

    /// Arithmetic X
    pub x: W16,
    /// Arithmetic Y
    pub y: W16,
    /// Carry flip-flop
    pub ci: bool,

    // Storage of the computer
    /// Erasable (read-write) memory storage
    erasable_storage: ErasableStorage,
    /// Fixed (read-only) memory storage
    fixed_storage: FixedStorage,

    // Emulation parameters
    pub current_timepulse: TimePulse,

    /// Value of S at T1
    ///
    /// This is necessary because even if another address is written
    /// to the S register, the original address is used when writing
    /// back to erasable memory
    current_s: AddressRegister,
    /// At next T12, read the next instruction into register SQ
    ///
    /// This is generated by control pulse NISQ.
    nisq: bool,
    /// Value of ST at next MCT
    next_st: W3,
    inhibit_interrupts: bool,
}

impl Cpu {
    /// Create a CPU from a fixed storage ROM
    ///
    /// All internal parameters will be initialized to zero and
    /// the CPU will be reset, ready to perform a GOJAM.
    pub fn new(fixed_storage: FixedStorage) -> Self {
        Cpu {
            a: W16::zero(),
            l: W16::zero(),
            q: W16::zero(),
            z: W16::zero(),
            ebank: W3::zero(),
            fbank: W5::zero(),

            b: W16::zero(),
            g: W16::zero(),
            s: AddressRegister::new(),
            sq: SequenceRegister::new(W6::zero(), false),
            ext: false,
            st: W3::from(0o1),
            x: W16::zero(),
            y: W16::zero(),
            ci: true,

            erasable_storage: ErasableStorage::new(),
            fixed_storage,

            current_timepulse: TimePulse::T1,
            current_s: AddressRegister::new(),
            nisq: false,
            next_st: W3::zero(),
            inhibit_interrupts: false,
        }
    }

    pub fn current_subinstruction(&self) -> &'static Subinstruction {
        // STD2 is always executed if ST = 0b010
        if self.st == W3::from(0b010) {
            return &STD2;
        }

        if !self.sq.is_extended() {
            // Non-extended subinstructions
            match self.sq.order_code().as_u16() {
                0b000 => match self.st.as_u16() {
                    0b000 => &TC0,
                    0b001 => &GOJ1,
                    _ => panic!("opcode {} with st {} does not exist", self.sq, self.st),
                },
                0b011 => match self.st.as_u16() {
                    0b000 => &CA0,
                    _ => panic!("opcode {} with st {} does not exist", self.sq, self.st),
                }
                0b101 => match self.sq.extended_code().as_u16() {
                    0b110|0b111 => match self.st.as_u16() {
                        0b000 => &XCH0,
                        _ => panic!("opcode {} with st {} does not exist", self.sq, self.st),
                    }
                    _ => unimplemented!("opcode {}", self.sq),
                }
                _ => unimplemented!("opcode {}", self.sq),
            }
        } else {
            // Extended subinstructions
            unimplemented!("opcode {}", self.sq)
        }
    }

    fn execute_control_pulses(&mut self, t: TimePulse) {
        let control_pulses = self.current_subinstruction().control_pulses(t);

        let mut wl = W16::zero();
        for control_pulse in control_pulses {
            wl |= (control_pulse.exec_write_wl)(self);
        }
        for control_pulse in control_pulses {
            (control_pulse.exec_read_wl)(self, wl);
        }
    }

    /// Run a single step, i.e. a single action
    pub fn step_control_pulse(&mut self) {
        // Execute the control pulses
        self.execute_control_pulses(self.current_timepulse);

        // Execute additional task
        match self.current_timepulse {
            TimePulse::T4 => {
                // Perform erasable memory read
                match self.current_s.address() {
                    MemoryAddress::UnswitchedErasableMemory(bank, address) => {
                        self.g = self
                            .erasable_storage
                            .read(bank, address)
                            .as_register_value();
                    }
                    MemoryAddress::SwitchedErasableMemory(address) => {
                        self.g = self
                            .erasable_storage
                            .read(self.ebank, address)
                            .as_register_value();
                    }
                    _ => (),
                };
            }
            TimePulse::T6 => {
                // Perform fixed memory read
                match self.current_s.address() {
                    MemoryAddress::UnswitchedFixedMemory(bank, address) => {
                        self.g = self
                            .fixed_storage
                            .read(bank.into(), address)
                            .as_register_value();
                    }
                    MemoryAddress::SwitchedFixedMemory(address) => {
                        // TODO: take into account super-bit
                        self.g = self
                            .fixed_storage
                            .read(self.ebank.into(), address)
                            .as_register_value();
                    }
                    _ => (),
                };
            }
            TimePulse::T10 => {
                // Perform erasable memory write
                match self.current_s.address() {
                    MemoryAddress::UnswitchedErasableMemory(bank, address) => {
                        self.erasable_storage.write(
                            bank,
                            address,
                            MemoryWord::with_proper_parity(self.g.into()),
                        );
                    }
                    MemoryAddress::SwitchedErasableMemory(address) => {
                        self.erasable_storage.write(
                            self.ebank,
                            address,
                            MemoryWord::with_proper_parity(self.g.into()),
                        );
                    }
                    _ => (),
                };
            }
            TimePulse::T12 => {
                // If NISQ was triggered, load next instruction into SQ
                // TODO: perhaps use the actual control pulses RB and WSQ?
                // TODO: should also re-enable some interrupts
                if self.nisq {
                    self.sq = SequenceRegister::new(W6::from(self.b >> 9), self.ext);
                    self.nisq = false;
                }

                // Update current S value
                self.current_s = self.s;

                // Set stage counter
                self.st = self.next_st;
                self.next_st = W3::zero();
            }
            _ => ()
        }

        // Increment timepulse counter
        self.current_timepulse = self.current_timepulse.next();
    }

    /// Run a single subinstruction, i.e. a single MCT
    pub fn step_subinstruction(&mut self) {
        // execute at least one control pulse
        self.step_control_pulse();

        // continue until we read T1
        while self.current_timepulse != TimePulse::T1 {
            self.step_control_pulse();
        }
    }

    pub fn current_subsintruction_name(&self) -> &'static str {
        self.current_subinstruction().name
    }

    // Read content of the adder unit
    fn u(&self) -> W16 {
        // TODO: do the actual calculation here, this is way too imprecise
        let mut result = self.x.as_u16() + self.y.as_u16();
        if self.ci {
            result += 1;
        }
        W16::from(result)
    }
}
