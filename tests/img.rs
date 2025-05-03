use bikes::{AppConf, Tile};

#[tokio::test]
#[ignore]
async fn img_request_works() {
    let api_key = AppConf::from_env().unwrap().api_key();
    let (lon, lat) = (24.9314, 60.16847);
    let tile0 = Tile::ref_point(15, lon, lat);
    let img0 = tile0.img_request(&api_key).await.unwrap();
    assert_eq!(img0.len(), 109000);

    // different result with different coordinates
    let (lon, lat) = (24.94, 60.17);
    let tile1 = Tile::ref_point(15, lon, lat);
    let img1 = tile1.img_request(&api_key).await.unwrap();
    assert_eq!(img1.len(), 126494);
}
