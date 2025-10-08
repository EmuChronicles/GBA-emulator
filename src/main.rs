mod cpu; // todos os módulos devem ser adicionados aqui
mod memory;
fn main() {
    println!("CPU Interpreter");
    let mut cpu = cpu::Arm7Tdmi::new();
    cpu.reset();
    // cpu.step();
}
