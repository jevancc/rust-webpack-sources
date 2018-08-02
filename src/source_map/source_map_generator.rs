use super::types::{Mapping, MappingList};
use linked_hash_map::LinkedHashMap;
use source_map_mappings::{parse_mappings, Mappings as _Mappings};
use std::collections::{HashMap, HashSet};
use std::str;
use types::string_slice::*;
use types::SourceMap;
use vlq;

#[derive(Debug)]
pub struct SourceMapGenerator {
    file: Option<i32>,
    source_root: Option<StringSlice>,
    skip_validation: bool,
    sources: LinkedHashMap<i32, usize>,
    names: LinkedHashMap<i32, usize>,
    pub mappings: MappingList,
    pub sources_contents: HashMap<i32, i32>,
}

impl SourceMapGenerator {
    pub fn new(
        file: Option<i32>,
        source_root: Option<StringSlice>,
        skip_validation: bool,
    ) -> SourceMapGenerator {
        SourceMapGenerator {
            file,
            source_root,
            skip_validation,
            sources: LinkedHashMap::new(),
            names: LinkedHashMap::new(),
            mappings: MappingList::new(),
            sources_contents: HashMap::new(),
        }
    }

    #[inline]
    fn add_source(&mut self, source: i32) {
        let len = self.sources.len();
        self.sources.entry(source).or_insert(len);
    }

    #[inline]
    fn add_name(&mut self, name: i32) {
        let len = self.names.len();
        self.names.entry(name).or_insert(len);
    }

    pub fn add_mapping(&mut self, map: Mapping) {
        if !self.skip_validation {
            SourceMapGenerator::validate_mapping(&map).unwrap();
        }

        map.source.map(|source| self.add_source(source));
        map.name.map(|name| self.add_name(name));

        self.mappings.add(map);
    }

    pub fn set_source_content(&mut self, source: i32, source_content: Option<i32>) {
        if let Some(content) = source_content {
            self.sources_contents.entry(source).or_insert(content);
        } else {
            self.sources_contents.remove(&source);
        }
    }

    pub fn to_source_map(&mut self) -> SourceMap {
        let version = 3;
        let sources: Vec<i32> = self.sources.keys().map(|sidx| *sidx).collect();
        let names: Vec<i32> = self.names.keys().map(|sidx| *sidx).collect();
        let mappings = self.serialize_mappings();
        let file = self.file;
        let source_root = self.source_root.clone().map(|sp| sp.into_string());
        let mut sources_content: Vec<i32> = Vec::new();

        for src in self.sources.keys() {
            if let Some(content) = self.sources_contents.get(src) {
                sources_content.push(*content);
            }
        }
        return SourceMap {
            version,
            sources,
            names,
            mappings,
            file,
            source_root,
            sources_content,
        };
    }

    fn validate_mapping(map: &Mapping) -> Result<(), &'static str> {
        if let Some((original_line, _)) = map.original.clone() {
            if map.source.is_some() && original_line > 0 && map.generated.0 > 0 {
                Ok(())
            } else {
                Err("Invalid mapping")
            }
        } else {
            if map.source.is_none() && map.name.is_none() && map.generated.0 > 0 {
                Ok(())
            } else {
                Err("Invalid mapping")
            }
        }
    }

    fn serialize_mappings(&mut self) -> String {
        // (line, column)
        let mut previous_generated: (usize, usize) = (1, 0);
        let mut previous_original: (usize, usize) = (0, 0);
        let mut previous_name: usize = 0;
        let mut previous_source: usize = 0;
        let mut result = String::new();
        let mut buf = Vec::<u8>::new();

        self.mappings.sort();
        for (i, mapping) in self.mappings.list.iter().enumerate() {
            if mapping.generated.0 != previous_generated.0 {
                previous_generated.1 = 0;
                for _ in 0..(mapping.generated.0 - previous_generated.0) {
                    buf.push(b';');
                }
                previous_generated.0 = mapping.generated.0;
            } else if i > 0 {
                //     if (
                //         !util.compareByGeneratedPositionsInflated(
                //             mapping,
                //             mappings[i - 1]
                //         )
                //     ) {
                //         continue;
                //     }
                // }
                buf.push(b',');
            }

            vlq::encode(
                mapping.generated.1 as i64 - previous_generated.1 as i64,
                &mut buf,
            ).unwrap();
            previous_generated.1 = mapping.generated.1;
            if let Some(ref source) = mapping.source {
                let source_idx = self.sources.get(source).unwrap();
                vlq::encode(*source_idx as i64 - previous_source as i64, &mut buf).unwrap();
                previous_source = *source_idx;

                let mapping_original = mapping.original.unwrap();
                // lines are stored 0-based in SourceMap spec version 3
                vlq::encode(
                    mapping_original.0 as i64 - 1 - previous_original.0 as i64,
                    &mut buf,
                ).unwrap();
                previous_original.0 = mapping_original.0 - 1;

                vlq::encode(
                    mapping_original.1 as i64 - previous_original.1 as i64,
                    &mut buf,
                ).unwrap();
                previous_original.1 = mapping_original.1;

                if let Some(ref name) = mapping.name {
                    let name_idx = self.names.get(name).unwrap();
                    vlq::encode(*name_idx as i64 - previous_name as i64, &mut buf).unwrap();
                    previous_name = *name_idx;
                }
            }
            result += unsafe { str::from_utf8_unchecked(&buf) };
            buf.clear();
        }
        result
    }

    // originate from `SourceMapConsumer.OriginalPositionFor`
    pub fn original_position_for(&mut self, line: usize, column: usize) -> Mapping {
        self.mappings.sort();

        let empty_mapping = Mapping {
            generated: (0, 0),
            source: None,
            name: None,
            original: None,
        };

        let mapping = match self
            .mappings
            .list
            .binary_search_by(|probe| probe.generated.cmp(&(line, column)))
        {
            Ok(i) => &self.mappings.list[i],
            Err(i) => if i == 0 {
                return empty_mapping;
            } else {
                &self.mappings.list[i - 1]
            },
        };

        if mapping.generated.0 == line {
            mapping.clone()
        } else {
            empty_mapping
        }
    }

    pub fn apply_source_map_generator(
        &mut self,
        generator: &mut SourceMapGenerator,
        source_file: Option<i32>,
    ) {
        let source_file = if source_file.is_none() {
            if generator.file.is_none() {
                panic!("SourceMapGenerator.prototype.applySourceMap requires either an explicit source file,
                        or the source map's \"file\" property. Both were omitted.");
            }
            generator.file
        } else {
            source_file
        };

        // let source_root = self.source_root.clone();
        // process source_root
        self.mappings.set_unsorted();
        let mut new_sources = Vec::<i32>::with_capacity(self.sources.len());
        let mut new_names = Vec::<i32>::with_capacity(self.names.len());
        for mapping in self.mappings.list.iter_mut() {
            if let Some((line, column)) = mapping.original {
                if mapping.source == source_file {
                    let original = generator.original_position_for(line, column);
                    if original.source.is_some() {
                        mapping.source = original.source;
                        // process source_root
                        if original.name.is_some() {
                            mapping.name = original.name;
                        }
                        mapping.original = original.original;
                    }
                }
            }
            mapping.source.map(|source| new_sources.push(source));
            mapping.name.map(|name| new_names.push(name));
        }
        if !self.mappings.list.is_empty() {
            self.sources.clear();
        }
        for source in new_sources {
            self.add_source(source);
        }
        self.names.clear();
        for name in new_names {
            self.add_name(name);
        }

        for (source, content) in &generator.sources_contents {
            self.set_source_content(*source, Some(*content));
        }
    }

    pub fn from_source_map(
        sources: Vec<i32>,
        sources_content: Vec<i32>,
        mappings: StringSlice,
        names: Vec<i32>,
        file: Option<i32>,
        source_root: Option<StringSlice>,
        check_dup: bool,
    ) -> SourceMapGenerator {
        let mut generator = SourceMapGenerator::new(file, source_root, true);

        let mut contents = sources_content.into_iter();
        let sources: Vec<i32> = if check_dup {
            let mut set: HashSet<i32> = HashSet::new();
            sources
                .into_iter()
                .filter(|sidx| {
                    generator.add_source(*sidx);
                    generator.set_source_content(*sidx, contents.next());
                    set.insert(*sidx)
                })
                .collect()
        } else {
            sources
                .into_iter()
                .map(|sidx| {
                    generator.set_source_content(sidx, contents.next());
                    sidx
                })
                .collect()
        };
        let names: Vec<i32> = if check_dup {
            let mut set: HashSet<i32> = HashSet::new();
            names.into_iter().filter(|sidx| set.insert(*sidx)).collect()
        } else {
            names
        };

        let mappings: _Mappings<()> = parse_mappings(mappings.as_bytes()).unwrap();
        let mappings = mappings.by_generated_location();
        for mapping in mappings {
            let generated = (
                mapping.generated_line as usize + 1,
                mapping.generated_column as usize,
            );
            let (original, source, name) = if let Some(original) = mapping.original.clone() {
                let name = original.name.map(|idx| names[idx as usize].clone());
                let source = sources[original.source as usize].clone();
                (
                    Some((
                        original.original_line as usize + 1,
                        original.original_column as usize,
                    )),
                    Some(source),
                    name,
                )
            } else {
                (None, None, None)
            };
            generator.add_mapping(Mapping {
                generated,
                original,
                source,
                name,
            })
        }
        generator
    }
}
