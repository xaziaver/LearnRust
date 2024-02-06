# git & GitHub

## common `git` commands
- `git init` - Initialize a new Git repository.
- `git clone <repository-url>` - Clone an existing repository.
- `git add .` - Stage all changes for commit.
- `git add <file-name>` - Stage specific file for commit.
- `git commit -m "commit message"` - Commit staged changes with a message.
- `git push origin <branch-name>` - Push commits to a remote repository branch.
- `git pull` - Fetch and merge changes from the remote branch to local.
- `git status` - Show the working tree status.
- `git branch <branch-name>` - Create a new branch.
- `git checkout <branch-name>` - Switch to a specified branch.
- `git merge <branch-name>` - Merge a branch into the current branch.
- `git log` - Show commit logs.

## common `gh` commands

- `gh repo clone <repository>` - Clone a repository.
- `gh issue create` - Create a new issue.
- `gh issue list` - List and filter issues in a repository.
- `gh pr create` - Create a new pull request.
- `gh pr checkout <pr-number>` - Check out a pull request locally.
- `gh pr list` - List and filter pull requests in a repository.
- `gh pr merge <pr-number>` - Merge a pull request.
- `gh workflow run <workflow-id>` - Trigger a GitHub Actions workflow.
- `gh auth login` - Authenticate with GitHub.

## common github api commands
- List Repositories:
  ```bash
  curl -H "Authorization: token YOUR_PAT" https://api.github.com/user/repos
  ```
- Create an Issue:
  ```bash
  curl -X POST -H "Authorization: token YOUR_PAT" -d '{"title":"Issue Title", "body":"Issue body."}' https://api.github.com/repos/username/repo/issues
  ```
- List Pull Requests:
  ```bash
  curl -H "Authorization: token YOUR_PAT" https://api.github.com/repos/username/repo/pulls
  ```
- Merge a Pull Request (Replace `:number` with the PR number):
  ```bash
  curl -X PUT -H "Authorization: token YOUR_PAT" https://api.github.com/repos/username/repo/pulls/:number/merge
  ```
- Trigger a Workflow (Replace `:workflow_id` with the workflow ID and `:ref` with the branch or tag):
  ```bash
  curl -X POST -H "Authorization: token YOUR_PAT" -d '{"ref":"branch-name"}' https://api.github.com/repos/username/repo/actions/workflows/:workflow_id/dispatches
  ```

## useful links
***[Pro Git Book](https://git-scm.com/book/en/v2)***
comprehensive resource covering Git basics to advanced topics

***[Git Reference Manual](https://git-scm.com/docs)***
official Git reference manual, detailed documentation of every Git command

***[GitHub Learning Lab](https://lab.github.com/)***
interactive tutorials to learn GitHub

***[GitHub CLI Documentation](https://cli.github.com/manual/)***

***[GitHub API Documentation](https://docs.github.com/en/rest)***


# Markdown

## useful links

***[Markdown Cheat Sheet](https://www.markdownguide.org/cheat-sheet/)***
quick reference to common syntax

***[Markdown Guide](https://www.markdownguide.org)***
Comprehensive guide

***[GitHub Flavored Markdown Spec](https://github.github.com/gfm/)***
GitHub supported syntax (task lists, tables, fenced code blocks etc.)


# Miscellaneous

## Adding Fork as a Submodule and Making Commits
1. **Add Fork as a Submodule**:
   - In your main repository, add the fork as a submodule to include it as part of your project.
   - Command: `git submodule add <url-of-your-fork> path/to/submodule`

2. **Initialize and Update Submodule After Cloning**:
   - To initialize and pull changes from your fork to the submodule after cloning 
     - `git submodule init`
     - `git submodule update`

3. **Committing Changes Within the Submodule**:
   - Navigate to the submodule directory: `cd path/to/submodule`
   - Add and commit your changes: `git add .` and `git commit -m "Your message"`
   - Push changes to your fork: `git push`

4. **Updating Submodule Reference in Main Repository**:
   - After making changes in the submodule (fork), update the reference in your main repository.
   - `git add path/to/submodule`
   - `git commit -m "Update submodule reference"`
   - `git push`

