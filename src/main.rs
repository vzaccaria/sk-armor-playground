pub struct Riscv {}
pub struct Arm {}

pub trait Architecture {
    //                V-- trait bound, must define a type which
    type Ops: CpuOps;
    fn get_cpu_ops(&self) -> Self::Ops;
}

pub trait CpuOps {
    fn cpu_do_init(&mut self, master: bool);
    fn cpu_do_idle(&self);
    fn cpu_do_wakeup(&self);
}

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
    fn get_cpu_ops(&self) -> <Self as Architecture>::Ops {
        ArmCpuOps {}
    }
}

fn main() {
    boot(Arm {});
}

// Platform agnostic code:

fn boot(arch: impl Architecture) {
    arch.get_cpu_ops().cpu_do_wakeup();
}
