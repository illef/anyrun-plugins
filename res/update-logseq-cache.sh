#!/bin/bash

mkdir -p ~/.local/state/anyrun

npx @logseq/cli query $LOGSEQ_GRAPH \
    '[:find (pull ?b [:block/tags :block/uuid :block/title :block/updated-at]) :where [?tag :block/name "page"] [?b :block/tags ?tag]]' \
    | jet --to json > /tmp/logseq.json

mv /tmp/logseq.json ~/.local/state/anyrun/logseq.json

npx @logseq/cli query $LOGSEQ_GRAPH \
    '[:find (pull ?b [:db/id :block/title :logseq.property/icon]) :where [?tag :db/ident :logseq.class/Tag] [?b :block/tags ?tag]]' \
    | jet --to json > /tmp/logseq-tags.json

mv /tmp/logseq-tags.json ~/.local/state/anyrun/logseq-tags.json
