/// # Arguments
/// * `preferred` - A string of the preferred file type (epub, pdf, etc.)
/// # Returns
/// * A vector of file types in order of preference
/// # Example
/// ```
/// let preferred: String = "epub".to_string();
/// let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
/// ```
/// # Options for preferred file type (in default order)
/// * epub
/// * pdf
/// * cbz
/// * zip
/// * rar
/// * cbr
/// * tar.gz
/// * 7zip
/// * 7z
/// * cb7
/// * cbt
/// * png
/// * jpeg
/// * jpg
/// * webp
/// * gif
/// * avif
pub fn get_preferred_vector(preferred: String) -> Vec<&'static str> {
    let mut order: Vec<&str> = vec!["epub", "pdf", "cbz", "zip", "rar", "cbr", "tar.gz", "7zip", "7z", "cb7", "cbt", "png", "jpeg", "jpg", "webp", "gif", "avif"];
    let preferred_index: usize = order.iter().position(|&r| r == preferred).unwrap_or(0);
    if preferred_index > 0 {
        let preferred_item = order.remove(preferred_index);
        order.insert(0, preferred_item);
    }
    order
}

// go through each file and group them by file name (not considering the extension)
/// # Arguments
/// * `files` - A vector of file paths
/// # Returns
/// * A hashmap of grouped files
/// # Example
/// ```
/// let files: Vec<String> = vec!["file1.epub", "file2.epub", "file2.cbz", "file2.pdf"];
/// let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
/// ```
pub fn group_files(files: Vec<String>) -> std::collections::HashMap<String, Vec<String>> {
    // go through each file and group them by file name (not considering the extension)
    let mut grouped_files: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for file in files.iter() {
        let pos: usize = file.find('.').unwrap_or(0);
        let file_name: String = file[..pos].to_string();
        let _file_extension: String = file[pos+1..].to_string();
        let file_group: String = file_name.to_string();
        let file_group: String = if grouped_files.contains_key(&file_group) {
            file_group
        } else {
            grouped_files.insert(file_group.to_string(), vec![]);
            file_group
        };
        grouped_files
            .get_mut(&file_group)
            .unwrap()
            .push(file.to_string());
    }
    grouped_files
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_preferred_vector_epub() {
        let preferred: String = "epub".to_string();
        let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
        assert_eq!(preferred_type_order, vec!["epub", "pdf", "cbz", "zip", "rar", "cbr", "tar.gz", "7zip", "7z", "cb7", "cbt", "png", "jpeg", "jpg", "webp", "gif", "avif"]);
    }

    #[test]
    fn test_get_preferred_vector_pdf() {
        let preferred: String = "pdf".to_string();
        let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
        assert_eq!(preferred_type_order, vec!["pdf", "epub", "cbz", "zip", "rar", "cbr", "tar.gz", "7zip", "7z", "cb7", "cbt", "png", "jpeg", "jpg", "webp", "gif", "avif"]);
    }

    #[test]
    fn test_get_preferred_vector_targz() {
        let preferred: String = "tar.gz".to_string();
        let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
        assert_eq!(preferred_type_order, vec!["tar.gz", "epub", "pdf", "cbz", "zip", "rar", "cbr", "7zip", "7z", "cb7", "cbt", "png", "jpeg", "jpg", "webp", "gif", "avif"]);
    }

    #[test]
    fn test_group_files() {
        let files: Vec<String> = vec!["file1.epub".to_string(), "file2.epub".to_string(), "file2.pdf".to_string()];
        let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
        assert_eq!(grouped_files.len(), 2);
        assert_eq!(grouped_files.get("file1").unwrap(), &vec!["file1.epub"]);
        assert_eq!(
            grouped_files.get("file2").unwrap(),
            &vec!["file2.epub", "file2.pdf"]
        );
    }

    #[test]
    fn test_group_files_complex() {
        let files: Vec<String> = vec![
            "file2.cbz".to_string(),
            "file2.pdf".to_string(),
            "file3.jpg".to_string(),
            "file3.tar.gz".to_string(),
        ];
        let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
        assert_eq!(grouped_files.len(), 2);
        assert_eq!(
            grouped_files.get("file2").unwrap(),
            &vec!["file2.cbz", "file2.pdf"]
        );
        assert_eq!(
            grouped_files.get("file3").unwrap(),
            &vec!["file3.jpg", "file3.tar.gz"]
        );
    }

    #[test]
    fn test_group_files_empty() {
        let files: Vec<String> = vec![];
        let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
        assert_eq!(grouped_files.len(), 0);
    }

    #[test]
    fn test_group_files_single() {
        let files: Vec<String> = vec!["file1.epub".to_string()];
        let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
        assert_eq!(grouped_files.len(), 1);
        assert_eq!(grouped_files.get("file1").unwrap(), &vec!["file1.epub"]);
    }
}