use reqwest::{Client, Response};
use scraper::{Html, Selector};

pub async fn competitions(client: &Client) -> Response {
    let url = "https://discgolfmetrix.com/competitions_list_server.php?view=2&my_all=1&view=2&from=1&to=20&page=my";
    client.get(url).send().await.unwrap()
}

fn parse_competitions(xml : &str) -> Vec<Competition> {
    let fragment = Html::parse_fragment(xml);

    let trselector = Selector::parse("tbody > tr").unwrap();
    let tdselector = Selector::parse("td").unwrap();

    let mut vec = Vec::new();
    for tr in fragment.select(&trselector) {
        let onclick = tr.value().attr("onclick").unwrap();
        for td in tr.select(&tdselector) {

        }
        vec.push(Competition { id: onclick.to_string()})
    }

    vec


}


// Create competition Lions Åland
//https://discgolfmetrix.com/?u=competition_add&create_training=1&competitiontype=1&courseid=15904&parentid=&record_type=2
// redirects to:
//https://discgolfmetrix.com/?u=competition_start_players&ID=2066509&message_ok=added

//start
//curl 'https://discgolfmetrix.com/?u=competition_start_players&ID=2066509' -X POST -H 'Content-Type: application/x-www-form-urlencoded' --data-raw 'Mode=1&ModePlayers=1&HoleNo=1&ActionNext=Start...'
// Update hole 9 with score 3
//https://discgolfmetrix.com/score2_savechanges.php?changes=7366114,9,3,0,,,,,0

struct Competition {
    pub id: String
}

#[cfg(test)]
mod tests {
    use crate::competitions;

    #[test]
    fn it_works() {
        let infile_name = "src/competitions.html";
        let xml = std::fs::read_to_string(infile_name).expect("Failed to read {}");

        let vec = competitions::parse_competitions(&xml);
        assert_eq!(vec.len(), 78);
    }
}
