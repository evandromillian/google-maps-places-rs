# Google Maps Places for Rust

Simple crate to get Google Maps Place results.

This library can be used to fetch details from Google Maps Places API
using `place_id` from Place Search.

## Getting started

1. Add to `[dependencies]`

    ```toml
    google-maps-places = "0.1"
    ```

    Available `features`:
    
    - `async-graphql`

1. Fetch place results

    ```rust
    use google_maps_places::{Places, Response};

    let places = &Places { api_key: "api-key" };
    let res = match places.get_map_place("ChIJATaCWGU3zDER32m__CAwDyY") {
        Ok(b) => b,
        Err(e) => {
            println!("Error {:?}", e);
            return;
        }
    };
    match res {
        Response::OK { result } => {
            println!("id                : {}", result.place_id);
            println!("name              : {}", result.name);
            println!("formatted_address : {}", result.formatted_address);
            println!("");

            println!("street_number : {}", result.street_number().unwrap_or(""),);
            println!("route         : {}", result.route().unwrap_or(""));
            println!("sublocality   : {}", result.sublocality().unwrap_or(""));
            println!("postal_code   : {}", result.postal_code().unwrap_or(""));
            println!("city          : {}", result.city().unwrap_or(""));
            println!("state         : {}", result.state().unwrap_or(""));
            println!("country       : {}", result.country().unwrap_or(""));
        }
        Response::ZeroResults => {
            println!("Zero results");
        }
        Response::InvalidRequest => {
            println!("Invalid Request");
        }
        Response::OverQueryLimit => {
            println!("Over Query Limit");
        }
        Response::RequestDenied { error_message } => {
            println!("Request Denied: {}", error_message);
        }
        Response::UnknownError => {
            println!("Unknown Error")
        }
    };
    ```

## Todo:
- [X] Documentation
- [X] Helper methods to simplify obtaining location details (ie., country, state, city)
- [ ] Cover more Google Maps Places API responses
