# Markdown Resources

## Markdown Cheat Sheet

Quick reference to common syntax

- [Markdown Cheat Sheet](https://www.markdownguide.org/cheat-sheet/)

## Markdown Guide

Comprehensive guide

- [Markdown Guide](https://www.markdownguide.org)

## GitHub Flavored Markdown (GFM)

GitHub supported syntax (task lists, tables, fenced code blocks etc.)

- [GitHub Flavored Markdown Spec](https://github.github.com/gfm/)


# git Miscellaneous

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
