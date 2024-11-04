
# Mira Game Development

This document give you the instruction for handle the code correct<br>
and how to push commits to our **GitHub** repository.

---

## IDEA Plugins you need
In this section we talk about IDEA plugins and why we need them. Note if you code with another IDE like<br>
Vscode, you need to find plugin which like the Jetbrains plugins.

> Note this table can update every time. You need to look for updates if we have<br>
> new plugins installed.

- Rust support
- Env File Support
- GitHub Actions Manager

---

## CI / CD and Branches

If you work on the project you push your code to a separate branch like **feature/issue-33-player-animations**. <br>
We don't push directly on develop, staging or main. The CI will check your code and build them. If the <br>
CI actions was failed your commit (Pull Request) will bot be merged. At the following we talk about our branches.

### Develop (default)

The develop branch is the default one for us programmer. If you create a pull request make sure in head to develop. <br>
This branch is in main time up to date and this is absolutely needed for the tester team. <br>
If you need Tester that test your code, you need to merge from develop into staging. More about that in his own section.

### Staging (Tester)
The staging branch is for tester cases. Here is the rule only merge from develop! <br>
If you merge from develop then the CI will create a Pre-Release version from the current develop <br>
branch. This is useful because the tester don't need to send request to the developer team. If <br>
staging becomes a new merge then the Pre Build Release will change so the tester can do here jobs.

### Main (Release)
This main branch is only be used if the current test (stating) successfully finished. Then the tester can <br>
merge from the staging branch at main branch to trigger a release build. Note only Head Tester allowed that to do!

---

## Code syntax