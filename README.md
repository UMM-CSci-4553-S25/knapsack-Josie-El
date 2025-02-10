# Knapsack Tournament Selection

## The Question

How does tournament size impact outputs of knapsack problem solving algorithms?
The sizes we are looking at are 2 and 8.

## The Framework

We are running on Rust (DEAP)

## Sack information

We are going to run two small capacity knapsacks and two large. These files are located at `main/knapsacks`

The specific ones are:

File Name | Origin | # of items | Capacity |
----------|--------|----------- | ---------|
BigProblem3.txt | BigProblem1.txt with adjusted capacity | 25,000 | 10,000,000,000 |
BigProblem4.txt | BigProblem2.txt with adjusted capacity | 25,000 | 10,000,000,000 |
SmallProblem3.txt | SmallProblem1.txt with adjusted capacity | 5,000 | 10,000 |
SmallProblem4.txt | SmallProblem2.txt with adjusted capacity | 5,000 | 10,000 |

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

1. Create a new subfolder in the Output folder (located at `main/Outputs`). The name of the folder should be `ts_[tournament size]_pID_[knapsack seed ID]`. If a folder with that name already exists, see the extra step under step 2.
2. In the file `main/script.sh` update line 6 to be the location of the folder created in step 1.
   1. If you are doing additional trials with parameters you have run before alter the range in line 3 to have the first number be one more than the sum of previously run trials. The second number should be that sum + how many trials you are wanting to run now. For example if you hade run 30 trial before and want to run 10 more now the range would be {31 .. 40}
3. In the file `main/src/main.rs` check that the tournament size on line 48 and the knapsack seed location on line 49 are correct.
4. In the main directory run `./script.sh`

### To Extract Info From Output Files

Enter the folder where your target outputs are (the directory should like `main/outputs/ts_*_pID_*`)

To get the the overall best score from a trial from the terminal run `for f in output_*.txt; do tail --lines 1 $f | head --lines 1; done | sed 's/.*Score(\(.*\)) }).*/\1/'`. This will spit the value for each output file into the terminal where it can be copy/pasted in a different location.

To get the best score in the final round run `for f in output_*.txt; do tail --lines 4 $f | head --lines 1; done | sed 's/.*Score(\(.*\)).*/\1/'`. This will spit all the values into the terminal.
