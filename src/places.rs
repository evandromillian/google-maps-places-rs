#[cfg(test)]
use mockito;

use crate::error::GoogleMapPlaceError;
use crate::response::Response;

pub struct Places {
    pub api_key: String,
}

impl Places {
    pub fn get_map_place(&self, id: &str) -> Result<Response, GoogleMapPlaceError> {
        if id.is_empty() {
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
            base_url, id, self.api_key
        );

        let res = match ureq::get(&url)
            .call() {
                Ok(r) => r,
                Err(e) => {
                    return Err(GoogleMapPlaceError::Unknown(e.to_string()));
                },
            };

        let body = match res.into_json::<Response>() {
            Ok(r) => r,
            Err(e) => {
                return Err(GoogleMapPlaceError::Unknown(e.to_string()));
            },
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
        let place = Places { api_key: API_KEY.into() };
        let res = match place.get_map_place("place-001") {
            Ok(b) => b,
            Err(_e) => {
                assert!(false);
                return;
            }
        };

        if let Response::OK { result } = res {
            assert_eq!(result.address_components.len(), 7);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_invalid_map_place() {
        let _m = setup_mock("place-invalid");

        let place = Places { api_key: API_KEY.into() };
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

        let place = Places { api_key: API_KEY.into() };
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
