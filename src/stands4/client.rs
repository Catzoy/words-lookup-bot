use crate::networking::api_client::ApiClient;
use crate::stands4::config::Stands4Config;
use crate::stands4::fix_response_middleware::FixEmptyResponseMiddleware;
use crate::stands4::responses::Results;
use rustify::Endpoint;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Stands4Client {
    client: reqwest::Client,
    config: Stands4Config,
}

impl Stands4Client {
    pub fn new(user_id: String, token: String) -> Self {
        Stands4Client {
            client: Default::default(),
            config: Stands4Config::new(user_id, token),
        }
    }

    fn client(&self) -> ApiClient {
        ApiClient {
            client: rustify::Client::new(
                "https://www.stands4.com/services/v2",
                self.client.clone(),
            ),
        }
    }

    /// Execute a prepared HTTP request and convert the JSON results into domain entities.
    ///
    /// If the HTTP response body is empty, this returns an empty `Vec`.
    ///
    /// # Parameters
    ///
    /// * `request` - A `reqwest::RequestBuilder` that will be executed.
    ///
    /// # Returns
    ///
    /// A `Vec` of `Response::Output` parsed from the JSON `Results<Response>` payload; `Vec::new()` if the response body is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use reqwest::Client;
    /// # use crate::stands4::client::Stands4Client;
    /// # use crate::stands4::responses::WordResult;
    /// # async fn example() -> anyhow::Result<()> {
    /// let client = Stands4Client::new("uid".into(), "token".into());
    /// let req = client.client.get("https://example.com/api"); // RequestBuilder
    /// let items: Vec<<WordResult as crate::stands4::traits::ToEntity>::Output> = client.handle_request::<WordResult>(req).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exec<Entity, Response, Endpoint: rustify::Endpoint<Response = Results<Response>>>(
        &self,
        request: Endpoint,
    ) -> anyhow::Result<Vec<Entity>>
    where
        Response: Debug + Send + Sync + DeserializeOwned,
        Entity: From<Response>,
    {
        let request = request
            .with_middleware(&self.config)
            .with_middleware(&FixEmptyResponseMiddleware);
        self.client().exec(request).await?
    }
}

#[cfg(test)]
mod tests {
    use crate::stands4::client::Results;
    use crate::stands4::responses::{PhraseResult, VecMixedType, WordResult};

    #[test]
    fn parsing_words_works() {
        let response = "{\"result\":[{\"term\":\"sugar, refined sugar\",\"definition\":\"a white crystalline carbohydrate used as a sweetener and preservative\",\"example\":{},\"partofspeech\":\"noun\"},{\"term\":\"carbohydrate, saccharide, sugar\",\"definition\":\"an essential structural component of living cells and source of energy for animals; includes simple sugars with small molecules as well as macromolecular substances; are classified according to the number of monosaccharide groups they contain\",\"example\":{},\"partofspeech\":\"noun\"},{\"term\":\"boodle, bread, cabbage, clams, dinero, dough, gelt, kale, lettuce, lolly, lucre, loot, moolah, pelf, scratch, shekels, simoleons, sugar, wampum\",\"definition\":\"informal terms for money\",\"example\":{},\"partofspeech\":\"verb\"},{\"term\":\"sugar, saccharify\",\"definition\":\"sweeten with sugar\",\"example\":\"\\\"sugar your tea\\\"\",\"partofspeech\":\"verb\"},{\"term\":\"sugar\",\"definition\":\"Sucrose in the form of small crystals, obtained from sugar cane or sugar beet and used to sweeten food and drink.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"When used to sweeten drink, an amount of such crystalline sucrose approximately equal to five grams or one teaspoon.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"Any of various small carbohydrates that are used by organisms to store energy.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"A generic term for sucrose, glucose, fructose, etc.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"A term of endearment.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"A kiss.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"Effeminacy in a male, often implying homosexuality.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"Diabetes.\",\"example\":{},\"partofspeech\":\"Noun\"},{\"term\":\"sugar\",\"definition\":\"To add sugar to; to sweeten with sugar.\",\"example\":{},\"partofspeech\":\"Verb\"},{\"term\":\"sugar\",\"definition\":\"To make (something unpleasant) seem less so.\",\"example\":{},\"partofspeech\":\"Verb\"},{\"term\":\"sugar\",\"definition\":\"Used in place of shit!\",\"example\":{},\"partofspeech\":\"Interjection\"},{\"term\":\"Sugar\",\"definition\":\"Sugar is the generalised name for a class of chemically-related sweet-flavored substances, most of which are used as food. They are carbohydrates, composed of carbon, hydrogen and oxygen. There are various types of sugar derived from different sources. Simple sugars are called monosaccharides and include glucose, fructose and galactose. The table or granulated sugar most customarily used as food is sucrose, a disaccharide. Other disaccharides include maltose and lactose. Chemically-different substances may also have a sweet taste, but are not classified as sugars. Some are used as lower-calorie food substitutes for sugar described as artificial sweeteners.\\nSugars are found in the tissues of most plants but are only present in sufficient concentrations for efficient extraction in sugarcane and sugar beet. Sugarcane is a giant grass and has been cultivated in tropical climates in the Far East since ancient times. A great expansion in its production took place in the 18th century with the setting up of sugar plantations in the West Indies and Americas. This was the first time that sugar became available to the common people who had previously had to rely on honey to sweeten foods. Sugar beet is a root crop and is cultivated in cooler climates and became a major source of sugar in the 19th century when methods for extracting the sugar became available. Sugar production and trade has changed the course of human history in many ways. It influenced the formation of colonies, the perpetuation of slavery, the transition to indentured labour, the migration of peoples, wars between sugar trade-controlling nations in the 19th century, and the ethnic composition and political structure of the new world.\",\"example\":{},\"partofspeech\":{}},{\"term\":\"Sugar\",\"definition\":\"Sugar is the generic name for sweet-tasting, soluble carbohydrates, many of which are used in food. Simple sugars, also called monosaccharides, include glucose, fructose, and galactose. Compound sugars, also called disaccharides or double sugars, are molecules made of two bonded monosaccharides; common examples are sucrose (glucose + fructose), lactose (glucose + galactose), and maltose (two molecules of glucose). White sugar is a refined form of sucrose. In the body, compound sugars are hydrolysed into simple sugars.\\nLonger chains of monosaccharides (>2) are not regarded as sugars, and are called oligosaccharides or polysaccharides. Starch is a glucose polymer found in plants, the most abundant source of energy in human food. Some other chemical substances, such as glycerol and sugar alcohols, may have a sweet taste, but are not classified as sugar.\\nSugars are found in the tissues of most plants. Honey and fruits are abundant natural sources of simple sugars. Sucrose is especially concentrated in sugarcane and sugar beet, making them ideal for efficient commercial extraction to make refined sugar. In 2016, the combined world production of those two crops was about two billion tonnes. Maltose may be produced by malting grain. Lactose is the only sugar that cannot be extracted from plants. It can only be found in milk, including human breast milk, and in some dairy products. A cheap source of sugar is corn syrup, industrially produced by converting corn starch into sugars, such as maltose, fructose and glucose.\\nSucrose is used in prepared foods (e.g. cookies and cakes), is sometimes added to commercially available processed food and beverages, and may be used by people as a sweetener for foods (e.g. toast and cereal) and beverages (e.g. coffee and tea). The average person consumes about 24 kilograms (53 pounds) of sugar each year, with North and South Americans consuming up to 50 kg (110 lb) and Africans consuming under 20 kg (44 lb).As sugar consumption grew in the latter part of the 20th century, researchers began to examine whether a diet high in sugar, especially refined sugar, was damaging to human health. Excessive consumption of sugar has been implicated in the onset of obesity, diabetes, cardiovascular disease, and tooth decay. Numerous studies have tried to clarify those implications, but with varying results, mainly because of the difficulty of finding populations for use as controls that consume little or no sugar. In 2015, the World Health Organization recommended that adults and children reduce their intake of free sugars to less than 10%, and encouraged a reduction to below 5%, of their total energy intake.\",\"example\":{},\"partofspeech\":{}}]}";
        let parsed = serde_json::from_slice::<Results<WordResult>>(response.as_bytes())
            .map_err(anyhow::Error::msg);
        let results = match parsed {
            Ok(results) => results,
            Err(_) => panic!("Failed to parse results from stands4"),
        };
        let array = results.result.ok_or_else(|| {
            panic!("Failed to parse result values from stands4");
        });
        let vec = match array {
            Ok(array) => array,
            Err(_) => panic!("Failed to parse result into a vec from stands4"),
        };
        let vec = match vec {
            VecMixedType::Vec(vec) => vec,
            VecMixedType::Single(value) => vec![value],
            VecMixedType::Other(it) => panic!("Empty state unexpected - {}", it),
        };
        assert_eq!(vec.len(), 17, "every item should be properly parsed");
    }
    #[test]
    fn parsing_phrases_works() {
        let response = "{\"result\":{\"term\":\"buckle up\",\"explanation\":\"To fasten one's seat belt or safety belt.\",\"example\":\"Buckle up every time you drive somewhere in a car, and make sure your passengers buckle up, too.\"}}";
        let parsed = serde_json::from_slice::<Results<PhraseResult>>(response.as_bytes())
            .map_err(anyhow::Error::msg);
        let results = match parsed {
            Ok(results) => results,
            Err(_) => panic!("Failed to parse results from stands4"),
        };
        let array = results.result.ok_or_else(|| {
            panic!("Failed to parse result values from stands4");
        });
        let vec = match array {
            Ok(array) => array,
            Err(_) => panic!("Failed to parse result into a vec from stands4"),
        };
        let vec = match vec {
            VecMixedType::Vec(vec) => vec,
            VecMixedType::Single(value) => vec![value],
            VecMixedType::Other(it) => panic!("Empty state unexpected - {}", it),
        };
        assert_eq!(vec.len(), 1, "every item should be properly parsed");
    }
}
