use replace_source::*;
use wasm_api::_SourceListMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _ReplaceSource {
    val: ReplaceSource,
}

#[wasm_bindgen]
impl _ReplaceSource {
    // store `source:String` and `name: String` in JS
    pub fn _new() -> _ReplaceSource {
        _ReplaceSource {
            val: ReplaceSource::new(),
        }
    }

    pub fn _replace_number_number_string_number_number(
        &mut self,
        start: i32,
        end: i32,
        new_value: String,
        ord_s: i32,
        ord_e: i32,
    ) {
        self.val.replace(start, end, new_value, ord_s, ord_e);
    }

    pub fn _insert_number_string_number(&mut self, pos: i32, new_value: String, ord: i32) {
        self.val.insert(pos, new_value, ord);
    }

    pub fn _source_string(&mut self, code: String) -> String {
        self.val.replace_string(&code)
    }

    pub fn _list_map_sourcelistmap(&mut self, map: _SourceListMap) -> _SourceListMap {
        let map = self.val.list_map(map.get_raw());
        _SourceListMap::new(map)
    }

    pub fn _replacements_to_string(&mut self) -> String {
        self.val.replacements_to_string()
    }
}
