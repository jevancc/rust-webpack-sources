const path = require("path");
const assert = require("assert");
const chalk = require("chalk");

exports.Source = require("./Source");

exports.RawSource = require("./RawSource");
exports.OriginalSource = require("./OriginalSource");
exports.SourceMapSource = require("./SourceMapSource");
exports.LineToLineMappedSource = require("./LineToLineMappedSource");

exports.CachedSource = require("./CachedSource");
exports.ConcatSource = require("./ConcatSource");
exports.ReplaceSource = require("./ReplaceSource");
exports.PrefixSource = require("./PrefixSource");

exports.fromStringWithSourceMap = require("./wasm-source-list-map").fromStringWithSourceMap;
exports.SourceListMap = require("./wasm-source-list-map").SourceListMap;
exports.SingleLineNode = require("./wasm-source-list-map").SingleLineNode;
exports.SourceNode = require("./wasm-source-list-map").SourceNode;
exports.CodeNode = require("./wasm-source-list-map").CodeNode;

function clear() {
    require("./StringCache").clear();
    require("./WasmObjectPool").clear();
    require("./RawSource")._clearPtrCache();
    require("./OriginalSource")._clearPtrCache();
}
exports.clear = clear;

const moduleResolvePath = path.resolve(__dirname, "../").split(path.sep);
const moduleResolveName = moduleResolvePath[moduleResolvePath.length - 1];
function register() {
    try {
        assert(require.cache[require.resolve("webpack-sources")] === undefined);
        require.cache[require.resolve("webpack-sources")] =
            require.cache[require.resolve("wasm-webpack-sources")];

        console.log(
            chalk.yellow(`Override:
        ${require.resolve("webpack-sources")} -> ${require.resolve(
                "wasm-webpack-sources"
            )}`)
        );
        console.log(
            chalk.yellow(
                "You are now using experimental package `wasm-webpack-sources`\n"
            )
        );
    } catch (err) {
        console.log(err.stack);
        console.log(chalk.red("Fail to override `webpack-sources`\n"));
    }
}
exports.register = register;

if (moduleResolveName === "wasm-webpack-sources") {
    register();
}
