# How to contribute to Rayhunter

## Filing issues and starting discussions

Our issue tracker is [on GitHub](https://github.com/EFForg/rayhunter/issues).

- If your rayhunter has found an IMSI-catcher, we strongly encourage you to
  [send us that information
  privately.](https://efforg.github.io/rayhunter/faq.html#help-rayhunters-line-is-redorangeyellowdotteddashed-what-should-i-do) via Signal.

- Issues should be actionable. If you don't have a
  specific feature request or bug report, consider [creating a
  discussion](https://github.com/EFForg/rayhunter/discussions) instead.

  Example of a good bug report:

  - "Installer broken on TP-Link M7350 v3.0"
  - "Display does not update to green after finding"
  - "The documentation is wrong" (though we encourage you to file a pull request directly)

  Example of a good feature request:

  - "Use LED on device XYZ for showing recording status"

  Example of something that belongs into discussion:

  - "In region XYZ, do I need an activated SIM?"
  - "Where to buy this device in region XYZ?"
  - "Can this device be supported?" While this is a valid feature
    request, we just get this request too often, and without some exploratory
    work done upfront it's often unclear initially if that device can be
    supported at all.

- The issue templates are mostly there to give you a clue what kind of
  information is needed from you, and whether your request belongs into the issue
  tracker. Fill them out to be on the safe side, but they are not mandatory.

## Contributing patches

To edit documentation or fix a bug, make a pull request. If you're about to
write a substantial amount of code or implement a new feature, we strongly
encourage you to talk to us before implementing it or check if any issues have
been opened for it already. Otherwise there is a chance we will reject your
contribution after you have spent time on it.

On the other hand, for small documentation fixes you can file a PR without
filing an issue.

Otherwise:

- Refer to [installing from
  source](https://efforg.github.io/rayhunter/installing-from-source.html) for
  how to build Rayhunter from the git repository.

- Ensure that `cargo fmt` and `cargo clippy` have been run.

- If you add new features, please do your best to both write tests for and also
  manually test them. Our test coverage isn't great, but as new features are
  added we are trying to prevent it from becoming worse.

If you have any questions [feel free to open a discussion or chat with us on Mattermost.](https://efforg.github.io/rayhunter/support-feedback-community.html)

## Making releases

This one is for maintainers of Rayhunter.

1. Make a PR changing the versions in `Cargo.toml` and other files.
   This could be automated better but right now it's manual. You can do this easily with sed:
   `sed -i "" -E 's/x.x.x/y.y.y/g' */Cargo.toml`

2. Merge PR and make a tag.

3. [Run release workflow.](https://github.com/EFForg/rayhunter/actions/workflows/release.yml)

4. Write changelog, edit it into the release, announce on mattermost.
