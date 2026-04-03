use bikes::{AppConf, Station, StationData, Tile};

#[tokio::test]
#[ignore]
async fn station_data_get_works() {
    let api_key = AppConf::from_env().unwrap().api_key();
    let (lon, lat) = (24.94, 60.17);
    let ref_point = Tile::ref_point(15, lon, lat);

    let station_data_n = StationData::get(&api_key, lon, lat, 1000, 2).await.unwrap();
    let px = 350;

    let stations = station_data_n.into_stations(&ref_point, px);
    // response matches request limit
    // Station {
    let station0 = Station {
        id: String::from("022"),
        name: String::from("Rautatientori / länsi"),
        count: 0,
        x: 188,
        y: 119,
        distance: 99,
    };
    let station1 = Station {
        id: String::from("024"),
        name: String::from("Mannerheimintie"),
        count: 0,
        x: 155,
        y: 147,
        distance: 183,
    };

    let stations_exp = [station0, station1];
    for i in 0..2 {
        assert_eq!(stations[i].id, stations_exp[i].id);
        assert_eq!(stations[i].name, stations_exp[i].name);
        assert_eq!(stations[i].x, stations_exp[i].x);
        assert_eq!(stations[i].y, stations_exp[i].y);
        assert_eq!(stations[i].distance, stations_exp[i].distance);
    }
}

#[tokio::test]
#[ignore]
async fn station_data_get_limits_work() {
    let api_key = AppConf::from_env().unwrap().api_key();
    let (lon, lat) = (24.9314, 60.16847);
    let ref_point = Tile::ref_point(15, lon, lat);

    let n = 5;
    let station_data_n = StationData::get(&api_key, lon, lat, 1000, n as u8)
        .await
        .unwrap();
    let px = 350;

    let stations_n = station_data_n.into_stations(&ref_point, px);
    // response matches request limit
    assert_eq!(stations_n.len(), n);

    let max_dist = 300;
    let station_data_dist = StationData::get(&api_key, lon, lat, max_dist, 10)
        .await
        .unwrap();
    let stations_dist = station_data_dist.into_stations(&ref_point, px);
    assert!(!stations_dist.is_empty());

    // stations match request limit
    for s in stations_dist.iter() {
        assert!(s.distance < max_dist);
    }

    // same coordinates means result contains same stations in same order
    // counts might differ as requests have some time in between them
    for i in 0..stations_dist.len().min(n) {
        assert_eq!(stations_n[i].id, stations_dist[i].id);
        assert_eq!(stations_n[i].name, stations_dist[i].name);
        assert_eq!(stations_n[i].x, stations_dist[i].x);
        assert_eq!(stations_n[i].y, stations_dist[i].y);
        assert_eq!(stations_n[i].distance, stations_dist[i].distance);
    }
}
