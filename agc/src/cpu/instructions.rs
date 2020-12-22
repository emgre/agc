use crate::cpu::TimePulse;
use crate::cpu::control_pulses::*;
use crate::word::W2;

use super::registers::BranchRegister;

pub struct Subinstruction {
    pub name: &'static str,
    pub t1: Actions,
    pub t2: Actions,
    pub t3: Actions,
    pub t4: Actions,
    pub t5: Actions,
    pub t6: Actions,
    pub t7: Actions,
    pub t8: Actions,
    pub t9: Actions,
    pub t10: Actions,
    pub t11: Actions,
    pub t12: Actions,
}
pub type Actions = &'static [Action];

pub enum Action {
    BrXX(&'static ControlPulse),
    BrX0(&'static ControlPulse),
    Br0X(&'static ControlPulse),
    BrX1(&'static ControlPulse),
    Br1X(&'static ControlPulse),
    Br00(&'static ControlPulse),
    Br01(&'static ControlPulse),
    Br10(&'static ControlPulse),
    Br11(&'static ControlPulse),
}

impl Action {
    pub fn execute(&self, br: BranchRegister) -> bool {
        match self {
            Self::BrXX(_) => true,
            Self::BrX0(_) => !br.br1(),
            Self::Br0X(_) => !br.br2(),
            Self::BrX1(_) => br.br1(),
            Self::Br1X(_) => br.br2(),
            Self::Br00(_) => br.inner() == W2::from(0b00),
            Self::Br01(_) => br.inner() == W2::from(0b01),
            Self::Br10(_) => br.inner() == W2::from(0b10),
            Self::Br11(_) => br.inner() == W2::from(0b11),
        }
    }

    pub fn control_pulse(&self) -> &'static ControlPulse {
        match self {
            Self::BrXX(control_pulse) => control_pulse,
            Self::BrX0(control_pulse) => control_pulse,
            Self::Br0X(control_pulse) => control_pulse,
            Self::BrX1(control_pulse) => control_pulse,
            Self::Br1X(control_pulse) => control_pulse,
            Self::Br00(control_pulse) => control_pulse,
            Self::Br01(control_pulse) => control_pulse,
            Self::Br10(control_pulse) => control_pulse,
            Self::Br11(control_pulse) => control_pulse,
        }
    }
}

impl Subinstruction {
    pub fn actions(&self, t: TimePulse) -> Actions {
        match t {
            TimePulse::T1 => self.t1,
            TimePulse::T2 => self.t2,
            TimePulse::T3 => self.t3,
            TimePulse::T4 => self.t4,
            TimePulse::T5 => self.t5,
            TimePulse::T6 => self.t6,
            TimePulse::T7 => self.t7,
            TimePulse::T8 => self.t8,
            TimePulse::T9 => self.t9,
            TimePulse::T10 => self.t10,
            TimePulse::T11 => self.t11,
            TimePulse::T12 => self.t12,
        }
    }
}

pub static CA0: Subinstruction = Subinstruction {
    name: "CA0",
    t1: &[],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG)],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[],
    t7: &[Action::BrXX(&RG), Action::BrXX(&WB)],
    t8: &[Action::BrXX(&RZ), Action::BrXX(&WS), Action::BrXX(&ST2)],
    t9: &[Action::BrXX(&RB), Action::BrXX(&WG)],
    t10: &[Action::BrXX(&RB), Action::BrXX(&WA)],
    t11: &[],
    t12: &[],
};

pub static GOJ1: Subinstruction = Subinstruction {
    name: "GOJ1",
    t1: &[],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG)],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[],
    t7: &[],
    t8: &[Action::BrXX(&RSTRT), Action::BrXX(&WS), Action::BrXX(&WB)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static INCR0: Subinstruction = Subinstruction {
    name: "INCR0",
    t1: &[Action::BrXX(&RL10BB), Action::BrXX(&WS)],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG)],
    t3: &[],
    t4: &[],
    t5: &[Action::BrXX(&RG), Action::BrXX(&WY), Action::BrXX(&TSGN), Action::BrXX(&TMZ), Action::BrXX(&TPZG)],
    t6: &[Action::BrXX(&PONEX)],
    t7: &[Action::BrXX(&RU), Action::BrXX(&WSC), Action::BrXX(&WG), Action::BrXX(&WOVR)],
    t8: &[Action::BrXX(&RZ), Action::BrXX(&WS), Action::BrXX(&ST2)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static STD2: Subinstruction = Subinstruction {
    name: "STD2",
    t1: &[Action::BrXX(&RZ), Action::BrXX(&WY12), Action::BrXX(&CI)],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG), Action::BrXX(&NISQ)],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[Action::BrXX(&RU), Action::BrXX(&WZ)],
    t7: &[],
    t8: &[Action::BrXX(&RAD), Action::BrXX(&WB), Action::BrXX(&WS)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static TC0: Subinstruction = Subinstruction {
    name: "TC0",
    t1: &[Action::BrXX(&RB), Action::BrXX(&WY12), Action::BrXX(&CI)],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG), Action::BrXX(&NISQ)],
    t3: &[Action::BrXX(&RZ), Action::BrXX(&WQ)],
    t4: &[],
    t5: &[],
    t6: &[Action::BrXX(&RU), Action::BrXX(&WZ)],
    t7: &[],
    t8: &[Action::BrXX(&RAD), Action::BrXX(&WB), Action::BrXX(&WS)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static TCF0: Subinstruction = Subinstruction {
    name: "TCF0",
    t1: &[Action::BrXX(&RB), Action::BrXX(&WY12), Action::BrXX(&CI)],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG), Action::BrXX(&NISQ)],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[Action::BrXX(&RU), Action::BrXX(&WZ)],
    t7: &[],
    t8: &[Action::BrXX(&RAD), Action::BrXX(&WB), Action::BrXX(&WS)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static TS0: Subinstruction = Subinstruction {
    name: "TS0",
    t1: &[Action::BrXX(&RL10BB), Action::BrXX(&WS)],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG)],
    t3: &[Action::BrXX(&RA), Action::BrXX(&WB), Action::BrXX(&TOV)],
    t4: &[Action::BrXX(&RZ), Action::BrXX(&WY12), Action::Br01(&CI), Action::Br10(&CI)],
    t5: &[Action::Br01(&RB1), Action::Br01(&WA), Action::Br10(&R1C), Action::Br10(&WA)],
    t6: &[Action::BrXX(&RU), Action::BrXX(&WZ)],
    t7: &[Action::BrXX(&RB), Action::BrXX(&WSC), Action::BrXX(&WG)],
    t8: &[Action::BrXX(&RZ), Action::BrXX(&WS), Action::BrXX(&ST2)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static XCH0: Subinstruction = Subinstruction {
    name: "XCH0",
    t1: &[Action::BrXX(&RL10BB), Action::BrXX(&WS)],
    t2: &[Action::BrXX(&RSC), Action::BrXX(&WG)],
    t3: &[Action::BrXX(&RA), Action::BrXX(&WB)],
    t4: &[],
    t5: &[Action::BrXX(&RG), Action::BrXX(&WA)],
    t6: &[],
    t7: &[Action::BrXX(&RB), Action::BrXX(&WSC), Action::BrXX(&WG)],
    t8: &[Action::BrXX(&RZ), Action::BrXX(&WS), Action::BrXX(&ST2)],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};
