use std::{env::args, os::unix::ffi::OsStrExt};

use ignore::{DirEntry, WalkBuilder, WalkState};
use pycabe::{module_complexities, ItemComplexity};

#[inline]
fn is_python_file(entry: &DirEntry) -> bool {
    entry.file_type().unwrap().is_dir() || entry.file_name().as_bytes().ends_with(b".py")
}

fn main() {
    WalkBuilder::new(args().nth(1).unwrap_or(".".to_string()))
        .follow_links(true)
        .filter_entry(is_python_file)
        .build_parallel()
        .run(|| {
            Box::new(|entry| {
                if let Ok(entry) = entry {
                    if entry.file_type().unwrap().is_dir() {
                        return WalkState::Continue;
                    }

                    let source = std::fs::read_to_string(entry.path()).unwrap();
                    for ItemComplexity {
                        scope,
                        name,
                        complexity,
                    } in module_complexities(&source)
                    {
                        println!(
                            "{}:{}:{} {}",
                            entry.path().display(),
                            scope.join(":"),
                            name,
                            complexity
                        );
                    }
                }

                WalkState::Continue
            })
        });
}
