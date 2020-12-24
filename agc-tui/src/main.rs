use agc::cpu::Cpu;
use agc::memory::load_yayul_img_file;
use crossterm::cursor::*;
use crossterm::event::*;
use crossterm::style::*;
use crossterm::terminal::*;
use crossterm::*;
use std::io::{stdout, Stdout, Write};
use std::path::PathBuf;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        SetTitle("AGC Emulator"),
        Clear(ClearType::All), // Clear the terminal
        MoveTo(0, 0),          // Move to top-left
        Hide,                  // Hide the cursor
        DisableLineWrap,       // Disable automatic line wrapping
    )?;

    // Initialize the emulator
    let mut cpu = init_emulator()?;

    // Initialize the registers
    let mut registers = Registers::new();

    // Run the emulator
    redraw(&mut stdout, &cpu, &mut registers)?;
    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Right => {
                    cpu.step_control_pulse();
                }
                KeyCode::Down => {
                    cpu.step_subinstruction();
                }
                KeyCode::Esc => {
                    break;
                }
                _ => (),
            },
            _ => (),
        }

        redraw(&mut stdout, &cpu, &mut registers)?;
    }

    // Restore terminal
    execute!(
        stdout,
        //SetSize(original_size.0, original_size.1), // Restore original size
        ResetColor,            // Reset color to default
        Clear(ClearType::All), // Clear the terminal
        MoveTo(0, 0),          // Move to top-left
        Show,                  // Show cursor
        EnableLineWrap,        // Enable automatic line wrapping
    )?;
    disable_raw_mode()?;

    Ok(())
}

fn init_emulator() -> std::result::Result<Cpu, Box<dyn std::error::Error>> {
    let mut filepath = PathBuf::from(file!());
    filepath.pop();
    filepath.pop();
    filepath.pop();
    filepath.push("listings");
    filepath.push("Aurora12.bin");
    let fixed_memory = load_yayul_img_file(filepath)?;
    Ok(Cpu::new(fixed_memory))
}

fn redraw(stdout: &mut Stdout, cpu: &Cpu, registers: &mut Registers) -> Result<()> {
    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(PrintStyledContent(
            format!(
                "T{:02} - {}",
                usize::from(cpu.current_timepulse),
                cpu.current_subsintruction_name()
            )
            .reverse(),
        ))?
        .queue(MoveToNextLine(1))?;

    // Print the next control pulses
    let control_pulses = cpu
        .current_subinstruction()
        .actions(cpu.current_timepulse)
        .iter()
        .filter(|action| action.execute(cpu.br))
        .map(|action| action.control_pulse().name)
        .collect::<Vec<_>>()
        .join(" ");
    stdout.queue(Print(format!("Next pulses: [{}]", control_pulses)))?;
    stdout.queue(MoveToNextLine(1))?;

    registers.print_public_registers(stdout, cpu)?;
    stdout.queue(MoveToNextLine(1))?;
    registers.print_private_registers(stdout, cpu)?;
    stdout.queue(MoveToNextLine(1))?;

    stdout.flush()?;

    Ok(())
}

struct Registers {
    // Public registers
    a: PrintedRegister,
    l: PrintedRegister,
    q: PrintedRegister,
    z: PrintedRegister,
    ebank: PrintedRegister,
    fbank: PrintedRegister,

    // Private registers
    b: PrintedRegister,
    g: PrintedRegister,
    s: PrintedRegister,
    sq: PrintedRegister,
    st: PrintedRegister,
    x: PrintedRegister,
    y: PrintedRegister,
    ci: PrintedRegister,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: PrintedRegister::new("A", |cpu| cpu.a.to_string()),
            l: PrintedRegister::new("L", |cpu| cpu.l.to_string()),
            q: PrintedRegister::new("Q", |cpu| cpu.q.to_string()),
            z: PrintedRegister::new("Z", |cpu| cpu.z.to_string()),
            ebank: PrintedRegister::new("EBANK", |cpu| cpu.ebank.to_string()),
            fbank: PrintedRegister::new("FBANK", |cpu| cpu.fbank.to_string()),

            b: PrintedRegister::new("B", |cpu| cpu.b.to_string()),
            g: PrintedRegister::new("G", |cpu| cpu.g.to_string()),
            s: PrintedRegister::new("S", |cpu| cpu.s.to_string()),
            sq: PrintedRegister::new("SQ", |cpu| cpu.sq.to_string()),
            st: PrintedRegister::new("ST", |cpu| cpu.st.to_string()),
            x: PrintedRegister::new("X", |cpu| cpu.x.to_string()),
            y: PrintedRegister::new("Y", |cpu| cpu.y.to_string()),
            ci: PrintedRegister::new("CI", |cpu| cpu.ci.to_string()),
        }
    }

    fn print_public_registers(&mut self, stdout: &mut Stdout, cpu: &Cpu) -> Result<()> {
        self.a.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.l.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.q.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.z.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.ebank.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.fbank.print(stdout, cpu)?;
        Ok(())
    }

    fn print_private_registers(&mut self, stdout: &mut Stdout, cpu: &Cpu) -> Result<()> {
        self.b.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.g.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.s.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.sq.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.st.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.x.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.y.print(stdout, cpu)?;
        stdout.queue(Print(" "))?;
        self.ci.print(stdout, cpu)?;
        Ok(())
    }
}

struct PrintedRegister {
    name: &'static str,
    getter: fn(&Cpu) -> String,
    last_value: Option<String>,
}

impl PrintedRegister {
    fn new(name: &'static str, getter: fn(&Cpu) -> String) -> Self {
        Self {
            name,
            getter,
            last_value: None,
        }
    }

    fn print(&mut self, stdout: &mut Stdout, cpu: &Cpu) -> Result<()> {
        // Get the current value
        let current_value = (self.getter)(cpu);

        // Check if we need to highlight the value because it changed
        let reverse = if let Some(old_value) = &self.last_value {
            old_value != &current_value
        } else {
            false
        };

        // Update the stored last value
        self.last_value.replace(current_value.clone());

        stdout.queue(Print(format!("{}: ", self.name)))?;

        // Print the value
        if reverse {
            stdout.queue(PrintStyledContent(current_value.reverse()))?;
        } else {
            stdout.queue(Print(current_value))?;
        }

        Ok(())
    }
}
