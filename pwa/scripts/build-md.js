// scripts/build-md.js
import fs from 'fs-extra';
import path from 'path';
import globby from 'globby';
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true
});

const srcDir = 'src/content';
const outDir = 'src/generated';

async function build() {
  const files = await globby(`${srcDir}/**/*.md`);

  await fs.ensureDir(outDir);

  for (const file of files) {
    const content = await fs.readFile(file, 'utf-8');
    const html = md.render(content);

    const relative = path.relative(srcDir, file);
    const outputFile = path.join(outDir, relative.replace(/\.md$/, '.json'));

    await fs.ensureDir(path.dirname(outputFile));
    await fs.writeJSON(outputFile, { html });
    console.log(`✔️ Built ${outputFile}`);
  }
}

build();

