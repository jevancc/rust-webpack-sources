#[macro_export]
macro_rules! source_map {
    (
        {
            version:
            $version:expr,file:
            $file:expr,names:
            $names:expr,sources:
            $sources:expr,sources_content:
            $sources_content:expr,source_root:
            $source_root:expr,mappings:
            $mappings:expr $(,)*
        },
        $str_cache:ident
    ) => {
        SourceMap {
            version: $version,
            file: ($file).map(|s| $str_cache.add(s)),
            names: ($names).into_iter().map(|s| $str_cache.add(s)).collect(),
            sources: ($sources).into_iter().map(|s| $str_cache.add(s)).collect(),
            sources_content: ($sources_content)
                .into_iter()
                .map(|s| $str_cache.add(s))
                .collect(),
            source_root: $source_root,
            mappings: ($mappings).to_string(),
        }
    };
}

#[macro_export]
macro_rules! from_source_map_test {
    (
        $test_name:ident, {
            version:
            $version:expr,file:
            $file:expr,names:
            $names:expr,sources:
            $sources:expr,sources_content:
            $sources_content:expr,source_root:
            $source_root:expr,mappings:
            $mappings:expr $(,)*
        }
    ) => {
        #[test]
        fn $test_name() {
            let mut str_cache = StringCache::new();
            let map = SourceMap {
                version: $version,
                file: ($file).map(|s| str_cache.add(s)),
                names: ($names).into_iter().map(|s| str_cache.add(s)).collect(),
                sources: ($sources).into_iter().map(|s| str_cache.add(s)).collect(),
                sources_content: ($sources_content)
                    .into_iter()
                    .map(|s| str_cache.add(s))
                    .collect(),
                source_root: $source_root,
                mappings: ($mappings).to_string(),
            };

            let mut generator = map.to_generator();
            assert_eq!(generator.to_source_map(), map);
        }
    };
}
