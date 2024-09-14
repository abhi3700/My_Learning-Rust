//! Play with Airbnb dataset
//!
//! Sample doc:
//! ```json
//! {
//!   "_id": "10009999",
//!   "listing_url": "https://www.airbnb.com/rooms/10009999",
//!   "name": "Horto flat with small garden",
//!   "summary": "One bedroom + sofa-bed in quiet and bucolic neighbourhood right next to the Botanical Garden. Small garden, outside shower, well equipped kitchen and bathroom with shower and tub. Easy for transport with many restaurants and basic facilities in the area.",
//!   "space": "Lovely one bedroom + sofa-bed in the living room, perfect for two but fits up to four comfortably.  There´s a small outside garden with a shower There´s a well equipped open kitchen with both 110V / 220V wall plugs and one bathroom with shower, tub and even a sauna machine! All newly refurbished!",
//!   "description": "One bedroom + sofa-bed in quiet and bucolic neighbourhood right next to the Botanical Garden. Small garden, outside shower, well equipped kitchen and bathroom with shower and tub. Easy for transport with many restaurants and basic facilities in the area. Lovely one bedroom + sofa-bed in the living room, perfect for two but fits up to four comfortably.  There´s a small outside garden with a shower There´s a well equipped open kitchen with both 110V / 220V wall plugs and one bathroom with shower, tub and even a sauna machine! All newly refurbished! I´ll be happy to help you with any doubts, tips or any other information needed during your stay. This charming ground floor flat is located in Horto, a quiet and bucolic neighborhood just next to the Botanical Garden, where most of the descendants of it´s first gardeners still live. You´ll be 30 minutes walk from waterfalls in the rainforest with easy hiking trails! There are nice bars and restaurants as well as basic facilities - pharmacy, b",
//!   "neighborhood_overview": "This charming ground floor flat is located in Horto, a quiet and bucolic neighborhood just next to the Botanical Garden, where most of the descendants of it´s first gardeners still live. You´ll be 30 minutes walk from waterfalls in the rainforest with easy hiking trails! There are nice bars and restaurants as well as basic facilities - pharmacy, bakery, small market - in the area.",
//!   "notes": "There´s a table in the living room now, that does not show in the photos.",
//!   "transit": "Easy access to transport (bus, taxi, car) and easy free parking around. Very close to Gávea, Leblon, Ipanema, Copacabana and Botafogo.",
//!   "access": "",
//!   "interaction": "I´ll be happy to help you with any doubts, tips or any other information needed during your stay.",
//!   "house_rules": "I just hope the guests treat the space as they´re own, with respect to it as well as to my neighbours! Espero apenas que os hóspedes tratem o lugar com carinho e respeito aos vizinhos!",
//!   "property_type": "Apartment",
//!   "room_type": "Entire home/apt",
//!   "bed_type": "Real Bed",
//!   "minimum_nights": "2",
//!   "maximum_nights": "1125",
//!   "cancellation_policy": "flexible",
//!   "last_scraped": {
//!     "$date": "2019-02-11T05:00:00.000Z"
//!   },
//!   "calendar_last_scraped": {
//!     "$date": "2019-02-11T05:00:00.000Z"
//!   },
//!   "accommodates": 4,
//!   "bedrooms": 1,
//!   "beds": 2,
//!   "number_of_reviews": 0,
//!   "bathrooms": {
//!     "$numberDecimal": "1.0"
//!   },
//!   "amenities": [
//!     "Wifi",
//!     "Wheelchair accessible",
//!     "Kitchen",
//!     "Free parking on premises",
//!     "Smoking allowed",
//!     "Hot tub",
//!     "Buzzer/wireless intercom",
//!     "Family/kid friendly",
//!     "Washer",
//!     "First aid kit",
//!     "Essentials",
//!     "Hangers",
//!     "Hair dryer",
//!     "Iron",
//!     "Laptop friendly workspace"
//!   ],
//!   "price": {
//!     "$numberDecimal": "317.00"
//!   },
//!   "weekly_price": {
//!     "$numberDecimal": "1492.00"
//!   },
//!   "monthly_price": {
//!     "$numberDecimal": "4849.00"
//!   },
//!   "cleaning_fee": {
//!     "$numberDecimal": "187.00"
//!   },
//!   "extra_people": {
//!     "$numberDecimal": "0.00"
//!   },
//!   "guests_included": {
//!     "$numberDecimal": "1"
//!   },
//!   "images": {
//!     "thumbnail_url": "",
//!     "medium_url": "",
//!     "picture_url": "https://a0.muscache.com/im/pictures/5b408b9e-45da-4808-be65-4edc1f29c453.jpg?aki_policy=large",
//!     "xl_picture_url": ""
//!   },
//!   "host": {
//!     "host_id": "1282196",
//!     "host_url": "https://www.airbnb.com/users/show/1282196",
//!     "host_name": "Ynaie",
//!     "host_location": "Rio de Janeiro, State of Rio de Janeiro, Brazil",
//!     "host_about": "I am an artist and traveling is a major part of my life. I love treating visitors the way I like to be treated when I´m abroad and I'm usually renting my flat while I'm away. I can recommend some cool parties and nights out as well as advise on some hidden secrets of Rio’s nature!",
//!     "host_thumbnail_url": "https://a0.muscache.com/im/pictures/9681e3cc-4af1-4046-b294-2881dffb4ff8.jpg?aki_policy=profile_small",
//!     "host_picture_url": "https://a0.muscache.com/im/pictures/9681e3cc-4af1-4046-b294-2881dffb4ff8.jpg?aki_policy=profile_x_medium",
//!     "host_neighbourhood": "Jardim Botânico",
//!     "host_is_superhost": false,
//!     "host_has_profile_pic": true,
//!     "host_identity_verified": false,
//!     "host_listings_count": 1,
//!     "host_total_listings_count": 1,
//!     "host_verifications": [
//!       "email",
//!       "phone",
//!       "facebook"
//!     ]
//!   },
//!   "address": {
//!     "street": "Rio de Janeiro, Rio de Janeiro, Brazil",
//!     "suburb": "Jardim Botânico",
//!     "government_area": "Jardim Botânico",
//!     "market": "Rio De Janeiro",
//!     "country": "Brazil",
//!     "country_code": "BR",
//!     "location": {
//!       "type": "Point",
//!       "coordinates": [
//!         -43.23074991429229,
//!         -22.966253551739655
//!       ],
//!       "is_location_exact": true
//!     }
//!   },
//!   "availability": {
//!     "availability_30": 0,
//!     "availability_60": 0,
//!     "availability_90": 0,
//!     "availability_365": 0
//!   },
//!   "review_scores": {},
//!   "reviews": []
//! }
//! ```
use futures::{stream::TryStreamExt, StreamExt};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ListingsAndReviews {
	listing_url: String,
	name: String,
	amenities: Vec<String>,
	reviews: Vec<Review>,
	review_scores: ReviewScores,
}

#[derive(Debug, Deserialize, Serialize)]
struct Review {}

#[derive(Debug, Deserialize, Serialize)]
struct ReviewScores {
	review_scores_value: i32,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
	dotenv::from_path("./.env").expect("Failed to load the env file");

	let mongodb_uri = std::env::var("MONGODB_URI").expect("Invalid MONGODB_URI");
	let client = mongodb::Client::with_uri_str(mongodb_uri)
		.await
		.expect("Failed to connect to MongoDB");

	let db = client.database("sample_airbnb");
	let listings_and_reviews = db.collection::<ListingsAndReviews>("listingsAndReviews");

	let start = std::time::Instant::now();
	let mut cursor = listings_and_reviews.find(None, None).await.unwrap();
	let first_doc = cursor.try_next().await.unwrap().unwrap();
	// NOTE: Both
	// println!("Took {}ms {:#?}", start.elapsed().as_millis(),
	// cursor.collect::<Vec<_>>().await[0]); // Slow 🐢 as it will download all the
	// documents matching the filter. Here it is set to None.

	// println!("Took {}ms {:#?}", start.elapsed().as_millis(), first_doc); // Fast 🏎️💨
	// let start = std::time::Instant::now();
	// let first_doc = listings_and_reviews.find_one(None, None).await.unwrap().unwrap();
	// println!("Took {}ms {:#?}", start.elapsed().as_millis(), first_doc); // Fastest 🚀

	// NOTE: So, if you need to get only one document, use `find_one` instead of `find` and
	// `collect`.

	// ================================================
	// Now, let's get amenities in the first doc with at least 5 `reviews` & at least 8
	// `review_scores_value`.
	let start = std::time::Instant::now();
	let best_hotel = listings_and_reviews
		.find_one(
			doc! {
				"$expr": {
					"$and": [
						{ "$gte": [{ "$size": "$reviews" }, 5] },
						{ "$gte": ["$review_scores.review_scores_value", 8] }
					]
				}
			},
			None,
		)
		.await?;

	if let Some(best_hotel) = best_hotel {
		println!("Took {}ms {:#?}", start.elapsed().as_millis(), best_hotel.amenities.len());
	}

	// ================================================
	// TODO: Find out the first doc where amenities contain `Wifi`, `Kitchen` and `Free parking`

	// ================================================
	// TODO: Find out the how many hotels with `listing_url` contains "airbnb".
	let cursor = listings_and_reviews
		.find(doc! {"listing_url": {"$regex": "airbnb"}}, None)
		.await?;
	println!("Total hotels: {:?}", cursor.collect::<Vec<_>>().await.len());

	Ok(())
}
