## 2018-09-10, Version 0.1.0
### Commits
- [[`b40679371f`](https://github.com/danreeves/cargo-cmd/commit/b40679371f8cd45ccca6becae9b2079e8ccdeeca)] fix token (Dan Reeves)
- [[`45e1c14c74`](https://github.com/danreeves/cargo-cmd/commit/45e1c14c743d147f9fd65f60f76300ad21657fdd)] crossgen files (Dan Reeves)
- [[`771555ba78`](https://github.com/danreeves/cargo-cmd/commit/771555ba78bab95985dfcecb7bba4211f62023d3)] some nice stuff (Dan Reeves)
- [[`4bd996ec56`](https://github.com/danreeves/cargo-cmd/commit/4bd996ec569675065292717ec76a07252700e828)] intial implementation (Dan Reeves)

### Stats
```diff
 .gitignore                |   2 +-
 .travis.yml               |  55 ++++++++++++++++++++++-
 Cargo.lock                | 118 +++++++++++++++++++++++++++++++++++++++++++++++-
 Cargo.toml                |  16 ++++++-
 LICENSE                   |   9 ++++-
 README.md                 |  29 ++++++++++++-
 scripts/before_deploy.ps1 |  23 +++++++++-
 scripts/before_deploy.sh  |  32 +++++++++++++-
 scripts/install.sh        |  51 ++++++++++++++++++++-
 scripts/script.sh         |  23 +++++++++-
 src/main.rs               |  93 +++++++++++++++++++++++++++++++++++++-
 11 files changed, 451 insertions(+)
```
## 2018-10-05, Version 0.2.0
### Commits
- [[`c4765a6794`](https://github.com/danreeves/cargo-cmd/commit/c4765a6794c1cef58e909c67c67b05d26525485b)] bump version (Dan Reeves)
- [[`4ba48eaf34`](https://github.com/danreeves/cargo-cmd/commit/4ba48eaf34916238aed256e269354a965e37b2e1)] Passing extra args + clap refactor (#7) (Dan Reeves)
- [[`07a2d7cfbb`](https://github.com/danreeves/cargo-cmd/commit/07a2d7cfbb2b6247e566def6dc10c0bc71256082)] bump version (Dan Reeves)
- [[`c6214aceee`](https://github.com/danreeves/cargo-cmd/commit/c6214aceee35a9f54dd0e632faa4fb79fc75a9be)] Add winapi dependency to fix compilation Windows (Dan Reeves)
- [[`1582f1bedd`](https://github.com/danreeves/cargo-cmd/commit/1582f1bedd79439624e3fdf4fe6f778350e171af)] Add badges and Cargo.toml meta fields (#6) (Dan Reeves)
- [[`3ed647b767`](https://github.com/danreeves/cargo-cmd/commit/3ed647b76731d41d9b63baf17e8daf7dfe594433)] add changelog (Dan Reeves)
- [[`386acb30d0`](https://github.com/danreeves/cargo-cmd/commit/386acb30d0e59ddd2ff312487211d8be26bc298c)] add description (Dan Reeves)

### Stats
```diff
 CHANGELOG.md |  22 ++++++-
 Cargo.lock   | 205 ++++++++++++++++++++++++++++++++++++++++++++++++------------
 Cargo.toml   |  26 +++++---
 README.md    |  19 ++++++-
 src/main.rs  |  66 +++++++++++--------
 5 files changed, 264 insertions(+), 74 deletions(-)
```
## 2018-10-12, Version 0.2.1
### Commits
- [[`af985e4098`](https://github.com/danreeves/cargo-cmd/commit/af985e40982d5176406b7f1cb4618119c40b3ff9)] 0.2.1 (Dan Reeves)
- [[`ba0b6f7a1c`](https://github.com/danreeves/cargo-cmd/commit/ba0b6f7a1ce7ea54302bd133ca598c4ede2088aa)] CI in Windows (#8) (Dan Reeves)
- [[`b7b4db96a5`](https://github.com/danreeves/cargo-cmd/commit/b7b4db96a535c8314fba418a7d57f37828aed154)] disable tests on linux because assert_cli uses backtrace and backtrace needs extra deps (Dan Reeves)
- [[`7dce324ddd`](https://github.com/danreeves/cargo-cmd/commit/7dce324ddd32a85681d89871046f22bc7a4e5cea)] another test (Dan Reeves)
- [[`51942aad9b`](https://github.com/danreeves/cargo-cmd/commit/51942aad9becda01c78de05cc157757103cd4622)] tests and fix an issue (Dan Reeves)
- [[`6e80fe42b9`](https://github.com/danreeves/cargo-cmd/commit/6e80fe42b9bd8c37ecdebbb5013cf389a93604b7)] update changelog (Dan Reeves)

### Stats
```diff
 .travis.yml        |   7 ++-
 CHANGELOG.md       |  19 +++++++-
 Cargo.lock         | 151 +++++++++++++++++++++++++++++++++++++++++++++++++++++-
 Cargo.toml         |  10 +++-
 scripts/install.sh |  19 ++-----
 scripts/script.sh  |  20 ++++---
 src/main.rs        |   4 +-
 tests/test.rs      |  57 ++++++++++++++++++++-
 8 files changed, 262 insertions(+), 25 deletions(-)
```
## 2018-11-16, Version 0.3.0
### Commits
- [[`c69fdd1c68`](https://github.com/danreeves/cargo-cmd/commit/c69fdd1c68573f26d71349d845c4af94e30dfb6c)] bump version (Dan Reeves)
- [[`e5f6a8ddfe`](https://github.com/danreeves/cargo-cmd/commit/e5f6a8ddfecb73a338807eaec8107db8f4282909)] Pre- & Post- commands (#14) (Dan Reeves)
- [[`52f4b24378`](https://github.com/danreeves/cargo-cmd/commit/52f4b24378d856d85f34fece008583327ea12003)] Windows releases (#9) (Dan Reeves)
- [[`40a032a13e`](https://github.com/danreeves/cargo-cmd/commit/40a032a13ebbffa54de4061713f5a8da99d0537e)] nightly → stable (#12) (Dan Reeves)
- [[`03132c1352`](https://github.com/danreeves/cargo-cmd/commit/03132c1352861cc30398ecb74bb46b1f56e0dde6)] fix docs.rs link (#10) (Dan Reeves)
- [[`50b49f162c`](https://github.com/danreeves/cargo-cmd/commit/50b49f162c49c6263c08798a5197ab835ffa7507)] update lockfile (Dan Reeves)
- [[`22706b59cd`](https://github.com/danreeves/cargo-cmd/commit/22706b59cda965507af684aa7ebe1a295a29520b)] upadte changelog (Dan Reeves)

### Stats
```diff
 .travis.yml              | 10 ++++----
 CHANGELOG.md             | 21 +++++++++++++++++-
 Cargo.lock               | 17 +++++++-------
 Cargo.toml               | 18 +++++++++++++--
 README.md                | 50 +++++++++++++++++++++++++++++++++++++----
 scripts/before_deploy.sh | 12 +++++++++-
 scripts/install.sh       |  4 +---
 scripts/script.sh        |  2 +-
 src/main.rs              | 60 +++++++++++++++++++++++++++++++------------------
 tests/test.rs            | 52 +++++++++++++++++++++++++++++++++++++-----
 10 files changed, 195 insertions(+), 51 deletions(-)
```
