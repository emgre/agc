use crate::word::*;
use std::fmt;

/// 7-bit register used to keep track of what subinstruction
/// is currently being executed
#[derive(Copy, Clone)]
pub struct SequenceRegister {
    inner: W7,
}

impl SequenceRegister {
    pub fn new(value: W6, extend: bool) -> Self {
        let mut value = W7::from(value);
        if extend {
            value.set(6, true);
        }
        Self { inner: value }
    }

    pub fn is_extended(&self) -> bool {
        self.inner.get(6)
    }

    pub fn order_code(&self) -> W3 {
        (self.inner >> 3).into()
    }

    pub fn extended_code(&self) -> W3 {
        self.inner.into()
    }

    pub fn set_extended(&mut self) {
        self.inner.set(6, true);
    }

    pub fn inner(self) -> W7 {
        self.inner
    }
}

impl fmt::Display for SequenceRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

pub enum MemoryAddress {
    Register(W3),
    UnswitchedErasableMemory(W3, W8),
    SwitchedErasableMemory(W8),
    UnswitchedFixedMemory(W5, W10),
    SwitchedFixedMemory(W10),
}

/// A 12-bit address register (the S register)
#[derive(Copy, Clone)]
pub struct AddressRegister {
    inner: W12,
}

impl AddressRegister {
    pub fn zero() -> Self {
        Self::from(W12::zero())
    }

    pub fn from(value: W12) -> Self {
        Self { inner: value }
    }

    pub fn address(&self) -> MemoryAddress {
        if self.inner.as_u16() < 8 {
            MemoryAddress::Register(self.inner.into())
        } else {
            match W2::from(self.inner >> 10).as_u16() {
                0b00 => {
                    // Erasable memory
                    let address = W8::from(self.inner);
                    match W2::from(self.inner >> 8).as_u16() {
                        0b11 => MemoryAddress::SwitchedErasableMemory(address),
                        _ => {
                            let bank = W3::from(self.inner >> 8);
                            MemoryAddress::UnswitchedErasableMemory(bank, address)
                        }
                    }
                }
                0b01 => {
                    // Switched fixed memory
                    let address = W10::from(self.inner);
                    MemoryAddress::SwitchedFixedMemory(address)
                }
                _ => {
                    // Unswitched fixed memory
                    let bank = W5::from(self.inner >> 10);
                    let address = W10::from(self.inner);
                    MemoryAddress::UnswitchedFixedMemory(bank, address)
                }
            }
        }
    }

    pub fn inner(self) -> W12 {
        self.inner
    }
}

impl fmt::Display for AddressRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Copy, Clone)]
pub struct BranchRegister {
    inner: W2,
}

impl BranchRegister {
    pub fn zero() -> Self {
        Self::from(W2::zero())
    }

    pub fn from(value: W2) -> Self {
        Self {
            inner: W2::from(value)
        }
    }

    pub fn br1(&self) -> bool {
        self.inner.get(0)
    }

    pub fn set_br1(&mut self, value: bool) {
        self.inner.set(0, value);
    }

    pub fn br2(&self) -> bool {
        self.inner.get(1)
    }

    pub fn set_br2(&mut self, value: bool) {
        self.inner.set(1, value);
    }

    pub fn set(&mut self, value: W2) {
        self.inner = value;
    }

    pub fn reset(&mut self) {
        self.inner = W2::zero();
    }

    pub fn inner(self) -> W2 {
        self.inner
    }
}

impl fmt::Display for BranchRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
