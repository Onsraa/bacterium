mod genome;
mod fitness;
mod selection;
mod crossover;
mod mutation;

pub use genome::{Genome, random_genome, bits_to_u8, u8_to_bits,
                 get_number_of_flagella, set_number_of_flagella,
                 get_flagella_size, set_flagella_size,
                 get_feeding_mode,
                 get_longevity, set_longevity, create_population, determine_color
};

pub use fitness::calculate_fitness;
pub use selection::roulette_wheel_selection;
pub use crossover::random_crossover;
pub use mutation::mutate;
