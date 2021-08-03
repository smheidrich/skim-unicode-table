# skim-unicode-table

Interactive fuzzy-searchable Unicode table using [lotabout's skim][1].

Can be used to quickly search for a character and copy it to the clipboard.


## Demo video

https://user-images.githubusercontent.com/3827982/127855578-b052bf0b-f4e5-4686-b171-5e5fea784326.mov


## Installation

Install via pip:

(**NOTE:** If prebuilt wheels are not available for your operating system,
installation will take a long time and use a lot of space because it has to
compile a fork of skim from scratch.)

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

### Optional dependencies

- [atanunq's viu][2]: Required for enlarged character previews like in the demo
  video above to work. Make sure it can be found from `PATH`.

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


## Development notes

### Why use a forked skim?

`fuzzy-matcher` (used internally by skim) had default scoring that didn't seem
very suitable for this use case, as it kept returning something unexpected as
the best match, so I had to [tweak it a bit][3] and fork skim so that it uses
this modified version.


[1]: https://github.com/lotabout/skim "lotabout/skim on GitHub"
[2]: https://github.com/atanunq/viu "atanunq/viu on GitHub"
[3]: https://github.com/smheidrich/fuzzy-matcher/compare/master...smheidrich:for_skim_unicode#diff-2099478acb23e56398caeea19c6a315098d4d7ae4d6d642b8cbacb647ce3f2e8
