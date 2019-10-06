# Contributing

Please note we have a [code of conduct](https://github.com/samueldple/date_time/blob/master/CODE_OF_CONDUCT.md). Please follow it in all your interactions with the project.

## Issues and Feedback Process

-   If you spot any issues with the project or wish to request any new features, feel free to add an issue in the GitHub issue tracker.
-   You can also use the issue tracker to leave feedback which I'll respond to if required.

## Pull Request Process

1.  Pick up any of our issues in the issue tracker (feel free to self-assign these).
2.  If you have any questions, feel free to ask on the issue. I'll respond as quickly as possible and am happy to help newcomers out if you don't know how to get started.
3.  Once you've edited the repo, put in a PR. Assign [@samueldple](https://github.com/samueldple/) and attach any labels you think fit the PR. Our bots will add more labels once you submit the PR.
4.  I'll get back to you with any changes that need making before the PR can be merged, and merge it once it all looks good

## Coding Standards

Code in this repository is held to a high standard, but don't let that put you off from contributing; it's all about following these simple rules:

-   Code submitted should be formatted using rustfmt.
-   Code submitted should pass `cargo clippy -- -D warnings`.
-   All code submitted should be covered by tests unless a good reason for not testing them is provided. This is assessed by our code coverage score.

These good practices are enforced by the following procedures:

-   Continuous Integration is used to check that code is formatted correctly and all works as expected.
-   All PRs will be reviewed by a maintainer. Here, suggestions will be made on how to improve the code until it is agreed that it is of the required standard.
-   Bors will be used to merge all PRs. This ensures that the `master` branch is always green.
-   Dependabot will ensure dependencies are kept up to date.
