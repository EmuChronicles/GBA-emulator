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
}
