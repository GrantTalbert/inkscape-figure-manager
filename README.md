<h1 align=center>Inkscape Figure Manager</h1>

<h3 align=center>A manager for inserting and editing inkscape figures inside a vim workflow.</h3>

**This project is designed for Linux systems, and likely will not work on MacOS, Windows, and other operating systems.** This project is intended to be a rewrite of [Gilles Castel's figure manager](https://github.com/gillescastel/inkscape-figures) in rust, because I didn't feel like figuring out how his python script worked because it looked kind of messy. I also learned rust *while writing the code* for this project, so the code is a ChatGPT inspired mess. Feel free to make PRs fixing stuff if you feel like it.

**This project requires you to have [Rofi](https://github.com/davatorium/rofi) or one of the [various forks](https://github.com/lbonn/rofi) installed.** If you would like to use a different dmenu, you will need to modify [this file](https://github.com/GrantTalbert/inkscape-figure-manager/blob/master/src/utils/rofi.rs), and recompile the project yourself.

In order to set up this project, you need to `mkdir inkscape-figure-manager` in your `.config` directory, and you need to add a file named `template.svg` into that directory. This is the file that will be used as a template every time you create a new file.

<h2 align=center>Implements the following commands:</h2>

- `--insert [path]`: Looks for all files with the `pdf_tex` extension at the directory `path`, and lists them all in a rofi prompt. Once the user selects a figure, it returns the necessay code to insert it into a LaTeX documnet.
- `--create [title] [path]`: Creates the file `path/title.svg` and opens it in Inkscape.
  - The program requires a `template.svg` file to be in your `~/.config/inkscape-figure-manager` directory.
  - If I ever write an installation script, it will make this directory and copy a default template to the directory.
- `--edit [path]`: Lists all `.svg` files in the `path` directory and lists them in a rofi menu. Opens the selected file for editing.
- `--daemon [start/kill]` **WORK IN PROGRESS** Running this command without the `kill` parameter starts the daemon, which watches for all instances of inkscape opening as a cause of this script and exits inkscape when the file is saved. Including the `kill` parameter kills the daemon.

<h2 align=center>Suggested Implementation</h2>
If you use [Neovim](https://github.com/neovim/neovim), then you can add the following command to your `init.lua`, or any other configuration file.
```lua
local function manage_figures(command, title)
	local root = vim.b.vimtex and vim.b.vimtex.root
	if not root then
		vim.notify("Vintex root not found. Are you working in a LaTeX project?")
		return
	end
	
	local figures_dir = root .. "/figures"
	local stat = vim.loop.fs_stat(figures_dir)
	if not stat or stat.type ~= "directory" then
		vim.notify("No figures directory found in the project root")
		return
	end

	local cmd
	if title then
		cmd = string.format("inkscape-figure-manager --%s %s %s", command, title, figures_dir)
	else
		cmd = string.format("inkscape-figure-manager --%s %s", command, figures_dir)
	end

	local handle = io.popen(cmd)
	local result = handle:read("*a")
	handle:close()

	if result and #result > 0 then
		vim.api.nvim_put(vim.split(result, "\n"), "c", true, true)
	else
		vim.notify("Executed with no output")
	end
end
```
This function takes the `command` parameter, which is the command you want to run. For example, passing `edit` will call `inkscape-figure-manager --edit`. This command assumes that in the directory containing your `master.tex`, there is a subdirectory named `figures`. If you have a different file structure, edit the definition of `figures_dir` to accomidate this. This function also takes the optional `title` parameter, which should only be passed when calling the `create` command. This will be the title of the file you create.

You can bind this functionality to keybinds. For example,
```lua
-- Keybind 1
vim.keymap.set("n", "<C-f>", function()
	manage_figures("insert")
end, { noremap = true, silent = true })

-- Keybind 2
vim.keymap.set("i", "<C-f>", function()
	local current_line = vim.api.nvim_get_current_line()
	if not current_line or #current_line == 0 then
		vim.notify("Current line is empty, cannot create a title")
		return
	end

	local title = current_line:gsub("%s", "-")

	vim.api.nvim_buf_set_lines(0, vim.fn.line(".") - 1, vim.fn.line("."), false, {})
	manage_figures("create", title)
end, { noremap = true, silent = true })
```
Keybind 1 occurs when you press <C-f> in *normal* mode, and it will open a Rofi dialogue which will display all svg files in your figures directory. Selecting one will insert the following LaTeX code:
```tex
\begin{figure}[ht]
    \centering
    \incfig{figure-name}
    \caption{figure-name}\label{fig:figure-name}
\end{figure}
```
This code assumes your preamble includes the function declaration
```tex
\newcommand{\incfig}[1]{%
    \def\svgwidth{\columnwidth}
    \import{./figures/}{#1.pdf_tex}
}
```
To change the exact block of code that is inserted, you need to modify the `get_latex_code` function in [this file](https://github.com/GrantTalbert/inkscape-figure-manager/blob/master/src/utils/general.rs). I may add a config file which allows for more dynamic modification of this in the future.

<h2 align=center>Installation</h2>

I don't really feel like writing an install script, so you will need to do a lot of manual installation. First, download the latest release. You'll need to find a location in your system's PATH to move the binary to. Open a terminal and run
```bash
export $PATH
```
Usually, `/usr/local/bin` will be in your PATH, and this is where I recommend installing the binary. However, you may need to move it somewhere else, if it is not in your PATH.
```bash
cd Downloads
sudo mv inkscape-figure-manager /usr/local/bin
```
Next, you need to create your template file in the config directory. Prepare a `template.svg` file to use as your template. Then
```bash
cd ~/.config
mkdir inskcape-figure-manager
cd inskcape-figure-manager
cp /path/to/your/template.svg ./
```
The inkscape figure manager is now installed. However, you still need to create a method for starting the daemon. I personally recommend the following two options:
- Start the daemon as a background process on system boot;
- Start the daemon every time you open a LaTeX file in nvim.

The former is fairly simple. I personally use [Hyprland](https://hyprland.org/) as my window manager, so in my `hyprland.conf`, I can put
```
exec=once = inkscape-figure-manager --daemon start
```
The latter is likely not complicated, but I personally don't feel lik doing it.

<h2 align=center>Manual Compilation</h2>
If you want to manually compile the project so you can modify the source code, first run
```bash
git clone https://github.com/GrantTalbert/inkscape-figure-manager
cd inkscape-figure-manager
```
To build the project, run
```bash
cargo run --package inkscape-figure-manager --bin inkscape-figure-manager
```
The binary will be located at `inkscape-figure-manager/target/debug/inkscape-figure-manager`.