use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng, SeedableRng,
};

type RngCore = rand::prelude::StdRng;

pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomNumberGenerator {
    /// Creates a default `RandomNumberGenerator`, with a randomly
    /// selected starting seed.
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_entropy(),
        }
    }

    /// Creates a new `RandomNumberGenerator`, with a user-specified random seed.
    /// It will produce the same results each time (given the same requests).
    ///
    /// # Arguments(3)
    ///
    /// * `seed` - the random seed to use.
    ///
    /// # Example(4)
    ///
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng1 = RandomNumberGenerator::seeded(1);
    /// let mut rng2 = RandomNumberGenerator::seeded(1);
    /// let results: (u32, u32) = ( rng1.next(), rng2.next() );
    /// assert_eq!(results.0, results.1);
    /// ```
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }

    /// Generates a new random number of the requested type.
    pub fn next<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        self.rng.gen()
    }

    /// Generates a random number within the specified range.
    ///
    /// # Arguments
    ///
    /// * `range` - the range (inclusive or exclusive) within which to
    /// generate a random number.
    ///
    /// # Example
    ///
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng = RandomNumberGenerator::new();
    /// let one_to_nine = rng.range(1..10);
    /// let one_to_ten = rng.range(1..=10);
    /// ```
    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }

    pub fn get_core(&mut self) -> &mut RngCore {
        &mut self.rng
    }

    pub fn d100(&mut self) -> i32 {
        self.range(1..=100)
    }

    pub fn d20(&mut self) -> i32 {
        self.range(1..=20)
    }

    pub fn d6(&mut self) -> i32 {
        self.range(1..=6)
    }

    pub fn d3(&mut self) -> i32 {
        self.range(1..=3)
    }
}
