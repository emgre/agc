use crate::cpu::TimePulse;
use crate::cpu::control_pulses::*;

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
pub type Actions = &'static [&'static ControlPulse];

impl Subinstruction {
    pub fn control_pulses(&self, t: TimePulse) -> Actions {
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
    t2: &[&RSC, &WG],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[],
    t7: &[&RG, &WB],
    t8: &[&RZ, &WS, &ST2],
    t9: &[&RB, &WG],
    t10: &[&RB, &WA],
    t11: &[],
    t12: &[],
};

pub static GOJ1: Subinstruction = Subinstruction {
    name: "GOJ1",
    t1: &[],
    t2: &[&RSC, &WG],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[],
    t7: &[],
    t8: &[&RSTRT, &WS, &WB],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static STD2: Subinstruction = Subinstruction {
    name: "STD2",
    t1: &[&RZ, &WY12, &CI],
    t2: &[&RSC, &WG, &NISQ],
    t3: &[],
    t4: &[],
    t5: &[],
    t6: &[&RU, &WZ],
    t7: &[],
    t8: &[&RAD, &WB, &WS],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static TC0: Subinstruction = Subinstruction {
    name: "TC0",
    t1: &[&RB, &WY12, &CI],
    t2: &[&RSC, &WG, &NISQ],
    t3: &[&RZ, &WQ],
    t4: &[],
    t5: &[],
    t6: &[&RU, &WZ],
    t7: &[],
    t8: &[&RAD, &WB, &WS],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};

pub static XCH0: Subinstruction = Subinstruction {
    name: "XCH0",
    t1: &[&RL10BB, &WS],
    t2: &[&RSC, &WG],
    t3: &[&RA, &WB],
    t4: &[],
    t5: &[&RG, &WA],
    t6: &[],
    t7: &[&RB, &WSC, &WG],
    t8: &[&RZ, &WS, &ST2],
    t9: &[],
    t10: &[],
    t11: &[],
    t12: &[],
};
