# Seed Format

In Phink, a seed is structured to guide the fuzzing process effectively. The seed is composed of:

- **4 bytes**: Represents the balance value to be transferred to the message if it's payable.
- **1 byte**: Specifies the origin, applicable if fuzzing origin is enabled in the configuration.
- **4 bytes**: Identifies the message selector.
- **Remaining bytes**: Contains the SCALE-encoded parameters for the message.

If your configuration allows more than one message per input, Phink uses the delimiter `"********"` to separate multiple
messages within a single input. This allows for comprehensive testing across multiple scenarios from a single seed.

## Example

Here's an example explanation for the seed
`0000000001fa80c2f6002a2a2a2a2a2a2a2a0000000103ba70c3aa18040008000f00100017002a00` :

| Segment            | Bytes                        | Description                                                    |
|--------------------|------------------------------|----------------------------------------------------------------|
| Balance Transfer   | `00000000`                   | 4 bytes for balance (no transfer in this case).                |
| Origin             | `01`                         | 1 byte indicating the origin (Alice) (enabled in config).      |
| Message Selector 1 | `fa80c2f6`                   | 4 bytes for the first message selector.                        |
| Parameters 1       | `00`                         | SCALE-encoded parameters for the first message.                |
| Message Delimiter  | `2a2a2a2a2a2a2a2a`           | Delimits the first and second messages (`********`).           |
| Balance Transfer   | `00000001`                   | 4 bytes for balance (1 unit transfered).                       |
| Origin             | `03`                         | 1 byte indicating the origin (Charlie) for the second message. |
| Message Selector 2 | `ba70c3aa`                   | 4 bytes for the second message selector.                       |
| Parameters 2       | `18040008000f00100017002a00` | SCALE-encoded vector: [4, 8, 15, 16, 23, 42]                   |

### Explanation

- **Balance Transfer**: The 4 bytes representing the balance transfer amount (set to `00000000` for the first message),
  indicating no value is being transferred for either message.
- **Origin**: A single byte is used (`01` for the first message and `03` for the second) to specify the origin of the
  call, useful for testing scenarios with different origins.
- **Message Selector**: The first message for example begins with a 4-byte identifier (`fa80c2f6`), indicating which
  message within the contract is being invoked.
- **Parameters**: Following the message selector, SCALE-encoded parameters are specified (example: `00`), representing
  the input data for each message.
- **Message Delimiter**: This seed uses the delimiter `********` (represented as `2a2a2a2a2a2a2a2a`) to separate
  multiple messages within a single input, allowing more complex interactions to be tested.

# Running One Seed

To execute a single seed, use the following command:

```bash
phink execute my_seed.bin
```

This command runs the specific seed `my_seed.bin`, providing targeted fuzzing for individual transaction testing.

# Running All the Seeds

Run all seeds sequentially using:

```bash
phink run
```

This command iterates over the `corpus` folder, executing each seed to ensure a comprehensive fuzzing process covering
all previously discovered cases.

# Generating a Seed

To generate a new seed, you need to construct it using the prescribed format. Start with the required byte sequences for
balance, origin, message selector, and parameters, then save it in your designated seed directory.

## Importance of seed generation

The ability to manually create seeds is crucial for enhancing the effectiveness of the fuzz testing process. By creating
custom seeds, developers can guide the fuzzer to explore paths and scenarios that might not be easily discovered through
automated means. This, in turn, increases the overall coverage of the fuzzing campaign, ensuring that more potential
vulnerabilities and edge cases are identified and addressed. For generating the SCALE-encoded parameters, developers can
utilize tools like `cargo contract`
or [Polkadot.js](https://polkadot.js.org/apps/).

# Adding a Seed to the Corpus

To add a custom seed to the corpus, you can use the following command:

```bash
cargo ziggy add-seeds -i my_custom_seeds/ -z output/
```

- `my_custom_seeds/`: Directory containing your custom seeds.
- `output/`: Directory where the fuzzing output is stored.

Once added, the corpus will use these seeds in subsequent fuzzing processes.

# Viewing and Editing Seeds

To view the hexadecimal content of a seed, use:

```bash
xxd -c 3000 -p output/phink/corpus/one_seed.bin > abc.out
```

This command converts the binary seed file into hex for easier reading and editing.

To edit a seed:

1. Open the hex file in your editor:

    ```bash
    vim abc.out
    ```

2. Save changes and revert the hex file to binary:

    ```bash
    rm seed.bin # used to bypass cached seed
    xxd -r -p abc.out seed.bin
    ```

3. Execute the updated seed with:

    ```bash
    phink execute seed.bin
    ```
