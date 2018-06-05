var cp = require("child_process");

exports.run = function run(cmd, options) {
    console.log("> " + cmd.join(" "));
    cp.execSync(cmd.join(" "), options);
};
