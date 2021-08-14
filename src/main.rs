use std::env;

use google_maps_places::places::Places;

#[tokio::main]
async fn main() {
    let api_key = match env::var("GOOGLE_MAPS_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("GOOGLE_MAPS_API_KEY env required");
            return;
        },
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("place_id arg required");
        return;
    }

    let place_id = &args[1];

    let places = Places {
        api_key: api_key,
    };
    let place = match places.get_map_place(place_id).await {
        Ok(b) => b,
        Err(e) => {
            println!("Error {:?}", e);
            return;
        }
    };

    println!("Place: \n{:?}", place);
}
