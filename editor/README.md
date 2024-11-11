# Editor

This crate controls building and using the editing space (and line numbers widget) of the editor app. It involved loading in the file, controlling the cursor, and editing (plus saving) the text of the file. Going forward, some functionality of this crate may be offloaded to other crates as other parts of the editor app expand (i.e. move the cursor controls out of this crate). For now, this is the bulk of the editor app.

## EditorSpace

The struct for the editing space of the app. Each `EditorSpace` opens its own file and handles the IO for editing.

### Public Methods

- `new`: Construct a new `EditorSpace` over a file
- `handle_input`: Take input from the terminal and perform its associated action
- `render_ui`: Draw the `EditorSpace` and its line numbers to the terminal
