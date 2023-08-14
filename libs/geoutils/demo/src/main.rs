//! Calculate distance between 2 geo coordinates on earth.
//! The best formula to calculate the Earth's distance between two given geo-coordinates is the Haversine formula. It calculates the great-circle distance between two points on a sphere, given their longitudes and latitudes[6]. The formula is as follows:

//! 1. Convert the latitude and longitude values from decimal degrees to radians.
//! 2. Use the Haversine formula:
//!    a = sin²((lat2 - lat1) / 2) + cos(lat1) * cos(lat2) * sin²((lon2 - lon1) / 2)
//!    c = 2 * atan2(√a, √(1 - a))
//!    d = R * c
//!
//! where:
//! - lat1 and lat2 are the latitudes of the two points
//! - lon1 and lon2 are the longitudes of the two points
//! - R is the Earth's radius (mean radius = 6,371 km or 3,963 miles)
//! - d is the distance between the two points along a great circle of the sphere[2]
//!
//! Keep in mind that this formula assumes a spherical Earth, which is accurate enough for most purposes. However, for very precise calculations, you may need to consider the Earth's ellipsoidal shape[2].
//!
//! References:
//! [1] https://community.powerbi.com/t5/Desktop/How-to-calculate-lat-long-distance/td-p/1488227
//! [2] https://www.movable-type.co.uk/scripts/latlong.html
//! [3] https://www.geeksforgeeks.org/program-distance-two-points-earth/
//! [4] https://www.igismap.com/haversine-formula-calculate-geographic-distance-earth/
//! [5] https://stackoverflow.com/questions/365826/calculate-distance-between-2-gps-coordinates
//! [6] https://en.wikipedia.org/wiki/Haversine_formula
//! [7] https://www.omnicalculator.com/other/latitude-longitude-distance
//! [8] https://www.themathdoctors.org/distances-on-earth-3-planar-approximation/

use geoutils::Location;

fn main() {
    let berlin = Location::new(52.518611, 13.408056);
    let moscow = Location::new(55.751667, 37.617778);
    let distance = berlin.distance_to(&moscow).unwrap();
    println!("Distance (in kms) = {}", distance.meters() / 1000f64);
}
