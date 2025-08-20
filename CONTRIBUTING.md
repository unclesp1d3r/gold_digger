<!-- omit in toc -->

# Contributing to gold_digger

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways
to help and details about how this project handles them. Please make sure to read the relevant section before making
your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The
community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support
> the project and show your appreciation, which we would also be very happy about:
>
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Enhancements](#suggesting-enhancements)
- [Your First Code Contribution](#your-first-code-contribution)
- [Improving The Documentation](#improving-the-documentation)
- [Styleguides](#styleguides)
- [Commit Messages](#commit-messages)
- [IDE and Editor Configuration](#ide-and-editor-configuration)
- [Join The Project Team](#join-the-project-team)

## Code of Conduct

This project and everyone participating in it is governed by the
[gold_digger Code of Conduct](https://github.com/unclesp1d3r/gold_diggerblob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to .

## I Have a Question

> If you want to ask a question, we assume that you have read the
> available [Documentation](https://github.com/unclesp1d3r/gold_digger/wiki).

Before you ask a question, it is best to search for existing [Issues](https://github.com/unclesp1d3r/gold_digger/issues)
that might help you. In case you have found a suitable issue and still need clarification, you can write your question
in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/unclesp1d3r/gold_digger/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
>
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the
> necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

<!-- omit in toc -->

#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to
investigate carefully, collect information and describe the issue in detail in your report. Please complete the
following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment
  components/versions (Make sure that you have read
  the [documentation](https://github.com/unclesp1d3r/gold_digger/wiki). If you are looking for support, you might want
  to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there
  is not already a bug report existing for your bug or error in
  the [bug tracker](https://github.com/unclesp1d3r/gold_diggerissues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside the GitHub community have
  discussed the issue.
- Collect information about the bug:
- Stack trace (Traceback)
- OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
- Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
- Possibly your input and the output
- Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->

#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue
> tracker, or elsewhere in public. Instead, sensitive bugs must be sent by email to .

<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/unclesp1d3r/gold_digger/issues/new). (Since we can't be sure at this point whether
  it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the _reproduction steps_ that someone else can follow to
  recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem
  and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no
  obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs
  with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such
  as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

<!-- You might want to create an issue template for bugs and errors that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for gold_digger, **including completely new
features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the
community to understand your suggestion and find related suggestions.

<!-- omit in toc -->

#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://github.com/unclesp1d3r/gold_digger/wiki) carefully and find out if the functionality
  is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/unclesp1d3r/gold_digger/issues) to see if the enhancement has already been
  suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to
  convince the project's developers of the merits of this feature. Keep in mind that we want features that will be
  useful to the majority of our users and not just a small subset. If you're just targeting a minority of users,
  consider writing an add-on/plugin library.

<!-- omit in toc -->

#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/unclesp1d3r/gold_digger/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point
  you can also tell which alternatives do not work for you.
- You may want to **include screenshots and animated GIFs** which help you demonstrate the steps or point out the part
  which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS
  and Windows, and [this tool](https://github.com/colinkeenan/silentcast)
  or [this tool](https://github.com/GNOME/byzanz) on
  Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most gold_digger users. You may also want to point out the other
  projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your First Code Contribution

#### Development Environment Setup

1. **Clone the repository**:

   ```bash
   git clone https://github.com/unclesp1d3r/gold_digger.git
   cd gold_digger
   ```

2. **Set up development tools**:

   ```bash
   # Install Rust components and development tools
   just setup

   # Install additional tools (optional)
   just install-tools
   ```

3. **Install pre-commit hooks** (recommended):

   ```bash
   # Install pre-commit
   pip install pre-commit

   # Install hooks for this repository
   pre-commit install

   # Test hooks (optional)
   pre-commit run --all-files
   ```

4. **Verify setup**:

   ```bash
   # Run all quality checks
   just ci-check
   ```

#### Code Quality Standards

Before submitting any code, ensure it passes all quality gates:

- **Formatting**: `cargo fmt --check` (100-character line limit)
- **Linting**: `cargo clippy -- -D warnings` (zero tolerance for warnings)
- **Testing**: `cargo test` (all tests must pass)
- **Security**: `cargo audit` (no known vulnerabilities)
- **Pre-commit hooks**: All hooks must pass

The pre-commit configuration automatically enforces:

- Rust code formatting and linting
- YAML/JSON formatting with Prettier
- Markdown formatting with mdformat
- Shell script validation with ShellCheck
- GitHub Actions workflow validation
- Conventional commit message format
- Documentation link checking

### Improving The Documentation

<!-- TODO
Updating, improving and correcting the documentation

-->

## Styleguides

### Code Style

Gold Digger follows strict code style guidelines enforced through automated tools:

#### Rust Code Style

- **Formatting**: Use `rustfmt` with 100-character line limit (configured in `rustfmt.toml`)
- **Linting**: Zero tolerance for clippy warnings (`cargo clippy -- -D warnings`)
- **Error Handling**: Use `anyhow::Result<T>` for fallible functions
- **Documentation**: Document all public APIs with `///` comments
- **Feature Gates**: Use `#[cfg(feature = "...")]` for conditional compilation

#### File Formatting

- **YAML/JSON**: Formatted with Prettier
- **Markdown**: Formatted with mdformat (GitHub Flavored Markdown)
- **Shell Scripts**: Must pass ShellCheck validation
- **Line Endings**: Unix-style (LF) enforced by `.editorconfig`

### Commit Messages

Gold Digger uses [Conventional Commits](https://www.conventionalcommits.org/) format, enforced by pre-commit hooks:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

#### Examples

```
feat(csv): add support for custom delimiters
fix(json): handle null values in nested objects
docs(api): update configuration examples
test(integration): add TLS connection tests
chore(deps): update mysql crate to v24.0
```

#### Pre-commit Validation

The `commitizen` hook validates commit messages automatically. If your commit message doesn't follow the convention, the commit will be rejected with guidance on the correct format.

## IDE and Editor Configuration

This project uses `.editorconfig` to maintain consistent coding style across different editors and IDEs. The configuration ensures:

- UTF-8 encoding
- Unix-style line endings (LF)
- Final newline at end of files
- Trailing whitespace removal (except in Markdown files)
- Consistent indentation (4 spaces for Rust/TOML, 2 spaces for YAML/JSON)

### IDE Metadata

**JetBrains IDE files (`.idea/`, `*.iml`, `*.ipr`, `*.iws`) are intentionally excluded from version control.** These files contain user-specific settings and workspace configurations that should not be shared across contributors.

If you're using a JetBrains IDE (IntelliJ, CLion, etc.), the project will automatically configure itself based on the `.editorconfig` settings. Any IDE-specific settings you need should be configured locally and will not be committed.

### Supported Editors

The project works well with:

- Visual Studio Code (with EditorConfig extension)
- JetBrains IDEs (IntelliJ, CLion, etc.)
- Vim/Neovim (with EditorConfig plugin)
- Emacs (with EditorConfig mode)
- Any editor that supports EditorConfig

## Join The Project Team

<!-- TODO -->

<!-- omit in toc -->

## Attribution

This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
