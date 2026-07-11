use assert_fs::TempDir;
use initium::generators::ConfigGenerator;

#[tokio::test]
async fn test_fail_on_exists_basic_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_basic(true).await;
    assert!(result.is_ok());

    // Second generation with fail_on_exists=true should fail
    let result = generator.generate_basic(true).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        initium::error::InitiumError::FileExists(_)
    ));

    // Third generation with fail_on_exists=false should succeed
    let result = generator.generate_basic(false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_with_force() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation
    let result = generator.generate_basic(true).await;
    assert!(result.is_ok());

    // Second generation with force=true should succeed even with fail_on_exists=true
    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    let result = force_generator.generate_basic(true).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_ruby_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_ruby_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_ruby_with_template("default").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_python_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_python_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_python_with_template("default").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_node_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_node_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_node_with_template("default").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_go_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_go_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_go_with_template("default").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_rust_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_rust_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_rust_with_template("default").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_individual_files() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test individual file generation with fail_on_exists=true
    let result = generator.generate_editor_config(true).await;
    assert!(result.is_ok());

    // Second generation should fail
    let result = generator.generate_editor_config(true).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        initium::error::InitiumError::FileExists(_)
    ));

    // With fail_on_exists=false should succeed
    let result = generator.generate_editor_config(false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test with different templates
    let templates = vec!["default", "google", "airbnb"];

    for template in templates {
        // First generation should succeed
        let result = generator.generate_basic_with_template(true, template).await;
        assert!(result.is_ok(), "Failed for template: {}", template);

        // Clean up for next iteration
        let _ = std::fs::remove_file(temp_dir.path().join(".editorconfig"));
        let _ = std::fs::remove_file(temp_dir.path().join(".prettierrc"));
        let _ = std::fs::remove_file(temp_dir.path().join("justfile"));
    }
}

#[tokio::test]
async fn test_fail_on_exists_edge_cases() {
    let temp_dir = TempDir::new().unwrap();

    // Test with force=true and fail_on_exists=true
    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    let result = force_generator.generate_basic(true).await;
    assert!(result.is_ok());

    // Test with force=false and fail_on_exists=false
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());
    let result = generator.generate_basic(false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_file_specific_behavior() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test that each file type respects fail_on_exists
    let files = vec![
        ".editorconfig",
        ".prettierrc",
        "justfile",
        ".ruby-version",
        ".node-version",
        ".python-version",
        "go.mod",
        "rustfmt.toml",
    ];

    for file in files {
        let file_path = temp_dir.path().join(file);

        // Create the file first
        std::fs::write(&file_path, "test content").unwrap();

        // Try to generate with fail_on_exists=true, should fail
        let result = generator.generate_basic(true).await;
        assert!(result.is_err());

        // Clean up
        let _ = std::fs::remove_file(&file_path);
    }
}

#[tokio::test]
async fn test_fail_on_exists_language_specific_overrides() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Generate basic project first
    let result = generator.generate_basic(true).await;
    assert!(result.is_ok());

    // Now generate Ruby project - should succeed because it sets fail_on_exists=false
    let result = generator.generate_ruby_with_template("default").await;
    assert!(result.is_ok());

    // Verify that Ruby-specific files were created
    assert!(temp_dir.path().join(".ruby-version").exists());
    assert!(temp_dir.path().join(".node-version").exists());
    assert!(temp_dir.path().join(".rubocop.yml").exists());
}

#[tokio::test]
async fn test_fail_on_exists_mixed_scenarios() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Scenario 1: Basic project with fail_on_exists=true
    let result = generator.generate_basic(true).await;
    assert!(result.is_ok());

    // Scenario 2: Try to generate basic again with fail_on_exists=true (should fail)
    let result = generator.generate_basic(true).await;
    assert!(result.is_err());

    // Scenario 3: Generate Python project (should succeed, sets fail_on_exists=false)
    let result = generator.generate_python_with_template("default").await;
    assert!(result.is_ok());

    // Scenario 4: Generate Python again (should succeed)
    let result = generator.generate_python_with_template("default").await;
    assert!(result.is_ok());

    // Scenario 5: Try basic again with force=true (should succeed)
    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    let result = force_generator.generate_basic(true).await;
    assert!(result.is_ok());
}
