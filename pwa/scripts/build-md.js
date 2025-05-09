// scripts/build-md.js
import fs from "fs-extra";
import path from "path";
import { globby } from "globby";
import MarkdownIt from "markdown-it";
import attrs from "markdown-it-attrs";

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
}).use(attrs);

const srcDir = "src/content";
const outDir = "src/generated";

async function build() {
  const files = await globby(`${srcDir}/**/*.md`);

  await fs.ensureDir(outDir);

  for (const file of files) {
    const content = await fs.readFile(file, "utf-8");
    const html = groupSections(md.render(content));

    const relative = path.relative(srcDir, file);
    const outputFile = path.join(outDir, relative.replace(/\.md$/, ".json"));

    await fs.ensureDir(path.dirname(outputFile));
    await fs.writeJSON(outputFile, { html });
    console.log(`✔️ Built ${outputFile}`);
  }
}

function groupSections(html) {
  const parts = html.split(/(<h2[^>]*>.*?<\/h2>)/);
  const output = [];

  for (let i = 0; i < parts.length; i++) {
    const chunk = parts[i];

    if (chunk.match(/^<h2/)) {
      const cls = chunk.match(/class="(.*)"/);
      const heading = chunk;
      const content = parts[i + 1] || "";
      output.push(
        `<div class="section ${cls?.[1]}">${heading}<div class="paragraphs">${content}</div></div>`,
      );
      i++; // skip the content chunk since we handled it
    } else if (!chunk.includes("<h2")) {
      output.push(chunk); // push other content, like the ToC
    }
  }

  return output.join("");
}

build();
