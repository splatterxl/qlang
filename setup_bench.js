const fs = require("fs");

const lines = "import * from \"src\";\n";
fs.writeFileSync("./mil.q", lines.repeat(1_000_000));
