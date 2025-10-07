mod cpu;
fn main() {
    println!("CPU Interpreter");
    let mut cpu = cpu::Arm7Tdmi::new();
    cpu.reset();
    // cpu.step();
}
