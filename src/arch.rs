pub trait Architecture {
    type Ops: CpuOps;
    fn get_cpu_ops(&self) -> Self::Ops;
}

pub trait CpuOps {
    fn cpu_do_init(&mut self, master: bool);
    fn cpu_do_idle(&self);
    fn cpu_do_wakeup(&self);
} 