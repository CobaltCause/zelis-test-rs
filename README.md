# Zelis Code Test (Rust)
---
You are given a large JSON file, ````data/test.json````. The file has been delivered by the client corrupt and is unable to be processed normally. After some inspection, you realize that at least one ````:```` has been mistyped as ````;```` within the file. The file must be processed by the end of the day and it is not feasible for the client to correct. Write a program that solves this problem.

- The program should take input from STDIN and write output to STDOUT.
- Optionally, the program can take ````--input```` and ````--output```` parameters, specifying file locations.
- Correct output will be valid JSON.
- Given a known schema, you can be confident that ````;```` does not exist within any valid input. As such, it's safe to simply replace them without concern for having accidentally replaced a valid character.
- This is a common problem that we've seen before and we will need to be able to use the program again on files much larger than the example, including files that are larger than the system's available memory.
- The primary goal is performance. The process will have to be run frequently for years to come, so it is important that it be as efficient as possible. It also needs to finish in time on short notice.

Create a new repository implementing your solution. When complete, share with ````@jgmartin```` and ````@waysidekoi```` for review. Thanks and good luck!
