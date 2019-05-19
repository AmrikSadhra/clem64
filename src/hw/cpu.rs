const NUM_GPR: usize = 32;
const NUM_FPR: usize = 32;

pub enum LlBitStatus {
    LOAD,
    STORE // No idea
}

pub struct Cpu {
    reg_gpr: [u64; NUM_GPR], // r0 hardwired to zero, r31 is the link register used by JAL and JALR
    reg_fpr: [f64; NUM_FPR],

    reg_pc: u64,
    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: LlBitStatus,

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: cp0::Cp0
}