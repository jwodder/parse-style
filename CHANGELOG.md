v0.3.0 (2026-04-03)
-------------------
- Increased MSRV to 1.87
- Added the following methods to `AttributeSet`:
    - `len()`
    - `insert()`
    - `remove()`
    - `clear()`
    - `is_disjoint()`
    - `is_subset()`
    - `is_superset()`
- Gave `AttributeSet` a `From<[Attribute; N]>` impl

v0.2.0 (2025-12-26)
-------------------
- The `ratatui` feature now depends on `ratatui-core` v0.1.0 instead of
  `ratatui` v0.29.0
- Increased MSRV to 1.86

v0.1.0 (2025-06-25)
-------------------
Initial release
