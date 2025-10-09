pub struct Memory {
    // Mapa de memoria
    rom: Vec<u8>, // ROM / Read-Only-Memory, é daqui de onde vem as informaçoes do jogo.
    // Vec<u8> é um vetor de bytes (nesse caso 8)
    ewram: Vec<u8>, // 256 KB, RAM mais lenda porém maior, usada para armazenamento de dados temporários
    iwram: Vec<u8>, // 32 KB, RAM mais rapida porém menor, usada para armazenamento de dados criticos
                    // que precisam de calculos rapidos.
}

impl Memory {
    pub const ROM_BASE: u32 = 0x0800_0000;
    pub const EWRAM_BASE: u32 = 0x0200_0000;
    pub const IWRAM_BASE: u32 = 0x0300_0000;

    // Cada um desses valores hexadecimais representa o
    // endereço inicial de uma região específica da memória
    // no mapa de memória do GBA.

    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ewram: vec![0; 256 * 1024], // inicializa EWRAM com 256KB de zeros
            // vec![] é um macro em rust para iniciar um vetor dinamico
            // no nosso exemplo, vec![0; 256 * 1024] cria um vetor de tamanho 256 KB,
            // preenchido com o valor 0 em todos os elementos.
            iwram: vec![0; 32 * 1024], // inicializa IWRAM com 32KB de zeros
        }
    }

    #[inline]
    pub fn read8(&self, addr: u32) -> u8 {
        // Lê um byte (8 bits) do endereço de memória especificado.
        // addr: endereço de 32 bits no mapa de memória do GBA.
        // Retorna: valor de 8 bits (u8) lido daquele endereço.
        match addr {
            Self::EWRAM_BASE..=0x02FF_FFFF => {
                let idx = ((addr - Self::EWRAM_BASE) & ((self.ewram.len() - 1) as u32)) as usize; // usize é o tipo que RUST requer para buscar um indice no vetor                                                                             // idx = index
                self.ewram[idx]
            }

            Self::IWRAM_BASE..=0x03FF_FFFF => {
                let idx = ((addr - Self::IWRAM_BASE) & ((self.iwram.len() - 1) as u32)) as usize;
                self.iwram[idx]
            }

            Self::ROM_BASE..=0x09FF_FFFF => {
                if self.rom.is_empty() {
                    return 0xFF;
                }
                let idx = ((addr - Self::ROM_BASE) & ((self.rom.len() - 1) as u32)) as usize;
                self.rom[idx]
            }

            _ => 0,
        }
    }

    #[inline]
    pub fn read_u16(&self, addr: u32) -> u16 {
        // Lê um halfword (16 bits) em little-endian.
        let lo = self.read8(addr) as u16; // Byte menos significativo (bits 0-7)
        let hi = self.read8(addr.wrapping_add(1)) as u16; // Byte mais significativo (bits 8-15)
        lo | (hi << 8) // Combina os bytes em little-endian
    }

    #[inline]
    pub fn read_u32(&self, addr: u32) -> u32 {
        // Lê uma word (32 bits) em little-endian.
        let b0 = self.read8(addr) as u32; // Bits 0-7
        let b1 = self.read8(addr.wrapping_add(1)) as u32; // Bits 8-15
        let b2 = self.read8(addr.wrapping_add(2)) as u32; // Bits 16-23
        let b3 = self.read8(addr.wrapping_add(3)) as u32; // Bits 24-31
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24) // Combina os 4 bytes em little-endian
    }
}
