use reqwest::Client;

pub async fn dumb_file_tests(client: &Client) {
    println!("Testing root file...");
    let root = client
        .get("http://localhost:8080/")
        .send()
        .await
        .expect("Failed to get site root");
    assert_eq!(
        root.text().await.unwrap(),
        include_str!("../../src/resources/root.html")
    );
    println!("Root check succeeded!");
    println!("Testing 404 file...");
    let root = client
        .get("http://localhost:8080/404")
        .send()
        .await
        .expect("Failed to get 404 page");
    assert_eq!(
        root.text().await.unwrap(),
        include_str!("../../src/resources/404.html")
    );
    println!("404 check succeeded!");
    println!("Testing doc file...");
    let root = client
        .get("http://localhost:8080/simpleshortener/api")
        .send()
        .await
        .expect("Failed to get doc page");
    assert_eq!(
        root.text().await.unwrap(),
        include_str!("../../src/resources/doc.html")
    );
    println!("doc check succeeded!");
    println!("Testing setup file...");
    let panel = client
        .get("http://localhost:8080/simpleshortener")
        .send()
        .await
        .expect("Failed to get new user html");
    assert_eq!(
        panel.text().await.unwrap(),
        include_str!("../../src/resources/newuser.html")
    );
    println!("Setup check succeeded!");
    println!("Testing logo file...");
    let panel = client
        .get("http://localhost:8080/simpleshortener/static/link.png")
        .send()
        .await
        .expect("Failed to get logo");
    assert_eq!(
        panel.bytes().await.unwrap().to_vec(),
        include_bytes!("../../src/resources/logo.png")
    );
    println!("Logo check succeeded!");
    println!("Testing favicon file...");
    let panel = client
        .get("http://localhost:8080/favicon.ico")
        .send()
        .await
        .expect("Failed to get favicon");
    assert_eq!(
        panel.bytes().await.unwrap().to_vec(),
        include_bytes!("../../src/resources/favicon.ico")
    );
    println!("Favicon check succeeded!");
}
