# run from main directory using `./script.sh`

for run_num in {1..30} #should be 30
do
    echo "This is run number $run_num"
    cargo run --release knapsack_from_scratch > ./Outputs/ts_8_pID_Big3/output_$run_num.txt
    # update `ts_2_pID_Tiny` in above line with correct info
done
