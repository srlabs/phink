# Troubleshooting

## Debugging Phink

### AFL++ Logs

If you encounter unexpected behavior, examining the AFL++ logs can provide good insights. In most cases, developers
will find more information by executing:

```sh
tail -f your_output/phink/logs/afl.log
```

Replace `your_output` with the directory defined in your `phink.toml` under `fuzz_output`. This will give you a
real-time view of the log output, helping you identify any issues during the fuzzing process.

### Executing a Single Seed

To debug specific cases where a contract crashes, you can execute a single seed. This method allows you to instantiate a
contract and identify crash points more easily:

```sh
phink execute output/phink/corpus/selector_1.bin
```

This command runs a single fuzzing input, making it easier to pinpoint problems.

### Harness Coverage

Use the harness coverage feature if you need insights into Phink’s functionality, particularly if you plan to contribute
or debug the tool itself:

```sh
phink harness-cover
```

Be aware that this is primarily for those who want to dive deeper into the coverage of Phink and is not generally
necessary for regular debugging.

### Support Channels

For additional help, you can join us on [Discord](https://discord.gg/4MakDGwFEK) where our community and team are
active. Alternatively, feel free to message me at [kevin[🎩]srlabs.de](mailto:kevin[🎩]srlabs.de).

Happy fuzzing!