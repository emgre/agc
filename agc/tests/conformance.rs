//! This test uses a Verilog simulation mainly written by Mike Stewart
//! available here: https://github.com/virtualagc/agc_simulation
//!
//! I wrote a custom verilog script that outputs in a CSV file the status
//! of all registers at the beginning of each time pulse. For conformance,
//! I run the Rust emulator and check at each time pulse if the registers
//! fit with the Verilog simulation. Since the Verilog simulation is
//! accurate at a transistor-level, it should prove that the Rust emulator
//! is also accurate.
//!
//! The `test_agc.v` file is the Verilog file I used to create the
//! `verilog_sim.csv` file with the `agc_simulation` package.

use std::path::PathBuf;

use agc::cpu::Cpu;
use agc::memory::load_yayul_img_file;
use agc::word::*;

const NUM_SUBINSTRUCTIONS: usize = 18;

#[test]
fn conformance() {
    // Setup emulator
    let mut filepath = PathBuf::from("");
    filepath.push("..");
    filepath.push("listings");
    filepath.push("Aurora12.bin");
    let fixed_memory = load_yayul_img_file(filepath).unwrap();
    let mut cpu = Cpu::new(fixed_memory);

    // Load Verilog simulation data
    let verilog_sim = include_str!("verilog_sim.csv");

    for (line_num, line) in verilog_sim
        .lines()
        .enumerate()
        .skip(1)
        .take(NUM_SUBINSTRUCTIONS * 12)
    {
        let line_num = line_num + 1;

        // Read register status from Verilog file
        let expected_registers = RegisterStatus::parse(line, line_num);

        // Compare the registers
        let actual_registers = RegisterStatus::from_cpu(&cpu);
        assert_eq!(
            actual_registers, expected_registers,
            "different registers at line {}",
            line_num
        );

        // Step the simulation
        cpu.step_control_pulse();
    }
}

#[derive(Debug, PartialEq)]
struct RegisterStatus {
    // Public registers
    a: W16,
    l: W16,
    q: W16,
    z: W16,
    ebank: W3,
    fbank: W5,

    // Private registers
    b: W16,
    g: W16,
    s: W12,
    sq: W6,
    st: W3,
    x: W16,
    y: W16,
    br: W2,
}

impl RegisterStatus {
    fn parse(line: &str, line_num: usize) -> Self {
        let parse_octal = |src: Option<&str>, register_name: &str| -> u16 {
            match src {
                Some(value) => u16::from_str_radix(value, 8).expect(&format!(
                    "invalid \"{}\" value at line {}",
                    register_name, line_num
                )),
                None => panic!("missing \"{}\" value", register_name),
            }
        };

        let mut it = line.split_terminator(';');

        // Skip some information data
        it.next();
        it.next();

        // Extract the public registers
        let a = W16::from(parse_octal(it.next(), "A"));
        let l = W16::from(parse_octal(it.next(), "L"));
        let q = W16::from(parse_octal(it.next(), "Q"));
        let z = W16::from(parse_octal(it.next(), "Z"));
        let ebank = W3::from(parse_octal(it.next(), "EBANK"));
        let fbank = W5::from(parse_octal(it.next(), "FBANK"));

        // Extract the private registers
        let b = W16::from(parse_octal(it.next(), "B"));
        let g = W16::from(parse_octal(it.next(), "G"));
        let s = W12::from(parse_octal(it.next(), "S"));
        let sq = W6::from(parse_octal(it.next(), "SQ"));
        let st = W3::from(parse_octal(it.next(), "ST"));
        let x = W16::from(parse_octal(it.next(), "X"));
        let y = W16::from(parse_octal(it.next(), "Y"));
        let br = W2::from(parse_octal(it.next(), "BR"));

        Self {
            a,
            l,
            q,
            z,
            ebank,
            fbank,
            b,
            g,
            s,
            sq,
            st,
            x,
            y,
            br,
        }
    }

    fn from_cpu(cpu: &Cpu) -> Self {
        Self {
            a: cpu.a,
            l: cpu.l,
            q: cpu.q,
            z: cpu.z,
            ebank: cpu.ebank,
            fbank: cpu.fbank,

            b: cpu.b,
            g: cpu.g,
            s: cpu.s.inner(),
            sq: cpu.sq.inner().into(),
            st: cpu.st,
            x: cpu.x,
            y: cpu.y,
            br: cpu.br.inner(),
        }
    }
}
