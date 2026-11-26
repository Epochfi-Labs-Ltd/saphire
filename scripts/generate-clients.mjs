// Generate the Saphire Rust client from the Anchor IDL via Codama.
// Run `anchor build` first so target/idl/saphire.json is current.
//   pnpm generate   (anchor build + this script)
//   pnpm codama     (this script only)
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor as renderRustVisitor } from '@codama/renderers-rust';
import { createFromRoot } from 'codama';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const idl = JSON.parse(readFileSync(path.join(root, 'target/idl/saphire.json'), 'utf8'));

const codama = createFromRoot(rootNodeFromAnchor(idl));

await codama.accept(
  renderRustVisitor(path.join(root, 'clients/rust'), {
    formatCode: true,
    toolchain: '+1.89.0',
    anchorTraits: false,
    deleteFolderBeforeRendering: true,
  }),
);
