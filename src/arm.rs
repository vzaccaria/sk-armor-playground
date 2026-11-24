use crate::arch::{Architecture, CpuOps};

pub struct Arm {}

pub struct ArmCpuOps {}

impl CpuOps for ArmCpuOps {
    fn cpu_do_init(&mut self, master: bool) {
        println!("Arm Init! of {}", master)
    }
    fn cpu_do_wakeup(&self) {
        println!("Arm Init!")
    }
    fn cpu_do_idle(&self) {
        println!("Arm Init!")
    }
}

impl Architecture for Arm {
    type Ops = ArmCpuOps;
    fn get_cpu_ops(&self) -> Self::Ops {
        ArmCpuOps {}
    }
} 