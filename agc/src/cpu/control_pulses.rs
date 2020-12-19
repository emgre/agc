use crate::cpu::registers::{AddressRegister, MemoryAddress};
use crate::cpu::Cpu;
use crate::word::*;

type WriteLine = W16;

/// Control pulses are sequence generator signals which regulates data flow within the AGC.
///
/// For simulation purposes, each control pulse is divided in two steps:
/// - Execute operations that modifies the WL
/// - Execute operations that reads from the WL
/// The majority of control pulses will implement only one of the two steps.
pub struct ControlPulse {
    /// Mnemonic of the control pulse
    pub name: &'static str,
    /// Execute operations that writes to the WL (executed first)
    ///
    /// The returned result is ORed to the write line
    pub exec_write_wl: fn(&mut Cpu) -> WriteLine,
    /// Execute operations that reads from the WL (executed second)
    pub exec_read_wl: fn(&mut Cpu, WriteLine),
}

/// Insert carry bit into bit position 1 of the adder.
pub static CI: ControlPulse = ControlPulse {
    name: "CI",
    exec_write_wl: |cpu| {
        cpu.ci = true;
        W16::zero()
    },
    exec_read_wl: exec_read_wl_null,
};

/// Load next instruction into register SQ at next T12.
///
/// Also frees certain restrictions; permits execution of instruction RUPT and counter instructions.
/// See control pulses RB and WSQ.
pub static NISQ: ControlPulse = ControlPulse {
    name: "NISQ",
    exec_write_wl: |cpu| {
        cpu.nisq = true;
        W16::zero()
    },
    exec_read_wl: exec_read_wl_null,
};

/// Read address of next cycle.
///
/// RAD appears at the end of an instruction and is normally interpreted
/// as RG. If the next instruction is INHINT, RELINT, or EXTEND, RAD is
/// interpreted as RZ and ST2 instead. It also sets the proper flip-flops
/// in the SQ circuitry. See p. 4-365 for more details.
pub static RAD: ControlPulse = ControlPulse {
    name: "RAD",
    exec_write_wl: |cpu| {
        let is_special = match cpu.g.as_u16() {
            0o00003 => { // RELINT
                cpu.inhibit_interrupts = false;
                true
            }
            0o00004 => { // INHINT
                cpu.inhibit_interrupts = true;
                true
            }
            0o00006 => { // EXTEND
                cpu.ext = true;
                true
            }
            _ => false
        };

        if is_special {
            cpu.next_st |= 0b010u16;
            cpu.z
        } else {
            cpu.g
        }
    },
    exec_read_wl: exec_read_wl_null,
};

/// Read bits 16 through 1 of register A to WL's 16 through 1.
pub static RA: ControlPulse = ControlPulse {
    name: "RA",
    exec_write_wl: |cpu| cpu.a,
    exec_read_wl: exec_read_wl_null,
};

/// Read bits 16 through 1 of register B to WL's 16 through 1.
pub static RB: ControlPulse = ControlPulse {
    name: "RB",
    exec_write_wl: |cpu| cpu.b,
    exec_read_wl: exec_read_wl_null,
};

/// Read bits 16 through 1 of register G to WL's 16 through 1.
pub static RG: ControlPulse = ControlPulse {
    name: "RG",
    exec_write_wl: |cpu| cpu.g,
    exec_read_wl: exec_read_wl_null,
};

/// Read low 10 bits of register B to WL's 10 through 1.
pub static RL10BB: ControlPulse = ControlPulse {
    name: "RL10BB",
    exec_write_wl: |cpu| W10::from(cpu.b).into(),
    exec_read_wl: exec_read_wl_null,
};

/// Read the content of register CP defined by the content of register S; bits 16 through 1 are read
/// to WL's 16 through 1.
///
/// See ND-1021042, p. 4-373/4-374 for details
pub static RSC: ControlPulse = ControlPulse {
    name: "RSC",
    exec_write_wl: |cpu| match cpu.s.address() {
        MemoryAddress::Register(value) => match value.as_u16() {
            0o0 => cpu.a,
            0o1 => cpu.l,
            0o2 => cpu.q,
            0o3 => todo!(),
            0o4 => todo!(),
            0o5 => todo!(),
            0o6 => todo!(),
            0o7 => todo!(),
            _ => panic!("Unexpected 3-bit value"),
        },
        _ => W16::zero(),
    },
    exec_read_wl: exec_read_wl_null,
};

/// Place octal 4000 (start address) on WL's.
pub static RSTRT: ControlPulse = ControlPulse {
    name: "RSTRT",
    exec_write_wl: |_cpu| W16::from(0o4000),
    exec_read_wl: exec_read_wl_null,
};

/// Read bits 16 through 1 of adder output gates (U) to WL's 16 through 1.
pub static RU: ControlPulse = ControlPulse {
    name: "RU",
    exec_write_wl: |cpu| cpu.u(),
    exec_read_wl: exec_read_wl_null,
};

/// Read bits 16 through 1 of register Z to WL's 16 through 1.
pub static RZ: ControlPulse = ControlPulse {
    name: "RZ",
    exec_write_wl: |cpu| cpu.z,
    exec_read_wl: exec_read_wl_null,
};

/// Set stage 1 flip-flop to logic ONE at next T12.
pub static ST1: ControlPulse = ControlPulse {
    name: "ST1",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, _wl| {
        cpu.next_st |= 0b001u16;
    },
};

/// Set stage 2 flip-flop to logic ONE at next T12.
pub static ST2: ControlPulse = ControlPulse {
    name: "ST2",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, _wl| {
        cpu.next_st |= 0b010u16;
    },
};

/// Clear register A and write the contents of WL's 16 through 1 into bit positions 16 through 1.
pub static WA: ControlPulse = ControlPulse {
    name: "WA",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        cpu.a = wl;
    },
};

/// Clear register B and write the contents of WL's 16 through 1 into bit positions 16 through 1.
pub static WB: ControlPulse = ControlPulse {
    name: "WB",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        cpu.b = wl;
    },
};

/// Clear register G and write the contents of WL's 16 through 1 into bit positions 16 through 1
/// (except if register S contains octal addresses 20 through 23).
pub static WG: ControlPulse = ControlPulse {
    name: "WG",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        // TODO: Check if S contains octal address 20 through 23
        cpu.g = wl;
    },
};

/// Clear register S and write the contents of WL's 12 through 1 into bit positions 12 through 1.
pub static WS: ControlPulse = ControlPulse {
    name: "WS",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        cpu.s = AddressRegister::from(wl.into());
    },
};

/// Clear the CP register specified by the contents of register S and write the contents of WL's 16
/// through 1 into bit positions 16 through 1 of this register.
pub static WSC: ControlPulse = ControlPulse {
    name: "WSC",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        match cpu.s.address() {
            MemoryAddress::Register(reg) => match reg.as_u16() {
                0o0 => cpu.a = wl,
                0o1 => cpu.l = wl,
                0o2 => cpu.q = wl,
                0o3 => todo!(),
                0o4 => todo!(),
                0o5 => cpu.z = wl,
                0o6 => todo!(),
                0o7 => (), // Do nothing
                _ => panic!("Unexpected 3-bit value"),
            }
            _ => (), // Do nothing
        }
    },
};

/// Clear register Q and write the contents of WL's 16 through 1 into bit positions 16 through 1.
pub static WQ: ControlPulse = ControlPulse {
    name: "WQ",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        cpu.q = wl;
    },
};

/// Clear registers X and Y; write the contents of WL’s 12 through 1 into bit positions 12 through 1
/// of register Y. Also clears the carry bit.
pub static WY12: ControlPulse = ControlPulse {
    name: "WY12",
    exec_write_wl: |cpu| {
        cpu.x = W16::zero();
        cpu.y = W16::zero();
        cpu.ci = false;
        W16::zero()
    },
    exec_read_wl: |cpu, wl| {
        cpu.y = wl & 0b000_111_111_111_111u16;
    },
};

/// Clear register Z and write the contents of WL's 16 through 1 into bit positions 16 through 1.
pub static WZ: ControlPulse = ControlPulse {
    name: "WZ",
    exec_write_wl: exec_write_wl_null,
    exec_read_wl: |cpu, wl| {
        cpu.z = wl;
    },
};

// Helper functions
fn exec_write_wl_null(_cpu: &mut Cpu) -> WriteLine {
    W16::zero()
}

fn exec_read_wl_null(_cpu: &mut Cpu, _wl: WriteLine) {}
