/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

var Source = require("./Source");
var SourceNode = require("source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var fromStringWithSourceMap = require("./wasm-source-list-map")
    .fromStringWithSourceMap;
var SourceMapConsumer = require("source-map").SourceMapConsumer;
var wasm = require("./build/webpack_sources");

class ReplaceSource extends wasm._ReplaceSource {
    constructor(source, name) {
        super(0);
        this._source = source;
        this._name = name;
        this._source_cache = null;
        this._replacements = null;
        this.ptr = ReplaceSource._new().ptr;
    }

    replace(start, end, newValue) {
        if (typeof newValue !== "string")
            throw new Error(
                "insertion must be a string, but is a " + typeof newValue
            );

        this._replacements = null;
        this._replace_number_number_string_number_number(
            Math.floor(start),
            Math.floor(end),
            newValue,
            Math.floor((start % 1) * 16),
            Math.floor((end % 1) * 16)
        );
    }

    insert(pos, newValue) {
        if (typeof newValue !== "string")
            throw new Error(
                "insertion must be a string, but is a " +
                    typeof newValue +
                    ": " +
                    newValue
            );
        this._replacements = null;
        this._insert_number_string_number(
            Math.floor(pos),
            newValue,
            Math.floor((pos % 1) * 16)
        );
    }

    size() {
        return this.source().length;
    }

    source(options) {
        var source = this._source.source();
        if (this._source_cache === null) {
            this._source_cache = this._source_string(source);
        }
        return this._source_cache;
    }

    original() {
        return this._source;
    }

    replacements() {
        if (this._replacements == null) {
            this._replacements = JSON.parse(this._replacements_to_string());
        }
        return this._replacements;
    }

    node(options) {
        var replacements = this.replacements();
        var result = [this._source.node(options)];
        replacements.forEach(function(repl) {
            var remSource = result.pop();
            var splitted1 = this._splitSourceNode(remSource, repl[1] + 1);
            var splitted2;
            if (Array.isArray(splitted1)) {
                splitted2 = this._splitSourceNode(splitted1[0], repl[0]);
                if (Array.isArray(splitted2)) {
                    result.push(
                        splitted1[1],
                        this._replacementToSourceNode(splitted2[1], repl[2]),
                        splitted2[0]
                    );
                } else {
                    result.push(
                        splitted1[1],
                        this._replacementToSourceNode(splitted1[1], repl[2]),
                        splitted1[0]
                    );
                }
            } else {
                splitted2 = this._splitSourceNode(remSource, repl[0]);
                if (Array.isArray(splitted2)) {
                    result.push(
                        this._replacementToSourceNode(splitted2[1], repl[2]),
                        splitted2[0]
                    );
                } else {
                    result.push(repl[2], remSource);
                }
            }
        }, this);
        result = result.reverse();
        return new SourceNode(null, null, null, result);
    }

    listMap(options) {
        var map = this._source.listMap(options);
        var ret_map = new SourceListMap(-1);
        ret_map.ptr = this._list_map_sourcelistmap(map).ptr;
        return ret_map;
    }

    _replacementToSourceNode(oldNode, newString) {
        var map = oldNode.toStringWithSourceMap({
            file: "?"
        }).map;
        var original = new SourceMapConsumer(map.toJSON()).originalPositionFor({
            line: 1,
            column: 0
        });
        if (original) {
            return new SourceNode(
                original.line,
                original.column,
                original.source,
                newString
            );
        } else {
            return newString;
        }
    }

    _splitSourceNode(node, position) {
        if (typeof node === "string") {
            if (node.length <= position) return position - node.length;
            return position <= 0
                ? ["", node]
                : [node.substr(0, position), node.substr(position)];
        } else {
            for (var i = 0; i < node.children.length; i++) {
                position = this._splitSourceNode(node.children[i], position);
                if (Array.isArray(position)) {
                    var leftNode = new SourceNode(
                        node.line,
                        node.column,
                        node.source,
                        node.children.slice(0, i).concat([position[0]]),
                        node.name
                    );
                    var rightNode = new SourceNode(
                        node.line,
                        node.column,
                        node.source,
                        [position[1]].concat(node.children.slice(i + 1)),
                        node.name
                    );
                    leftNode.sourceContents = node.sourceContents;
                    return [leftNode, rightNode];
                }
            }
            return position;
        }
    }
}

require("./SourceAndMapMixin")(ReplaceSource.prototype);

module.exports = ReplaceSource;
