# SDKWork Workspace Metadata

This directory holds source-controlled SDKWork workspace metadata, local skills, and local plugins for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.

## Structure

- `skills/`: local agent skills specific to this repository.
- `plugins/`: local agent plugins specific to this repository.
- `.gitignore`: ignores generator-owned and runtime-owned files.

## Rules

- Do not add secrets, local runtime data, generated transport output, or user-private files here.
- Generated SDK output `.sdkwork/sdkwork-generator-*.json` remains generator-owned.
- Runtime `~/.sdkwork/<application-code>` remains user-private runtime state governed by `RUNTIME_DIRECTORY_SPEC.md`.
