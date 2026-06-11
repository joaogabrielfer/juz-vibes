# Juz implementation

The primary integrated CLI is `pyx`. Initial implementation may support pragmatic transitional commands like `pyx build <file.juz>` before the documented package/ini command model is fully implemented.
Do not modify the VM implementation unless explicitly requested; treat `bytecode-spec.md` and provided VM binaries as the compatibility boundary.

For implementing language features and creating plans for it, always follow the documentation present on @site/docs/, unless asked otherwise. Inside the docs folder, there will be another set of folders, one for each version. You shall only access the latest one when you need to reference it. The versions are not sorted alphabetically, in a sense that v0.12 is more recent than v0.2, so you need to follow v0.12.
When referencing docs, determine the latest version with version-aware sorting and only read that version unless explicitly asked.

- Create test whenever a new feature is added.
- Create new CI/CD tests as well.
- Give a suggestion to the user if you feel something would be good to be added to the AGENTS.md file.

If the feature you are to implement is large enough, create a branch for it and go on commiting on it. Otherwise, you can commit on main.

Whenever there has been a substancial change to the language, update the README.md file to reflect it. The docs in the site/docs directory should be the 'idealized' version of the language, where the README contains what is really implemented.
The README also should contain current project status, docs version, and how to run tests.

Whenever we talk about some language, CLI, tooling feature that diverges or build upon the existing docs, you should create a report of what we have done, so i can to update the docs for the next iteration. The docs and implementation should not diverge for more than one version ideally.
