use ratatui::{prelude::*, widgets::*};

use app::run;

fn main() {
    match run() {
        Ok(_) => (),
        _ => panic!("Unresolved issue"),
    };
}