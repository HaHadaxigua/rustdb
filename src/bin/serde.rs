use serde::{Serialize, Deserialize};

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 1, y: 2 };

        let serialized_json = serde_json::to_string(&point).unwrap();
        println!("serialized_json = {}", serialized_json);

        // Convert the JSON string back to a Point.
        let deserialized_json: Point = serde_json::from_str(&serialized_json).unwrap();
        println!("deserialized_json = {:?}", deserialized_json);


        let serialized_yaml = serde_yaml::to_string(&point).unwrap();
        println!("serialized_yaml = {}", serialized_yaml);

        // Convert the YAML string back to a Point.
        let deserialized_yaml: Point = serde_yaml::from_str(&serialized_yaml).unwrap();
        println!("deserialized_yaml = {:?}", deserialized_yaml)
    }
}