use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn compare_directories(dir1: &Path, dir2: &Path) {
    // Helper function for recursive comparison
    fn compare_recursive(path1: &Path, path2: &Path, indent: &str) {
        let entries1: Vec<_> = fs::read_dir(path1)
            .expect("Could not read directory")
            .filter_map(Result::ok)
            .collect();
        let entries2: Vec<_> = fs::read_dir(path2)
            .expect("Could not read directory")
            .filter_map(Result::ok)
            .collect();

        // Sort the entries by file name to make comparison easier
        let mut entries1 = entries1.iter().map(|e| e.file_name()).collect::<Vec<_>>();
        let mut entries2 = entries2.iter().map(|e| e.file_name()).collect::<Vec<_>>();
        entries1.sort();
        entries2.sort();

        let mut i = 0;
        let mut j = 0;

        while i < entries1.len() || j < entries2.len() {
            let entry1 = entries1.get(i);
            let entry2 = entries2.get(j);

            match (entry1, entry2) {
                (Some(name1), Some(name2)) => {
                    match name1.cmp(name2) {
                        std::cmp::Ordering::Equal => {
                            // Both directories contain this entry
                            let path1_item = path1.join(name1);
                            let path2_item = path2.join(name2);

                            // Check if it is a file or a directory
                            let is_dir1 = path1_item.is_dir();
                            let is_dir2 = path2_item.is_dir();

                            if is_dir1 && is_dir2 {
                                // Recursively compare subdirectories
                                println!("{}{}", indent, name1.to_string_lossy().cyan());
                                compare_recursive(
                                    &path1_item,
                                    &path2_item,
                                    &(indent.to_string() + "  "),
                                );
                            } else if !is_dir1 && !is_dir2 {
                                // Compare files by size
                                let size1 = fs::metadata(&path1_item).unwrap().len();
                                let size2 = fs::metadata(&path2_item).unwrap().len();
                                if size1 != size2 {
                                    println!(
                                        "{}~ {} ({} bytes in {}, {} bytes in {})",
                                        indent,
                                        name1.to_string_lossy().yellow(),
                                        size1,
                                        path1.display(),
                                        size2,
                                        path2.display()
                                    );
                                } else {
                                    // Files are identical
                                    println!("{}= {}", indent, name1.to_string_lossy().green());
                                }
                            } else {
                                // One is a file, the other is a directory
                                println!(
                                    "{}{} (one is a file, the other is a directory)",
                                    indent,
                                    name1.to_string_lossy().red()
                                );
                            }

                            i += 1;
                            j += 1;
                        }
                        std::cmp::Ordering::Less => {
                            // Only in dir1
                            println!("{}- {}", indent, name1.to_string_lossy().red());
                            i += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            // Only in dir2
                            println!("{}+ {}", indent, name2.to_string_lossy().green());
                            j += 1;
                        }
                    }
                }
                (Some(name1), None) => {
                    // Only in dir1
                    println!("{}- {}", indent, name1.to_string_lossy().red());
                    i += 1;
                }
                (None, Some(name2)) => {
                    // Only in dir2
                    println!("{}+ {}", indent, name2.to_string_lossy().green());
                    j += 1;
                }
                (None, None) => break,
            }
        }
    }

    // Start the comparison
    println!(
        "Comparing directories: {} vs {}",
        dir1.display(),
        dir2.display()
    );
    compare_recursive(dir1, dir2, "");
}
