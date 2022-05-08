# Kitten: A mini cat command 🐱

In this repo, we will use Rust to build a mini version of the command line tool cat.

## Clone

Please **do not download** the repo as a zipped file.
We will be updating its content as time goes on.

If you are using University computers, note that git and GitHub Desktop are both available via AppsAnywhere.

If you are using Replit, note that you can import a GitHub repository into an empty Repl.

Otherwise, on your own computers, feel free to install [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git), the [GitHub client](https://github.com/cli/cli) or the [GitHub Desktop](https://desktop.github.com) app.

1. use `git` to clone it:

   ```shell
   git clone https://github.com/manighahrmani/kitten.git
   ```

2. use the GitHub CLI tool:

   ```shell
   gh repo clone manighahrmani/kitten
   ```

3. click the Code button and select Open with GitHub Desktop.

## Select the branch

As we go through every worksheet, a corresponding branch will be added here.

The branch for the first worksheet is `worksheet-0` and is the default branch.
To access the code for any other branch, you must switch your current branch.

Follow the instructions below to switch your branch to worksheet-x (where x is a number):

1. use `git` to checkout a branch:

   ```shell
   git checkout worksheet-x
   ```

   and to view all available branches:

   ```shell
   git branch -v
   ```

2. if you are using the GitHub Desktop client, follow this [instruction on how to switch branches on GitHub](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/making-changes-in-a-branch/managing-branches#switching-between-branches)

## Run

Note that the repo will only contain the source code (not the compiled executable) contained within the `src` folder.
You should thus build and execute the code using Cargo.

We recommend you install Cargo using [Rustup](https://rustup.rs#).
Then, from your terminal, navigate to the cloned kitten folder and run the following to compile the package:

```shell
cargo build
```

Now run the compiled package by running:

```shell
cargo run
```

Open the documentation for the package by running:

```shell
cargo doc --open
```

Test the entire package by running the following. But note that we have not included the text files used in some of the tests. You would need to create files as outlined by the comments in the `lib.rs`.

```shell
cargo test
```

## Worksheets

- [Worksheet 0: Getting started with Rust](https://docs.google.com/document/d/1-Whyq8QIRzTl00rSeeeO70deLPhB3_tEREvbEE2ALkE/edit?usp=sharing) guides you through the installation of Cargo and the creation of a Hello World package.

- [Worksheet 1: Types, Variable & Input/Output](https://docs.google.com/document/d/1J5LmgJFnPYLjJBGiVSL7BezFMMhLZB2oqxctPSc2cHc/edit?usp=sharing) introduces you to variables and data types in Rust and ends with reading the user's input from standard input.

- [Worksheet 2: Strings, Errors & Control flow](https://docs.google.com/document/d/1HfJy0VWwNgMSGts4NSk02kDhHqGVJ6PN8dktgqJS074/edit?usp=sharing) discusses methods on the &str/String types and errors that may arise while working with them. Ends with an introduction to control flow in the language.

- [Worksheet 3: Modularization & Documentation](https://docs.google.com/document/d/18jFJCItgXFX6bDAsA7tSs69jKCeuzHi-N7-y-MgeUcQ/edit?usp=sharing) to increase the maintainability of your program, you should start to modularize your code and document it. This worksheet aims to help you with both.

- [Worksheet 4: Testing & File handling basics](https://docs.google.com/document/d/1Ni4DrGqYZI3D0Xa5FdesZctYAuFeZ5GsYCRgpivPI4U/edit?usp=sharing) this worksheet introduces you to testing plus teaches you the basics of file handling with Test Driven Development.

## Further reading

The textbook used to write the worksheets is [The Rust Programming Language textbook](https://doc.rust-lang.org/book/).
