# Known Limitations

## Heuristic Parsing

`parse_str` is not a full Dust parser. It scans lines and applies keyword heuristics. Complex syntax may not be recognized precisely.

## Kind and Name Extraction

`kind` and `name` are derived from first and second whitespace tokens in a signature line. This can mislabel complex signatures (for example `unsafe` prefixed declarations).

## Extension Validation

CLI help references `.dust` and `.dpaper`, but no extension checks are enforced.

## Extra CLI Positionals

Only first positional (`source`) and second positional (`output`) are used. Additional positionals are ignored.

## Resource Badge Heuristic

Resource annotations in Markdown are based on `item.name` string patterns, which may not align with all declaration forms.

## No Semantic Linking

Generated docs include raw signatures and comments, but do not resolve cross-references, symbol links, or full module hierarchy.
