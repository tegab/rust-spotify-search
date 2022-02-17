//
//  main.rs
//  Note: Search music on spotify using the spotify api.
//  Requirement: Token from Spotify
//  Created by OneOn on 2022-02-17.
//


use futures::executor::block_on;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};


// ***  Spotify Json to struct  ******
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}



#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
/*********END*********/



fn print_trkdetail(tracks: Vec<&Track>) {
    for track in tracks {
        println!("Track: {}", track.name);
        println!("Album: {}", track.album.name);
        println!(
            "Artist {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("url: {}", track.external_urls.spotify);
        println!("******************************************");
    }
}

#[tokio::main]


async fn main() {

    while true {

        println!("Search Spotify :" );
        let mut spotify_srch: String = String::new();
        std::io::stdin().read_line(&mut spotify_srch).unwrap();
        let is_success = block_on(rest_spotify(spotify_srch));
        if is_success ==  false { //break if spotify is unsuccessful
            break;
        }  

    }

}


async fn rest_spotify(search_query:String) -> bool {

    /*  Require token information in auth_token   */
    let auth_token  =   "BQCHHgmUINKd8jk3OyvGjF9R8J8sSwcZSJ2Dzcw56OPlLmogmgzoaVCppZuujrUob7XHJiGJHGZtqV-nIsVTNw801lVfFvrIpf43ED2cfS7ukXOE0jIX_p0yxBTeZp0haSI-ViWUKfry7WTvvLTKPaqtwb-gO_VuObw";
    /*     */
    
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) =>{ 
                    print_trkdetail(parsed.tracks.items.iter().collect());
                    return true;
                },
                Err(_) => { 
                    println!("Error Unexpected data");
                    return false;
                },
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Invalid token");
            return false;
        }
        other => {
            panic!("Failed: {:?}", other);
          
        }
    };


   


}
