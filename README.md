# Knapsack Tournament Selection

## The Question

How does tournament size impact outputs of knapsack problem solving algorithms?
The sizes we are looking at are 2 and 8.

## The Framework

We are running on Rust (DEAP)

## Sack information

We are going to run two small capacity knapsacks and two large
The specific ones are:

File Name | Origin | Seed Data
----------|--------|-----------
BigProblem3.txt | BigProblem1.txt with adjusted capacity | ?
BigProblem4.txt | BigProblem2.txt with adjusted capacity | ?
SmallProblem3.txt | SmallProblem1.txt with adjusted capacity | ?
SmallProblem4.txt | SmallProblem2.txt with adjusted capacity | ?

They are generated from [this code](https://github.com/JorikJooken/knapsackProblemInstances/tree/master).

## Run information

We will perform 30 runs for each knapsack and each tournament size.
This will be 240 runs total. (30 runs x 4 knapsacks x 2 sizes)

## Data

We plan to collect the best overall legal value and best legal final value.

We will collect it in a [google spreadsheet](https://docs.google.com/spreadsheets/d/1-jJcc_ciIstQLY3AB9haknGzVyXA9n3BpGcqxdgunGY/edit?usp=sharing).

We will share our information as an RMD file.

We will run this Tuesday night(2/4/25) in the secondary computer science lab on four computers.
Both of us will be there running 2 computers each. Josie will step out for a meeting at some point.

## Misc Notes

Use `% cargo run --release` and `% cargo build --release` to reduce run time. Run out of target/release/--- directory.

### How to Run Trials

### To Extract Info From Output Files

Enter the folder where your target outputs are (the directory should like `main/outputs/ts_*_pID_*`)

To the the overall best score from a trial can be obtained using ``