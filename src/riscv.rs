use crate::arch::{Architecture, CpuOps};

pub struct Riscv {}

pub struct RiscvCpuOps {}

impl CpuOps for RiscvCpuOps {
    fn cpu_do_init(&mut self, master: bool) {
        println!("RISC-V Init! of {}", master)
    }
    fn cpu_do_wakeup(&self) {
        println!("RISC-V Wakeup!")
    }
    fn cpu_do_idle(&self) {
        println!("RISC-V Idle!")
    }
}

impl Architecture for Riscv {
    type Ops = RiscvCpuOps;
    fn get_cpu_ops(&self) -> Self::Ops {
        RiscvCpuOps {}
    }
} 