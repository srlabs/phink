# Troubleshooting

## Debugging Phink

### AFL++ logs

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

### Harness coverage

Use the harness coverage feature for debugging. You should only use it if you want to have a coverage of Phink itself.
For instance, if you're planning to contribute to Phink, or to debug it.

```sh
phink harness-cover
```

Be aware that this is primarily for those who want to dive deeper into the coverage of Phink and is not generally
necessary for regular debugging.

### Support channels

You can find us on [Discord](https://discord.gg/4MakDGwFEK). Alternatively, you can message me
on [kevin[🎩]srlabs.de](kevin[🎩]srlabs.de).

Happy fuzzing!