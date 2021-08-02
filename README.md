# skim-unicode-table

Interactive fuzzy-searchable Unicode table using [lotabout's skim][1].

Can be used to quickly search for a character and copy it to the clipboard.


## Demo video

TODO


## Installation

Install via pip:

(**NOTE:** If prebuilt wheels are not available for your operating system,
installation will take a long time and use a lot of space because it has to
compile skim from scratch.)

```bash
pip install skim-unicode-table
```

This will put two scripts into pip's preferred binary directory:
`skim-unicode-table` and `skim-unicode-table-xsel`. The former just displays
the table and prints the selected character and its various names separated by
two spaces when the user presses `Enter`, then exits. The latter also copies
the selected character into the clipboard using `xsel` (check if this is
installed!), which might not work on all platforms. If it doesn't work, you can
try to build your own script for copying the character into the clipboard using
`skim-unicode-table`.

If you want enlarged character previews like in the demo video above to work,
you'll also have to install [atanunq's viu][2] and make sure it can be found
from `PATH`.

### Launching in new terminal window

The most common setup will probably be to bind `skim-unicode-table-xsel` to
a hotkey that launches it in a new terminal window. How to do this depends on
the specific terminal emulator used (there is no unified CLI, unfortunately).

In `gnome-terminal`'s case, it would be (absolute path in case pip's preferred
binary installation folder isn't in `PATH` for the application that handles
hotkeys, commonly the window manager):

```bash
gnome-terminal -- /path/to/skim-unicode-table-xsel
```



[1]: https://github.com/lotabout/skim "lotabout/skim on GitHub"
[2]: https://github.com/atanunq/viu "atanunq/viu on GitHub"
