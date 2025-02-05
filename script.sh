# run from main directory using `./script.sh`

for run_num in {1..30} # should be 30
do
    echo "This is run number $run_num"
    cargo run --release knapsack_from_scratch > ./Outputs/ts_2_pID_Tiny/output_$run_num.txt
    # make folder under Outputs folder for each round of tests, 
    # then rename `ts_2_pID_Tiny` in above line with correct info
done
