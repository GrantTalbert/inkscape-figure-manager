# Inkscape Figure Manager

A manager for inserting and editing inkscape figures inside a vim workflow.

Implements the following commands:
- `--insert [path]`: Looks for all files with the `pdf_tex` extension at the directory `path`, and lists them all in a rofi prompt. Once the user selects a figure, it returns the necessay code to insert it into a LaTeX documnet.
- `--create [title] [path]`: Creates the file `path/title.svg` and opens it in Inkscape.
  - The program requires a `template.svg` file to be in your `~/.config/inkscape-figure-manager` directory.
  - If I ever write an installation script, it will make this directory and copy a default template to the directory.
- `--edit [path]`: Lists all `.svg` files in the `path` directory and lists them in a rofi menu. Opens the selected file for editing.
- `--daemon [kill]` **WORK IN PROGRESS** Running this command without the `kill` parameter starts the daemon, which watches for all instances of inkscape opening as a cause of this script and exits inkscape when the file is saved. Including the `kill` parameter kills the daemon.