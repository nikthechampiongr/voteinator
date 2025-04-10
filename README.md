# The Vote-inator!

I was simply bored and wanted to quickly create a rust program that implements
STV with a quota and weighted vote transfers for surplus votes.

The votes.csv is collected from a vote on a Discourse forum.

May alter it to be able to put caps on the maximum amount of people that can get
in from certain groups.

Usage: voteinator vote-csv num_of_seats

where vote-csv is a csv from a ranked choice vote on Discourse and number of
seats is the number of seats available.

The quota is: num of votes / num of seats

Votes are weighed as follows: When a candidate wins, their votes are reassigned
and their current vote is multiplied like this curr_weight * (1 - quota / number
of votes).
