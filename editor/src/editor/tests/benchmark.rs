use super::*;
use std::time::Instant;

#[test]
#[ignore]
fn benchmark() {
	// Get the current time
	let time = Instant::now();

	script();

	// Get the elapsed time (could also run as solo test and look at completion time)
	let elapsed = time.elapsed();
	println!("Time elapsed: {:?}", elapsed);
}

// The actions being benchmarked
fn script() {
	// Create an EditorSpace
	let mut editor = construct_editor(BENCHMARK_FILE);

	// Add 50 newlines
	for i in 0..50 {
		// Ensure the blocks are correct
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		// Add newlines
		editing_keys::enter_key(&mut editor);
	}

	// Move down 5000 lines
	for i in 0..5000 {
		// Ensure the blocks are correct
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		// Move down
		navigation_keys::down_arrow(&mut editor);
	}

	// Enter 500 new lines and '~'
	for i in 0..500 {
		// Ensure the blocks are correct
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		// Enter
		editing_keys::enter_key(&mut editor);
		// Add char
		editing_keys::char_key(&mut editor, '~');
	}

	// Move to the end of the BENCHMARK_FILE
	for i in 0..30000 {
		// Ensure the blocks are correct
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		// Move down
		navigation_keys::down_arrow(&mut editor);
	}

	// Delete the entire BENCHMARK_FILE
	navigation_keys::end_key(&mut editor, true);
	for i in 0..30000 {
		// Ensure the blocks are correct
		if i % 50 == 0 {
			editor.get_paragraph();
		}
		highlight_keys::highlight_up(&mut editor);
	}
	highlight_keys::highlight_home(&mut editor);
	editor.delete_selection();
}
