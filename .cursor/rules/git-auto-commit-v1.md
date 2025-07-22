---
description: ALWAYS commit changes after each code modification using conventional commits format. NEVER `git add .`
alwaysApply: false
comment: This rule works quite reliably, but I'm evaluating a newer version with more XML-like syntax. This copy is kept for backup/reference purposes.
---

# Auto Git Commit Rule

After any code change or file modification, automatically create a Git commit with the following
process:

## Commit Process

1. Stage ONLY modifications made as part of the current task being worked on (NEVER `git add .`)
   - Use `git add <specific-file1> <specific-file2>` for targeted staging
   - Only include files that were created or modified as part of the current task
   - Avoid staging unrelated files that may have been modified separately
2. Create a commit with conventional commits format
3. Include a concise explanation of what was changed and why

## Conventional Commits Format

Use the following structure: `<type>[optional scope]: <description>`

### Common Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Code formatting changes
- `refactor`: Code refactoring
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI configuration changes
- `build`: Build system changes
- `revert`: Reverts a previous commit

### Examples with Scope

- `feat(auth): add user authentication system`
- `fix(api): resolve null pointer exception in user service`
- `docs(readme): update installation instructions`
- `refactor(utils): simplify date formatting functions`

## Commit Message Guidelines

- **Title**: Use conventional commits format
- **Body**: Concisely explain:
  - What was changed
  - Why the change was made
  - Reference the original user prompt/request that triggered the change

## Example Commit Messages

```text
feat(server): add sentiment analysis endpoint

- Added new /analyze-sentiment endpoint to server.py
- Implemented sentiment analysis tool integration
- User requested: "Add sentiment analysis functionality"
```

```text
fix(database): resolve connection timeout

- Increased connection timeout from 5s to 30s
- Added retry logic for failed connections
- User reported: "Database keeps timing out"
```

## Execution

Execute the git commit immediately after completing any code modification, ensuring all changes are
properly staged and committed with descriptive messages that trace back to the original user
request.
