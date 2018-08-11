extern crate webpack_sources;

#[macro_use]
mod source_map_utils;

use source_map_utils::string_cache::*;
use source_map_utils::traits::*;
use webpack_sources::source_map::*;
use webpack_sources::types::string_slice::*;
use webpack_sources::types::SourceMap;

#[cfg(test)]
mod to_string_with_source_map {
    use super::*;

    #[test]
    fn merging_duplicate_mappings() {
        let mut str_cache = StringCache::new();
        let mut input = SourceNode::new(None, None, None, None);
        input.add_sources(
            &[
                (1, 0, "a.js", "(function", None),
                (1, 0, "a.js", "() {\n", None),
                (-1, -1, "", "  ", None),
                (1, 0, "a.js", "var Test = ", None),
                (1, 0, "b.js", "{};\n", None),
                (2, 0, "b.js", "Test", None),
                (2, 0, "b.js", ".A", Some("A")),
                (2, 20, "b.js", " = { value: ", Some("A")),
                (-1, -1, "", "1234", None),
                (2, 40, "b.js", " };\n", Some("A")),
                (-1, -1, "", "}());\n", None),
                (-1, -1, "", "/* Generated Source */", None),
            ],
            &mut str_cache,
        );
        let input = input.to_string_with_source_map(Some(str_cache.add("foo.js")), None);

        assert_eq!(
            input.source,
            [
                "(function() {",
                "  var Test = {};",
                "Test.A = { value: 1234 };",
                "}());",
                "/* Generated Source */"
            ]
                .join("\n")
        );

        let mut correct_map = SourceMapGenerator::new(Some(str_cache.add("foo.js")), None, false);
        correct_map.add_mappings(
            &[
                (1, 0, Some("a.js"), 1, 0, None),
                (2, 2, Some("a.js"), 1, 0, None),
                (2, 13, Some("b.js"), 1, 0, None),
                (3, 0, Some("b.js"), 2, 0, None),
                (3, 4, Some("b.js"), 2, 0, Some("A")),
                (3, 6, Some("b.js"), 2, 20, Some("A")),
                (3, 18, None, -1, -1, None),
                (3, 22, Some("b.js"), 2, 40, Some("A")),
            ],
            &mut str_cache,
        );

        let input_map = input.map;
        let correct_map = correct_map.to_source_map();
        assert_eq!(input_map, correct_map);
        assert_eq!(
            input_map.mappings,
            "AAAA;EAAA,WCAA;AACA,IAAAA,EAAoBA,Y,IAAoBA"
        );
    }

    #[test]
    fn multi_line_source_nodes() {
        let mut str_cache = StringCache::new();
        let mut input = SourceNode::new(None, None, None, None);
        input.add_sources(
            &[
                (
                    1,
                    0,
                    "a.js",
                    "(function() {\nvar nextLine = 1;\nanotherLine();\n",
                    None,
                ),
                (2, 2, "b.js", "Test.call(this, 123);\n", None),
                (2, 2, "b.js", "this['stuff'] = 'v';\n", None),
                (2, 2, "b.js", "anotherLine();\n", None),
                (-1, -1, "", "/*\nGenerated\nSource\n*/\n", None),
                (3, 4, "c.js", "anotherLine();\n", None),
                (-1, -1, "", "/*\nGenerated\nSource\n*/", None),
            ],
            &mut str_cache,
        );
        let input = input.to_string_with_source_map(Some(str_cache.add("foo.js")), None);

        assert_eq!(
            input.source,
            [
                "(function() {",
                "var nextLine = 1;",
                "anotherLine();",
                "Test.call(this, 123);",
                "this['stuff'] = 'v';",
                "anotherLine();",
                "/*",
                "Generated",
                "Source",
                "*/",
                "anotherLine();",
                "/*",
                "Generated",
                "Source",
                "*/"
            ]
                .join("\n")
        );

        let mut correct_map = SourceMapGenerator::new(Some(str_cache.add("foo.js")), None, false);
        correct_map.add_mappings(
            &[
                (1, 0, Some("a.js"), 1, 0, None),
                (2, 0, Some("a.js"), 1, 0, None),
                (3, 0, Some("a.js"), 1, 0, None),
                (4, 0, Some("b.js"), 2, 2, None),
                (5, 0, Some("b.js"), 2, 2, None),
                (6, 0, Some("b.js"), 2, 2, None),
                (11, 0, Some("c.js"), 3, 4, None),
            ],
            &mut str_cache,
        );

        let input_map = input.map;
        let correct_map = correct_map.to_source_map();
        assert_eq!(input_map, correct_map);
        assert_eq!(input_map.mappings, "AAAA;AAAA;AAAA;ACCE;AAAA;AAAA;;;;;ACCE");
    }

    #[test]
    fn with_empty_string() {
        let node = SourceNode::new(
            Some((1, 0)),
            Some(0),
            None,
            Some(types::Node::NString(StringSlice::from(""))),
        );
        let result = node.to_string_with_source_map(None, None);
        assert_eq!(result.source, "");
    }

    #[test]
    fn with_consecutive_newlines() {
        let mut str_cache = StringCache::new();
        let mut input = SourceNode::new(None, None, None, None);
        input.add_sources(
            &[
                (-1, -1, "", "/***/\n\n", None),
                (1, 0, "a.js", "'use strict';\n", None),
                (2, 0, "a.js", "a();", None),
            ],
            &mut str_cache,
        );
        let input = input.to_string_with_source_map(Some(str_cache.add("foo.js")), None);

        assert_eq!(
            input.source,
            ["/***/", "", "'use strict';", "a();"].join("\n")
        );

        let mut correct_map = SourceMapGenerator::new(Some(str_cache.add("foo.js")), None, false);
        correct_map.add_mappings(
            &[
                (3, 0, Some("a.js"), 1, 0, None),
                (4, 0, Some("a.js"), 2, 0, None),
            ],
            &mut str_cache,
        );

        let input_map = input.map;
        let correct_map = correct_map.to_source_map();
        assert_eq!(input_map, correct_map);
        assert_eq!(input_map.mappings, ";;AAAA;AACA");
    }

    #[test]
    fn with_set_source_content() {
        let mut str_cache = StringCache::new();
        let mut child_node = SourceNode::new(
            Some((1, 1)),
            Some(str_cache.add("a.js")),
            None,
            Some(types::Node::NString(StringSlice::from("a"))),
        );
        child_node.set_source_content(str_cache.add("a.js"), str_cache.add("someContent"));

        let mut node = SourceNode::new(None, None, None, None);
        node.add_sources(
            &[
                (-1, -1, "", "(function () {\n", None),
                (-1, -1, "", "  ", None),
            ],
            &mut str_cache,
        );
        node.add(types::Node::NSourceNode(child_node));
        node.add_sources(
            &[
                (-1, -1, "", "  ", None),
                (1, 1, "b.js", "b", None),
                (-1, -1, "", "}());", None),
            ],
            &mut str_cache,
        );
        node.set_source_content(str_cache.add("b.js"), str_cache.add("otherContent"));

        let map = node
            .to_string_with_source_map(Some(str_cache.add("foo.js")), None)
            .map;

        assert_eq!(map.sources, [str_cache.add("a.js"), str_cache.add("b.js")]);
        assert_eq!(
            map.sources_content,
            [str_cache.add("someContent"), str_cache.add("otherContent")]
        );
        assert_eq!(map.mappings, ";EAAC,C,ECAA,C");
    }
}

#[cfg(test)]
mod from_source_map {
    use super::*;

    from_source_map_test!(from_source_map, {
        version: 3,
        file: Some("min.js"),
        names: vec!["bar", "baz", "n"],
        sources: vec!["one.js", "two.js"],
        sources_content: vec![],
        source_root: None,
        mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
    });

    from_source_map_test!(with_sources_content, {
        version: 3,
        file: Some("min.js"),
        names: vec!["bar", "baz", "n"],
        sources: vec!["one.js", "two.js"],
        sources_content: vec![
            " ONE.foo = function (bar) {\nreturn baz(bar);\n};",
            " TWO.inc = function (n) {\nreturn n + 1;\n};",
        ],
        source_root: None,
        mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
    });

    from_source_map_test!(map_single_source, {
        version: 3,
        file: Some("min.js"),
        names: vec!["bar", "baz"],
        sources: vec!["one.js"],
        sources_content: vec![],
        source_root: None,
        mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID"
    });

    from_source_map_test!(map_empty_mappings, {
        version: 3,
        file: Some("min.js"),
        names: vec![],
        sources: vec!["one.js", "two.js"],
        sources_content: vec![
        " ONE.foo = 1;",
        " TWO.inc = 2;",
        ],
        source_root: None,
        mappings: ""
    });

    from_source_map_test!(map_multi_sources_mapping_refers_single_source_only, {
        version: 3,
        file: Some("min.js"),
        names: vec!["bar", "baz"],
        sources: vec!["one.js", "withoutMappings.js"],
        sources_content: vec![],
        source_root: None,
        mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID"
    });
}

#[cfg(test)]
mod apply_source_map {
    use super::*;

    #[test]
    fn apply_source_map() {
        let mut str_cache = StringCache::new();
        let mut node = SourceNode::new(None, None, None, None);
        node.add_sources(
            &[
                (2, 0, "fileX", "lineX2\n", None),
                (-1, -1, "", "genA1\n", None),
                (2, 0, "fileY", "lineY2\n", None),
                (-1, -1, "", "genA2\n", None),
                (1, 0, "fileX", "lineX1\n", None),
                (-1, -1, "", "genA3\n", None),
                (1, 0, "fileY", "lineY1\n", None),
            ],
            &mut str_cache,
        );
        let mut map_step1_generator = node
            .to_string_with_source_map_generator(Some(str_cache.add("fileA")), None)
            .generator;
        map_step1_generator.set_source_content(
            str_cache.add("fileX"),
            Some(str_cache.add("lineX1\nlineX2\n")),
        );
        let map_step1 = map_step1_generator.to_source_map();

        let mut node = SourceNode::new(None, None, None, None);
        node.add_sources(
            &[
                (-1, -1, "", "gen1\n", None),
                (1, 0, "fileA", "lineA1\n", None),
                (2, 0, "fileA", "lineA2\n", None),
                (3, 0, "fileA", "lineA3\n", None),
                (4, 0, "fileA", "lineA4\n", None),
                (1, 0, "fileB", "lineB1\n", None),
                (2, 0, "fileB", "lineB2\n", None),
                (-1, -1, "", "gen2\n", None),
            ],
            &mut str_cache,
        );
        let mut map_step2_generator = node
            .to_string_with_source_map_generator(Some(str_cache.add("fileGen")), None)
            .generator;
        map_step2_generator.set_source_content(
            str_cache.add("fileB"),
            Some(str_cache.add("lineB1\nlineB2\n")),
        );
        let map_step2 = map_step2_generator.to_source_map();

        let mut node = SourceNode::new(None, None, None, None);
        node.add_sources(
            &[
                (-1, -1, "", "gen1\n", None),
                (2, 0, "fileX", "lineA1\n", None),
                (2, 0, "fileA", "lineA2\n", None),
                (2, 0, "fileY", "lineA3\n", None),
                (4, 0, "fileA", "lineA4\n", None),
                (1, 0, "fileB", "lineB1\n", None),
                (2, 0, "fileB", "lineB2\n", None),
                (-1, -1, "", "gen2\n", None),
            ],
            &mut str_cache,
        );
        let mut expected_map = node
            .to_string_with_source_map_generator(Some(str_cache.add("fileGen")), None)
            .generator;
        expected_map.set_source_content(
            str_cache.add("fileX"),
            Some(str_cache.add("lineX1\nlineX2\n")),
        );
        expected_map.set_source_content(
            str_cache.add("fileB"),
            Some(str_cache.add("lineB1\nlineB2\n")),
        );
        let expected_map = expected_map.to_source_map();

        map_step2_generator.apply_source_map_generator(&mut map_step1_generator, None);
        let actual_map = map_step2_generator.to_source_map();
        assert_eq!(actual_map, expected_map);

        // test SourceMapGenerator::from_source_map
        let mut map_step1_generator = map_step1.to_generator();
        let mut map_step2_generator = map_step2.to_generator();
        assert_eq!(map_step1_generator.to_source_map(), map_step1);
        assert_eq!(map_step2_generator.to_source_map(), map_step2);
        map_step2_generator.apply_source_map_generator(&mut map_step1_generator, None);
        let actual_map = map_step2_generator.to_source_map();
        assert_eq!(actual_map, expected_map);
    }

    #[test]
    #[should_panic]
    fn panic_when_file_is_missing() {
        let mut str_cache = StringCache::new();
        let mut map = SourceMapGenerator::new(Some(str_cache.add("file")), None, true);
        let mut map2 = SourceMapGenerator::new(None, None, true);

        map.apply_source_map_generator(&mut map2, None);
    }

    #[test]
    fn name_handling() {
        // Imagine some CoffeeScript code being compiled into JavaScript and then minified.
        let assert_name = |coffee_name: Option<&str>,
                           js_name: Option<&str>,
                           expected_name: Option<&str>| {
            let mut str_cache = StringCache::new();
            let mut minified_map =
                SourceMapGenerator::new(Some(str_cache.add("test.js.min")), None, true);
            minified_map.add_mappings(&[(1, 4, Some("test.js"), 1, 4, js_name)], &mut str_cache);

            let mut coffee_map =
                SourceMapGenerator::new(Some(str_cache.add("test.js")), None, true);
            coffee_map.add_mappings(
                &[(1, 4, Some("test.coffee"), 1, 0, coffee_name)],
                &mut str_cache,
            );

            minified_map.apply_source_map_generator(&mut coffee_map, None);
            let expected_name = expected_name.map(|s| str_cache.add(s));
            for mapping in minified_map.mappings.list {
                assert_eq!(mapping.name, expected_name);
            }
        };

        // `foo = 1` -> `var foo = 1;` -> `var a=1`
        // CoffeeScript doesn’t rename variables, so there’s no need for it to
        // provide names in its source maps. Minifiers do rename variables and
        // therefore do provide names in their source maps. So that name should be
        // retained if the original map lacks names.
        assert_name(None, Some("foo"), Some("foo"));

        // `foo = 1` -> `var coffee$foo = 1;` -> `var a=1`
        // Imagine that CoffeeScript prefixed all variables with `coffee$`. Even
        // though the minifier then also provides a name, the original name is
        // what corresponds to the source.
        assert_name(Some("foo"), Some("coffee$foo"), Some("foo"));

        // `foo = 1` -> `var coffee$foo = 1;` -> `var coffee$foo=1`
        // Minifiers can turn off variable mangling. Then there’s no need to
        // provide names in the source map, but the names from the original map are
        // still needed.
        assert_name(Some("foo"), None, Some("foo"));

        // `foo = 1` -> `var foo = 1;` -> `var foo=1`
        // No renaming at all.
        assert_name(None, None, None);
    }

    #[test]
    fn with_unexact_match() {
        let mut str_cache = StringCache::new();
        let mut map1 = SourceMapGenerator::new(Some(str_cache.add("bundled-source")), None, true);
        map1.add_mappings(
            &[
                (1, 4, Some("transformed-source"), 1, 4, None),
                (2, 4, Some("transformed-source"), 2, 4, None),
            ],
            &mut str_cache,
        );

        let mut map2 =
            SourceMapGenerator::new(Some(str_cache.add("transformed-source")), None, true);
        map2.add_mappings(
            &[(2, 0, Some("original-source"), 1, 0, None)],
            &mut str_cache,
        );

        let mut expected_map =
            SourceMapGenerator::new(Some(str_cache.add("bundled-source")), None, true);
        expected_map.add_mappings(
            &[
                (1, 4, Some("transformed-source"), 1, 4, None),
                (2, 4, Some("original-source"), 1, 0, None),
            ],
            &mut str_cache,
        );

        map1.apply_source_map_generator(&mut map2, None);
        assert_eq!(map1.to_source_map(), expected_map.to_source_map());
    }

    #[test]
    fn with_empty_mappings() {
        let mut str_cache = StringCache::new();
        let test_map = source_map!({
            version: 3,
            file: Some("min.js"),
            names: vec![],
            sources: vec![
                "one.js",
                "two.js",
            ],
            sources_content: vec![
                " ONE.foo = 1;",
                " TWO.inc = 2;",
            ],
            source_root: None,
            mappings: "",
        }, str_cache);

        let mut generator = test_map.to_generator();
        generator.apply_source_map_generator(&mut test_map.to_generator(), None);
        assert_eq!(generator.to_source_map(), test_map);
    }

    #[test]
    fn mappings_with_same_generated_but_different_original_positions() {
        let mut str_cache = StringCache::new();
        let mut generator = SourceMapGenerator::new(None, None, true);
        generator.add_mappings(
            &[
                (1, 10, Some("a.js"), 1, 10, None),
                (1, 10, Some("b.js"), 2, 20, None),
            ],
            &mut str_cache,
        );
        assert_eq!(generator.mappings.list.len(), 2);
    }
}
