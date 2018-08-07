const fs = require("fs");
const path = require("path");
const utils = require("./utils");

const ROOT = path.join(__dirname, "../");
const BUILD_DIR = path.join(ROOT, "wasm-api/build");
const CWD = path.join(ROOT, ".");
const CRATE_NAME = "webpack_sources";

function main() {
    if (!fs.existsSync(BUILD_DIR)) {
        fs.mkdirSync(BUILD_DIR);
    }

    let options = {
        cwd: CWD,
        stdio: "inherit"
    };

    utils.run(
        [
            "cargo",
            "+nightly",
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--release"
        ],
        options
    );
    utils.run(
        [
            "wasm-bindgen",
            "target/wasm32-unknown-unknown/release/" + CRATE_NAME + ".wasm",
            "--out-dir",
            BUILD_DIR,
            "--nodejs",
            "--no-typescript"
        ],
        options
    );
}

main();
