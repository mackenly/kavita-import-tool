/// # Arguments
/// * `preferred` - A string of the preferred file type (epub, mobi, pdf)
/// # Returns
/// * A vector of file types in order of preference
/// # Example
/// ```
/// let preferred: String = "epub".to_string();
/// let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
/// ```
pub fn get_preferred_vector(preferred: String) -> Vec<&'static str> {
    let preferred_type_order: Vec<&str> = match preferred.as_str() {
        "epub" => vec!["epub", "mobi", "pdf"],
        "mobi" => vec!["mobi", "epub", "pdf"],
        "pdf" => vec!["pdf", "epub", "mobi"],
        _ => vec!["epub", "mobi", "pdf"],
    };
    preferred_type_order
}

// go through each file and group them by file name (not considering the extension)
/// # Arguments
/// * `files` - A vector of file paths
/// # Returns
/// * A hashmap of grouped files
/// # Example
/// ```
/// let files: Vec<String> = vec!["file1.epub", "file2.epub", "file2.mobi", "file2.pdf"];
/// let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
/// ```
pub fn group_files(files: Vec<String>) -> std::collections::HashMap<String, Vec<String>> {
    // go through each file and group them by file name (not considering the extension)
    let mut grouped_files: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for file in files.iter() {
        let file_name: String = file.split('.').collect::<Vec<&str>>()[0].to_string();
        let _file_extension: String = file.split('.').collect::<Vec<&str>>()[1].to_string();
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
        assert_eq!(preferred_type_order, vec!["epub", "mobi", "pdf"]);
    }

    #[test]
    fn test_get_preferred_vector_mobi() {
        let preferred: String = "mobi".to_string();
        let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
        assert_eq!(preferred_type_order, vec!["mobi", "epub", "pdf"]);
    }

    #[test]
    fn test_get_preferred_vector_pdf() {
        let preferred: String = "pdf".to_string();
        let preferred_type_order: Vec<&str> = get_preferred_vector(preferred);
        assert_eq!(preferred_type_order, vec!["pdf", "epub", "mobi"]);
    }

    #[test]
    fn test_group_files() {
        let files: Vec<String> = vec!["file1.epub".to_string(), "file2.epub".to_string(), "file2.mobi".to_string(), "file2.pdf".to_string()];
        let grouped_files: std::collections::HashMap<String, Vec<String>> = group_files(files);
        assert_eq!(grouped_files.len(), 2);
        assert_eq!(grouped_files.get("file1").unwrap(), &vec!["file1.epub"]);
        assert_eq!(
            grouped_files.get("file2").unwrap(),
            &vec!["file2.epub", "file2.mobi", "file2.pdf"]
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