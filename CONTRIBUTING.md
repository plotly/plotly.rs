# Contributing to Ploty.rs

Contribution in the form of suggestions, bug reports, pull requests and feedback is welcome from everyone. In this document
you'll find guidance if you are considering to offer your help to this project.

## Submitting bug reports and feature requests

When reporting a bug or asking for help, please include enough details so that
the people helping you can reproduce the behavior you are seeing. For some tips
on how to approach this, read about how to produce a [Minimal, Complete, and
Verifiable example].

[Minimal, Complete, and Verifiable example]: https://stackoverflow.com/help/mcve

When making a feature request, please make it clear what problem you intend to
solve with the feature, any ideas for how Plotly.rs could support solving that
problem, any possible alternatives, and any disadvantages.

## Process

Before spending time and effort in making changes to the library, please open an issue first explaining the proposed change 
and indicate that you would be happy to implement/fix the issue. Once the utility of the addition is discussed and verified,
you're welcome to proceed.

Fork [plotly](https://igiagkiozis.github.io/plotly/) to your own account and checkout the `dev` branch. Once the bug/feature is complete, make sure you update 
appropriately the [change log](CHANGELOG.md). 

For change-log updates please use previous entries as reference, namely classify
the nature of the change, e.g. fixed, changed, added etc., and a brief description. The entry in the change-log that should be
used for the update will always be marked with YEAR-XX-XX, e.g. 2022-XX-XX.

Check that the test suite passes locally and that all examples still execute and produce the expected results before submitting a pull request.

Make a pull request with your changes directly to the `dev` branch. Please note, pull requests to the `master` branch will not
be considered. The `master` branch is reserved for release.


Also, include a description of our changes as documented in the 
[change-log](CHANGELOG.md); just copy paste these additions in the pull request description. Wait for one of the reviewers 
to look at your code and either merge it or give feedback which you should adapt to.

## Code of Conduct

In all forums, we follow the [Rust Code of Conduct]. For escalation or moderation issues please contact Ioannis (i.giagkiozis@gmail.com) instead of the Rust moderation team.

[Rust Code of Conduct]: https://www.rust-lang.org/conduct.html

## Attribution

Parts of this document are adapted from the [serde](https://github.com/serde-rs/serde) contributing guide.