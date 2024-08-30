# Contribution guidelines

First off, thank you for considering contributing to Phink.

If your contribution is not straightforward, please first discuss the change you wish to make by
creating a new issue before making the change, or starting a discussion on
[discord](https://discord.gg/gAahQMGE).

## Reporting issues

Before reporting an issue on the [issue tracker](https://github.com/srlabs/phink/issues),
please check that it has not already been reported by searching for some related keywords. Please
also check [`phink` issues](https://github.com/srlabs/phink/issues/) and link any related issues
found.

## Pull requests

All contributions are obviously welcome. Please include as many details as possible in your PR
description to help the reviewer (follow the provided template). Make sure to highlight changes
which may need additional attention, or you are uncertain about. Any idea with a large scale impact
on the crate or its users should ideally be discussed in a "Feature Request" issue beforehand.

### Keep PRs small, intentional, and focused

Try to do one pull request per change. The time taken to review a PR grows exponential with the size
of the change. Small focused PRs will generally be much more faster to review. PRs that include both
refactoring (or reformatting) with actual changes are more difficult to review as every line of the
change becomes a place where a bug may have been introduced. Consider splitting refactoring /
reformatting changes into a separate PR from those that make a behavioral change, as the tests help
guarantee that the behavior is unchanged.

### Code formatting

Run `cargo make format` before committing to ensure that code is consistently formatted with
rustfmt. Configuration is in [`.rustfmt.toml`](./.rustfmt.toml).

### Use conventional commits

We use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) and check for them as
a lint build step. To help adhere to the format, we recommend to install
[Commitizen](https://commitizen-tools.github.io/commitizen/). The summary helps expand on that to provide information that helps provide more context,
describes the nature of the problem that the commit is solving and any unintuitive effects of the
change. It's rare that code changes can easily communicate intent, so make sure this is clearly
documented.

### Sign your commits

We use commit signature verification, which will block commits from being merged via the UI unless
they are signed. To set up your machine to sign commits, see [managing commit signature
verification](https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification)
in GitHub docs.

### Documentation

Every public API **must** be documented.

#### Format

- First line is summary, second is blank, third onward is more detail

```rust
/// Summary
///
/// A detailed description
/// with examples.
fn foo() {}
```

- Max line length is 100 characters
See [VS Code rewrap extension](https://marketplace.visualstudio.com/items?itemName=stkb.rewrap)

- Doc comments are above macros
i.e.

```rust
/// doc comment
#[derive(Debug)]
struct Foo {}
```

- Code items should be between backticks
i.e. ``[`Block`]``, **NOT** ``[Block]``

### Deprecation notice

We generally want to wait at least two versions before removing deprecated items, so users have
time to update. However, if a deprecation is blocking for us to implement a new feature we may
*consider* removing it in a one version notice.

## Continuous Integration

We use GitHub Actions for the CI where we perform the following checks:

- The tests (docs, lib, tests and examples) should pass.
- The code should conform to the default format enforced by `rustfmt`.
- The code should not contain common style issues `clippy`.

You can also check most of those things yourself locally using `cargo make ci` which will offer you
a shorter feedback loop than pushing to github.
