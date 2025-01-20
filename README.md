# Creating an implementation of the `knapsack` problem from scratch

This documents creating an implementation of a program using genetic algorithms
to evolve solutions to the knapsack problem using the `unhindered-ec` library.

## Create the project and add dependencies

Create the project using `cargo init knapsack`.

Then add the (course-specific, and therefore temporary) registry so we can add dependencies.
We need to add the following **to `.cargo/config.toml` in the project** NOT `Cargo.toml`.
This allows to project to access this registry.

```toml
[registries.ec-course]
index = "https://github.com/UMM-CSci-4553-S25/registry.git"
```

to `.cargo/config.toml` in the project so it has access to the registry.

Now we can add `ec-core`, `ec-linear`, and `course-helpers` as dependencies:

```bash
cargo add --registry ec-course ec-core ec-linear course-helpers
```

Since we want to keep the error handling as simple as possible, we're adding the `anyhow`
crate, which allows us to return out any occurring errors with Rust's `?` operator.

```bash
cargo add anyhow
```

We also added the `test_case` library which simplifies certain testing patterns.

```bash
cargo add test_case
```

## Implement `Knapsack`

For whatever problem you're trying to solve, you'll have to implement a model of that problem.
In this case, that is the type `Knapsack` and its helper type `Item`. In this example, those
types are fairly straightforward data containers, but the parsing from a file is a little more
complex, especially if you're new to Rust.

The details of this will be problem dependent. You should definitely ask for help if you're
stuck while trying to implement a new problem.

- [ ] Add documentation to `Knapsack` and `Item`.

## Have `main` return `anyhow::Result`

To simplify the error handling, we want `main` to return `anyhow::Result<()>`, which essentially
says that `main` can return either the unit type `()` if successful, or any error type using the `?`
operator.

```rust
fn main() -> anyhow::Result<()> {
    // The returns the unit type `()` wrapped in the `Ok` variant of
    // `Result`. The lack of a semicolon (`;`) at the end of the line
    // makes this the last value in the function, which is what Rust
    // will return in the absence of an explicit `return` statement.
    Ok(())
}
```

## Create a problem instance

## Initial decisions

After creating our instance of the knapsack problem, we have to build the run. Our `Run` uses
the _builder pattern_ which allows us to specify the various values and properties a run must
have and then assemble the final complete `Run`. In our example this looks like:

```rust
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
```

We have to decide:

- What instance of the knapsack problem are we trying to solve?
  - We might need to create a file with the appropriate representation of that problem instance.
- What is our representation for solutions? In this case it will be fixed length `Bitstring` from `ec-linear`.
- We have to implement some kind of scoring (that will be problem specific)
- We need to have some kind of selection
  - It's not obvious how to use `Lexicase` selection, so we'll probably just use `Tournament` selection. Maybe binary or possibly larger tournaments if we large population sizes.
- We need to have a mutator and crossover; presumably something from `ec-linear` will do.
  - `WithOneOverLength` for mutation
  - `UniformXo` for crossover
- We also need simple like population size and max number of generations, but these don't need to happen until runtime.

We also need a specific instance of the problem that we want to try to solve.

## STUFF STILL TO-DO

- [ ] Add `rand` crate
- [ ] How to implement scoring
- [ ] How to choose/implement different mutators, recombinators, selectors

## Possible improvements

- [ ] Add command-line argument parsing via `clap` to allow for specification of things like problem instance file, population size, etc.
