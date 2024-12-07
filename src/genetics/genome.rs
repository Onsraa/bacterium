use bevy::color::Luminance;
use bevy::prelude::Color;
use bitvec::prelude::*;
use rand::Rng;

pub type Genome = BitArr!(for 32, in u8, Msb0);

pub fn bits_to_u8(bits: &BitSlice<u8, Msb0>) -> u8 {
    let mut value = 0u8;
    for bit in bits {
        value <<= 1;
        value |= *bit as u8;
    }
    value
}

/// Convertit un slice de bits en u16 (max 16 bits)
pub fn bits_to_u16(bits: &BitSlice<u8, Msb0>) -> u16 {
    let mut value = 0u16;
    for bit in bits {
        value <<= 1;
        value |= *bit as u16;
    }
    value
}

/// Convertit un u8 en bits
pub fn u8_to_bits(value: u8, bits: &mut BitSlice<u8, Msb0>) {
    let length = bits.len();
    let mut mask = 1 << (length - 1);
    for mut bit in bits {
        *bit = (value & mask) != 0;
        mask >>= 1;
    }
}

/// Convertit un u16 en bits
pub fn u16_to_bits(value: u16, bits: &mut BitSlice<u8, Msb0>) {
    let length = bits.len();
    let mut mask = 1 << (length - 1);
    for mut bit in bits {
        *bit = (value & mask) != 0;
        mask >>= 1;
    }
}

/// Génère un génome aléatoire
pub fn random_genome() -> Genome {
    let mut rng = rand::thread_rng();
    let mut g = Genome::ZERO;
    for i in 0..g.len() {
        g.set(i, rng.gen_bool(0.5));
    }
    g
}

/// Mode de Nourrissage (2 bits) [0..2)
pub fn get_feeding_mode(genome: &Genome) -> u8 {
    bits_to_u8(&genome[0..2])
}

/// Agressivité (4 bits) [2..6)
pub fn get_aggressivity(genome: &Genome) -> u8 {
    bits_to_u8(&genome[2..6])
}

/// Détermine la couleur en fonction du mode de nourrissage et de l’agressivité
pub fn determine_color(genome: &Genome) -> Color {
    let feeding_mode = get_feeding_mode(genome) % 3;
    let aggressivity = get_aggressivity(genome) as f32 / 15.0;
    let intensity = 1.0 - aggressivity * 0.8;

    let base_color = match feeding_mode {
        0 => Color::srgb(0.0, intensity, 0.0),      // Herbivore (Vert)
        1 => Color::srgb(intensity, 0.0, 0.0),      // Carnivore (Rouge)
        2 => Color::srgb(0.0, 0.0, intensity),       // Omnivore (Bleu)
        _ => panic!("No feeding behavior detected."),
    };

    base_color
}

pub fn set_aggressivity(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[2..6];
    u8_to_bits(val & 0b1111, &mut slice);
}

/// Sociabilité (4 bits) [6..10)
pub fn get_sociability(genome: &Genome) -> u8 {
    bits_to_u8(&genome[6..10])
}
pub fn set_sociability(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[6..10];
    u8_to_bits(val & 0b1111, &mut slice);
}

/// Nombre de Flagelles (2 bits) [10..12)
pub fn get_number_of_flagella(genome: &Genome) -> u8 {
    bits_to_u8(&genome[10..12])
}
pub fn set_number_of_flagella(genome: &mut Genome, n: u8) {
    let mut slice = &mut genome[10..12];
    u8_to_bits(n & 0b11, &mut slice);
}

/// Taille du Flagelle (3 bits) [12..15)
pub fn get_flagella_size(genome: &Genome) -> u8 {
    bits_to_u8(&genome[12..15])
}
pub fn set_flagella_size(genome: &mut Genome, size_val: u8) {
    let mut slice = &mut genome[12..15];
    u8_to_bits(size_val & 0b111, &mut slice);
}

/// Longévité (5 bits) [15..20)
pub fn get_longevity(genome: &Genome) -> u8 {
    bits_to_u8(&genome[15..20])
}
pub fn set_longevity(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[15..20];
    u8_to_bits(val & 0b11111, &mut slice);
}

/// Énergie (12 bits) [20..32)
pub fn get_energy(genome: &Genome) -> u16 {
    bits_to_u16(&genome[20..32]) // max 4095
}
pub fn set_energy(genome: &mut Genome, val: u16) {
    let mut slice = &mut genome[20..32];
    u16_to_bits(val & 0xFFF, &mut slice); // 0xFFF = 4095 max sur 12 bits
}

/// Crée une population de `count` espèces aléatoires
pub fn create_population(count: usize) -> Vec<Genome> {
    (0..count).map(|_| random_genome()).collect()
}