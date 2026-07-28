# Fiscally decisive track Level-2 dependency-rerun schema

This record reruns HLT, OAS, PAY, NET, and DEF against the shared accounting and
release gates after Level-1 envelope selection. Each row must state which
dependencies passed, which remain blocked, and whether any numeric output is
admitted. A reviewed reblock counts as bounded advancement but not output-ready
completion.
