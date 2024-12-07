mod genome;
mod fitness;
mod selection;
mod crossover;
mod mutation;

pub use genome::{Genome, random_genome, bits_to_u8, u8_to_bits,
                 get_r, set_r, get_g, set_g, get_b, set_b,
                 get_number_of_flagella, set_number_of_flagella,
                 get_flagella_size, set_flagella_size,
                 get_feeding_mode, set_feeding_mode,
                 get_resilience, set_resilience,
                 get_longevity, set_longevity,
                 get_propension_echange, set_propension_echange,
                 get_compatibilite_genetique, set_compatibilite_genetique,
                 interpret_behavior_from_color, create_population
};

pub use fitness::calculate_fitness;
pub use selection::roulette_wheel_selection;
pub use crossover::random_crossover;
pub use mutation::mutate;
