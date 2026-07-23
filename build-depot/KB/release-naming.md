# Release Naming

Named releases of the Reflective release train use city names, assigned in
strict alphabetical order — one city per letter, A through Z. A release name
refers to a coordinated shipment of the whole train (see `release-train.yaml`
for train membership and publish order); individual crate versions stay
semver and are recorded per-crate as usual.

**Current release: Acapulco** (in progress as of 2026-07-02).

## The sequence

| Letter | City          | Status      |
|--------|---------------|-------------|
| A      | Acapulco      | In progress |
| B      | Bruges        | Planned     |
| C      | Casablanca    | Planned     |
| D      | Dubrovnik     | Planned     |
| E      | Edinburgh     | Planned     |
| F      | Fez           | Planned     |
| G      | Granada       | Planned     |
| H      | Havana        | Planned     |
| I      | Istanbul      | Planned     |
| J      | Jaipur        | Planned     |
| K      | Kyoto         | Planned     |
| L      | Luang Prabang | Planned     |
| M      | Marrakesh     | Planned     |
| N      | Nairobi       | Planned     |
| O      | Oaxaca        | Planned     |
| P      | Petra         | Planned     |
| Q      | Queenstown    | Planned     |
| R      | Reykjavík     | Planned     |
| S      | Samarkand     | Planned     |
| T      | Tangier       | Planned     |
| U      | Udaipur       | Planned     |
| V      | Valparaíso    | Planned     |
| W      | Wellington    | Planned     |
| X      | Xi'an         | Planned     |
| Y      | Yogyakarta    | Planned     |
| Z      | Zanzibar City | Planned     |

## Conventions

- Names advance strictly in alphabetical order; a name is never reused or
  skipped. What comes after Zanzibar City is deliberately undecided — pick a
  new A–Z theme when the time comes.
- When a release ships, flip its Status to `Shipped (YYYY-MM-DD)` here.
- In branch names and other machine contexts, use the lowercase,
  ASCII-folded, hyphenated form: `acapulco`, `luang-prabang`, `reykjavik`,
  `valparaiso`, `xian`, `zanzibar-city`.

## Cross-references

- [[release-acapulco]] — definition and scope of the current release
- `KB/08-roadmap/release-train.yaml` — train membership and publish order
- [[release-history]] — yank-and-replace trail for published crates
