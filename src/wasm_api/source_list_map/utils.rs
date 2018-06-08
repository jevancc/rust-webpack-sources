use serde_json::value::Value;
use source_list_map::*;

pub fn string_with_srcmap_to_json(obj: &StringWithSrcMap) -> Value {
    let map = &obj.map;
    let mut map_json = json!({
        "version": map.version,
        "file": map.file,
        "sources": if map.sources.is_empty() {
            json!([null])
        } else {
            json!(map.sources)
        },
        "mappings": map.mappings
    });

    if !map.sources_content.is_empty() {
        if let Value::Object(ref mut m) = map_json {
            m.insert(String::from("sourcesContent"), json!(map.sources_content));
        }
    }

    json!({
        "source": obj.source,
        "map": map_json,
    })
}
