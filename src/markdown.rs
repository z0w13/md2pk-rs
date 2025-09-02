use std::iter::FilterMap;
use walkdir::{DirEntry, FilterEntry, WalkDir};

#[expect(
    clippy::type_complexity,
    reason = "don't think there's a way to simplify the type"
)]
pub(crate) fn walker(
    path: &str,
    recursive: bool,
) -> FilterMap<
    FilterEntry<walkdir::IntoIter, impl FnMut(&DirEntry) -> bool>,
    impl FnMut(Result<DirEntry, walkdir::Error>) -> Option<DirEntry>,
> {
    if recursive {
        WalkDir::new(path)
    } else {
        WalkDir::new(path).max_depth(1)
    }
    .into_iter()
    .filter_entry(|entry| {
        entry.file_type().is_dir() || entry.path().extension().is_some_and(|ext| ext == "md")
    })
    .filter_map(move |entry| match entry {
        Err(err) => {
            println!("ERROR {path}: {err}");
            None
        }
        Ok(path) => {
            println!("OK    {}", path.path().display());
            path.file_type().is_file().then_some(path)
        }
    })
}
