## 2020-03-03, Version 0.12.0
### Commits
- [[`9779383689`](https://github.com/datrs/merkle-tree-stream/commit/9779383689adaa88b9fafc04d43c7055fcdaf2e5)] (cargo-release) version 0.12.0 (Bruno Tavares)
- [[`a15ebae355`](https://github.com/datrs/merkle-tree-stream/commit/a15ebae355e329ab00cebc26df5055d7f60f411d)] Point flat-tree to crates version (Bruno Tavares)
- [[`5e24897cc4`](https://github.com/datrs/merkle-tree-stream/commit/5e24897cc4f0e4bbb7d2d230ccdfdda69f8260b6)] Change from usize to u64 (#29) (Bruno Tavares)
- [[`18f5f2f2ad`](https://github.com/datrs/merkle-tree-stream/commit/18f5f2f2adbb07ba79f4928b40e14a5f6ab10a9a)] Update changelog (Bruno Tavares)

### Stats
```diff
 .travis.yml         |  6 +++---
 CHANGELOG.md        | 21 +++++++++++++++++++++
 Cargo.toml          | 10 +++++-----
 src/default_node.rs | 12 ++++++------
 src/lib.rs          | 16 ++++++++--------
 src/partial_node.rs | 10 +++++-----
 tests/lib.rs        |  6 +++---
 7 files changed, 51 insertions(+), 30 deletions(-)
```


## 2020-02-18, Version 0.11.0
### Commits
- [[`7582272d2c`](https://github.com/datrs/merkle-tree-stream/commit/7582272d2c117d8413768219ccff60988ec87aa7)] (cargo-release) version 0.11.0 (Bruno Tavares)
- [[`5452a3af65`](https://github.com/datrs/merkle-tree-stream/commit/5452a3af6545029ed6f719f42b1eee300065446f)] Make structures Send (#28) (Bruno Tavares)
- [[`9790978c22`](https://github.com/datrs/merkle-tree-stream/commit/9790978c224e4afc6208602f5be75a797a1a1df1)] Update hex requirement from 0.3.2 to 0.4.0 (dependabot-preview[bot])
- [[`9b09398359`](https://github.com/datrs/merkle-tree-stream/commit/9b093983598c5ad7d1c6ff7a7586c392b43e8a15)] Update quickcheck requirement from 0.8.1 to 0.9.0 (dependabot-preview[bot])
- [[`0e15f1fff2`](https://github.com/datrs/merkle-tree-stream/commit/0e15f1fff23ade27f4c43b6b5a1863770e175aad)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md      | 19 +++++++++++++++++++
 Cargo.toml        |  8 +++++---
 README.md         |  4 ++--
 examples/async.rs | 49 +++++++++++++++++++++++++++++++++++++++++++++++++
 examples/main.rs  |  4 ++--
 src/lib.rs        | 36 ++++++++++++++++++------------------
 tests/lib.rs      | 18 +++++++++---------
 7 files changed, 104 insertions(+), 34 deletions(-)
```


## 2018-11-26, Version 0.9.0
### Commits
- [[`d7f3fddd69`](https://github.com/datrs/merkle-tree-stream/commit/d7f3fddd6973bd3f2c5b696390f3e84a4589dfb7)] (cargo-release) version 0.9.0 (Yoshua Wuyts)
- [[`c2bfa8c81a`](https://github.com/datrs/merkle-tree-stream/commit/c2bfa8c81a9a34ff9c314d6fd698d286724af809)] Contrain Node as: From<NodeParts> (#21) (Scott Trinh)
- [[`83fa0f313e`](https://github.com/datrs/merkle-tree-stream/commit/83fa0f313e251d714aac7718eaad0d524d6cdf37)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md        | 19 +++++++++++++++++++
 Cargo.toml          |  2 +-
 README.md           | 11 +++++++----
 examples/main.rs    | 16 +++++++---------
 src/default_node.rs |  8 +++++++-
 src/lib.rs          | 47 ++++++++++++++++++++++++++---------------------
 tests/lib.rs        |  4 ----
 7 files changed, 67 insertions(+), 40 deletions(-)
```


## 2018-11-26, Version 0.9.0
### Commits
- [[`d7f3fddd69`](https://github.com/datrs/merkle-tree-stream/commit/d7f3fddd6973bd3f2c5b696390f3e84a4589dfb7)] (cargo-release) version 0.9.0 (Yoshua Wuyts)
- [[`c2bfa8c81a`](https://github.com/datrs/merkle-tree-stream/commit/c2bfa8c81a9a34ff9c314d6fd698d286724af809)] Contrain Node as: From<NodeParts> (#21) (Scott Trinh)
- [[`83fa0f313e`](https://github.com/datrs/merkle-tree-stream/commit/83fa0f313e251d714aac7718eaad0d524d6cdf37)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md        | 19 +++++++++++++++++++
 Cargo.toml          |  2 +-
 README.md           | 11 +++++++----
 examples/main.rs    | 16 +++++++---------
 src/default_node.rs |  8 +++++++-
 src/lib.rs          | 47 ++++++++++++++++++++++++++---------------------
 tests/lib.rs        |  4 ----
 7 files changed, 67 insertions(+), 40 deletions(-)
```


## 2018-10-28, Version 0.8.0
### Commits
- [[`b6096eacff`](https://github.com/datrs/merkle-tree-stream/commit/b6096eacffc96765271307cbf30bd613b414b4af)] (cargo-release) version 0.8.0 (Yoshua Wuyts)
- [[`770347ec82`](https://github.com/datrs/merkle-tree-stream/commit/770347ec82cdd5bf5b207088be2535f76364576f)] Use NodeKind enum instead of Option for PartialNode data (#18) (Scott Trinh)
- [[`04b163a5d1`](https://github.com/datrs/merkle-tree-stream/commit/04b163a5d1ccb06f995576152280bde5241c0ef4)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md        | 25 +++++++++++++++++++++++++
 Cargo.toml          |  2 +-
 examples/main.rs    |  7 +++++--
 src/default_node.rs |  6 +++---
 src/lib.rs          | 24 +++++++++++++++---------
 src/partial_node.rs | 14 +++++++++++---
 tests/lib.rs        | 20 +++++++++++---------
 7 files changed, 71 insertions(+), 27 deletions(-)
```


## 2018-10-17, Version 0.7.0
### Commits
- [[`8fe517e043`](https://github.com/datrs/merkle-tree-stream/commit/8fe517e0432d35563c3f89956a7065ec3cbdb9ef)] (cargo-release) version 0.7.0 (Yoshua Wuyts)
- [[`3468de73e7`](https://github.com/datrs/merkle-tree-stream/commit/3468de73e7dc011838d762dc3be18e7cea026049)] Fix features (#17) (Yoshua Wuyts)
- [[`961426a0b4`](https://github.com/datrs/merkle-tree-stream/commit/961426a0b42cebdfafc75a6d6001a03f16929019)] Add doctest for MerkleTreeStream (#16) (Kuba)
- [[`42eb089703`](https://github.com/datrs/merkle-tree-stream/commit/42eb0897034fae63f443364b82c1115f15cc16aa)] Update rust_sodium requirement from 0.7.0 to 0.10.0 (#14) (dependabot[bot])
- [[`f2552f963f`](https://github.com/datrs/merkle-tree-stream/commit/f2552f963f4d29d7a9da5df59b66e579d19782b1)] Update quickcheck requirement from 0.6.2 to 0.7.1 (#15) (dependabot[bot])
- [[`2543672317`](https://github.com/datrs/merkle-tree-stream/commit/2543672317e82e3879fc7942cec74d6e83b49c37)]  Keep up with modern times in clippy invocation (#13) (Szabolcs Berecz)
- [[`bfe9fa5630`](https://github.com/datrs/merkle-tree-stream/commit/bfe9fa56307a8db8245abb6d32e8ecd944d1e980)] Update .github (Yoshua Wuyts)

### Stats
```diff
 .github/ISSUE_TEMPLATE.md                 |  40 +----------
 .github/ISSUE_TEMPLATE/bug_report.md      |  23 ++++++-
 .github/ISSUE_TEMPLATE/feature_request.md |  30 ++++++++-
 .github/ISSUE_TEMPLATE/question.md        |  18 +++++-
 .travis.yml                               |   5 +-
 Cargo.toml                                |   7 +--
 examples/main.rs                          |  43 ++++--------
 src/lib.rs                                | 113 +++++++++++++++++++++++++++++--
 tests/lib.rs                              |  41 ++++-------
 9 files changed, 221 insertions(+), 99 deletions(-)
```


