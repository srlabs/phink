Command line used to find this crash:

/Users/kevinvalerio/.local/share/afl.rs/rustc-1.80.0-nightly-b92758a/afl.rs-0.15.5/afl/bin/afl-fuzz -c0 -Mmainaflfuzzer -i./output/phink/corpus/ -pexplore -o./output/phink/afl -g1 -G1048576 -c- -P600 ./target/afl/debug/phink

If you can't reproduce a bug outside of afl-fuzz, be sure to set the same
memory limit. The limit used for this fuzzing session was 0 B.

Need a tool to minimize test cases before investigating the crashes or sending
them to a vendor? Check out the afl-tmin that comes with the fuzzer!

Found any cool bugs in open-source tools using afl-fuzz? If yes, please post
to https://github.com/AFLplusplus/AFLplusplus/issues/286 once the issues
 are fixed :)

