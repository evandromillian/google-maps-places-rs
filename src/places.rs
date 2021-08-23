#[cfg(test)]
use mockito;

use crate::{GoogleMapPlaceError, Response};

/// Initiate Places struct with `api_key`.
/// ```
/// use google_maps_places::Places;
/// let places = &Places { api_key: "api-key" };
/// ```
pub struct Places<'a> {
    pub api_key: &'a str,
}

impl<'a> Places<'a> {
    /// Run API call to Google Maps
    ///
    /// ## Arguments
    ///
    /// * `place_id` - Place ID obtained from Google Place Search / Autocomplete
    ///
    /// ## Examples
    ///
    /// ```
    /// use google_maps_places::{Places, Response};
    ///
    /// let places = &Places { api_key: "api-key" };
    /// let res = match places.get_map_place("ChIJATaCWGU3zDER32m__CAwDyY") {
    ///     Ok(b) => b,
    ///     Err(e) => {
    ///         println!("Error {:?}", e);
    ///         return;
    ///     }
    /// };
    ///
    /// match res {
    ///     Response::Ok { result } => {
    ///         println!("Result: {:?}", result);
    ///     }
    ///     Response::ZeroResults => {
    ///         println!("Zero results");
    ///     }
    ///     Response::InvalidRequest => {
    ///         println!("Invalid Request");
    ///     }
    ///     Response::OverQueryLimit => {
    ///         println!("Over Query Limit");
    ///     }
    ///     Response::RequestDenied { error_message } => {
    ///         println!("Request Denied: {}", error_message);
    ///     }
    ///     Response::UnknownError => {
    ///         println!("Unknown Error")
    ///     }
    /// };
    /// ```
    pub fn get_map_place(&self, place_id: &str) -> Result<Response, GoogleMapPlaceError> {
        if place_id.is_empty() {
            return Err(GoogleMapPlaceError::BadRequest(
                "Place id is required".to_string(),
            ));
        }

        #[cfg(not(test))]
        let base_url = "https://maps.googleapis.com".to_string();

        #[cfg(test)]
        let base_url = mockito::server_url();

        let url = format!(
            "{}/maps/api/place/details/json?place_id={}&key={}",
            base_url, place_id, self.api_key
        );

        let res = match ureq::get(&url).call() {
            Ok(r) => r,
            Err(e) => {
                return Err(GoogleMapPlaceError::Unknown(e.to_string()));
            }
        };

        let body = match res.into_json::<Response>() {
            Ok(r) => r,
            Err(e) => {
                return Err(GoogleMapPlaceError::Unknown(e.to_string()));
            }
        };

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;
    use mockito::{mock, Matcher, Mock};
    const API_KEY: &str = "google-maps-secret-key";

    fn setup_mock(place_id: &str) -> Mock {
        let mut file = File::open(format!("./test/{}.json", place_id)).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        mock("GET", "/maps/api/place/details/json")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("place_id".into(), place_id.into()),
                Matcher::UrlEncoded("key".into(), API_KEY.into()),
            ]))
            .with_status(200)
            .with_body(data)
            .create()
    }

    #[test]
    fn test_valid_map_place() {
        let _m = setup_mock("place-001");
        let place = Places {
            api_key: API_KEY.into(),
        };
        let res = match place.get_map_place("place-001") {
            Ok(b) => b,
            Err(_e) => {
                assert!(false);
                return;
            }
        };

        let result = match res {
            Response::Ok { result } => result,
            _ => {
                assert!(false);
                return;
            }
        };

        let geometry = result.geometry.as_ref().unwrap();
        assert_eq!(geometry.location.lat, 3.0270637);
        assert_eq!(geometry.location.lng, 101.4379739);
        assert_eq!(geometry.viewport.northeast.lat, 3.028371480291502);
        assert_eq!(geometry.viewport.northeast.lng, 101.4393057302915);
        assert_eq!(geometry.viewport.southwest.lat, 3.025673519708498);
        assert_eq!(geometry.viewport.southwest.lng, 101.4366077697085);
        assert_eq!(result.street_number().unwrap(), "7");
        assert_eq!(result.route().unwrap(), "Leboh Palas");
        assert_eq!(result.sublocality().unwrap(), "Taman Selatan");
        assert_eq!(result.postal_code().unwrap(), "41200");
        assert_eq!(result.city().unwrap(), "Klang");
        assert_eq!(result.state().unwrap(), "Selangor");
        assert_eq!(result.country().unwrap(), "Malaysia");
        assert_eq!(result.country_code().unwrap(), "MY");
    }

    #[test]
    fn test_invalid_map_place() {
        let _m = setup_mock("place-invalid");

        let place = Places {
            api_key: API_KEY.into(),
        };
        let res = match place.get_map_place("place-invalid") {
            Ok(b) => b,
            Err(_e) => {
                assert!(false);
                return;
            }
        };

        if let Response::InvalidRequest = res {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_denied_map_place() {
        let _m = setup_mock("place-denied");

        let place = Places {
            api_key: API_KEY.into(),
        };
        let res = match place.get_map_place("place-denied") {
            Ok(b) => b,
            Err(_e) => {
                assert!(false);
                return;
            }
        };

        if let Response::RequestDenied { error_message } = res {
            assert!(true);
            assert_eq!(error_message, "The provided API key is invalid.");
        } else {
            assert!(false);
        }
    }
}
