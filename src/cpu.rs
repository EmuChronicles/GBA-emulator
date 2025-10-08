use crate::memory::Memory; // importa a struct Memory do modulo memory

enum InstructionClass {
    Branch,             // B
    SingleDataTransfer, // LDR
    DataProcessing,     // DP
    BlockDataTransfer,  // LDM
    Undefined,
}

// Tabela de Conversão Binário (4 bits) para Hexadecimal
// ----------------------------------------------------
// 0000 = 0
// 0001 = 1
// 0010 = 2
// 0011 = 3
// 0100 = 4
// 0101 = 5
// 0110 = 6
// 0111 = 7
// 1000 = 8
// 1001 = 9
// 1010 = A
// 1011 = B
// 1100 = C
// 1101 = D
// 1110 = E
// 1111 = F
// ----------------------------------------------------

const MASK_B: u32 = 0x0E00_0000; // 0000 1110 0000 0000 0000 0000 0000 0000
const VALUE_B: u32 = 0x0A00_0000; // 0000 1010 0000 0000 0000 0000 0000 0000

const MASK_LDR: u32 = 0x0C00_0000; // 0000 1100 0000 0000 0000 0000 0000 0000
const VALUE_LDR: u32 = 0x0400_0000; // 0000 0100 0000 0000 0000 0000 0000 0000

const MASK_DP: u32 = 0x0C00_0000; // 0000 1100 0000 0000 0000 0000 0000 0000
const VALUE_DP: u32 = 0x0000_0000; // 0000 0000 0000 0000 0000 0000 0000 0000

const MASK_LDM: u32 = 0x0E00_0000; // 0000 1110 0000 0000 0000 0000 0000 0000
const VALUE_LDM: u32 = 0x0800_0000; // 0000 1000 0000 0000 0000 0000 0000 0000

// MASK_DP e MASK_LDR são iguais (0x0C00_0000) porque ambos olham os bits 27..26. A diferença está no VALUE (00 vs 01).
// MASK_B e MASK_LDM são iguais (0x0E00_0000) porque ambos olham os bits 27..25. A diferença está no VALUE (101 vs 100).

pub struct Arm7Tdmi {
    // struct só indica os tipos e formatos de dados do elemento
    // registradores principais (mínimo)
    pub r: [u32; 16], // pub r torna o elemento publico || u32 = capacidade de cada registrador (32 bits) || 16 registradores
    pub cpsr: u32,
}

impl Arm7Tdmi {
    pub fn new() -> Self {
        // impl implementa funçoes/metodos a um elemento, no caso a funçao new() -> Self cria uma nova instancia
        // do elemento onde esta havendo a implementaçao com valores padrao (geralmente zeros ou valores minimos)
        Self {
            r: [0; 16],
            cpsr: 0,
        }
    }

    fn classify_arm(&self, opcode: u32) -> InstructionClass {
        if (opcode & MASK_B) == VALUE_B {
            return InstructionClass::Branch;
        }
        if (opcode & MASK_LDR) == VALUE_LDR {
            return InstructionClass::SingleDataTransfer;
        }
        if (opcode & MASK_DP) == VALUE_DP {
            return InstructionClass::DataProcessing;
        }
        if (opcode & MASK_LDM) == VALUE_LDM {
            return InstructionClass::BlockDataTransfer;
        }

        InstructionClass::Undefined
    }

    fn thumb_mode(&self) -> bool {
        (self.cpsr & (1 << 5)) != 0 // shift left (<<) serve para deslocar o bit x vezes para a esquerda. para decidir thumb ou arm,
                                    // precisamos verificar o bit T que é o bit 5, a expressao basicamente diz:
                                    // se bit 5 != 0 -> thumb (true)
                                    // se bit 5 == 0 -> ARM (false)
    }

    pub fn reset(&mut self) {
        // a funçao reset reconfigura uma instancia existente do cpu para um estado de inicial de operaçao
        self.r.fill(0); // os self. referenciam uma instancia e .fill() preenche os dados dessa instancia
        self.cpsr = 0x0000_0013; // modo Supervisor (exemplo)
        self.r[15] = 0; // PC inicial placeholder, é importante sempre iniciar o PC (16o registrador) como 0
    }

    pub fn fetch(&self, mem: &Memory) -> u32 {
        let pc = self.r[15];

        if self.thumb_mode() {
            // thumb
            let effective_adress = pc.wrapping_add(4) & !1;
            mem.read_u16(effective_adress) as u32 // NECESSARIO IMPL READS
        } else {
            // ARM
            let effective_adress = pc.wrapping_add(8) & !3;
            mem.read_u32(effective_adress)
        }
    }

    pub fn step(&mut self) {
        // step é a funçao que de fato executa o TODO
        // TODO: fetch-decode-execute
    }
}
