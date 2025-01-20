mod cliff_score;
mod cliff_scorer;
mod item;
mod knapsack;

use cliff_scorer::CliffScorer;
use course_helpers::ec_run::Run;
use ec_core::operator::selector::{best::Best, tournament::Tournament, Selector};
use ec_linear::{
    mutator::with_one_over_length::WithOneOverLength, recombinator::uniform_xo::UniformXo,
};
use knapsack::Knapsack;

fn main() -> anyhow::Result<()> {
    let file_path = "knapsacks/big.txt";
    let knapsack = Knapsack::from_file_path(file_path)?;

    let run = Run::builder()
        // The number of bits should equal the number of items.
        .bit_length(knapsack.num_items())
        // The maximum number of generations to run; this is somewhat arbitrary
        .max_generations(1_000)
        // The population size, which is also somewhat arbitrary, but larger is better
        // until it's so big that memory management becomes a problem.
        .population_size(1_000)
        // How do we want to select parent individuals? This takes two individuals at
        // random from the population, and then chooses the better of the two from this
        // tournament. You can change this to larger tournaments by changing `2` to your
        // desired tournament size.
        .selector(Tournament::of_size::<2>())
        // How do we want to mutate individual knapsack solutions? This flips
        // on average one bit, thereby adding or removing one item from the solution.
        .mutator(WithOneOverLength)
        // How do we want to recombine parent solutions? This randomly chooses for
        // each bit whether to take it from the first or the second parent, giving
        // use a "shuffled" set of choices from both parents.
        .recombinator(UniformXo)
        // Do we want to use parallel evaluation? If this is `true`, the run will use
        // all the available cores to evaluate the population in parallel. This can speed
        // up the process considerably, at the cost of heating up your CPU.
        .parallel_evaluation(true)
        // How do we want to score different knapsack "solutions"? This is the only
        // problem dependent part of building the run. We'll start with a simple scorer
        // that returns a `CliffScore`. This is an `enum` with two variants: `Score(v)`
        // where `v` is the value of the items if they fit in the knapsack
        // and `Overloaded` otherwise.  This is implemented so that `Overloaded` is
        // always worse than any `Score(v)` value.
        .scorer(CliffScorer::new(knapsack))
        // Now that we've specified all the elements, we can build the run.
        .build();

    let final_population = run.execute()?;

    let mut rng = rand::rng();
    let best = Best.select(&final_population, &mut rng)?;

    println!("{best:?}");

    // The returns the unit type `()` wrapped in the `Ok` variant of
    // `Result`. The lack of a semicolon (`;`) at the end of the line
    // makes this the last value in the function, which is what Rust
    // will return in the absence of an explicit `return` statement.
    Ok(())
}
