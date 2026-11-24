mod arch;
mod arm;
mod riscv;

use arch::Architecture;
use arch::CpuOps;
use arm::Arm;

fn boot(arch: impl Architecture) {
    arch.get_cpu_ops().cpu_do_init(true);
} 

fn main() {
    boot(Arm {});
}
