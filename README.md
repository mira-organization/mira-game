[![Main Pipeline](https://github.com/mira-organization/mira-game/actions/workflows/cargo.yml/badge.svg?branch=develop)](https://github.com/mira-organization/mira-game/actions/workflows/cargo.yml)

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
CI actions was failed your commit (Pull Request) will not be merged. At the following we talk about our branches.

### Develop (Default)

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

## IDEA (Jetbrains) Setup

We use RustRover for the Project and recommended this as default IDEA. First we clone the rust project <b>
``git@github.com:mira-organization/mira-game.git`` this shows like: <br>
![Git Version Clone](/docs/IDEA_VERSION_CONTROL.png)

After cloning, you will become a popup window which ask for Trust. **Trust**.
Now your IDEA looks like this:
![IDEA Main](/docs/IDEA_MAIN.png)

Now you need to start the program. You can do that when you click the Play button. If their no Play button <br>
go to main.rs which storage in ``src/main.rs`` and click in the ``fn main() -> AppExit {}`` function the Play button. <br>
Is no Play button visible it is currently at build state look at build (Cargo)

---

## Code syntax

Here we describe how to code with bevy and rust. We will talk about code structure and in code documentation. <br>
The code structure is very important: <br>
mod.rs files only describe models (struct) or enums. The mod will handle like a main file. <br>
Here is an example:
````rust
use bevy::prelude::*;

pub struct Model {
    is_nice: bool,
}

pub enum Enum {
    Entry
}

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        // app can be used if the main function at main.rs was called!
    }
}
````

Note that this code is only an example but our structure shows like this.

### Unit Tests \ In Code Documentation

We use the rust unit tests. All .rs files needed ``#[cfg(test)]`` as functions for handle unit tests.
here is an example of the main unit test:

````rust
#[cfg(test)]
mod tests {
    use super::*;

    /// This is an in code document and will show at hover over the function
    /// put here examples.
    #[test]
    fn test_app_uses_vulkan_backend() {
        let settings = create_gpu_settings();

        assert_eq!(settings.backends, Some(Backends::VULKAN));
        assert!(settings.features.contains(WgpuFeatures::POLYGON_MODE_LINE));
    }
}
````

