use std::collections::HashMap;

use map_macro::hash_map;
use rhai::{Dynamic, Engine, Map, Scope};

fn main() {
    let input: Dynamic = hash_map! {
        "a" => 1,
        "b" => 2,
        "c" => 3,
    }
    .into();
    let mut scope = Scope::new();
    scope.push("input", input);

    let engine = Engine::new();
    let output = engine
        .eval_with_scope::<Map>(
            &mut scope,
            r#"
        input.a = 10;
        input
    "#,
        )
        .unwrap();
    let output = output
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.as_int().unwrap()))
        .collect::<HashMap<_, _>>();
    assert_eq!(
        output,
        hash_map! {
            "a".to_string() => 10,
            "b".to_string() => 2,
            "c".to_string() => 3,
        }
    )
}
