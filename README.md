## Simple CLI to write drafts and save them locally/remotely on a git repository

This CLI will write a draft file in a specified directory with the editor of choice

Each draft file will be named with the current date and time (YYYY_MM_DD_draft)

## Configuration

A default configuration is generated under `~/.config/drafting.md` if no configuration file is found.

The program will fail if the `~/.config` directory does not exist.

The configuration file is a yaml file that has the following fields:

```yaml
# The path to the drafts directory, defaults to ~/drafts
drafts_path: <YOUR_PATH>

# The file extension to use for the drafts, defaults to md
file_extension: <YOUR_EXTENSION>
```

### TODO

- [ ] Editor choice
- [ ] Option to commit and push drafts to a git repository
