# Editor

This crate controls building and using the editing space (and line numbers widget) of the editor app. It involved loading in the file, controlling the cursor, and editing (plus saving) the text of the file. Going forward, some functionality of this crate may be offloaded to other crates as other parts of the editor app expand (i.e. move the cursor controls out of this crate). For now, this is the bulk of the editor app.
