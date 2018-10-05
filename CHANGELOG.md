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
