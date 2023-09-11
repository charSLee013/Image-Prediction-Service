
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    pub version: u32,
    pub input_name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    models: Vec<Model>,
}

pub fn read_config_from_path(
    file_path: &str,
) -> Result<HashMap<String, Model>, Box<dyn std::error::Error>> {
    // 读取文件内容
    let file_contents = std::fs::read_to_string(file_path)?;

    // 解析 YAML 文件
    let config: Config = serde_yaml::from_str(&file_contents)?;

    // 构建 HashMap
    let mut model_map: HashMap<String, Model> = HashMap::new();
    for model in config.models {
        if model_map.contains_key(&model.name) {
            return Err(format!("Duplicate model name: {}", model.name).into());
        }
        model_map.insert(model.name.clone(), model);
    }

    Ok(model_map)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    // 测试文件不存在的情况
    #[test]
    fn test_nonexistent_file() {
        let result = read_config_from_path("nonexistent.yaml");
        assert!(result.is_err());
    }

    // 测试文件不是YAML格式的情况
    #[test]
    fn test_invalid_yaml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "This is not valid YAML.").unwrap();

        let result = read_config_from_path(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    // 测试文件中模型名称有重复的情况
    #[test]
    fn test_duplicate_model_names() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("duplicate.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "models:\n  - name: model1\n    version: 1\n    input_name: input1\n  - name: model1\n    version: 2\n    input_name: input2"
        )
        .unwrap();

        let result = read_config_from_path(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    // 测试正常情况
    #[test]
    fn test_valid_yaml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("valid.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "models:\n  - name: model1\n    version: 1\n    input_name: input1\n  - name: model2\n    version: 2\n    input_name: input2"
        )
        .unwrap();

        let result = read_config_from_path(file_path.to_str().unwrap());
        assert!(result.is_ok());
        let model_map = result.unwrap();
        assert_eq!(
            model_map.get("model1"),
            Some(&Model {
                version: 1,
                name: "model1".to_string(),
                input_name: "input1".to_string()
            })
        );
        assert_eq!(
            model_map.get("model2"),
            Some(&Model {
                version: 2,
                name: "model2".to_string(),
                input_name: "input2".to_string()
            })
        );
    }
}
