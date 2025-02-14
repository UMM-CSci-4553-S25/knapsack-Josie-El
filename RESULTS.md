# Tournament Size and the Knapsack Problem

by Josie and El

## Description

The knapsack problem is a style of puzzle where you are given a knapsack and a list of items to put in the sack. The sack has a set capacity(usually measured in weight) and each item is assigned a size and a value. The goal of the problem is to find the best combination of items to optimize the value in the bag while remaining under or at capacity.

## Variables

Our study is looking to see how tournament size effects the overall best and final best scores given by our program. We are testing this on tournament sizes 2 and 8. One additional variable we are changing is the knapsack seeds. We are running each trial on four different seeds. Two seeds have a capacity of 10 billion and 25 thousand items to pick from. The other two have a capacity of 10 thousand and five thousand items to pick from.

Our knapsack seeds can be found in [our repository](https://github.com/UMM-CSci-4553-S25/knapsack-Josie-El) under the knapsacks folder in the following files: BigProblem3.txt, BigProblem4.txt, SmallProblem3.txt, and SmallProblem4.txt. We used [this code](https://github.com/JorikJooken/knapsackProblemInstances/tree/master) to generate these knapsack problems. We used the parameters {capacity: , itmes: , --} for the big problems and {capacity: , itmes: , --} for the small ones. From those parameters we got the seeds that are stored in the knapsack folder under the names: BigProblem1.txt, BigProblem2.txt, SmallProblem1.txt, and SmallProblem2.txt. We attempted to run our study on these seeds, but too many of the scores were "Overloaded", so we altered the capacity of the sack to be 10 billion and 10 thousand. The capacity of the large sacks was taken from the seed titled big.txt. The capacity of the small seeds was chosen because it was the first reasonably small capacity we found that did not return "Overloaded" for the majority of the runs.

Here is a list of variables we are not changing:

- Mutation rate (1 over length)
- Scoring system
- Number of generations (1,000)
- Generation size (1,000)
- How we make children (Uniform Crossover)

The scoring system used is pretty straightforward. If the weight of the sack is over the capacity it is assigned a value of "Overloaded". All values marked "Overloaded" are equally poor and are set to be smaller than the lowest legal score. If the weight is at or under capacity then the score is the combined value of the items in the sack.

We are using a framework called unhindered-ec which runs in Rust.  
The framework can be found at: [https://github.com/unhindered-ec/unhindered-ec](https://github.com/unhindered-ec/unhindered-ec).  
The code we altered can be found at: [https://github.com/UMM-CSci-4553-S25/knapsack-ga](https://github.com/UMM-CSci-4553-S25/knapsack-ga).

## Experimental Design

We are running each tournament size on each knapsack seed 30 times. This totals to 240 runs. After each run we recorded the overall best score and the best legal score of the last generation.

We ran these tests over several days and three separate machines (two lab computers and one laptop). We did not attempt to run all of the trials in identical ways because our results were not related to time.

## Summary

A summary of your basic results

This is where you'd include tables and/or graphs summarizing your results. Don't include everything that you might have looked at; focus on the bits that are most informative for the reader. In the hill-climbing example that I shared, for example, I'd probably focus on the faceted boxplot and the rpart tree, highlighting in the text the relevant sections (the upper-right of the plot, and the left two leaves of the tree).

If there's a difference that you think is important, make sure you run some sort of appropriate statistical test on it to see if that difference is statistically significant.

In many sciences (biology is super strict about this), this results section only presents the results, but doesn't discuss them. You don't speculate on what things mean or why things might have happened. Computer science research is often sloppier about this, but I think it's a good practice so you might want to try to keep results and discussion (below) separate.

## Discussion

A discussion of your results

As mentioned above, there's value in having this separate from the results section.

This is where you'd talk about what all this might mean and why you think that treatment A did better than treatment B, or why those two treatments led to very similar results.

## Conclusion

A short conclusion

Wrap this up, summarizing what you explored, and what you learned.
