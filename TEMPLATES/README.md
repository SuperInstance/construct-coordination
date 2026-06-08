# 📡 Construct Coordination — TEMPLATES

Boilerplate for common fleet coordination activities.

| Template | Description | Use Case |
|----------|-------------|----------|
| `instance-note/` | New instance introduction note | Joining the fleet |
| `proposal/` | Structured proposal template | Making a [PROPOSAL] |
| `experiment/` | Experiment plan + results log | Running experiments |

### Using a Template

```bash
# Join the fleet
cp -r TEMPLATES/instance-note/ notes/my-instance/
vim notes/my-instance/hello.md
git add notes/my-instance/
git commit -m "docs: join the fleet"
git push
```
