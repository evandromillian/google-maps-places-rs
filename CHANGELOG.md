# Change Log

## [0.1.4] - 2021-08-17
### Added
- Added `response::PlaceResult` to lib exports

### Changed
- Changed `Response::OK` to `Response::Ok`

## [0.1.3] - 2021-08-17
### Added
- Added `GoogleMapPlaceError` to lib exports
- Added `Places` to lib exports
- Added `Response` to lib exports

### Changed
- Changed `Places` lifetime

## [0.1.2] - 2021-08-16
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

## [0.1.1] - 2021-08-15
### Changed
- Changed network dependency from `reqwest` to `ureq`

### Removed
- Removed `tokio` dependency

### Added
- Added `async-graphql` dependency
- Added `async-graphql` `SimpleObject` derive macros to response models
