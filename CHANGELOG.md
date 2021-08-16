# Change Log

## [0.1.2] - 2020-08-16
### Added
- Added `PlaceResult` helper methods to obtain address components like:
    - `street_number`
    - `route`
    - `sublocality`
    - `postal_code`
    - `city`
    - `state`
    - `country`
    - `country_code`

## [0.1.1] - 2020-08-15
### Changed
- Changed network dependency from `reqwest` to `ureq`

### Removed
- Removed `tokio` dependency

### Added
- Added `async-graphql` dependency
- Added `async-graphql` `SimpleObject` derive macros to response models
