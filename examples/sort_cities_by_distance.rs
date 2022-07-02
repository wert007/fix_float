use fix_float::*;

macro_rules! _to_deg {
    ($deg:literal $min:literal) => {
        ($deg as f64) + ($min as f64 / 60f64)
    };
}

macro_rules! lat {
	($deg:literal $min:literal N) => {
		_to_deg!($deg $min)
	};
	($deg:literal $min:literal S) => {
		- _to_deg!($deg $min)
	};
}

macro_rules! long {
	($deg:literal $min:literal E) => {
		_to_deg!($deg $min)
	};
	($deg:literal $min:literal W) => {
		- _to_deg!($deg $min)
	};
}

type City = (&'static str, &'static str, f64, f64);

const CITIES: &[City] = &[
    ("Aberdeen", "Scotland", lat!(57 9 N), long!(2 9 W)),
    ("Adelaide", "Australia", lat!(34 55 S), long!(138 36 E)),
    ("Algiers", "Algeria", lat!(36 50 N), long!(3 0 E)),
    ("Amsterdam", "Netherlands", lat!(52 22 N), long!(4 53 E)),
    ("Ankara", "Turkey", lat!(39 55 N), long!(32 55 E)),
    ("Asunción", "Paraguay", lat!(25 15 S), long!(57 40 W)),
    ("Athens", "Greece", lat!(37 58 N), long!(23 43 E)),
    ("Auckland", "New Zealand", lat!(36 52 S), long!(174 45 E)),
    ("Bangkok", "Thailand", lat!(13 45 N), long!(100 30 E)),
    ("Barcelona", "Spain", lat!(41 23 N), long!(2 9 E)),
    ("Beijing", "China", lat!(39 55 N), long!(116 25 E)),
    ("Belém", "Brazil", lat!(1 28 S), long!(48 29 W)),
    ("Belfast", "Northern Ireland", lat!(54 37 N), long!(5 56 W)),
    ("Belgrade", "Serbia", lat!(44 52 N), long!(20 32 E)),
    ("Berlin", "Germany", lat!(52 30 N), long!(13 25 E)),
    ("Birmingham", "England", lat!(52 25 N), long!(1 55 W)),
    ("Bogotá", "Colombia", lat!(4 32 N), long!(74 15 W)),
    ("Bombay", "India", lat!(19 0 N), long!(72 48 E)),
    ("Bordeaux", "France", lat!(44 50 N), long!(0 31 W)),
    ("Bremen", "Germany", lat!(53 5 N), long!(8 49 E)),
    ("Brisbane", "Australia", lat!(27 29 S), long!(153 8 E)),
    ("Bristol", "England", lat!(51 28 N), long!(2 35 W)),
    ("Brussels", "Belgium", lat!(50 52 N), long!(4 22 E)),
    ("Bucharest", "Romania", lat!(44 25 N), long!(26 7 E)),
    ("Budapest", "Hungary", lat!(47 30 N), long!(19 5 E)),
    ("Buenos Aires", "Argentina", lat!(34 35 S), long!(58 22 W)),
    ("Cairo", "Egypt", lat!(30 2 N), long!(31 21 E)),
    ("Calcutta", "India", lat!(22 34 N), long!(88 24 E)),
    ("Canton", "China", lat!(23 7 N), long!(113 15 E)),
    ("Cape Town", "South Africa", lat!(33 55 S), long!(18 22 E)),
    ("Caracas", "Venezuela", lat!(10 28 N), long!(67 2 W)),
    ("Cayenne", "French Guiana", lat!(4 49 N), long!(52 18 W)),
    ("Chicago", "USA", lat!(41 83 N), long!(87 68 W)),
    ("Chihuahua", "Mexico", lat!(28 37 N), long!(106 5 W)),
    ("Chongqing", "China", lat!(29 46 N), long!(106 34 E)),
    ("Copenhagen", "Denmark", lat!(55 40 N), long!(12 34 E)),
    ("Córdoba", "Argentina", lat!(31 28 S), long!(64 10 W)),
    ("Dakar", "Senegal", lat!(14 40 N), long!(17 28 W)),
    ("Darwin", "Australia", lat!(12 28 S), long!(130 51 E)),
    ("Djibouti", "Djibouti", lat!(11 30 N), long!(43 3 E)),
    ("Dublin", "Ireland", lat!(53 20 N), long!(6 15 W)),
    ("Durban", "South Africa", lat!(29 53 S), long!(30 53 E)),
    ("Edinburgh", "Scotland", lat!(55 55 N), long!(3 10 W)),
    ("Frankfurt", "Germany", lat!(50 7 N), long!(8 41 E)),
    ("Georgetown", "Guyana", lat!(6 45 N), long!(58 15 W)),
    ("Glasgow", "Scotland", lat!(55 50 N), long!(4 15 W)),
    ("Guatemala City", "Guatemala", lat!(14 37 N), long!(90 31 W)),
    ("Guayaquil", "Ecuador", lat!(2 10 S), long!(79 56 W)),
    ("Hamburg", "Germany", lat!(53 33 N), long!(10 2 E)),
    ("Hammerfest", "Norway", lat!(70 38 N), long!(23 38 E)),
    ("Havana", "Cuba", lat!(23 8 N), long!(82 23 W)),
    ("Helsinki", "Finland", lat!(60 10 N), long!(25 0 E)),
    ("Hobart", "Tasmania", lat!(42 52 S), long!(147 19 E)),
    ("Hong Kong", "China", lat!(22 20 N), long!(114 11 E)),
    ("Houston", "USA", lat!(29 78 N), long!(95 39 W)),
    ("Iquique", "Chile", lat!(20 10 S), long!(70 7 W)),
    ("Irkutsk", "Russia", lat!(52 30 N), long!(104 20 E)),
    ("Jakarta", "Indonesia", lat!(6 16 S), long!(106 48 E)),
    ("Johannesburg", "South Africa", lat!(26 12 S), long!(28 4 E)),
    ("Kingston", "Jamaica", lat!(17 59 N), long!(76 49 W)),
    ("Kinshasa", "Congo", lat!(4 18 S), long!(15 17 E)),
    ("Kuala Lumpur", "Malaysia", lat!(3 8 N), long!(101 42 E)),
    ("La Paz", "Bolivia", lat!(16 27 S), long!(68 22 W)),
    ("Leeds", "England", lat!(53 45 N), long!(1 30 W)),
    ("Lima", "Peru", lat!(12 0 S), long!(77 2 W)),
    ("Lisbon", "Portugal", lat!(38 44 N), long!(9 9 W)),
    ("Liverpool", "England", lat!(53 25 N), long!(3 0 W)),
    ("London", "England", lat!(51 32 N), long!(0 5 W)),
    ("Los Angeles", "USA", lat!(34 01 N), long!(118 41 W)),
    ("Lyons", "France", lat!(45 45 N), long!(4 50 E)),
    ("Madrid", "Spain", lat!(40 26 N), long!(3 42 W)),
    ("Manchester", "England", lat!(53 30 N), long!(2 15 W)),
    ("Manila", "Philippines", lat!(14 35 N), long!(120 57 E)),
    ("Marseilles", "France", lat!(43 20 N), long!(5 20 E)),
    ("Mazatlán", "Mexico", lat!(23 12 N), long!(106 25 W)),
    ("Mecca", "Saudi Arabia", lat!(21 29 N), long!(39 45 E)),
    ("Melbourne", "Australia", lat!(37 47 S), long!(144 58 E)),
    ("Mexico City", "Mexico", lat!(19 26 N), long!(99 7 W)),
    ("Milan", "Italy", lat!(45 27 N), long!(9 10 E)),
    ("Montevideo", "Uruguay", lat!(34 53 S), long!(56 10 W)),
    ("Moscow", "Russia", lat!(55 45 N), long!(37 36 E)),
    ("Munich", "Germany", lat!(48 8 N), long!(11 35 E)),
    ("Nagasaki", "Japan", lat!(32 48 N), long!(129 57 E)),
    ("Nagoya", "Japan", lat!(35 7 N), long!(136 56 E)),
    ("Nairobi", "Kenya", lat!(1 25 S), long!(36 55 E)),
    ("Nanjing (Nanking)", "China", lat!(32 3 N), long!(118 53 E)),
    ("Naples", "Italy", lat!(40 50 N), long!(14 15 E)),
    ("New Delhi", "India", lat!(28 35 N), long!(77 12 E)),
    ("New York", "USA", lat!(40 42 N), long!(74 0 W)),
    ("Newcastle-on-Tyne", "England", lat!(54 58 N), long!(1 37 W)),
    ("Odessa", "Ukraine", lat!(46 27 N), long!(30 48 E)),
    ("Osaka", "Japan", lat!(34 32 N), long!(135 30 E)),
    ("Oslo", "Norway", lat!(59 57 N), long!(10 42 E)),
    ("Panama City", "Panama", lat!(8 58 N), long!(79 32 W)),
    ("Paramaribo", "Suriname", lat!(5 45 N), long!(55 15 W)),
    ("Paris", "France", lat!(48 48 N), long!(2 20 E)),
    ("Perth", "Australia", lat!(31 57 S), long!(115 52 E)),
    ("Philadelphia", "USA", lat!(40 00 N), long!(75 13 W)),
    ("Phoenix", "USA", lat!(33 57 N), long!(112 09 W)),
    ("Plymouth", "England", lat!(50 25 N), long!(4 5 W)),
    (
        "Port Moresby",
        "Papua New Guinea",
        lat!(9 25 S),
        long!(147 8 E),
    ),
    ("Prague", "Czech Republic", lat!(50 5 N), long!(14 26 E)),
    ("Rangoon", "Myanmar", lat!(16 50 N), long!(96 0 E)),
    ("Reykjavík", "Iceland", lat!(64 4 N), long!(21 58 W)),
    ("Rio de Janeiro", "Brazil", lat!(22 57 S), long!(43 12 W)),
    ("Rome", "Italy", lat!(41 54 N), long!(12 27 E)),
    ("Salvador", "Brazil", lat!(12 56 S), long!(38 27 W)),
    ("Santiago", "Chile", lat!(33 28 S), long!(70 45 W)),
    ("St. Petersburg", "Russia", lat!(59 56 N), long!(30 18 E)),
    ("São Paulo", "Brazil", lat!(23 31 S), long!(46 31 W)),
    ("San Antonio", "USA", lat!(29 47 N), long!(98 52 W)),
    ("Shanghai", "China", lat!(31 10 N), long!(121 28 E)),
    ("Singapore", "Singapore", lat!(1 14 N), long!(103 55 E)),
    ("Sofia", "Bulgaria", lat!(42 40 N), long!(23 20 E)),
    ("Stockholm", "Sweden", lat!(59 17 N), long!(18 3 E)),
    ("Sydney", "Australia", lat!(34 0 S), long!(151 0 E)),
    ("Tananarive", "Madagascar", lat!(18 50 S), long!(47 33 E)),
    ("Teheran", "Iran", lat!(35 45 N), long!(51 45 E)),
    ("Tokyo", "Japan", lat!(35 40 N), long!(139 45 E)),
    ("Tripoli", "Libya", lat!(32 57 N), long!(13 12 E)),
    ("Venice", "Italy", lat!(45 26 N), long!(12 20 E)),
    ("Veracruz", "Mexico", lat!(19 10 N), long!(96 10 W)),
    ("Vienna", "Austria", lat!(48 14 N), long!(16 20 E)),
    ("Vladivostok", "Russia", lat!(43 10 N), long!(132 0 E)),
    ("Warsaw", "Poland", lat!(52 14 N), long!(21 0 E)),
    ("Wellington", "New Zealand", lat!(41 17 S), long!(174 47 E)),
    ("Zürich", "Switzerland", lat!(47 21 N), long!(8 31 E)),
];

const PI: f64 = std::f64::consts::PI;

// Returns distances between two points expressed in degrees (latitude and longitude)
// The distance is expressed in kilometers
// using the haversine formula from this website: https://www.movable-type.co.uk/scripts/latlong.html
fn compute_dist(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    let phi1 = lat1 * PI / 180f64;
    let phi2 = lat2 * PI / 180f64;

    let delta_phi = (lat2 - lat1) * PI / 180f64;
    let delta_lambda = (long2 - long1) * PI / 180f64;

    let a = (delta_phi / 2f64).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda / 2f64).sin().powi(2);
    let b = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());
    b * 6371f64
}

fn find_city(name: &str) -> Option<(f64, f64)> {
    for city in CITIES {
        if city.0 == name {
            return Some((city.2, city.3));
        }
    }

    None
}

fn main() {
    let origin_city = "Paris";

    let origin = find_city(origin_city);

    if origin.is_none() {
        eprintln!("Couldn'\'t find city `{}`", origin_city);
        return;
    }

    let (lat, long) = origin.unwrap();

    let mut nearest_cities: Vec<(ff64, City)> = Vec::with_capacity(CITIES.len());

    for &city in CITIES {
        let dist = compute_dist(lat, long, city.2, city.3);

        nearest_cities.push((ff64!(dist), city));
    }

    nearest_cities.sort_by_key(|e| e.0);

    println!("Nearest city from `{}`", origin_city);
    for (dist, city) in nearest_cities {
        println!("  {:>10.2} km: {:?}", *dist, city);
    }
}
