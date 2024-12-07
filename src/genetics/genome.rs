//! Gestion du génome et fonctions associées

use bitvec::prelude::*;
use rand::Rng;

pub type Genome = BitArr!(for 64, in u8, Msb0);

/// Convertit un slice de bits en u8
pub fn bits_to_u8(bits: &BitSlice<u8, Msb0>) -> u8 {
    let mut value = 0u8;
    for bit in bits {
        value <<= 1;
        value |= *bit as u8;
    }
    value
}

/// Convertit un u8 en bits sur la slice fournie
pub fn u8_to_bits(value: u8, bits: &mut BitSlice<u8, Msb0>) {
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

/// Accesseurs et mutateurs pour les champs du génome
///
/// On modifie les getters pour les couleurs afin qu'ils renvoient un `f32`
/// entre 0.0 et 1.0 directement.
pub fn get_r(genome: &Genome) -> f32 {
    bits_to_u8(&genome[0..8]) as f32 / 255.0
}
pub fn set_r(genome: &mut Genome, r_val: u8) {
    let mut slice = &mut genome[0..8];
    u8_to_bits(r_val, &mut slice);
}

pub fn get_g(genome: &Genome) -> f32 {
    bits_to_u8(&genome[8..16]) as f32 / 255.0
}
pub fn set_g(genome: &mut Genome, g_val: u8) {
    let mut slice = &mut genome[8..16];
    u8_to_bits(g_val, &mut slice);
}

pub fn get_b(genome: &Genome) -> f32 {
    bits_to_u8(&genome[16..24]) as f32 / 255.0
}
pub fn set_b(genome: &mut Genome, b_val: u8) {
    let mut slice = &mut genome[16..24];
    u8_to_bits(b_val, &mut slice);
}

pub fn get_number_of_flagella(genome: &Genome) -> u8 {
    bits_to_u8(&genome[24..26])
}
pub fn set_number_of_flagella(genome: &mut Genome, n: u8) {
    let mut slice = &mut genome[24..26];
    u8_to_bits(n & 0b11, &mut slice);
}

pub fn get_flagella_size(genome: &Genome, flag_num: u8) -> u8 {
    let start = match flag_num {
        1 => 26,
        2 => 29,
        3 => 32,
        _ => panic!("Flagelle invalide"),
    };
    bits_to_u8(&genome[start..(start+3)])
}
pub fn set_flagella_size(genome: &mut Genome, flag_num: u8, size_val: u8) {
    let start = match flag_num {
        1 => 26,
        2 => 29,
        3 => 32,
        _ => panic!("Flagelle invalide"),
    };
    let mut slice = &mut genome[start..(start+3)];
    u8_to_bits(size_val & 0b111, &mut slice);
}

pub fn get_feeding_mode(genome: &Genome) -> u8 {
    bits_to_u8(&genome[35..37])
}
pub fn set_feeding_mode(genome: &mut Genome, mode: u8) {
    let mut slice = &mut genome[35..37];
    u8_to_bits(mode & 0b11, &mut slice);
}

pub fn get_resilience(genome: &Genome) -> u8 {
    bits_to_u8(&genome[37..41])
}
pub fn set_resilience(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[37..41];
    u8_to_bits(val & 0b1111, &mut slice);
}

pub fn get_longevity(genome: &Genome) -> u8 {
    bits_to_u8(&genome[41..48])
}
pub fn set_longevity(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[41..48];
    u8_to_bits(val & 0b111_1111, &mut slice);
}

pub fn get_propension_echange(genome: &Genome) -> u8 {
    bits_to_u8(&genome[48..52])
}
pub fn set_propension_echange(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[48..52];
    u8_to_bits(val & 0b1111, &mut slice);
}

pub fn get_compatibilite_genetique(genome: &Genome) -> u8 {
    bits_to_u8(&genome[52..56])
}
pub fn set_compatibilite_genetique(genome: &mut Genome, val: u8) {
    let mut slice = &mut genome[52..56];
    u8_to_bits(val & 0b1111, &mut slice);
}

/// Interprétation de l’agressivité et de la sociabilité via la couleur
pub fn interpret_behavior_from_color(genome: &Genome) -> (f32, f32) {
    let r = get_r(genome);
    let g = get_g(genome);
    let b = get_b(genome);
    let aggressivite = (r - g + 1.0) / 2.0;
    let sociabilite = b;
    (aggressivite, sociabilite)
}

/// Crée une population de `count` espèces aléatoires
pub fn create_population(count: usize) -> Vec<Genome> {
    (0..count).map(|_| random_genome()).collect()
}