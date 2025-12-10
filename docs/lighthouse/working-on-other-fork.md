```markdown
# Working on Someone Else's Fork/Branch

## Initial Setup

1. **Add their fork as a remote**
   ```bash
   git remote add <remote-name> https://github.com/<their-username>/<repo-name>.git
   git fetch <remote-name>
   ```

2. **Check out their branch**
   ```bash
   git checkout -b <local-branch-name> <remote-name>/<their-branch-name>
   ```

## Making Changes

3. **Work normally**
   ```bash
   # Make your changes
   git add .
   git commit -m "your changes"
   ```

4. **Push to YOUR fork**
   ```bash
   git push origin <local-branch-name>
   ```

## Creating a PR

5. **On GitHub:**
   - Go to your fork or the target repo
   - Click "Compare & pull request" banner
   - Set base repo and branch (where it should merge)
   - Set head repo (your fork) and branch
   - Create PR

## Quick Reference

```bash
# View all remotes
git remote -v

# Update their branch with latest changes
git fetch <remote-name>
git merge <remote-name>/<their-branch-name>

# Typical remote names:
# origin -> your fork
# upstream -> main project repo
# <remote-name> -> collaborator's fork
```

## Notes

- `<local-branch-name>` is often the same as `<their-branch-name>`
- You can name `<remote-name>` anything (common: their GitHub username)
- Push always goes to `origin` (your fork) unless you have write access to theirs
```