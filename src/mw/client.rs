use crate::mw::entities::FoundWords;
use dom_query::Document;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct MerriamWebsterClient {
    client: Client,
}

impl MerriamWebsterClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn every_definition_of(mask: &String) -> String {
        format!(
            "https://www.merriam-webster.com/wordfinder/fill-in-blanks/all/{:}/{:}/1",
            mask.len(),
            mask
        )
    }

    async fn find_every_definition_of(&self, mask: &String) -> anyhow::Result<Vec<String>> {
        let html = self.client.find(Self::every_definition_of(mask)).await?;
        Document::find_words(html)
    }

    fn common_definitions_of(mask: &String) -> String {
        format!(
            "https://www.merriam-webster.com/wordfinder/fill-in-blanks/common/{:}/{:}/1",
            mask.len(),
            mask
        )
    }
    async fn find_common_definitions_of(&self, mask: &String) -> anyhow::Result<Vec<String>> {
        let html = self.client.find(Self::common_definitions_of(mask)).await?;
        Document::find_words(html)
    }

    pub async fn find(&self, mask: &String) -> anyhow::Result<FoundWords> {
        let (possible, common) = tokio::join!(
            self.find_every_definition_of(mask),
            self.find_common_definitions_of(mask),
        );
        Ok(FoundWords {
            possible: possible
                .inspect_err(|err| {
                    log::error!("MW::Possible: {}", err);
                })
                .unwrap_or_default(),
            common: common
                .inspect_err(|err| {
                    log::error!("MW::Common definitions: {}", err);
                })
                .unwrap_or_default(),
        })
    }
}

impl Default for MerriamWebsterClient {
    fn default() -> Self {
        Self::new()
    }
}

trait MerriamWebsterAccessor {
    async fn find(&self, url: String) -> anyhow::Result<String>;
}

impl MerriamWebsterAccessor for Client {
    async fn find(&self, url: String) -> anyhow::Result<String> {
        Ok(self.get(url).send().await?.text().await?)
    }
}

trait MerriamWebsterParser {
    fn find_words(html: String) -> anyhow::Result<Vec<String>>;
}

impl MerriamWebsterParser for Document {
    fn find_words(html: String) -> anyhow::Result<Vec<String>> {
        let html = Document::try_from(html)?;
        let elems = html
            .try_select("body ul.paginated-list-results li")
            .ok_or_else(|| anyhow::anyhow!("Could not find required html element"))?;
        let words = elems.iter().map(|li| li.text().to_string()).collect();
        Ok(words)
    }
}

#[cfg(test)]
mod tests {
    use crate::mw::client::MerriamWebsterParser;
    use dom_query::Document;

    #[test]
    fn parsing_words_works() {
        let response = r#"
<html lang="en" style="--toupee-banner-height: 40px;"><head>
        <meta http-equiv="x-ua-compatible" content="ie=edge">
            <meta name="referrer" content="unsafe-url">
            <meta property="fb:app_id" content="178450008855735">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="preconnect" href="https://www.google-analytics.com" crossorigin="">
            <link rel="preconnect" href="https://merriam-webster.com/assets">
            <link rel="preconnect" href="https://ajax.googleapis.com">
            <link rel="dns-prefetch" href="https://telemetry.art19.com">
            <link rel="dns-prefetch" href="https://entitlements.jwplayer.com">



            <title>6-Letter Words That Include AE | Merriam-Webster</title>
            <meta name="description" content="6-Letter Words Including AE: abased, abases, abated, abater, abates, abazes, abeles, abided, abider, abides, abodes, aboves">

            <meta name="robots" content="noindex,follow">
            <link rel="search" type="application/opensearchdescription+xml" href="/opensearch/dictionary.xml" title="Merriam-Webster Dictionary">


            <meta property="og:title" content="6-Letter Words That Include AE">
            <meta property="og:image" content="https://merriam-webster.com/assets/images/social-media/mwlogo_245x245.png">
            <meta property="og:url" content="https://www.merriam-webster.com/wordfinder/fill-in-blanks/all/6/a___e_/1">
            <meta property="og:description" content="6-Letter Words Including AE: abased, abases, abated, abater, abates, abazes, abeles, abided, abider, abides, abodes, aboves">
            <meta property="og:type" content="article">

            <meta name="twitter:title" content="6-Letter Words That Include AE">
            <meta name="twitter:image" content="https://merriam-webster.com/assets/images/social-media/mwlogo_245x245.png">
            <meta name="twitter:url" content="https://www.merriam-webster.com/wordfinder/fill-in-blanks/all/6/a___e_/1">
            <meta name="twitter:description" content="6-Letter Words Including AE: abased, abases, abated, abater, abates, abazes, abeles, abided, abider, abides, abodes, aboves">
            <meta name="twitter:card" content="summary">
            <meta name="twitter:site" content="@MerriamWebster">

            <link rel="icon" href="/favicon.svg">
            <link rel="mask-icon" href="/safari-pinned-tab.svg" color="305f7a">
            <link rel="apple-touch-icon" sizes="144x144" href="/apple-touch-icon.png">
            <link rel="manifest" href="/site.webmanifest">
            <meta name="msapplication-TileColor" content="\#2b5797">
            <meta name="theme-color" content="\#0f3850">



            <link rel="stylesheet" as="style" onload="this.onload=null;this.rel='stylesheet'" href="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/compiled/css/style-fonts.a085860e079f5453cb3f.css">


            <link rel="stylesheet" href="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/compiled/css/style-wordfinder-search-results.433c40e9ea6a8de1f1d1.css" media="screen">
            <style type="text/css">.adthrive-ad{margin-top:10px;margin-bottom:10px;text-align:center;overflow-x:visible;clear:both;line-height:0}</style><meta http-equiv="origin-trial" content="AlK2UR5SkAlj8jjdEc9p3F3xuFYlF6LYjAML3EOqw1g26eCwWPjdmecULvBH5MVPoqKYrOfPhYVL71xAXI1IBQoAAAB8eyJvcmlnaW4iOiJodHRwczovL2RvdWJsZWNsaWNrLm5ldDo0NDMiLCJmZWF0dXJlIjoiV2ViVmlld1hSZXF1ZXN0ZWRXaXRoRGVwcmVjYXRpb24iLCJleHBpcnkiOjE3NTgwNjcxOTksImlzU3ViZG9tYWluIjp0cnVlfQ=="><meta http-equiv="origin-trial" content="Amm8/NmvvQfhwCib6I7ZsmUxiSCfOxWxHayJwyU1r3gRIItzr7bNQid6O8ZYaE1GSQTa69WwhPC9flq/oYkRBwsAAACCeyJvcmlnaW4iOiJodHRwczovL2dvb2dsZXN5bmRpY2F0aW9uLmNvbTo0NDMiLCJmZWF0dXJlIjoiV2ViVmlld1hSZXF1ZXN0ZWRXaXRoRGVwcmVjYXRpb24iLCJleHBpcnkiOjE3NTgwNjcxOTksImlzU3ViZG9tYWluIjp0cnVlfQ=="><meta http-equiv="origin-trial" content="A9nrunKdU5m96PSN1XsSGr3qOP0lvPFUB2AiAylCDlN5DTl17uDFkpQuHj1AFtgWLxpLaiBZuhrtb2WOu7ofHwEAAACKeyJvcmlnaW4iOiJodHRwczovL2RvdWJsZWNsaWNrLm5ldDo0NDMiLCJmZWF0dXJlIjoiQUlQcm9tcHRBUElNdWx0aW1vZGFsSW5wdXQiLCJleHBpcnkiOjE3NzQzMTA0MDAsImlzU3ViZG9tYWluIjp0cnVlLCJpc1RoaXJkUGFydHkiOnRydWV9"><meta http-equiv="origin-trial" content="A93bovR+QVXNx2/38qDbmeYYf1wdte9EO37K9eMq3r+541qo0byhYU899BhPB7Cv9QqD7wIbR1B6OAc9kEfYCA4AAACQeyJvcmlnaW4iOiJodHRwczovL2dvb2dsZXN5bmRpY2F0aW9uLmNvbTo0NDMiLCJmZWF0dXJlIjoiQUlQcm9tcHRBUElNdWx0aW1vZGFsSW5wdXQiLCJleHBpcnkiOjE3NzQzMTA0MDAsImlzU3ViZG9tYWluIjp0cnVlLCJpc1RoaXJkUGFydHkiOnRydWV9"><meta http-equiv="origin-trial" content="A1S5fojrAunSDrFbD8OfGmFHdRFZymSM/1ss3G+NEttCLfHkXvlcF6LGLH8Mo5PakLO1sCASXU1/gQf6XGuTBgwAAACQeyJvcmlnaW4iOiJodHRwczovL2dvb2dsZXRhZ3NlcnZpY2VzLmNvbTo0NDMiLCJmZWF0dXJlIjoiQUlQcm9tcHRBUElNdWx0aW1vZGFsSW5wdXQiLCJleHBpcnkiOjE3NzQzMTA0MDAsImlzU3ViZG9tYWluIjp0cnVlLCJpc1RoaXJkUGFydHkiOnRydWV9"><script src="https://securepubads.g.doubleclick.net/pagead/managed/js/gpt/m202512020101/pubads_impl.js" async=""></script><link href="https://securepubads.g.doubleclick.net/pagead/managed/dict/m202512040101/gpt" rel="compression-dictionary"><style>.adthrive-act25-modal{z-index:2147483647;background-color:#0006;width:100%;height:100%;display:none;position:fixed;top:0;left:0;overflow:auto}.adthrive-act25-modal.show{display:block}.adthrive-act25-modal-content{background-color:#fefefe;border:1px solid #888;border-radius:10px;width:80%;max-width:592px;margin:auto;padding:20px 24px 24px;font-family:Verdana,Geneva,Tahoma,sans-serif;position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);box-shadow:0 0 10px #00000080}.adthrive-act25-modal-close{color:#000;font-size:28px;font-weight:700;position:fixed;top:-10px;right:3px}.adthrive-act25-modal-close:hover,.adthrive-act25-modal-close:focus{color:#000;cursor:pointer;text-decoration:none}.adthrive-act25-modal-header{padding:2px 16px}.adthrive-act25-modal-header h1{color:#000000de;font-size:20px;line-height:26px}.adthrive-act25-modal-body{max-height:50vh;margin-bottom:10px;padding:10px 16px;position:relative;overflow-y:auto}.adthrive-act25-modal-body p{font-size:14px;line-height:20px}.adthrive-act25-modal-footer{color:#fff;flex-direction:column;justify-content:space-between;padding:2px 16px;display:flex}.adthrive-act25-modal-accept,.adthrive-act25-modal-decline{color:#fff;cursor:pointer;border:none;border-radius:10px}.adthrive-act25-modal-accept{background-color:#010044}.adthrive-act25-modal-decline{color:#000;background-color:#fff}.adthrive-act25-modal-accept,.adthrive-act25-modal-decline{text-transform:uppercase;flex:1;margin:0 10px 10px 0}.adthrive-act25-modal-accept:hover,.adthrive-act25-modal-accept:focus{color:#c4c4c4;background-color:#010044}.adthrive-act25-modal-decline:hover,.adthrive-act25-modal-decline:focus{color:#000;background-color:#fff}@media (width<=600px){.adthrive-act25-modal-content{width:90%}.adthrive-act25-modal-body{border-bottom:1px solid #c4c4c4;box-shadow:inset 0 -100px 30px -100px #0000001a}.adthrive-act25-modal-accept,.adthrive-act25-modal-decline{font-size:14px;line-height:14px}}@media (width>=600px){.adthrive-act25-modal-footer{flex-direction:row}.adthrive-act25-modal-accept,.adthrive-act25-modal-decline{flex:none;margin:0}}.adthrive-act25-footer{margin-bottom:30px}.adthrive-act25-footer-text{color:#a9a9a9;font-size:14px}.adthrive-act25-footer-text p{color:#a9a9a9;text-decoration:underline}.adthrive-act25-footer-link{cursor:pointer;font-size:14px;text-decoration:underline}.adthrive-ad{text-align:center;clear:both;margin-top:10px;margin-bottom:10px;line-height:0;overflow-x:visible}.adthrive-ad-cls{flex-wrap:wrap;justify-content:center;align-items:center;display:flex}.adthrive-ad-cls>div,.adthrive-ad-cls>iframe{flex-basis:100%}.adthrive-interstitial{margin-top:0;margin-bottom:0}.adthrive-native-recipe{display:inline-block}.adthrive-recipe{z-index:1;min-width:300px;position:relative}.adthrive-footer-mobile,.adthrive-header-mobile,.adthrive-footer-phone,.adthrive-header-phone{min-height:50px}.adthrive-footer-desktop,.adthrive-header-desktop,.adthrive-header-tablet,.adthrive-footer-tablet{min-height:90px}.adthrive-content,.adthrive-recipe,.adthrive-sidebar,.adthrive-below-post{min-height:250px}.adthrive-sidebar9-fallback{flex-direction:column;justify-content:flex-start;align-items:center;margin:10px 0;display:flex}.adthrive-sidebar9-fallback>div{flex-basis:unset;top:5px;position:sticky!important}.adthrive-fit-content .adthrive-content{min-height:250px;width:fit-content!important;min-width:300px!important;max-width:fit-content!important;margin-left:auto!important;margin-right:auto!important}.adthrive-fit-content .adthrive-ad.adthrive-content:has(.raptive-sales-mural){width:100%!important;max-width:100%!important}.adthrive-device-desktop .adthrive-recipe,.adthrive-device-tablet .adthrive-recipe{max-width:320px}.adthrive-stuck.adthrive-sticky.adthrive-sidebar,.adthrive-stuck.adthrive-sticky.adthrive-header{z-index:9999;position:fixed;top:0}.adthrive-stuck.adthrive-header{margin-top:0}.adthrive-stuck.adthrive-sticky-outstream{z-index:2147483644;display:none;position:fixed}.adthrive-sticky.adthrive-footer{text-align:center;z-index:1000001;box-sizing:content-box;background-color:#fffc;border-top:2px solid #e1e1e1cc;width:100%;max-height:100px;margin:0;position:fixed;bottom:0;left:0;overflow:hidden}.adthrive-sticky.adthrive-footer>.adthrive-close{cursor:pointer;color:#b2b2b2;background:#fff;border:1px solid #b2b2b2;border-radius:20px;padding:0 5px;font-family:Arial,sans-serif;font-size:20px;line-height:20px;display:inline-block;position:absolute;top:5px;right:5px}.adthrive-device-desktop .adthrive-sticky.adthrive-footer>.adthrive-close{top:10px;right:10px}.adthrive-footer-message,.adthrive-privacy-preferences,.adthrive-ccpa-link{text-align:center;margin-top:5px}.adthrive-footer>div,.adthrive-header>div{max-width:1200px}.raptive-player-reset>*{box-sizing:border-box;float:none;clear:both;box-shadow:none;font-variant:normal;letter-spacing:normal;overflow-wrap:normal;text-align:left;text-transform:none;word-spacing:normal;white-space:normal;color:inherit;direction:ltr;background-color:#0000;border:0;margin:0;padding:0;font-family:Arial,Helvetica,sans-serif;font-size:16px;font-style:normal;font-weight:400;font-stretch:100%;line-height:1.2;text-decoration:none;display:block}#next-video,#stay-video{opacity:.9;text-align:center;cursor:pointer;color:#fff;text-transform:capitalize;background-color:#333;width:100px;height:40px;margin:5px;line-height:40px;position:relative}.adthrive-collapse-mobile #next-video,.adthrive-collapse-mobile #stay-video,.adthrive-player-position-mobile #next-video,.adthrive-player-position-mobile #stay-video,.raptive-player-container-mobile #next-video,.raptive-player-container-mobile #stay-video{width:75px;height:30px;line-height:30px}#next-stay-container{z-index:1;flex-flow:column;justify-content:center;margin:12px;display:flex;position:absolute;top:0;bottom:0;right:0}.adthrive-collapse-mobile #next-stay-container,.adthrive-player-position-mobile #next-stay-container,.raptive-player-container-mobile #next-stay-container{margin:0}#next-video:before{content:"";z-index:1;will-change:transform,opacity;transform-origin:0;background-color:red;width:100%;height:100%;display:block;position:absolute}#next-stay-container.active>#next-video:before{animation:5s linear scaleInFromLeft}#next-video span{z-index:2;position:relative}@keyframes scaleInFromLeft{0%{opacity:0;transform:scaleX(0)}to{opacity:1;transform:scaleX(1)}}.video-box-shadow{box-shadow:0 0 10px #00000080}.adthrive-wrapper-bar{background-color:#595959;border-top-left-radius:5px;border-top-right-radius:5px;height:36px;font-family:Arial,Helvetica,sans-serif;display:none}.adthrive-video-title{color:#fff;font-size:13px;font-weight:700;text-decoration:none}.adthrive-wrapper-title-wrapper{min-width:0;box-shadow:none;border:none;justify-content:center;align-items:center;margin-left:10px;margin-right:10px;display:inline-flex}.adthrive-wrapper-title-wrapper>a>svg,.adthrive-wrapper-title-wrapper>div>svg{vertical-align:middle;fill:#fff;margin-right:5px}.adthrive-wrapper-title-wrapper>a{text-decoration:none}.adthrive-video-text-cutoff{text-overflow:ellipsis;white-space:nowrap;color:#fff;overflow:hidden}.adthrive-collapse-player{border-top:1px solid #dbdbdb;border-bottom:1px solid #dbdbdb;width:90%;margin-left:auto;margin-right:auto;padding:8px!important}.adthrive-outstream-container{margin:0 auto;position:relative;flex-basis:unset!important}.adthrive-player-idle{transition:all .5s ease-out .5s;display:none}.raptive-player-ad-container-idle{display:none}.adthrive-player-playing{transition:all .5s ease-in .5s;display:block}.adthrive-sticky-outstream-active.adthrive-new-outstream-player{opacity:1;animation-name:adthrive-fade-in;animation-duration:.5s;animation-timing-function:ease-in;animation-iteration-count:1}.adthrive-sticky-outstream-idle.adthrive-new-outstream-player{opacity:0;animation-name:adthrive-fade-out;animation-duration:.5s;animation-timing-function:ease-out;animation-iteration-count:1;display:block}@keyframes adthrive-fade-in{0%{opacity:0}to{opacity:1}}@keyframes adthrive-fade-out{0%{opacity:1}to{opacity:0}}.adthrive-player-ad-controls-play{z-index:1001;margin:-5px 4px}.adthrive-player-icon-play>svg{margin-left:4px}.adthrive-player-ad-controls-pause{justify-content:center;display:flex}.adthrive-player-ad-controls-volume-container{z-index:1001;border-radius:36px;flex-direction:column-reverse;height:36px;display:flex;position:relative}.adthrive-player-ad-controls-volume-container-with-bar{top:-5px}.adthrive-player-ad-controls-volume-container>.adthrive-player-ad-controls-volume{flex-direction:column-reverse;height:55px;display:flex;position:relative}.adthrive-player-ad-controls>.adthrive-player-ad-controls-volume-container,.adthrive-player-ad-controls-volume>.adthrive-player-icon-volume-muted,.adthrive-player-ad-controls-volume>.adthrive-player-icon.adthrive-player-icon-volume-unmuted,.adthrive-player-ad-controls-play>.adthrive-player-icon.adthrive-player-icon-pause,.adthrive-player-ad-controls-volume>.adthrive-player-volume-control{margin-block-start:0}.adthrive-player-icon-volume-muted,.adthrive-player-icon-volume-unmuted{z-index:2}.adthrive-player-volume-control{z-index:1}.adthrive-player-ad-controls-play,.adthrive-player-ad-controls-volume{border-radius:36px;width:36px;height:36px;display:inline-block}.adthrive-player-ad-controls-play:hover,.adthrive-player-ad-controls-volume-container:hover,.adthrive-player-icon-fullscreen:hover,.adthrive-player-icon-fullscreen:focus{background:#0000004d}.adthrive-player-ad-controls-volume-container-with-slider:hover{background:0 0}.adthrive-player-ad-controls-volume-container.active{z-index:1001}.adthrive-player-icon{cursor:pointer;background:0 0;border:none;border-radius:100%;justify-content:center;align-items:center;width:36px;height:36px;margin-right:4px;padding:0;display:flex}.adthrive-hidden{display:none!important}.adthrive-count-down{z-index:1;pointer-events:none;color:#fff;text-shadow:0 0 1px #000,0 0 1px #000,0 0 1px #000,0 0 1px #000;background:#00000080;border-bottom-right-radius:4px;outline:none;width:auto;height:auto;padding:4px;font-size:12px;line-height:1.2;position:absolute;top:0;left:0}.adthrive-collapse-medium>.adthrive-count-down,.adthrive-collapse-small>.adthrive-count-down{padding:2px}.adthrive-player-icon svg{pointer-events:none;filter:drop-shadow(3px 5px 3px #00000080)}.adthrive-player-ad-container{margin:0;position:absolute;bottom:0}.adthrive-player-ad-container-loading{background-color:#000;justify-content:center;align-items:center;height:100%;display:flex}.adthrive-player-ad-container{overflow:hidden}.adthrive-loading-spinner-container{z-index:2147483647;justify-content:center;align-items:center;position:absolute;inset:0}.adthrive-loading-spinner{border:3px solid #ffffff4d;border-top-color:#fff;border-radius:50%;width:50px;height:50px;-webkit-animation:1s ease-in-out infinite spin;animation:1s ease-in-out infinite spin}@-webkit-keyframes spin{to{-webkit-transform:rotate(360deg)}}@keyframes spin{to{-webkit-transform:rotate(360deg)}}.adthrive-title-overlay{box-sizing:border-box;z-index:1;color:#fff;background:#0006;border-radius:0;flex-direction:row;justify-content:flex-start;align-items:center;height:0;transition:background .3s ease-out;display:flex;position:absolute;top:0;left:0;right:0}.adthrive-title-overlay.active{height:36px}.adthrive-title-overlay>.adthrive-wrapper-title-wrapper{flex-direction:row-reverse;align-items:center;width:100%;height:100%;text-decoration:none;display:none;overflow:hidden}.adthrive-title-overlay.active>.adthrive-wrapper-title-wrapper{display:flex}.raptive-player-ad-loading>.adthrive-title-overlay,.adthrive-title-overlay.active>.adthrive-wrapper-title-wrapper:after{display:none}.adthrive-wrapper-bar a[target=_blank]:after{display:none!important}.adthrive-no-pointer-events{pointer-events:none}.adthrive-display-none{display:none}.adthrive-small-text{font-size:13px}.adthrive-video-title-description-container{flex-direction:column;flex:1;height:100%;font-size:1.2em;line-height:1.1em;display:flex}#adthrive-video-title-text{flex:1;align-items:center;display:flex}.adthrive-player-position-mobile #adthrive-video-title-text,.raptive-player-container-mobile #adthrive-video-title-text{font-size:.85em}#adthrive-video-description-text{text-overflow:ellipsis;white-space:nowrap;flex:1;align-items:center;margin:-4px 0 0;font-size:1vw;display:flex;overflow:hidden}.adthrive-video-play-button{cursor:pointer;z-index:1;border-top:30px solid #0000;border-bottom:30px solid #0000;border-left:60px solid #fff;width:0;height:0}.raptive-player-position{clear:both;background-color:#000;width:100%;padding-top:56.25%;position:relative}.raptive-player-container{z-index:1;height:100%;position:absolute;inset:0}.raptive-player-hidden{display:none}.raptive-player-video{width:100%;height:100%;margin-block:0!important}.raptive-player-collapse{z-index:2147483644;background-color:#000;max-height:169px}.adthrive-video-overlay{cursor:pointer;z-index:4;background-color:#00000080;justify-content:center;align-items:center;margin-block-start:unset;display:flex;position:absolute;inset:0}.adthrive-player-big-play-button{color:#6b65ff;font-size:48px}@media (width<=480px){.adthrive-player-big-play-button{font-size:44px}}@media (width>=481px) and (width<=768px){.adthrive-player-big-play-button{font-size:77px}}@media (width>=769px){.adthrive-player-big-play-button{font-size:88px}}.adthrive-sticky-outstream.adthrive-sticky-outstream-mobile .adthrive-player-ad-container{margin-top:2px;margin-bottom:2px}.adthrive-player-ad-controls.adthrive-player-ad-controls-hidden{pointer-events:none;opacity:0!important;transition:opacity .5s!important}.adthrive-player-ad-controls{text-align:left;z-index:1000000;opacity:1;width:100%;height:50px;padding:0;line-height:18px;transition:opacity .5s ease-out,transform .2s ease-out;display:flex;position:absolute;bottom:0;left:0;transform:translateY(12px)}.adthrive-full-width{width:100%}.adthrive-player-ad-controls.adthrive-player-progress-bar-visible{transform:translateY(0)}.adthrive-player-progress-bar-container{cursor:pointer;z-index:1000;pointer-events:all;width:96%;height:20px;display:none;position:absolute;bottom:0;left:2%}.adthrive-collapse-mobile .adthrive-player-progress-bar-container{width:94%;left:3%}.adthrive-player-progress-bar-container.active{display:initial}.adthrive-player-progress-bar{background-color:#ccc;border-radius:36px;width:100%;height:6px;margin:auto 0;position:absolute;top:0;bottom:0;overflow:hidden}.adthrive-player-progress-bar div{transform-origin:0;content:"";background-color:#6b65ff;width:100%;height:100%;display:block}.adthrive-player-progress-bar-handle{touch-action:none;z-index:1001;background-color:#6b65ff;border-radius:50%;width:12px;height:12px;position:absolute;top:50%;left:0;transform:translate(-50%,-50%)scale(1);box-shadow:0 0 10px #0006}.adthrive-player-progress-bar-handle:before{content:"";width:20px;height:20px;display:block;transform:translate(-4px,-4px)}.adthrive-player-volume-control{opacity:0;z-index:1;transform-origin:18px;background:#00000080;border-radius:36px;justify-content:center;align-items:center;width:120px;height:36px;margin:auto;padding-left:30px;display:flex;position:absolute;transform:rotate(270deg)}.adthrive-player-volume-control.adPlaying{top:20px}.adthrive-player-volume-control input[type=range]{-webkit-appearance:none;vertical-align:bottom;cursor:pointer;width:65px;height:10px;box-shadow:none;z-index:1001;background-color:#777;border:0;border-radius:20px;outline:0;margin:0;padding:0;position:relative;overflow:hidden}.adthrive-player-volume-control input[type=range]::-webkit-slider-runnable-track{-webkit-appearance:none;color:#777;height:10px;margin-top:-1px}.adthrive-player-volume-control input[type=range]::-webkit-slider-thumb{-webkit-appearance:none;cursor:pointer;background:#fff;border-radius:50%;width:10px;height:10px;position:relative;top:.5px;box-shadow:-325px 0 0 320px #6b65ff,inset 0 0 0 40px #fff}.adthrive-player-volume-control input[type=range]:active::-webkit-slider-thumb{background:#fff;box-shadow:-325px 0 0 320px #6b65ff,inset 0 0 0 3px #fff}.adthrive-player-volume-control input[type=range]::-moz-range-thumb{width:10px;height:10px}.adthrive-player-volume-control input[type=range]::-moz-range-progress{background-color:#ddd;box-shadow:-325px 0 0 320px #6b65ff,inset 0 0 0 3px #6b65ff}.adthrive-player-volume-control input[type=range]::-moz-range-track{background-color:#777}.adthrive-player-volume-control input[type=range]::-ms-fill-lower{background-color:#ddd;box-shadow:-325px 0 0 320px #6b65ff,inset 0 0 0 3px #6b65ff}.adthrive-player-volume-control input[type=range]::-ms-fill-upper{background-color:#777}.adthrive-jw-player-collapse{z-index:2147483644}.adthrive-player-position.adthrive-collapse-float,.raptive-player-container.adthrive-collapse-float{position:fixed;width:300px!important}.adthrive-player-position.adthrive-collapse-float #adthrive-video-description-text,.raptive-player-container.adthrive-collapse-float #adthrive-video-description-text{display:none}.adthrive-player-position.adthrive-collapse-float.adthrive-collapse-right,.raptive-player-container.adthrive-collapse-float.adthrive-collapse-right{inset:0 5px auto auto}.adthrive-player-position.adthrive-collapse-float.adthrive-collapse-bottom-right,.raptive-player-container.adthrive-collapse-float.adthrive-collapse-bottom-right{inset:auto 5px 100px auto}.adthrive-player-position.adthrive-collapse-float.adthrive-collapse-bottom-left,.raptive-player-container.adthrive-collapse-float.adthrive-collapse-bottom-left{top:auto;bottom:100px;left:auto}.adthrive-player-position.adthrive-collapse-float>.adthrive-player-title,.raptive-player-container.adthrive-collapse-float>.adthrive-player-title{display:none}.adthrive-player-position.adthrive-collapse-sticky,.raptive-player-container.adthrive-collapse-sticky{z-index:9999;padding-top:20px;padding-bottom:20px;position:fixed}.adthrive-player-position.adthrive-collapse-sticky>.adthrive-player-title,.raptive-player-container.adthrive-collapse-sticky>.adthrive-player-title{display:none}.adthrive-sticky-outstream.adthrive-sticky-outstream-top-center.adthrive-sticky-outstream-mobile{top:0;bottom:auto!important;transform:translate(0)!important}body.adthrive-device-phone .adthrive-sticky-outstream.adthrive-sticky-outstream-active.adthrive-sticky-outstream-top-center{bottom:auto!important}.adthrive-collapse-mobile-background{z-index:99990;position:fixed;top:0;left:0}.adthrive-collapse-mobile-background.extra-height{height:163px!important}.adthrive-top-collapse-close{z-index:1;position:fixed;top:5px;left:-30px}.adthrive-sticky-outstream-top-center>.adthrive-top-collapse-close{position:fixed;top:48px;left:10px}.adthrive-sticky-outstream-top-center.adthrive-sticky-outstream-mobile>.adthrive-top-collapse-close{top:10px}.adthrive-top-collapse-wrapper-bar>* .adthrive-top-collapse-close{position:relative;top:0;left:0}.adthrive-top-collapse-wrapper-bar>* .adthrive-wrapper-float-close{float:none;margin-bottom:0;display:none}.adthrive-top-collapse-close-spacer{line-height:1.2}.adthrive-player-position.adthrive-collapse-mobile,.raptive-player-container.adthrive-collapse-mobile{z-index:99998;width:300px;position:fixed}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-small,.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-medium{width:178px}.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-small,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-medium{width:178px;height:101px;left:unset;right:unset;bottom:unset;top:unset}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-right,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-right{top:26px;right:10px}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-left,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-left{top:26px;left:5px}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-bottom-left,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-bottom-left{bottom:52px;left:5px}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-bottom-right,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-bottom-right{bottom:52px;right:10px}.adthrive-player-position.adthrive-collapse-mobile>.adthrive-player-title,.raptive-player-container.adthrive-collapse-mobile>.adthrive-player-title{display:none}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-center>.adthrive-wrapper-bar,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-center>.adthrive-wrapper-bar{display:none!important}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-center>.adthrive-top-collapse-wrapper-bar,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-center>.adthrive-top-collapse-wrapper-bar{display:block}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-center.adthrive-player-without-wrapper-text,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-center.adthrive-player-without-wrapper-text{inset:0 auto auto 50%;transform:translate(-50%);padding-top:0!important;padding-bottom:1px!important}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-center.adthrive-player-with-wrapper-text,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-center.adthrive-player-with-wrapper-text{inset:0 5px auto auto;padding-top:0!important;padding-bottom:1px!important}.adthrive-player-position.adthrive-collapse-mobile.adthrive-collapse-top-center,.raptive-player-container.adthrive-collapse-mobile.adthrive-collapse-top-center{transition:none!important}.adthrive-top-collapse-wrapper-bar{color:#fff;z-index:99998;width:-webkit-calc(100% - 178px - 10px);width:-moz-calc(100% - 178px - 10px);width:calc(100% - 188px);padding-top:5px;padding-bottom:5px;display:none;position:fixed;top:0;left:5px}.adthrive-top-collapse-wrapper-video-title{color:#fff;text-overflow:ellipsis;-webkit-line-clamp:3;line-clamp:3;-webkit-box-orient:vertical;font-size:13px;font-weight:700;text-decoration:none;display:-webkit-box;overflow:hidden}.adthrive-top-collapse-wrapper-bar a a.adthrive-learn-more-link{font-size:13px;display:inline-block}.adthrive-top-collapse-wrapper-bar a a.raptive-player-learn-more-link{display:none}.adthrive-top-collapse-wrapper-bar a a.raptive-player-learn-more-link.raptive-player-learn-more-link-active{display:inline-block}h3.adthrive-player-title{margin:10px 0}.adthrive-wrapper-close{color:#fff;justify-content:center;align-items:center;min-width:36px;height:36px;margin-left:auto;margin-right:0;font-size:36px}.adthrive-wrapper-float-close{float:right;cursor:pointer;margin-bottom:5px;display:none}.adthrive-top-left-outer{width:78px;height:78px;position:absolute;top:-55px;left:-30px}.adthrive-top-right-outer{width:78px;height:78px;position:absolute;top:-55px;right:-30px}.adthrive-top-left-outer>svg{position:absolute;bottom:30px;left:30px}.adthrive-top-right-outer>svg{position:absolute;bottom:30px;right:30px}.adthrive-top-left-inner{position:absolute;top:0}.adthrive-top-left-inner.adthrive-wrapper-float-close{color:#fff;pointer-events:none;z-index:99;background:#00000080;width:100%;padding:2px}.adthrive-new-outstream-player .adthrive-top-left-inner.adthrive-wrapper-float-close{z-index:10000}.adthrive-new-outstream-player .adthrive-ad{margin-top:0;margin-bottom:0}.adthrive-top-left-inner-wrapper.adthrive-wrapper-float-close{color:#fff;pointer-events:none;z-index:10000;background:#00000080;width:100%;padding:2px;position:absolute;top:36px}.adthrive-sticky-outstream>.adthrive-wrapper-float-close.adthrive-wrapper-close-outside-left{left:2px}.adthrive-sticky-outstream>.adthrive-wrapper-float-close.adthrive-wrapper-close-bkgd-50{color:#fff;pointer-events:none;background:#00000080;padding:2px 0;top:0;left:0}.adthrive-new-outstream-player{margin-bottom:20px;padding:0!important}.adthrive-new-outstream-player>.adthrive-wrapper-float-close.adthrive-wrapper-close-bkgd-50{top:0}.adthrive-close-in-container{line-height:0;position:relative}#adthrive-sticky-outstream-close.adthrive-top-collapse-close{position:inherit}.adthrive-sticky-outstream-idle>.adthrive-close-in-container{pointer-events:none}.adthrive-sticky-outstream-active>.adthrive-close-in-container{pointer-events:all}.adthrive-wrapper-close-bkgd-50>.adthrive-close-in-container{pointer-events:all;padding:2px 0 0 4px}.adthrive-sticky-outstream.adthrive-sticky-outstream-mobile,.raptive-player-container.raptive-player-collapse.raptive-player-sticky-bottom-right,.raptive-player-container.raptive-player-collapse.raptive-player-sticky-bottom-left{transition-property:transform;transition-duration:.25s;transition-delay:0s;bottom:52px}.adthrive-sticky-outstream.adthrive-sticky-outstream-mobile-right,.raptive-player-container.raptive-player-collapse.raptive-player-sticky-bottom-right{right:10px}.adthrive-sticky-outstream.adthrive-sticky-outstream-mobile-left,.raptive-player-container.raptive-player-collapse.raptive-player-sticky-bottom-left{left:10px;transform:none!important}.adthrive-sticky-outstream.adthrive-sticky-outstream-active.adthrive-sticky-outstream-top-center{background:#000;inset:0 0 auto;transform:none}.adthrive-sticky-outstream.adthrive-sticky-outstream-desktop{bottom:100px;right:5px}.adthrive-sticky-outstream.adthrive-sticky-outstream-top-center.adthrive-sticky-outstream-desktop,.adthrive-sticky-outstream.adthrive-sticky-outstream-top-center.adthrive-sticky-outstream-mobile{bottom:auto}.adthrive-sticky-outstream.adthrive-sticky-outstream-top-center.adthrive-sticky-outstream-mobile .adthrive-stickyoutstream-container{background-color:#000;margin:0 auto}.adthrive-sticky-outstream.adthrive-sticky-outstream-active{display:block}.adthrive-video-stickyoutstream{z-index:1;margin-top:0;position:relative}.adthrive-video-stickyoutstream div:first-of-type{margin:-1px}.adthrive-video-stickyoutstream-new-player div:first-of-type{margin:0}.adthrive-video-stickyoutstream-new-player>div:first-child{position:absolute}.adthrive-video-close{pointer-events:all}.adthrive-video-close-float-left{float:left}.adthrive-video-close-float-right{float:right}.adthrive-footer-message span,.adthrive-privacy-preferences a,.adthrive-ccpa-link,.adthrive-ccpa-link span{color:#a9a9a9;font-family:Arial,Helvetica Neue,Helvetica,sans-serif;font-size:13px}.adthrive-ccpa-link a{cursor:pointer;text-decoration:underline}.adthrive-device-phone .adthrive-footer-message{margin-bottom:60px}.adthrive-footer-message{margin-bottom:100px}.adthrive-footer-message>span{text-transform:uppercase;border-top:1px solid #b2b2b2;padding-top:5px}.adthrive-parallax-slot{height:400px;padding-top:5px;overflow:hidden}.adthrive-parallax-ad{transition:transform .3s ease-out;transform:translateY(0)}.adthrive-sticky-container.adthrive-parallax-slot{flex-direction:row;justify-content:center;padding-top:10px}#_inv_voicefive___{display:none}.adthrive-ad-debug{background-color:#ff000080!important;outline:1px solid red!important}.adthrive-ad-debug:hover{cursor:pointer;background-color:#f00c!important;outline:1px solid red!important}.adthrive-player-ad-controls-volume:hover>.adthrive-player-volume-control,.adthrive-player-volume-control:focus{opacity:1}@media print{div[data-gg-moat],body[data-gg-moat],iframe[data-gg-moat-ifr],div[class*=kargo-ad],#ogy-ad-slot,.adthrive-sticky-outstream,.adthrive-ad,.adthrive-comscore,.adthrive-native-recipe,.raptive-sales{visibility:hidden;width:0;height:0;display:none!important}}.raptive-player-captions{text-align:center;color:#fff;--raptive-player-captions-font-opacity:100%;--raptive-player-captions-background-opacity:0%;--raptive-player-captions-window-opacity:0%;border-radius:5px;width:90%;margin:0 auto;line-height:1.5;transition:transform .5s;display:none;position:absolute;bottom:60px;left:0;right:0;font-size:16px!important}.raptive-player-captions span{white-space:pre-wrap;background:#00000080;border-radius:5px;padding:2px 4px}.raptive-player-captions-button{border-style:unset;color:#fff;cursor:pointer;background-color:#0000;border-radius:9999px;width:36px;height:36px;margin:-5px 4px -5px auto;padding:0;font-size:.75em;font-weight:700;display:none}.raptive-player-captions-button:hover{background:#0000004d}.adthrive-full-width .raptive-player-captions-button.raptive-player-captions-button--visible{display:block}.adthrive-player-ad-controls-hidden+.raptive-player-captions{transform:translateY(40px)}.raptive-player-container-mobile .raptive-player-captions,.raptive-player-collapse .raptive-player-captions{font-size:10px!important}.raptive-player-container-mobile.raptive-player-collapse .raptive-player-captions{font-size:6px!important}.raptive-player-container-mobile.raptive-player-collapse .raptive-player-captions-button{display:none}.raptive-player-settings{color:#fff;z-index:999999999;background:#000;border-radius:5px;width:288px;height:192px;font-size:14px;display:none;position:absolute;bottom:60px;right:2%;text-transform:none!important;font-family:Arial,sans-serif!important;font-weight:400!important}.raptive-player-settings--active{display:block}.raptive-player-settings-header{box-sizing:content-box;background:#404040;border-radius:5px 5px 0 0;align-items:center;gap:4px;height:40px;padding:4px;display:flex}.raptive-player-settings-header>button{color:#fff;cursor:pointer;background:0 0;border:none;border-radius:5px;justify-content:center;align-items:center;width:40px;height:40px;padding:0;font-weight:700;position:relative}.raptive-player-settings-header>button.active{border-radius:5px 5px 0 0}.raptive-player-settings-header>button.active:after{content:"";background-color:#fff;width:100%;height:4px;position:absolute;bottom:-4px;left:0}.raptive-player-settings-header>button:hover{background:#000}.raptive-player-settings-header-title{margin-left:8px;font-weight:700}.raptive-player-settings--nested .raptive-player-settings-header-title{margin-left:0}.raptive-player-settings-body{scrollbar-color:#fff #646464;height:144px;overflow:scroll}.raptive-player-settings-body>div{display:none}.raptive-player-settings-body>div.raptive-player-settings-page--active{display:block}.raptive-player-settings-body::-webkit-scrollbar-thumb{background:#fff!important}.raptive-player-settings-close{margin:0 0 0 auto;display:flex}.raptive-player-settings-back{display:none}.raptive-player-settings-back>svg{margin-right:2px}.raptive-player-settings--nested .raptive-player-settings-back{display:flex}.raptive-player-settings-page{padding:4px}.raptive-player-settings-page>button:hover{background:#404040}.raptive-player-settings-page-item{cursor:pointer;background:unset;text-align:left;border:none;border-radius:5px;align-items:center;width:100%;margin:4px 0;padding:4px 8px;display:flex;color:#fff!important;letter-spacing:normal!important;text-transform:none!important;font-family:Arial,sans-serif!important;font-weight:400!important;line-height:1!important}.raptive-player-settings-page-item>span:nth-child(2){text-transform:capitalize;margin-left:auto}.raptive-player-settings-page-item>span:nth-child(3){display:flex}.raptive-player-settings-page-item--selected{background:#6b65ff!important}.raptive-player-settings-page-item--unavailable{display:none}.raptive-player-container-mobile .raptive-player-settings,.raptive-player-collapse .raptive-player-settings{border-radius:0;width:100%;height:100%;position:absolute;inset:0}.raptive-player-container-mobile .raptive-player-settings-body,.raptive-player-collapse .raptive-player-settings-body{height:auto;position:absolute;inset:48px 0 0}.raptive-player-container-mobile .raptive-player-settings-header,.raptive-player-collapse .raptive-player-settings-header{border-radius:0}.raptive-player-container-mobile.raptive-player-collapse .raptive-player-settings{display:none}.adthrive-player-icon-fullscreen .raptive-player-primary-icon,.raptive-player-fullscreen .adthrive-player-icon-fullscreen .raptive-player-secondary-icon{display:block}.adthrive-player-icon-fullscreen .raptive-player-secondary-icon,.raptive-player-fullscreen .adthrive-player-icon-fullscreen .raptive-player-primary-icon,.raptive-player-ios .adthrive-player-icon-fullscreen{display:none}.raptive-player-ios .adthrive-full-width .adthrive-player-icon-fullscreen{display:flex}.adthrive-player-ad-controls>.adthrive-player-icon-fullscreen,.adthrive-player-ad-controls.adthrive-full-width .raptive-player-captions-button.raptive-player-captions-button--visible+.adthrive-player-icon-fullscreen{margin:-5px 4px}.raptive-player-container-mobile .adthrive-player-ad-controls.adthrive-full-width>.adthrive-player-icon-fullscreen,.adthrive-player-ad-controls.adthrive-full-width .raptive-player-captions-button+.adthrive-player-icon-fullscreen{margin:-5px 4px -5px auto}.raptive-content-terms-footer-link{cursor:pointer;font-family:Arial,Helvetica Neue,Helvetica,sans-serif;font-size:13px;text-decoration:underline}.raptive-content-terms-modal{background:0 0;border:none;max-width:none;max-height:none;padding:0;overflow:visible}.raptive-content-terms-modal[open]{justify-content:center;align-items:center;margin:auto;display:flex;position:fixed;inset:0}.raptive-content-terms-modal:focus{outline:none}.raptive-content-terms-modal::backdrop{background-color:#00000080}.raptive-content-terms-modal-content{background-color:#fff;border-radius:8px;flex-direction:column;width:50vw;min-width:300px;max-width:800px;max-height:85vh;display:flex}.raptive-content-terms-modal-header{border-bottom:1px solid #e0e0e0;flex-shrink:0;justify-content:space-between;align-items:center;padding:16px 24px;display:flex}.raptive-content-terms-modal-title{color:#333;margin:0;font-size:20px;font-weight:600}.raptive-content-terms-modal-close{color:#757575;cursor:pointer;background:0 0;border:none;padding:0;font-size:24px}.raptive-content-terms-modal-close:hover{color:#000}.raptive-content-terms-modal-body{color:#555;padding:24px;overflow-y:auto}.raptive-content-terms-modal-body p{margin-top:0;margin-bottom:1em}.raptive-content-terms-modal-body p:last-child{margin-bottom:0}.raptive-content-terms-modal-body a{color:#007bff;text-decoration:none}.raptive-content-terms-modal-body a:hover{text-decoration:underline}@media (width<=768px){.raptive-content-terms-modal-content{width:90vw}}.raptive-player--native.adthrive-stickyoutstream-container{box-sizing:content-box;background:#fff;border:1px solid #000}.raptive-player--native #adthrive-stickyoutstream-ad-container{position:absolute;top:41px}.adthrive-ccpa-modal{z-index:2147483647;background-color:#0006;width:100%;height:100%;display:none;position:fixed;top:0;left:0;overflow:auto}.adthrive-ccpa-modal-content{background-color:#fefefe;border:1px solid #888;border-radius:10px;width:80%;max-width:592px;margin:0 auto;padding:20px 24px 24px;font-family:Verdana,Geneva,Tahoma,sans-serif;position:relative;top:50%;transform:translateY(-50%);box-shadow:0 0 10px #00000080}#adthrive-ccpa-modal-title{color:#000000de;font-size:20px;line-height:26px}.adthrive-ccpa-modal-btn:hover,.adthrive-ccpa-modal-btn:focus{color:#000;cursor:pointer;text-decoration:none}#adthrive-ccpa-modal-language{color:#000000de;margin:16px 0 32px;font-size:14px;line-height:20px;display:block}#adthrive-ccpa-modal-close-btn-container:hover,#adthrive-ccpa-modal-close-btn-container:focus,#adthrive-ccpa-modal-cancel-btn:hover,#adthrive-ccpa-modal-cancel-btn:focus{color:#000c;cursor:pointer;text-decoration:none}#adthrive-ccpa-modal-continue-btn:hover,#adthrive-ccpa-modal-continue-btn:focus{color:#fffc;cursor:pointer;text-decoration:none}#adthrive-ccpa-modal-close-btn-container{color:#000;font-size:20px;font-weight:700;line-height:20px;position:absolute;top:8px;right:8px}.adthrive-ccpa-lower-buttons-container{color:#000;font-size:18px}#adthrive-ccpa-modal-cancel-btn{text-align:left;width:calc(100% - 150px);display:inline-block}#adthrive-ccpa-modal-continue-btn{color:#fff;text-align:center;background-color:#010044;border-radius:10px;width:150px;height:44px;line-height:44px;display:inline-block}@media screen and (width<=896px){.adthrive-ccpa-modal-content{width:calc(100% - 80px);margin:0 auto;position:relative}#adthrive-ccpa-modal-title{font-size:16px;line-height:24px}#adthrive-ccpa-modal-language{text-align:left;font-size:12px;line-height:16px}.adthrive-ccpa-lower-buttons-container{font-size:14px}#adthrive-ccpa-modal-close-btn-container{font-size:14px;line-height:14px}}@media screen and (width<=350px){#adthrive-ccpa-modal-title{font-size:14px;line-height:24px}#adthrive-ccpa-modal-language{text-align:left;font-size:10px;line-height:14px}.adthrive-ccpa-lower-buttons-container{text-align:center;width:100%;font-size:12px;display:block}#adthrive-ccpa-modal-close-btn-container{font-size:12px;line-height:12px;display:block}#adthrive-ccpa-modal-continue-btn{text-align:center;width:100%;display:block}#adthrive-ccpa-modal-cancel-btn{text-align:center;width:100%;margin-bottom:10px;display:block}}.raptive-gated-print-modal-container{--raptive-gated-print-primary-color:#6e55f2;--raptive-gated-print-primary-hover-color:#5a45d6;--raptive-gated-print-disabled-color:#c5c5c5;--raptive-gated-print-border-color:#c5c5c5;--raptive-gated-print-placeholder-color:#c5c5c5;--raptive-gated-print-text-color:#333;--raptive-gated-print-title-color:#000;--raptive-gated-print-background-color:#fff;--raptive-gated-print-overlay-color:#000000b3;--raptive-gated-print-checkbox-check-color:#fff;background-color:var(--raptive-gated-print-overlay-color);z-index:999;overscroll-behavior:contain;width:100vw;height:100vh;display:none;position:fixed;top:0;left:0;overflow:hidden}.raptive-gated-print-modal-container .raptive-gated-print-modal-content{text-align:start;background-color:var(--raptive-gated-print-background-color);z-index:1000;border-radius:12px;flex-direction:column;align-items:start;gap:24px;width:550px;max-width:550px;padding:24px;display:flex;position:fixed;top:50%;left:50%;transform:translate(-50%,-50%);box-shadow:0 4px 12px #00000026}.raptive-gated-print-modal-close{color:var(--raptive-gated-print-text-color);cursor:pointer;z-index:1001;text-align:center;border-radius:50%;justify-content:center;align-items:center;width:32px;height:32px;font-size:24px;font-weight:700;transition:background-color .2s;display:flex;position:absolute;top:12px;right:12px}.raptive-gated-print-modal-close:hover,.raptive-gated-print-modal-close:focus{color:var(--raptive-gated-print-text-color);background-color:#0000001a;outline:none;text-decoration:none}.raptive-gated-print-modal-container #raptive-gated-print-modal-form{flex-direction:column;gap:16px;width:100%;display:flex}.raptive-gated-print-modal-container .raptive-gated-print-modal-title{color:var(--raptive-gated-print-title-color);font-size:1.5rem;font-weight:700}.raptive-gated-print-modal-container .raptive-gated-print-modal-body{color:var(--raptive-gated-print-text-color);font-size:1.1rem}.raptive-gated-print-modal-container label.raptive-gated-print-modal-body{color:var(--raptive-gated-print-text-color);cursor:pointer;font-size:1.1rem}.raptive-gated-print-modal-container .raptive-gated-print-modal-input{box-sizing:border-box;border:1px solid var(--raptive-gated-print-border-color);border-radius:5px;outline:none;width:100%;padding:12px;font-size:1.1rem}.raptive-gated-print-modal-container .raptive-gated-print-modal-input::placeholder{color:var(--raptive-gated-print-placeholder-color)}.raptive-gated-print-modal-container .raptive-gated-print-modal-input:focus{border:1px solid var(--raptive-gated-print-primary-color);box-shadow:0 0 2px var(--raptive-gated-print-primary-hover-color)}.raptive-gated-print-modal-container .raptive-gated-print-modal-actions{justify-content:space-between;align-items:center;width:100%;margin-top:8px;display:flex}.raptive-gated-print-modal-container .raptive-gated-print-modal-checkbox-container{align-items:center;display:flex}.raptive-gated-print-modal-container .raptive-gated-print-modal-checkbox{-webkit-appearance:none;appearance:none;border:2px solid var(--raptive-gated-print-border-color);cursor:pointer;background-color:var(--raptive-gated-print-background-color);border-radius:4px;outline:none;width:20px;height:20px;margin-right:8px;position:relative}.raptive-gated-print-modal-container .raptive-gated-print-modal-checkbox:checked{background-color:var(--raptive-gated-print-primary-color);border-color:var(--raptive-gated-print-primary-color)}.raptive-gated-print-modal-container .raptive-gated-print-modal-checkbox:checked:after{content:"✓";color:var(--raptive-gated-print-checkbox-check-color);font-size:14px;position:absolute;top:50%;left:50%;transform:translate(-50%,-50%)}.raptive-gated-print-modal-container .raptive-gated-print-modal-button{width:auto;color:var(--raptive-gated-print-checkbox-check-color);cursor:pointer;background-color:var(--raptive-gated-print-primary-color);border:none;border-radius:12px;padding:12px 24px;font-size:1.1rem;font-weight:700;transition:background-color .3s}.raptive-gated-print-modal-container .raptive-gated-print-modal-button:hover{background-color:var(--raptive-gated-print-primary-hover-color)}.raptive-gated-print-modal-container .raptive-gated-print-modal-button:disabled{background-color:var(--raptive-gated-print-disabled-color);cursor:not-allowed}@media screen and (width<=896px){.raptive-gated-print-modal-container .raptive-gated-print-modal-content{width:90%;padding:16px}}@media screen and (width<=350px){.raptive-gated-print-modal-container .raptive-gated-print-modal-content{width:95%;padding:12px}}@media print{.raptive-gated-print-modal-container .raptive-gated-print-modal-content{display:none!important}}.adthrive-us-cmp-modal{z-index:2147483647;background-color:#0006;width:100%;height:100%;display:none;position:fixed;top:0;left:0;overflow:auto}.adthrive-us-cmp-modal.show{display:block}.adthrive-us-cmp-modal-content{background-color:#fefefe;border:1px solid #888;border-radius:10px;width:80%;max-width:592px;margin:auto;padding:20px 24px 24px;font-family:Verdana,Geneva,Tahoma,sans-serif;position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);box-shadow:0 0 10px #00000080}.adthrive-us-cmp-modal-close{color:#000;font-size:28px;font-weight:700;position:fixed;top:-10px;right:3px}.adthrive-us-cmp-modal-close:hover,.adthrive-us-cmp-modal-close:focus{color:#000;cursor:pointer;text-decoration:none}.adthrive-us-cmp-modal-header{padding:2px 16px}.adthrive-us-cmp-modal-header h1{color:#000000de;font-size:20px;line-height:26px}.adthrive-us-cmp-modal-body{max-height:50vh;margin-bottom:10px;padding:10px 16px;position:relative;overflow-y:auto}.adthrive-us-cmp-modal-body p{font-size:14px;line-height:20px}.adthrive-us-cmp-modal-footer{color:#fff;flex-direction:column;justify-content:space-between;padding:2px 16px;display:flex}.adthrive-us-cmp-modal-accept,.adthrive-us-cmp-modal-decline{color:#fff;cursor:pointer;border:none;border-radius:10px}.adthrive-us-cmp-modal-accept{background-color:#010044}.adthrive-us-cmp-modal-accept.adthrive-us-cmp-modal-accept-strict-mode{align-self:flex-end;margin:0 0 10px auto;padding:12px 36px;background-color:green!important;font-size:18px!important}.adthrive-us-cmp-modal-decline{color:#000;background-color:#fff}.adthrive-us-cmp-modal-accept,.adthrive-us-cmp-modal-decline{text-transform:uppercase;flex:1;margin:0 10px 10px 0}.adthrive-us-cmp-modal-accept:hover,.adthrive-us-cmp-modal-accept:focus{color:#c4c4c4;background-color:#010044}.adthrive-us-cmp-modal-decline:hover,.adthrive-us-cmp-modal-decline:focus{color:#000;background-color:#fff}@media (width<=600px){.adthrive-us-cmp-modal-content{width:90%}.adthrive-us-cmp-modal-body{border-bottom:1px solid #c4c4c4;box-shadow:inset 0 -100px 30px -100px #0000001a}.adthrive-us-cmp-modal-accept,.adthrive-us-cmp-modal-decline{font-size:14px;line-height:14px}.adthrive-us-cmp-modal-accept.adthrive-us-cmp-modal-accept-strict-mode{align-self:center;margin:10px auto}}@media (width>=600px){.adthrive-us-cmp-modal-footer{flex-direction:row}.adthrive-us-cmp-modal-accept,.adthrive-us-cmp-modal-decline{flex:none;margin:0}}.adthrive-us-cmp-footer{margin-bottom:30px}.adthrive-us-cmp-footer-text{color:#a9a9a9;font-size:14px}.adthrive-us-cmp-footer-text p{color:#a9a9a9;text-decoration:underline}.adthrive-us-cmp-footer-link{cursor:pointer;font-size:14px;text-decoration:underline}</style><style type="text/css">
        .adthrive-ad.adthrive-sticky-sidebar {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        min-height: 1800px !important;
        padding-bottom: 0px;
        margin: 10px 0 10px 0;
        }
        .adthrive-ad.adthrive-sticky-sidebar > div {
        flex-basis: unset;
        position: sticky !important;
        top: 5px;
        }
        </style><link rel="stylesheet" type="text/css" href="https://ads.adthrive.com/sites/61575e8e934c48ea554b3caa/ads.min.css"><style type="text/css">
        .adthrive-recipe.adthrive-sticky {
        position: -webkit-sticky;
        position: sticky !important;
        top: 42px !important;
        margin-top: 42px !important;
        }
        .adthrive-recipe-sticky-container {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        min-height:400px !important;
        margin: 10px 0 10px 0;
        background-color: #FAFAFA;
        padding-bottom:0px;
        }
        </style><style type="text/css">
        .adthrive-recipe-sticky-container > div {
        flex-basis: unset;
        position: sticky !important;
        display: block;
        flex-direction: column;
        top: 5px;
        }
        </style><script src="https://config.aps.amazon-adsystem.com/configs/4fbba76f-7987-4fa2-9733-c27eb3a2170b" type="text/javascript" async="async"></script><style type="text/css">
        .adthrive-ad.adthrive-sticky-sidebar {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        min-height: 1800px !important;
        padding-bottom: 0px;
        margin: 10px 0 10px 0;
        }
        .adthrive-ad.adthrive-sticky-sidebar > div {
        flex-basis: unset;
        position: sticky !important;
        top: 5px;
        }
        </style><style type="text/css">
        .adthrive-ad.adthrive-sticky-sidebar {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        min-height: 1800px !important;
        padding-bottom: 0px;
        margin: 10px 0 10px 0;
        }
        .adthrive-ad.adthrive-sticky-sidebar > div {
        flex-basis: unset;
        position: sticky !important;
        top: 5px;
        }
        </style></head>
        <body class="position-relative overflow-auto toupee-banner-feat no-touch wordfinder-lexilicious cross-dungarees-lite adthrive-device-desktop" style="tabindex:0">
        <script>
        document.body.classList.add('cross-dungarees-lite');
        </script>

        <noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-WW4KHXF"
        height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript>
        <noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-PQ9SK33N"
        height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript>

        <div id="toupee-banner" class="toupee-banner w-100 text-white text-center">
        <a href="/collegiate-dictionary-twelfth-edition" class="toupee-banner-link">
        <div class="container-xxl py-2">
        <span class="toupee-banner-content d-inline-block">
        <span class="text-nowrap">✨📕&nbsp;<span class="d-none d-md-inline"><span class="toupee-banner-shout-text">The NEW</span></span><span class="d-inline-block d-md-none"><span class="toupee-banner-shout-text">The NEW</span></span></span><span class="d-none d-md-inline"><span class="toupee-banner-shout-text">&nbsp;Collegiate Dictionary, 12th Edition</span> Over 5,000 words added — <span class="toupee-banner-cta-text">Buy Now!</span></span><span class="d-inline-block d-md-none"><span class="toupee-banner-shout-text">&nbsp;Collegiate Dictionary</span> — <span class="toupee-banner-cta-text">Buy Now!</span></span>
        </span>
        </div>
        </a>
        </div>

        <header id="base-header" class="top-header position-fixed w-100 shrinkheader default-header-mobile" style="transform: translate3d(0px, 0px, 0px);">
        <nav class="navbar container-xxl flex-nowrap">
        <button class="navbar-toggler m-0" type="button" aria-expanded="false" aria-label="Toggle navigation">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="20px" viewBox="0 0 24 20" version="1.1" class="svg replaced-svg">
        <title>Menu Toggle</title>
        <g id="Menu" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
        <rect class="menu-rec" fill="\#ffffff" x="0" y="0" width="24" height="3" rx="1.5"></rect>
        <rect class="menu-rec" fill="\#ffffff" x="0" y="16" width="24" height="3" rx="1.5"></rect>
        <rect class="menu-rec" fill="\#ffffff" x="0" y="8" width="24" height="3" rx="1.5"></rect>
        </g>
        </svg>
        </button>

        <a href="/" class="navbar-brand d-flex m-0" title="Merriam Webster - established 1828">
        <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 41 42" fill="none" class="svg replaced-svg">
        <title>Merriam-Webster Logo</title>
        <path d="M0 20.5291C0 9.20443 9.20999 0.0291138 20.5723 0.0291138C31.9347 0.0291138 41 9.20881 41 20.5291C41 31.8538 31.9347 41.0291 20.5723 41.0291C9.21437 41.0335 0 31.8538 0 20.5291Z" fill="white"></path>
        <path d="M1.5 20.5269C1.5 10.0337 10.009 1.52911 20.5022 1.52911C30.9954 1.52911 39.5 10.0337 39.5 20.5269C39.5 31.0202 30.9954 39.5248 20.5022 39.5291C10.009 39.5291 1.50433 31.0202 1.5 20.5269ZM2.74342 20.5269C2.74342 25.4313 4.73201 29.8677 7.9467 33.0824C11.1614 36.2971 15.5978 38.2814 20.5022 38.2857C25.4065 38.2857 29.8429 36.2971 33.0576 33.0824C36.2723 29.8677 38.2609 25.4313 38.2609 20.5269C38.2609 15.6226 36.2723 11.1862 33.0576 7.97148C29.8429 4.7568 25.4065 2.7682 20.5022 2.7682C15.5935 2.7682 11.1614 4.75246 7.9467 7.97148C4.73201 11.1862 2.74342 15.6226 2.74342 20.5269Z" fill="\#D71920"></path>
        <path d="M22.58 25.1859C22.58 24.7011 22.9294 24.4041 23.3355 24.3997C23.8946 24.3997 24.1654 24.8321 24.1654 25.3999L24.4667 25.4174L24.5017 24.5439H24.4842L24.4973 24.5308C24.1523 24.2207 23.7636 24.1377 23.3224 24.1377C22.6367 24.1377 21.9991 24.6094 21.9991 25.4086C22.0078 27.0683 24.161 26.3564 24.1435 27.4875C24.1435 28.0073 23.816 28.3785 23.3093 28.3785H23.2875C22.6498 28.3654 22.2524 27.8413 22.2349 27.1556L21.8986 27.1381V27.1556L21.9074 27.2386L21.9947 28.3916C22.3834 28.6144 22.8114 28.6711 23.2264 28.6711C24.0212 28.6711 24.8161 28.23 24.8161 27.2692C24.803 25.6794 22.5581 26.1424 22.58 25.1859ZM19.9114 24.1813C19.7717 24.1813 19.6406 24.2076 19.5009 24.225L18.7715 24.688L18.8763 24.7753C19.0729 24.7142 19.2694 24.6792 19.4572 24.6792C20.4705 24.6792 20.9203 25.6226 20.9247 26.5791C20.9247 27.5705 20.4923 28.4091 19.597 28.4091C19.0117 28.4091 18.658 28.0029 18.658 27.4177V21.6045L18.4789 21.5259C18.112 21.6875 17.6665 21.8316 17.256 21.884L17.2472 22.05C17.5137 22.0675 17.7058 22.1112 17.9373 22.2247L17.9198 26.863C17.9198 27.0726 17.9198 27.2779 17.9198 27.4875C17.9198 27.8588 17.9155 28.23 17.8893 28.6187L18.0552 28.7104L18.3915 28.3873C18.6711 28.5532 19.0205 28.6842 19.3393 28.6886C20.9247 28.6842 21.6104 27.5487 21.6104 26.3171C21.6104 24.9195 20.9858 24.1813 19.9114 24.1813ZM14.8757 22.5479L14.8888 22.3208L12.7662 22.3033L12.7487 22.5304L13.5785 22.7532L13.4563 23.1069L12.0674 27.0333C11.9145 26.5791 10.779 23.1506 10.7004 22.858L10.8052 22.692L11.4953 22.5872L11.5084 22.3383L9.21542 22.3252V22.3426H9.19795V22.5741L9.82687 22.7182L9.94042 22.9279C9.86181 23.2205 8.67385 26.6009 8.51225 27.0508L7.13649 22.7663L7.1758 22.7532L7.84402 22.5872L7.8746 22.3383H7.85713H7.77414L5.48994 22.3164L5.46811 22.5479L6.28483 22.7488L8.16722 28.6973L8.45547 28.7104L10.1719 23.9804L11.7617 28.6842L12.0674 28.6973L14.1463 22.7401L14.8757 22.5479ZM7.12339 22.775L7.18016 22.9497L7.12339 22.775ZM13.5829 22.7401L13.5305 22.727L13.5829 22.7401ZM27.5807 27.6841L27.5545 27.7059C27.358 27.9025 27.0523 28.2082 26.8077 28.2082C26.3404 28.1994 26.1613 27.8763 26.1569 27.291C26.1569 27.2648 26.1569 27.2386 26.1569 27.2124V24.8802H26.1875L27.4279 24.8976L27.4454 24.4652L26.1395 24.4478L26.1569 23.3341L25.9342 23.3166C25.7551 23.9411 25.2616 24.308 24.7812 24.6574L24.7724 24.8802H24.7899V24.8976H25.4101L25.3926 27.5705C25.3926 28.4047 25.9298 28.7061 26.4146 28.7104C26.4321 28.7104 26.4539 28.7104 26.4714 28.7104C27.1571 28.6755 27.6244 27.9942 27.6288 27.9942L27.5851 27.7147L27.5807 27.6841ZM25.2922 20.0628H25.314C25.8337 20.0497 26.253 19.8532 26.6505 19.5431L26.6286 20.0715C26.6286 20.0715 26.633 20.0715 26.6374 20.0715H26.6461C26.6461 20.0715 26.6592 20.0672 26.6636 20.0672C26.8252 20.0497 27.6594 19.8619 28.035 19.792L28.0612 19.7876L27.9301 19.5867L27.3231 19.5256V16.9182C27.3231 16.1626 26.9955 15.822 26.467 15.822C26.4408 15.822 26.419 15.822 26.3928 15.822C26.3185 15.822 26.2399 15.8263 26.1657 15.8656L25.2092 16.4378C25.1131 16.4989 24.7681 16.6867 24.755 16.8527L24.7244 17.3287L24.7899 17.3812L25.2878 17.2807C25.349 17.2676 25.4407 17.272 25.4407 17.1671V17.154C25.4363 17.0711 25.4319 16.9924 25.4319 16.9182C25.4363 16.5863 25.5106 16.3373 26.1133 16.25C26.1351 16.2456 26.1569 16.2456 26.1788 16.2456C26.4801 16.2456 26.6592 16.5644 26.6592 17.1803V17.8092C26.2487 17.9358 25.7988 18.1149 25.3184 18.2721C24.9559 18.39 24.3925 18.5647 24.3925 19.2854C24.3925 19.6129 24.7812 20.0628 25.2922 20.0628ZM26.6461 20.0541H26.6417L26.6505 19.9274L26.6461 20.0541ZM26.6767 18.0232V18.0275C26.5981 18.0538 26.5282 18.0756 26.4539 18.0974C26.5282 18.0712 26.5981 18.0494 26.6767 18.0232ZM26.6767 18.0406V18.3202L26.6592 19.2548C26.4015 19.4164 25.9997 19.6217 25.7639 19.6217C25.445 19.6217 25.1175 19.3989 25.1175 19.1063C25.1131 18.5298 25.7289 18.3638 26.6767 18.0406ZM23.2264 14.7956C23.4273 14.7956 23.6151 14.634 23.6194 14.4156C23.6194 14.1929 23.4535 13.992 23.2264 13.9876C23.008 13.9876 22.8507 14.206 22.8507 14.4156C22.8507 14.6165 23.0342 14.7956 23.2264 14.7956ZM22.9425 19.6872L22.1651 19.8139L22.1519 20.0235L24.3837 20.041V20.0235H24.4012V19.8313L23.6238 19.6697L23.6369 15.9268L23.4709 15.8482C23.1259 16.001 22.6411 16.1364 22.2568 16.1888L22.2393 16.3985C22.4882 16.4159 22.7372 16.4596 22.9556 16.5688L22.9425 19.6872ZM16.7756 19.6566L16.0287 19.7833L16.0156 20.0279L18.2212 20.0453L18.2387 19.8008L17.4787 19.6392L17.4918 17.5471C17.4918 17.403 17.8849 16.4378 18.3435 16.4378C18.4439 16.4378 18.5881 16.512 18.6754 16.5775L18.9025 16.5994L19.0641 16.0272C18.9156 15.9006 18.575 15.8787 18.3959 15.8787C17.9417 15.8831 17.8063 16.4028 17.6272 16.7479L17.5137 16.9313V15.9311L17.3128 15.8525L17.3084 15.87L17.2997 15.8525C16.9634 16.0054 16.4655 16.1408 16.0724 16.1932L16.0549 16.4028C16.3126 16.4203 16.5659 16.464 16.7843 16.5732L16.7756 19.6566ZM7.05788 19.7964L6.27609 19.6348L6.71284 14.9179L6.75215 14.8L8.85292 19.8706L8.97084 19.8794L11.0148 14.8524L11.0716 14.9572L11.4734 19.6479L10.6349 19.7745L10.6218 20.0191L13.0282 20.0366L13.0457 19.792L12.3163 19.6304L11.7922 14.4244L12.6133 14.2977L12.609 14.2802H12.6264L12.6177 14.0706L10.8314 14.0531L9.04509 18.3944L7.23258 14.0619L5.41133 14.0531L5.38512 14.2802L6.27609 14.4462L5.77383 19.6523L5.08377 19.7789L5.07066 20.0235L7.04041 20.041L7.05788 19.7964ZM34.5644 17.2021C34.5687 16.3504 34.1364 15.87 33.2323 15.8438C33.2279 15.8438 33.2236 15.8438 33.2192 15.8438C33.11 15.8438 32.9353 15.8875 32.6907 16.0796L32.0574 16.6212C31.8303 16.1932 31.3979 15.8569 30.8913 15.8525C30.5594 15.8525 30.4371 15.8962 30.175 16.1015L29.5723 16.4946V15.9224L29.3758 15.8438C29.0395 15.9967 28.5634 16.1321 28.1878 16.1845L28.1704 16.3461C28.4149 16.3635 28.6595 16.4072 28.8692 16.5164L28.8648 19.6392L28.2053 19.7658L28.1922 19.9886L30.2318 20.006L30.2493 19.8051L29.5636 19.6217L29.5767 16.8221C29.8781 16.595 30.2187 16.3199 30.5769 16.3199C31.3412 16.3199 31.3586 17.0929 31.363 17.7786V19.6392L30.7035 19.7876L30.6904 19.9886L32.73 20.006L32.7475 19.8051L32.0618 19.6217L32.0749 16.8134C32.3675 16.5994 32.7606 16.3723 33.1231 16.3723C33.8132 16.3723 33.8568 16.9531 33.8612 17.569C33.8612 17.6738 33.8612 17.7742 33.8612 17.8791V19.6392L33.2017 19.7876L33.1886 19.9886L35.1977 20.006L35.2151 19.8051L34.56 19.6217L34.5644 17.2021ZM14.4564 20.137C14.8801 20.137 15.1771 20.0803 15.2426 20.041L16.2384 19.0059L15.9588 18.984C15.6356 19.3203 15.3212 19.6042 14.8626 19.6042C14.8408 19.6042 14.8189 19.6042 14.7971 19.6042C13.4999 19.5256 13.3209 18.5473 13.3165 17.4292H13.3427L15.5963 17.4467C15.8453 17.4467 15.9457 17.4161 15.9457 17.0885C15.9457 16.3592 15.3212 15.8525 14.452 15.8525C13.4257 15.8525 12.6002 16.6212 12.6002 17.9839C12.6046 19.3684 13.4213 20.1327 14.4564 20.137ZM14.3341 16.1058C14.9587 16.1058 15.2251 16.5513 15.2251 17.1278C15.2251 17.1584 15.2251 17.1934 15.2251 17.2283L13.3383 17.2108C13.382 16.6649 13.7489 16.1058 14.3341 16.1058ZM22.1869 16.0229C22.0384 15.8962 21.6977 15.8744 21.5187 15.8744C21.0644 15.8787 20.929 16.3985 20.75 16.7435L20.6364 16.9269V15.9268L20.4355 15.8482L20.4312 15.8656L20.4224 15.8482C20.0861 16.001 19.5882 16.1364 19.1952 16.1888L19.1821 16.3985C19.4397 16.4159 19.693 16.4596 19.9114 16.5688L19.9027 19.6566L19.1558 19.7833L19.1384 20.0235H19.1558H19.2126L21.344 20.041L21.3614 19.7964L20.6015 19.6348L20.619 17.5428C20.619 17.3986 21.012 16.4334 21.4663 16.4334C21.5667 16.4334 21.7108 16.5077 21.7982 16.5732L22.0253 16.595L22.1869 16.0229ZM32.8392 25.0723L32.7169 25.2689V24.2119L32.5029 24.1333L32.4986 24.1508L32.4898 24.1333C32.136 24.2949 31.6163 24.439 31.2014 24.4914L31.1839 24.7142C31.4547 24.7317 31.7211 24.7753 31.9482 24.8933L31.9395 28.1383L31.1534 28.2737L31.1403 28.527L33.455 28.5445L33.4725 28.2912L32.6776 28.1208L32.6907 25.9196C32.6907 25.7711 33.1013 24.7535 33.5861 24.7535C33.6909 24.7535 33.8437 24.8321 33.9355 24.902L34.1713 24.9238L34.3416 24.3211C34.1888 24.1901 33.8263 24.1682 33.6428 24.1682C33.1711 24.1595 33.0314 24.7098 32.8392 25.0723ZM16.0418 28.1339C16.0156 28.1339 15.9938 28.1339 15.9719 28.1339C14.6005 28.051 14.4127 27.0159 14.4084 25.8366L16.8192 25.8541C17.0813 25.8541 17.1861 25.8192 17.1861 25.4741C17.1861 24.7055 16.5266 24.1682 15.6138 24.1682C14.5307 24.1726 13.6615 24.9806 13.6572 26.4175C13.6615 27.8806 14.5219 28.693 15.6138 28.693C16.0637 28.693 16.3737 28.6318 16.4436 28.5925L17.4962 27.5007L17.2036 27.4744C16.8585 27.8369 16.5266 28.1339 16.0418 28.1339ZM15.4828 24.4434C16.1423 24.4434 16.4262 24.9151 16.4262 25.5265C16.4262 25.5615 16.4262 25.5964 16.4262 25.6314L14.4302 25.6139C14.4783 25.0286 14.8626 24.4434 15.4828 24.4434ZM31.2014 27.4482C30.8607 27.8064 30.5288 28.1034 30.044 28.1034C30.0178 28.1034 29.996 28.1034 29.9698 28.1034C28.5984 28.0204 28.4106 26.9853 28.4062 25.8061L30.8171 25.8235C31.0791 25.8235 31.1839 25.7886 31.1839 25.4436C31.1839 24.6749 30.5244 24.1377 29.6116 24.1377C28.5285 24.142 27.655 24.95 27.655 26.3869C27.655 27.8544 28.5198 28.6624 29.6116 28.6624C30.0571 28.6624 30.3716 28.6013 30.4415 28.562L31.494 27.4701L31.2014 27.4482ZM29.485 24.4128C30.1445 24.4128 30.4284 24.8845 30.4284 25.496C30.4284 25.5309 30.4284 25.5658 30.424 25.6008L28.428 25.5833C28.4805 25.0024 28.8692 24.4128 29.485 24.4128ZM35.1802 17.7044V18.294H36.7831V17.7044H35.1802Z" fill="\#004990"></path>
        </svg>
        </a>

        <a id="navbar-close-icon-link" href="javascript:void(0);" class="navbar-close" aria-label="Close Search">
        <div class="chevron-left-circle">
        <svg width="7" height="12" viewBox="0 0 7 12" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M0.667825 5.58964L5.76148 0.488749C6.05802 0.210202 6.51157 0.210202 6.79067 0.488749C7.06978 0.767297 7.06978 1.23734 6.79067 1.51589L2.2029 6.09451L6.79067 10.6731C7.06978 10.9691 7.06978 11.4217 6.79067 11.7003C6.51157 11.9788 6.05802 11.9788 5.76148 11.7003L0.667824 6.61678C0.38872 6.32083 0.38872 5.86819 0.667825 5.58964Z" fill="\#97BECE"></path>
        </svg>
        </div>
        </a>

        <div class="search-form-container d-flex col me-2 ms-3">
        <div class="w-100 button-switcher-container order-lg-1 order-2">
        <div class="button-switcher rounded">
        <button id="mw-search-toggle" class="btn btn-toggle rounded " data-toggle="button" autocomplete="off" data-search-url="/dictionary" title="Toggle Search Dictionary/Thesaurus" aria-label="Dictionary On. Search will provide dictionary results.">
        <div class="btn-switch"></div>
        </button>
        </div>
        </div>
        <div id="search-container" class="search-container d-flex col order-lg-2 order-1 flex-fill">
        <form id="search-form" class="d-flex w-100 position-relative search-form" autocomplete="off" novalidate="" action="/dictionary">
        <input id="search-term" class="form-control rounded border-dark search" type="search" placeholder="Search Dictionary" aria-label="Search" value="" autocomplete="off" spellcheck="false" autocorrect="off" autocapitalize="off">
        <button id="search-close-btn" class="btn position-absolute close-button" title="close" aria-label="Close Search" type="reset">
        <svg class="align-middle" width="12" height="13" viewBox="0 0 12 13" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M7.42616 6.5L11.7046 10.7785C12.0985 11.1723 12.0985 11.8108 11.7046 12.2046C11.3108 12.5985 10.6723 12.5985 10.2785 12.2046L6 7.92616L1.72153 12.2046C1.3277 12.5985 0.68919 12.5985 0.295367 12.2046C-0.0984557 11.8108 -0.0984557 11.1723 0.295367 10.7785L4.57384 6.5L0.295367 2.22153C-0.0984557 1.8277 -0.0984557 1.18919 0.295367 0.795367C0.68919 0.401544 1.3277 0.401544 1.72153 0.795367L6 5.07384L10.2785 0.795367C10.6723 0.401544 11.3108 0.401544 11.7046 0.795367C12.0985 1.18919 12.0985 1.8277 11.7046 2.22153L7.42616 6.5Z" fill="\#97BECE"></path>
        </svg>
        </button>
        <button id="search-form-submit-btn" class="btn position-absolute search-button search-dictionary" title="Search" aria-label="Search Word" type="submit">
        <svg class="svg replaced-svg align-middle" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="19px" height="21px" viewBox="0 0 26 29" version="1.1">
        <g class="Search" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
        <g class="g-search-icon" transform="translate(1.000000, 1.000000)" fill="\#fff" fill-rule="nonzero">
        <path d="M17.5185815,17.4073633 L24.5896493,24.4784311 C24.9801736,24.8689554 24.9801736,25.5021204 24.5896493,25.8926447 L23.1754357,27.3068582 C22.7849114,27.6973825 22.1517464,27.6973825 21.7612221,27.3068582 L14.6901543,20.2357904 C14.29963,19.8452661 14.29963,19.2121011 14.6901543,18.8215769 L16.1043679,17.4073633 C16.4948922,17.016839 17.1280572,17.016839 17.5185815,17.4073633 Z M10.5,21 C4.70101013,21 0,16.2989899 0,10.5 C0,4.70101013 4.70101013,0 10.5,0 C16.2989899,0 21,4.70101013 21,10.5 C21,16.2989899 16.2989899,21 10.5,21 Z M10.5,18 C14.6421356,18 18,14.6421356 18,10.5 C18,6.35786438 14.6421356,3 10.5,3 C6.35786438,3 3,6.35786438 3,10.5 C3,14.6421356 6.35786438,18 10.5,18 Z" class="Oval"></path>
        </g>
        </g>
        </svg>
        <span class="visually-hidden">Search</span>
        </button>
        </form>
        <div class="search-results-auto overflow-auto collapse shadow rounded"></div>
        </div>
        <div id="chatbot-mobile-nav-item" class="chatbot-nav-item-container order-lg-3 order-3 d-none d-lg-none" style="display: none;">
        <a class="w-100 text-white text-decoration-none btn btn-chatbot d-flex" href="/chatbot">
        <span class="d-none d-sm-block">Chatbot</span>
        </a>
        </div>
        </div>

        <div class="navigation-container order-md-4 d-none d-lg-flex">
        <div class="navigation d-lg-flex">
        <ul class="header-nav-list navigation-second-half d-flex flex-column flex-lg-row justify-content-start align-items-lg-center order-lg-1 p-0">
        <!-- Chatbot nav item for desktop only -->
        <li class="me-2 chatbot-nav-item d-none" style="display: flex;">
        <a class="w-100 text-white text-decoration-none btn btn-chatbot d-flex" href="/chatbot">
        <span class="d-none d-sm-block">Chatbot</span>
        </a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-games-mobile" class="position-relative me-2" style="display: flex;">
        <a id="mw-global-nav-games-quizzes" class="w-100 text-white text-decoration-none btn btn-blue" href="/games">Games</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-wod-mobile" class="position-relative me-2" style="display: flex;">
        <a id="mw-global-nav-wod" class="w-100 text-white text-decoration-none btn btn-blue" href="/word-of-the-day">Word of the Day</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-grammar-mobile" class="position-relative me-2" style="display: flex;">
        <a id="mw-global-nav-features-grammar" class="w-100 text-white text-decoration-none btn btn-blue" href="/grammar">Grammar</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-wordfinder-mobile" class="position-relative me-2" style="display: flex;">
        <a id="mw-global-nav-wordfinder" class="w-100 text-white text-decoration-none btn btn-blue" href="/wordfinder">Word Finder</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-slang-mobile" class="position-relative me-2" style="display: none;">
        <a id="mw-global-nav-slang-words" class="w-100 text-white text-decoration-none btn btn-blue" href="/slang">Slang</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-newsletter-mobile" class="position-relative me-2" style="display: none;">
        <span class="new-nav-link-badge">New</span>
        <a id="mw-global-nav-newsletter" class="w-100 text-white text-decoration-none btn btn-blue" href="/newsletters">Newsletters</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-wordplay-mobile" class="position-relative me-2" style="display: none;">
        <a id="mw-global-nav-features-wordplay" class="w-100 text-white text-decoration-none btn btn-blue" href="/wordplay">Wordplay</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-rhymes-mobile" class="position-relative me-2" style="display: none;">
        <a id="mw-global-nav-rhymes-words" class="w-100 text-white text-decoration-none btn btn-blue" href="/rhymes">Rhymes</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-thesaurus-mobile" class="position-relative me-2" style="display: none;">
        <a id="mw-global-nav-thesaurus" class="w-100 text-white text-decoration-none btn btn-blue" href="/thesaurus">Thesaurus</a>
        </li>
        <li data-matching-mobile-id="mw-slim-header-nav-joinmwu-mobile" class="position-relative me-xl-2 justify-content-between flex-column flex-lg-row d-mobile-sidebar-only" style="display: none;">
        <a class="w-100 text-white text-decoration-none btn btn-blue" href="https://premium.britannica.com/mw-unabridged/?utm_source=mw&amp;utm_medium=global-nav-join&amp;utm_campaign=evergreen">Join MWU</a>
        </li>            <li id="mw-slim-header-nav-more-dropdown" class="position-relative dropdown pe-1" style="display: flex;">
        <button id="mw-global-nav-show-more-mobile" class="w-100 text-white text-decoration-none btn btn-blue parent dropdown-toggle d-flex justify-content-between align-items-center" role="button" data-bs-toggle="dropdown" aria-expanded="false">
        <span>More</span>
        <span class="active-arrow">
        <div class="arrow-up"></div>
        </span>
        </button>
        <ul aria-labelledby="mw-global-nav-show-more-mobile" class="nav-dropdown-menu-responsive dropdown-menu submenu menu-list-items flex-row justify-content-between flex-column shadow pb-3 position-absolute overflow-hidden bg-white">
        <li class="position-relative" style="display: none;">
        <a id="mw-slim-header-nav-games-mobile" href="/games" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Games</a>
        </li>
        <li class="position-relative" style="display: none;">
        <a id="mw-slim-header-nav-wod-mobile" href="/word-of-the-day" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Word of the Day</a>
        </li>
        <li class="position-relative" style="display: none;">
        <a id="mw-slim-header-nav-grammar-mobile" href="/grammar" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Grammar</a>
        </li>
        <li class="position-relative" style="display: block;">
        <a id="mw-slim-header-nav-wordplay-mobile" href="/wordplay" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Wordplay</a>
        </li>
        <li class="position-relative" style="display: block;">
        <a id="mw-slim-header-nav-slang-mobile" href="/slang" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Slang</a>
        </li>
        <li class="position-relative" style="display: block;">
        <a id="mw-slim-header-nav-rhymes-mobile" href="/rhymes" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Rhymes</a>
        </li>
        <li class="position-relative" style="display: none;">
        <a id="mw-slim-header-nav-wordfinder-mobile" href="/wordfinder" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Word Finder</a>
        </li>
        <li class="position-relative" style="display: block;">
        <a id="mw-slim-header-nav-newsletter-mobile" href="/newsletters" class="position-relative dropdown-item d-flex w-100 text-decoration-none rounded">Newsletters <span class="new-nav-link-badge position-static d-flex justify-content-center align-items-center">New</span></a>
        </li>
        <li class="position-relative" style="display: block;">
        <a id="mw-slim-header-nav-thesaurus-mobile" href="/thesaurus" class="dropdown-item d-flex justify-content-between w-100 text-decoration-none rounded">Thesaurus</a>
        </li>
        <li class="position-relative" style="display: block;">
        <a id="mw-slim-header-nav-joinmwu-mobile" href="https://premium.britannica.com/mw-unabridged/?utm_source=mw&amp;utm_medium=global-nav-join&amp;utm_campaign=evergreen" target="_blank" rel="noopener noreferrer" class="dropdown-item d-flex justify-content-between w-100 text-white text-decoration-none rounded">
        <span>Join MWU</span>
        <span class="external-link d-none">
        <svg xmlns="http://www.w3.org/2000/svg" class="link-icon-svg svg replaced-svg" width="11" height="10" viewBox="0 0 11 10" fill="none" data-inject-url="https://www.merriam-webster.com/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/images/svg/link-icon.svg"><title>link icon</title>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M7.16171 1.04167H8.72261L4.86475 4.9024L5.60077 5.63897L9.45156 1.78531V3.33333H10.4924V1.04167H10.4926V0.743537L10.4995 0.73657L10.4926 0.729603V0H10.4924H9.7635H9.45156H7.16171V1.04167ZM0.5 1.66667H4.83011V2.70833H1.54089V8.9998H7.78623V5.66647H8.82712V9.99938L0.5 9.9998V1.66667Z" fill="\#265667"></path>
        </svg>
        </span>
        </a>
        </li>
        <li class="position-relative me-xl-2 justify-content-between flex-column flex-lg-row bg-white">
        <span class="d-flex justify-content-between w-100 text-white text-decoration-none border-bottom border-gold ps-0">
        <span class="fs-4 fw-bold font-logo">Shop</span>
        </span>
        </li>
        <li class="position-relative d-flex me-xl-2 justify-content-between flex-column flex-lg-row mt-4">
        <a id="mw-global-nav-shop-books" href="https://shop.merriam-webster.com/?utm_source=mwsite&amp;utm_medium=nav&amp;utm_content=header" target="_blank" rel="noopener noreferrer" class="dropdown-item d-flex justify-content-between w-100 text-white text-decoration-none rounded">
        <span>Books</span>
        <span class="external-link">
        <svg class="link-icon-svg svg replaced-svg" width="11" height="10" viewBox="0 0 11 10" fill="none" xmlns="http://www.w3.org/2000/svg" data-inject-url="https://www.merriam-webster.com/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/images/svg/link-icon.svg"><title>link icon</title>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M7.16171 1.04167H8.72261L4.86475 4.9024L5.60077 5.63897L9.45156 1.78531V3.33333H10.4924V1.04167H10.4926V0.743537L10.4995 0.73657L10.4926 0.729603V0H10.4924H9.7635H9.45156H7.16171V1.04167ZM0.5 1.66667H4.83011V2.70833H1.54089V8.9998H7.78623V5.66647H8.82712V9.99938L0.5 9.9998V1.66667Z" fill="\#265667"></path>
        </svg>
        </span>
        </a>
        </li>
        <li class="position-relative d-flex me-xl-2 justify-content-between flex-column flex-lg-row">
        <a id="mw-global-nav-shop-merch" href="https://merriamwebster.threadless.com/?utm_source=mwsite&amp;utm_medium=nav&amp;utm_content=header" target="_blank" rel="noopener noreferrer" class="dropdown-item d-flex justify-content-between w-100 text-white text-decoration-none rounded">
        <span>Merch</span>
        <span class="external-link">
        <svg class="link-icon-svg svg replaced-svg" width="11" height="10" viewBox="0 0 11 10" fill="none" xmlns="http://www.w3.org/2000/svg" data-inject-url="https://www.merriam-webster.com/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/images/svg/link-icon.svg"><title>link icon</title>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M7.16171 1.04167H8.72261L4.86475 4.9024L5.60077 5.63897L9.45156 1.78531V3.33333H10.4924V1.04167H10.4926V0.743537L10.4995 0.73657L10.4926 0.729603V0H10.4924H9.7635H9.45156H7.16171V1.04167ZM0.5 1.66667H4.83011V2.70833H1.54089V8.9998H7.78623V5.66647H8.82712V9.99938L0.5 9.9998V1.66667Z" fill="\#265667"></path>
        </svg>
        </span>
        </a>
        </li>
        <ul id="nav-dropdown-user-account-links" class="p-0 m-0 d-none d-lg-block">
        <li class="loggedout-links position-relative me-xl-2 justify-content-between flex-column flex-lg-row mt-3 no-hover">
        <a rel="nofollow" href="/login" class="ul-sign-in fw-bold text-center btn btn-gradient btn-md active w-100 text-white text-decoration-none d-flex justify-content-center align-items-center btn-link">
        Log In
        </a>
        </li>
        <li class="loggedin-links position-relative me-xl-2 justify-content-between flex-column flex-lg-row bg-white">
        <span class="d-flex justify-content-between w-100 text-white text-decoration-none border-bottom border-gold ps-0">
        <span class="fs-4 fw-bold font-logo mw-username text-truncate" style="max-width:250px">Username</span>
        </span>
        </li>
        <li class="loggedin-links position-relative me-xl-2 justify-content-between flex-column flex-lg-row mt-4">
        <a rel="nofollow" href="/saved-words" class="d-flex justify-content-between w-100 text-white text-decoration-none">
        <span>My Words</span>
        </a>
        </li>
        <li class="loggedin-links position-relative me-xl-2 justify-content-between flex-column flex-lg-row">
        <a rel="nofollow" href="/recents" class="d-flex justify-content-between w-100 text-white text-decoration-none">
        <span>Recents</span>
        </a>
        </li>
        <li class="loggedin-links position-relative me-xl-2 justify-content-between flex-column flex-lg-row">
        <a rel="nofollow" href="/settings" class="d-flex justify-content-between w-100 text-white text-decoration-none">
        <span>Account</span>
        </a>
        </li>
        <li class="loggedin-links position-relative me-xl-2 justify-content-between flex-column flex-lg-row mt-3 no-hover">
        <a rel="nofollow" href="/logout" class="ul-sign-out fw-bold text-center btn btn-gradient btn-md active w-100 text-white text-decoration-none d-flex justify-content-center align-items-center btn-link">
        Log Out
        </a>
        </li>
        </ul>              </ul>
        </li>
        </ul>
        </div>
        </div>
        </nav>
        </header>
        <div class="header-placeholder">
        <div class="container-xxl d-none d-lg-block position-relative">
        <div class="est font-logo rounded text-center" aria-hidden="true">
        Est. 1828
        </div>
        </div>
        </div>

        <mw-menu-games-mobile class="d-lg-none" style="line-height: 1.5;" id="menu-games-mobile"><template shadowrootmode="open"><!---->
        <div class="fixed nav-container fixed h-screen h-full z-[2147483647] top-[0] left-[0]">
        <div class="fixed h-full transition-bg duration-500 top-[0] left-[0]

          "></div>
        <button class="absolute top-[10px] transition-all left-[-330px]">
        <mw-icon-close><template shadowrootmode="open"><!---->
        <svg part="icon" width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_4219_2166)">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M11.1392 9L17.5569 15.4177C18.1477 16.0084 18.1477 16.9662 17.5569 17.5569C16.9662 18.1477 16.0084 18.1477 15.4177 17.5569L9 11.1392L2.58229 17.5569C1.99155 18.1477 1.03378 18.1477 0.443051 17.5569C-0.147684 16.9662 -0.147684 16.0084 0.443051 15.4177L6.86076 9L0.443051 2.58229C-0.147684 1.99155 -0.147684 1.03378 0.443051 0.443051C1.03378 -0.147684 1.99155 -0.147684 2.58229 0.443051L9 6.86076L15.4177 0.443051C16.0084 -0.147684 16.9662 -0.147684 17.5569 0.443051C18.1477 1.03378 18.1477 1.99155 17.5569 2.58229L11.1392 9Z" fill="\#CBE1EA"></path>
        </g>
        <defs>
        <clipPath id="clip0_4219_2166">
        <rect width="18" height="18" fill="white"></rect>
        </clipPath>
        </defs>
        </svg>
        </template></mw-icon-close>
        </button>

        <div class="games-nav scrollbar-none w-[250px] max-w-[250px] bg-[\#0F3850] transition-all h-full absolute top-[0] overflow-auto
        left-[-300px]
        ">
        <div class="bg-[\#0F3850]">
        <!--?lit$063844234$--> <div class="mw-login py-[10px]">
        <mw-login><template shadowrootmode="open"><!---->
        <div part="container" class="login-nav w-full">
        <!--?lit$063844234$--><div part="logo" class="w-full flex flex-col justify-center self-center items-center content-center">
        <a part="logo-link" class="" rel="external" href="/" aria-label="Game">
        <mw-icon-mw-logo-white><template shadowrootmode="open"><!---->
        <svg part="icon" viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg" width="70" height="70">
        <path d="M0.000976562 19.9977C0.000976562 8.95223 8.95777 0 20.0033 0C31.0487 0 40.001 8.95223 40.001 19.9977C40.001 31.0432 31.0487 39.9954 20.0033 40C8.95777 40 0.00553704 31.0432 0.000976562 19.9977ZM1.30984 19.9977C1.30984 25.1602 3.4031 29.8301 6.78698 33.214C10.1709 36.5979 14.8408 38.6866 20.0033 38.6911C25.1657 38.6911 29.8357 36.5979 33.2195 33.214C36.6034 29.8301 38.6967 25.1602 38.6967 19.9977C38.6967 14.8353 36.6034 10.1653 33.2195 6.78144C29.8357 3.39756 25.1657 1.3043 20.0033 1.3043C14.8362 1.3043 10.1709 3.393 6.78698 6.78144C3.4031 10.1653 1.30984 14.8353 1.30984 19.9977Z" fill="white"></path>
        <path d="M22.1906 24.9018C22.1906 24.3915 22.5584 24.0789 22.986 24.0743C23.5744 24.0743 23.8595 24.5294 23.8595 25.1271L24.1767 25.1454L24.2135 24.226H24.1951L24.2089 24.2122C23.8457 23.8858 23.4365 23.7984 22.9722 23.7984C22.2504 23.7984 21.5792 24.2949 21.5792 25.1363C21.5884 26.8833 23.8549 26.1339 23.8365 27.3246C23.8365 27.8717 23.4917 28.2625 22.9584 28.2625H22.9354C22.2642 28.2487 21.8458 27.697 21.8274 26.9752L21.4734 26.9568V26.9752L21.4826 27.0625L21.5746 28.2763C21.9837 28.5107 22.4343 28.5705 22.871 28.5705C23.7077 28.5705 24.5445 28.1062 24.5445 27.0947C24.5307 25.4213 22.1676 25.9086 22.1906 24.9018ZM19.3816 23.8444C19.2345 23.8444 19.0966 23.872 18.9495 23.8904L18.1817 24.3777L18.292 24.4696C18.4989 24.4053 18.7058 24.3685 18.9035 24.3685C19.9701 24.3685 20.4436 25.3615 20.4482 26.3683C20.4482 27.4119 19.9931 28.2946 19.0506 28.2946C18.4346 28.2946 18.0622 27.8671 18.0622 27.251V21.1319L17.8737 21.0492C17.4875 21.2193 17.0186 21.371 16.5864 21.4262L16.5772 21.6009C16.8577 21.6193 17.0599 21.6652 17.3036 21.7848L17.2852 26.6672C17.2852 26.8878 17.2852 27.1039 17.2852 27.3246C17.2852 27.7154 17.2806 28.1061 17.253 28.5153L17.4277 28.6119L17.7817 28.2717C18.076 28.4464 18.4438 28.5843 18.7794 28.5889C20.4482 28.5843 21.17 27.389 21.17 26.0925C21.17 24.6213 20.5126 23.8444 19.3816 23.8444ZM14.0809 22.125L14.0946 21.8859L11.8603 21.8675L11.8419 22.1066L12.7154 22.3411L12.5867 22.7134L11.1247 26.8465C10.9638 26.3683 9.76852 22.7594 9.68577 22.4514L9.79611 22.2767L10.5225 22.1664L10.5363 21.9043L8.12267 21.8905V21.9089H8.10428V22.1526L8.7663 22.3043L8.88583 22.5249C8.80308 22.833 7.55259 26.3913 7.38249 26.8649L5.93432 22.3548L5.9757 22.3411L6.67909 22.1664L6.71128 21.9043H6.69289H6.60554L4.20111 21.8813L4.17813 22.125L5.03783 22.3365L7.0193 28.5981L7.32273 28.6119L9.12949 23.6329L10.8029 28.5843L11.1247 28.5981L13.3131 22.3273L14.0809 22.125ZM5.92053 22.364L5.98029 22.5479L5.92053 22.364ZM12.72 22.3273L12.6649 22.3135L12.72 22.3273ZM27.4546 27.5315L27.427 27.5545C27.2201 27.7613 26.8983 28.0832 26.6409 28.0832C26.1489 28.074 25.9605 27.7338 25.9559 27.1177C25.9559 27.0901 25.9559 27.0625 25.9559 27.035V24.58H25.988L27.2937 24.5984L27.3121 24.1432L25.9375 24.1248L25.9559 22.9525L25.7214 22.9341C25.5329 23.5915 25.0134 23.9777 24.5077 24.3455L24.4985 24.58H24.5169V24.5984H25.1697L25.1513 27.4119C25.1513 28.29 25.7168 28.6073 26.2271 28.6119C26.2455 28.6119 26.2685 28.6119 26.2869 28.6119C27.0087 28.5751 27.5006 27.8579 27.5052 27.8579L27.4592 27.5637L27.4546 27.5315ZM25.0456 19.5091H25.0686C25.6157 19.4953 26.057 19.2884 26.4754 18.962L26.4524 19.5183C26.4524 19.5183 26.457 19.5183 26.4616 19.5183H26.4708C26.4708 19.5183 26.4846 19.5137 26.4891 19.5137C26.6593 19.4953 27.5373 19.2976 27.9327 19.224L27.9603 19.2194L27.8224 19.008L27.1833 18.9436V16.199C27.1833 15.4036 26.8385 15.045 26.2823 15.045C26.2547 15.045 26.2317 15.045 26.2041 15.045C26.126 15.045 26.0432 15.0496 25.965 15.091L24.9582 15.6933C24.8571 15.7576 24.4939 15.9553 24.4801 16.13L24.4479 16.6311L24.5169 16.6863L25.041 16.5806C25.1053 16.5668 25.2019 16.5714 25.2019 16.461V16.4472C25.1973 16.3599 25.1927 16.2771 25.1927 16.199C25.1973 15.8496 25.2754 15.5875 25.9099 15.4956C25.9329 15.491 25.9559 15.491 25.9788 15.491C26.2961 15.491 26.4846 15.8266 26.4846 16.4748V17.1368C26.0524 17.2702 25.5789 17.4586 25.0732 17.6242C24.6916 17.7483 24.0985 17.9322 24.0985 18.6907C24.0985 19.0355 24.5077 19.5091 25.0456 19.5091ZM26.4708 19.4999H26.4662L26.4754 19.3666L26.4708 19.4999ZM26.5029 17.3621V17.3667C26.4202 17.3943 26.3466 17.4173 26.2685 17.4403C26.3466 17.4127 26.4202 17.3897 26.5029 17.3621ZM26.5029 17.3805V17.6747L26.4846 18.6586C26.2133 18.8287 25.7903 19.0447 25.5421 19.0447C25.2065 19.0447 24.8617 18.8103 24.8617 18.5023C24.8571 17.8954 25.5053 17.7207 26.5029 17.3805ZM22.871 13.9646C23.0825 13.9646 23.2802 13.7945 23.2848 13.5647C23.2848 13.3302 23.1101 13.1187 22.871 13.1141C22.6412 13.1141 22.4756 13.344 22.4756 13.5647C22.4756 13.7762 22.6687 13.9646 22.871 13.9646ZM22.5722 19.1137L21.7539 19.247L21.7401 19.4677L24.0893 19.4861V19.4677H24.1077V19.2654L23.2894 19.0953L23.3032 15.1554L23.1285 15.0726C22.7653 15.2335 22.255 15.376 21.8504 15.4312L21.832 15.6519C22.0941 15.6703 22.3561 15.7162 22.586 15.8312L22.5722 19.1137ZM16.0807 19.0815L15.2946 19.2148L15.2808 19.4723L17.6024 19.4907L17.6208 19.2332L16.8209 19.0631L16.8347 16.861C16.8347 16.7093 17.2484 15.6933 17.7312 15.6933C17.8369 15.6933 17.9886 15.7714 18.0806 15.8404L18.3196 15.8634L18.4897 15.2611C18.3334 15.1278 17.9748 15.1048 17.7863 15.1048C17.3082 15.1094 17.1657 15.6565 16.9772 16.0197L16.8577 16.2128V15.16L16.6462 15.0772L16.6416 15.0956L16.6324 15.0772C16.2784 15.2381 15.7543 15.3806 15.3405 15.4358L15.3221 15.6565C15.5934 15.6749 15.86 15.7208 16.0899 15.8358L16.0807 19.0815ZM5.85157 19.2286L5.02864 19.0585L5.48838 14.0934L5.52975 13.9692L7.74109 19.3068L7.86521 19.316L10.0168 14.0244L10.0765 14.1348L10.4995 19.0723L9.61681 19.2056L9.60302 19.4631L12.1362 19.4815L12.1546 19.224L11.3868 19.0539L10.8351 13.5739L11.6994 13.4405L11.6948 13.4222H11.7132L11.704 13.2015L9.82369 13.1831L7.94337 17.7529L6.03546 13.1923L4.11836 13.1831L4.09078 13.4222L5.02864 13.5969L4.49994 19.0769L3.77356 19.2102L3.75977 19.4677L5.83318 19.4861L5.85157 19.2286ZM34.8058 16.4978C34.8104 15.6013 34.3552 15.0956 33.4036 15.068C33.399 15.068 33.3944 15.068 33.3898 15.068C33.2749 15.068 33.091 15.114 32.8335 15.3163L32.1669 15.8863C31.9278 15.4358 31.4727 15.0818 30.9394 15.0772C30.59 15.0772 30.4613 15.1232 30.1854 15.3393L29.551 15.753V15.1508L29.3441 15.068C28.9901 15.2289 28.489 15.3714 28.0936 15.4266L28.0752 15.5967C28.3327 15.6151 28.5901 15.6611 28.8108 15.776L28.8062 19.0631L28.112 19.1965L28.0982 19.4309L30.2452 19.4493L30.2636 19.2378L29.5418 19.0447L29.5556 16.0978C29.8728 15.8588 30.2314 15.5691 30.6084 15.5691C31.4129 15.5691 31.4313 16.3829 31.4359 17.1047V19.0631L30.7417 19.2194L30.7279 19.4309L32.8749 19.4493L32.8933 19.2378L32.1715 19.0447L32.1853 16.0886C32.4933 15.8634 32.9071 15.6243 33.2887 15.6243C34.015 15.6243 34.061 16.2357 34.0656 16.884C34.0656 16.9943 34.0656 17.1001 34.0656 17.2104V19.0631L33.3714 19.2194L33.3576 19.4309L35.4724 19.4493L35.4908 19.2378L34.8012 19.0447L34.8058 16.4978ZM13.6395 19.5872C14.0855 19.5872 14.3981 19.5275 14.467 19.4861L15.5152 18.3965L15.221 18.3735C14.8808 18.7275 14.5498 19.0264 14.0671 19.0264C14.0441 19.0264 14.0211 19.0264 13.9981 19.0264C12.6327 18.9436 12.4442 17.9138 12.4396 16.7369H12.4672L14.8394 16.7553C15.1015 16.7553 15.2072 16.7231 15.2072 16.3783C15.2072 15.6105 14.5498 15.0772 13.6349 15.0772C12.5545 15.0772 11.6856 15.8863 11.6856 17.3207C11.6902 18.7781 12.5499 19.5826 13.6395 19.5872ZM13.5108 15.3439C14.1682 15.3439 14.4486 15.8128 14.4486 16.4196C14.4486 16.4518 14.4486 16.4886 14.4486 16.5254L12.4626 16.507C12.5086 15.9323 12.8947 15.3439 13.5108 15.3439ZM21.7768 15.2565C21.6205 15.1232 21.2619 15.1002 21.0735 15.1002C20.5953 15.1048 20.4528 15.6519 20.2643 16.0151L20.1448 16.2082V15.1554L19.9333 15.0726L19.9287 15.091L19.9195 15.0726C19.5655 15.2335 19.0414 15.376 18.6277 15.4312L18.6139 15.6519C18.8851 15.6703 19.1518 15.7162 19.3816 15.8312L19.3724 19.0815L18.5863 19.2148L18.5679 19.4677H18.5863H18.646L20.8896 19.4861L20.9079 19.2286L20.108 19.0585L20.1264 16.8564C20.1264 16.7047 20.5402 15.6887 21.0183 15.6887C21.124 15.6887 21.2757 15.7668 21.3677 15.8358L21.6067 15.8588L21.7768 15.2565ZM32.9898 24.7823L32.8611 24.9891V23.8766L32.6358 23.7938L32.6312 23.8122L32.622 23.7938C32.2497 23.9639 31.7026 24.1156 31.2658 24.1708L31.2474 24.4053C31.5325 24.4237 31.8129 24.4696 32.052 24.5938L32.0428 28.0096L31.2152 28.1521L31.2015 28.4188L33.6381 28.4372L33.6564 28.1705L32.8197 27.9912L32.8335 25.6741C32.8335 25.5178 33.2657 24.4466 33.776 24.4466C33.8863 24.4466 34.0472 24.5294 34.1438 24.603L34.392 24.6259L34.5713 23.9915C34.4104 23.8536 34.0288 23.8306 33.8357 23.8306C33.3392 23.8214 33.1921 24.4007 32.9898 24.7823ZM15.3084 28.005C15.2808 28.005 15.2578 28.005 15.2348 28.005C13.7912 27.9177 13.5935 26.8281 13.5889 25.5868L16.1267 25.6052C16.4025 25.6052 16.5129 25.5684 16.5129 25.2052C16.5129 24.3961 15.8187 23.8306 14.8578 23.8306C13.7177 23.8352 12.8028 24.6857 12.7982 26.1982C12.8028 27.7384 13.7085 28.5935 14.8578 28.5935C15.3313 28.5935 15.6578 28.5291 15.7313 28.4877L16.8393 27.3384L16.5313 27.3108C16.1681 27.6924 15.8187 28.005 15.3084 28.005ZM14.7199 24.1202C15.4141 24.1202 15.7129 24.6167 15.7129 25.2604C15.7129 25.2972 15.7129 25.3339 15.7129 25.3707L13.6119 25.3523C13.6625 24.7363 14.0671 24.1202 14.7199 24.1202ZM31.2658 27.2832C30.9072 27.6602 30.5578 27.9728 30.0475 27.9728C30.0199 27.9728 29.9969 27.9728 29.9694 27.9728C28.5258 27.8855 28.3281 26.7959 28.3235 25.5546L30.8612 25.573C31.1371 25.573 31.2474 25.5362 31.2474 25.173C31.2474 24.3639 30.5532 23.7984 29.5924 23.7984C28.4522 23.803 27.5328 24.6535 27.5328 26.1661C27.5328 27.7108 28.443 28.5613 29.5924 28.5613C30.0613 28.5613 30.3923 28.4969 30.4659 28.4556L31.5738 27.3062L31.2658 27.2832ZM29.459 24.0881C30.1532 24.0881 30.4521 24.5846 30.4521 25.2282C30.4521 25.265 30.4521 25.3018 30.4475 25.3385L28.3465 25.3201C28.4017 24.7087 28.8108 24.0881 29.459 24.0881ZM35.454 17.0265V17.6471H37.1413V17.0265H35.454Z" fill="white"></path>
        </svg>
        </template></mw-icon-mw-logo-white>
        </a>
        </div>
        <!--?lit$063844234$-->
        </div>
        </template></mw-login>
        </div>
        <!--?lit$063844234$-->

        <div class="w-full justify-between flex items-center pt-1 px-2 bg-[\#265667] " style="

            ">
        <mw-menu-item id="gtm-gamesnav-link-allgames" img="" label="Games &amp; Quizzes" type="heading" class="block hover:shadow w-full p-2
                @hover:bg-[\#0F3850]
                " icon="down"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-allgames" href="javascript:void(0);" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Games &amp; Quizzes"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Games &amp; Quizzes</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-chevron-down class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" viewBox="0 0 17 10" fill="none" xmlns="http://www.w3.org/2000/svg" width="17" height="10">
        <g clip-path="url(\#clip0_4798_17992)">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M7.80539 9.212L0.787423 2.204C0.40419 1.796 0.40419 1.172 0.787423 0.787999C1.17066 0.403999 1.81736 0.403999 2.2006 0.787999L8.5 7.1L14.7994 0.788C15.2066 0.404 15.8293 0.404 16.2126 0.788C16.5958 1.172 16.5958 1.796 16.2126 2.204L9.21856 9.212C8.81138 9.596 8.18862 9.596 7.80539 9.212Z" fill="\#CBE1EA"></path>
        </g>
        <defs>
        <clipPath id="clip0_4798_17992">
        <rect width="9" height="16" fill="white" transform="translate(16.5 0.5) rotate(90)"></rect>
        </clipPath>
        </defs>
        </svg>
        </template></mw-icon-chevron-down>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </div>

        <div class="
            overflow-hidden transition-all
            max-h-[0]
          ">
        <div class="mb-[15px] mt-[20px] px-4 ">
        <mw-menu-item label="Games" icon="hidden" type="title"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><mw-headings-collection part="menu-title" tag="menu-title" label="Games"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-title text-xl font-bold leading-none text-[\#E4C04B] font-['Playfair_Display']"><!--?lit$063844234$-->Games</span>
        </template></mw-headings-collection>
        </template></mw-menu-item>
        </div>
        <ul class="
                flex flex-col gap-1 list-none
                px-4
                ">
        <!--?lit$063844234$--><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-reunion" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-reunion.webp" label="Reunion - NEW" link="/games/reunion" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-reunion" href="/games/reunion" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-reunion.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Reunion - NEW</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-quordle" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-quordle.webp" label="Quordle" link="/games/quordle/\#/" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-quordle" href="/games/quordle/\#/" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-quordle.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Quordle</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-revealed" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-revealed.png" label="Revealed" link="https://www.britannica.com/games/revealed" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-revealed" href="https://www.britannica.com/games/revealed" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-revealed.png">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Revealed</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-tightrope" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-tightrope.webp" label="Tightrope" link="https://www.britannica.com/quiz/tightrope" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-tightrope" href="https://www.britannica.com/quiz/tightrope" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-tightrope.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Tightrope</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-blossom" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-blossom.webp" label="Blossom" link="/games/blossom-word-game/" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-blossom" href="/games/blossom-word-game/" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-blossom.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Blossom</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-octordle" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-octordle.webp" label="Octordle" link="https://www.britannica.com/games/octordle/" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-octordle" href="https://www.britannica.com/games/octordle/" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-octordle.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Octordle</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-pilfer" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-pilfer2.webp" label="Pilfer" link="/games/pilfer" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-pilfer" href="/games/pilfer" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-pilfer2.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Pilfer</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-missingletter" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-missing-letter-daily.webp" label="The Missing Letter" link="/games/missing-letter" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-missingletter" href="/games/missing-letter" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-missing-letter-daily.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->The Missing Letter</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-twofer-goofer" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-twofer-goofer.png" label="Twofer Goofer" link="/games/twofer-goofer" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-twofer-goofer" href="/games/twofer-goofer" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-twofer-goofer.png">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Twofer Goofer</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-victordle" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-victordle.webp" label="Victordle" link="https://www.britannica.com/games/victordle/" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-victordle" href="https://www.britannica.com/games/victordle/" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-victordle.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Victordle</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!---->
        </ul>

        <div class="mb-[15px] mt-[30px] px-4 ">
        <mw-menu-item label="Quizzes" icon="hidden" type="title"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><mw-headings-collection part="menu-title" tag="menu-title" label="Quizzes"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-title text-xl font-bold leading-none text-[\#E4C04B] font-['Playfair_Display']"><!--?lit$063844234$-->Quizzes</span>
        </template></mw-headings-collection>
        </template></mw-menu-item>
        </div>
        <ul class="flex flex-col gap-1 list-none px-4 mb-5">
        <!--?lit$063844234$--><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-testvocab" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-test-your-vocabulary.webp" label="Test Your Vocabulary" link="/games/vocabulary-quiz" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-testvocab" href="/games/vocabulary-quiz" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-test-your-vocabulary.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Test Your Vocabulary</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-namething" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-name-that-thing.webp" label="Name That Thing" link="/games/name-that-thing" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-namething" href="/games/name-that-thing" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-name-that-thing.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Name That Thing</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-spellit" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-spell-it.webp" label="Spell It" link="/games/spell-it" icon="hidden"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-spellit" href="/games/spell-it" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-spell-it.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Spell It</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-latestquizzes" img="" label="Latest Quizzes" link="/games/see-all" icon="right"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-latestquizzes" href="/games/see-all" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Latest Quizzes</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-chevron-right class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" width="7" height="13" viewBox="0 0 7 13" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_4219_15472)">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.78139 7.00484L1.68774 12.1057C1.3912 12.3843 0.937651 12.3843 0.658547 12.1057C0.379443 11.8272 0.379443 11.3571 0.658547 11.0786L5.24632 6.49997L0.658547 1.92136C0.379443 1.6254 0.379443 1.17276 0.658547 0.894213C0.937651 0.615666 1.3912 0.615666 1.68774 0.894213L6.78139 5.9777C7.0605 6.27365 7.0605 6.72629 6.78139 7.00484Z" fill="\#EDF4F7"></path>
        </g>
        <defs>
        <clipPath id="clip0_4219_15472">
        <rect width="11.6293" height="6.5415" fill="white" transform="translate(0.449219 12.3147) rotate(-90)"></rect>
        </clipPath>
        </defs>
        </svg>
        </template></mw-icon-chevron-right>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!---->
        </ul>

        <div class="px-4">
        <hr class="h-px border-0 bg-[\#265667] my-2">
        </div>

        <div class="my-5">
        <!--?lit$063844234$--><!---->
        <!--?lit$063844234$--> <div class="my-2 py-2 px-4 hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-allgames" img="" label="All Games &amp; Quizzes" link="/games" icon="right"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-allgames" href="/games" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><span><!--?lit$063844234$-->All Games &amp; Quizzes</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-chevron-right class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" width="7" height="13" viewBox="0 0 7 13" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_4219_15472)">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.78139 7.00484L1.68774 12.1057C1.3912 12.3843 0.937651 12.3843 0.658547 12.1057C0.379443 11.8272 0.379443 11.3571 0.658547 11.0786L5.24632 6.49997L0.658547 1.92136C0.379443 1.6254 0.379443 1.17276 0.658547 0.894213C0.937651 0.615666 1.3912 0.615666 1.68774 0.894213L6.78139 5.9777C7.0605 6.27365 7.0605 6.72629 6.78139 7.00484Z" fill="\#EDF4F7"></path>
        </g>
        <defs>
        <clipPath id="clip0_4219_15472">
        <rect width="11.6293" height="6.5415" fill="white" transform="translate(0.449219 12.3147) rotate(-90)"></rect>
        </clipPath>
        </defs>
        </svg>
        </template></mw-icon-chevron-right>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </div>
        <!----><!---->
        <!--?lit$063844234$--> <div class="my-2 py-2 px-4 hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-wordfinder" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-word-finder.webp" label="Word Finder" link="/wordfinder" icon="right"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-wordfinder" href="/wordfinder" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-word-finder.webp">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Word Finder</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-chevron-right class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" width="7" height="13" viewBox="0 0 7 13" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_4219_15472)">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.78139 7.00484L1.68774 12.1057C1.3912 12.3843 0.937651 12.3843 0.658547 12.1057C0.379443 11.8272 0.379443 11.3571 0.658547 11.0786L5.24632 6.49997L0.658547 1.92136C0.379443 1.6254 0.379443 1.17276 0.658547 0.894213C0.937651 0.615666 1.3912 0.615666 1.68774 0.894213L6.78139 5.9777C7.0605 6.27365 7.0605 6.72629 6.78139 7.00484Z" fill="\#EDF4F7"></path>
        </g>
        <defs>
        <clipPath id="clip0_4219_15472">
        <rect width="11.6293" height="6.5415" fill="white" transform="translate(0.449219 12.3147) rotate(-90)"></rect>
        </clipPath>
        </defs>
        </svg>
        </template></mw-icon-chevron-right>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </div>
        <!----><!---->
        <!--?lit$063844234$--> <div class="my-2 py-2 px-4 hover:bg-gradient-to-r hover:from-[rgba(0,0,0,.15)] hover:to-transparent rounded ">
        <mw-menu-item id="gtm-gamesnav-link-gamesnewsletter" img="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-games-newsletter.png" label="Games Newsletter" link="/games-newsletter" icon="right"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-gamesnewsletter" href="/games-newsletter" class="p-1 gap-2 flex flex-row items-center w-full


                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><img part="img" class="" width="28" height="28" alt="" src="/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/assets/icon-games-newsletter.png">

        <!--?lit$063844234$--><span><!--?lit$063844234$-->Games Newsletter</span>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-chevron-right class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" width="7" height="13" viewBox="0 0 7 13" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_4219_15472)">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.78139 7.00484L1.68774 12.1057C1.3912 12.3843 0.937651 12.3843 0.658547 12.1057C0.379443 11.8272 0.379443 11.3571 0.658547 11.0786L5.24632 6.49997L0.658547 1.92136C0.379443 1.6254 0.379443 1.17276 0.658547 0.894213C0.937651 0.615666 1.3912 0.615666 1.68774 0.894213L6.78139 5.9777C7.0605 6.27365 7.0605 6.72629 6.78139 7.00484Z" fill="\#EDF4F7"></path>
        </g>
        <defs>
        <clipPath id="clip0_4219_15472">
        <rect width="11.6293" height="6.5415" fill="white" transform="translate(0.449219 12.3147) rotate(-90)"></rect>
        </clipPath>
        </defs>
        </svg>
        </template></mw-icon-chevron-right>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </div>
        <!---->
        </div>
        </div>
        </div>

        <div class="[&amp;&gt;*:last-child]:pb-5">
        <!--?lit$063844234$--><ul class="flex flex-col gap-1 list-none w-full py-2 bg-[\#265667]">
        <!--?lit$063844234$--><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Word of the Day" link="/word-of-the-day" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/word-of-the-day" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Word of the Day"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Word of the Day</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Grammar" link="/grammar" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/grammar" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Grammar"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Grammar</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Wordplay" link="/wordplay" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/wordplay" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Wordplay"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Wordplay</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Slang" link="/slang" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/slang" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Slang"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Slang</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Rhymes" link="/rhymes" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/rhymes" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Rhymes"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Rhymes</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Word Finder" link="/wordfinder/" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/wordfinder/" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Word Finder"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Word Finder</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Newsletters" link="/newsletters" icon="new" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/newsletters" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Newsletters"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Newsletters</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-new-badge class="flex items-center"><template shadowrootmode="open"><!---->
        <svg width="28" height="14" viewBox="0 0 28 14" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect width="28" height="14" rx="2" fill="\#F7EC48"></rect>
        <path d="M9.23145 11H7.30762L4.20215 5.59961H4.1582C4.17122 5.82422 4.18262 6.05046 4.19238 6.27832C4.20215 6.50618 4.21191 6.73405 4.22168 6.96191C4.23145 7.18652 4.24121 7.41276 4.25098 7.64062V11H2.89844V3.86133H4.80762L7.9082 9.20801H7.94238C7.93587 8.98665 7.92773 8.76693 7.91797 8.54883C7.9082 8.33073 7.89844 8.11263 7.88867 7.89453C7.88216 7.67643 7.87565 7.45833 7.86914 7.24023V3.86133H9.23145V11ZM15.1445 11H11.0332V3.86133H15.1445V5.10156H12.5469V6.66895H14.9639V7.90918H12.5469V9.75H15.1445V11ZM25.4082 3.86133L23.5918 11H21.8682L20.9014 7.25C20.8818 7.17839 20.8558 7.06934 20.8232 6.92285C20.7907 6.77637 20.7565 6.61686 20.7207 6.44434C20.6849 6.26855 20.6523 6.10417 20.623 5.95117C20.597 5.79492 20.5791 5.67122 20.5693 5.58008C20.5596 5.67122 20.54 5.79329 20.5107 5.94629C20.4847 6.09928 20.4538 6.26204 20.418 6.43457C20.3854 6.6071 20.3529 6.76823 20.3203 6.91797C20.2878 7.06771 20.2617 7.18164 20.2422 7.25977L19.2803 11H17.5615L15.7402 3.86133H17.2295L18.1426 7.75781C18.1686 7.875 18.1979 8.01497 18.2305 8.17773C18.2663 8.34049 18.3005 8.51139 18.333 8.69043C18.3688 8.86621 18.3997 9.03711 18.4258 9.20312C18.4551 9.36589 18.4762 9.50749 18.4893 9.62793C18.5055 9.50423 18.5267 9.361 18.5527 9.19824C18.5788 9.03223 18.6064 8.86458 18.6357 8.69531C18.6683 8.52279 18.7008 8.36328 18.7334 8.2168C18.766 8.07031 18.7952 7.9515 18.8213 7.86035L19.8613 3.86133H21.292L22.332 7.86035C22.3548 7.94824 22.3809 8.06706 22.4102 8.2168C22.4427 8.36328 22.4753 8.52279 22.5078 8.69531C22.5404 8.86784 22.5697 9.03711 22.5957 9.20312C22.625 9.36589 22.6462 9.50749 22.6592 9.62793C22.682 9.46517 22.7129 9.26823 22.752 9.03711C22.7943 8.80273 22.8382 8.56999 22.8838 8.33887C22.9326 8.10775 22.9749 7.91406 23.0107 7.75781L23.9189 3.86133H25.4082Z" fill="\#132631"></path>
        </svg>
        </template></mw-icon-new-badge>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Thesaurus" link="/thesaurus/" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="/thesaurus/" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Thesaurus"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Thesaurus</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!----><!---->
        <!--?lit$063844234$--> <li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" img="" label="Join MWU" link="https://premium.britannica.com/mw-unabridged/?utm_source=mw&amp;utm_medium=global-nav-join&amp;utm_campaign=evergreen" icon="hidden" type="heading"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="" href="https://premium.britannica.com/mw-unabridged/?utm_source=mw&amp;utm_medium=global-nav-join&amp;utm_campaign=evergreen" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_top">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Join MWU"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Join MWU</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li>
        <!---->
        </ul>
        <!--?lit$063844234$-->
        <ul class="flex flex-col gap-1 list-none pt-5 w-full bg-[\#265667]">
        <li class="w-full ">
        <mw-headings-collection class="w-full px-5" label="Shop" tag="header"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="header font-bold text-lg leading-none text-white font-['Playfair_Display']"><!--?lit$063844234$-->Shop</span>
        </template></mw-headings-collection>
        <div class="w-full pl-5">
        <hr class="h-px border-0 bg-[\#C5A332] my-2">
        </div>
        </li>
        <!--?lit$063844234$--><!----><li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" id="gtm-gamesnav-link-shop-books" img="" label="Books" link="https://shop.merriam-webster.com/?utm_source=mwsite&amp;utm_medium=nav&amp;utm_content=header" icon="out" type="heading" external=""><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-shop-books" href="https://shop.merriam-webster.com/?utm_source=mwsite&amp;utm_medium=nav&amp;utm_content=header" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_blank">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Books"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Books</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-link-out class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M14.4926 6.0625H16.8346L11.0479 11.8536L12.1519 12.9584L17.9276 7.17846V9.5H19.4889V5.61596L19.5 5.60485L19.4889 5.59375V4.5H14.4926V6.0625ZM4.5 7H10.9952V8.5625H6.06134V17.9997H15.4294V12.9997H16.9907V19.4991L4.5 19.4997V7Z" fill="\#CBE1EA"></path>
        </svg>
        </template></mw-icon-link-out>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li><!----><!----><li class="px-2 w-full">
        <mw-menu-item class="block hover:bg-[\#0F3850] hover:shadow w-full px-2 py-2" id="gtm-gamesnav-link-shop-merch" img="" label="Merch" link="https://merriamwebster.threadless.com/?utm_source=mwsite&amp;utm_medium=nav&amp;utm_content=header" icon="out" type="heading" external=""><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--> <div part="title" class="flex w-full font-['Open_Sans']
              text-white
              " style="">
        <a part="menu-link" rel="external" id="gtm-gamesnav-link-shop-merch" href="https://merriamwebster.threadless.com/?utm_source=mwsite&amp;utm_medium=nav&amp;utm_content=header" class="p-1 gap-2 flex flex-row items-center w-full
                  justify-between

                  " target="_blank">
        <span part="heading-container" class="flex gap-2 self-start items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->

        <!--?lit$063844234$--><mw-headings-collection part="menu-heading" tag="menu-heading" label="Merch"><template shadowrootmode="open"><!---->
        <!--?lit$063844234$--><span part="text" class="menu-heading font-bold leading-none text-white font-['Open_Sans']"><!--?lit$063844234$-->Merch</span>
        </template></mw-headings-collection>
        </span>

        <span part="icon" class="flex items-center">
        <!--?lit$063844234$-->
        <!--?lit$063844234$--><mw-icon-link-out class="flex items-center"><template shadowrootmode="open"><!---->
        <svg part="icon" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M14.4926 6.0625H16.8346L11.0479 11.8536L12.1519 12.9584L17.9276 7.17846V9.5H19.4889V5.61596L19.5 5.60485L19.4889 5.59375V4.5H14.4926V6.0625ZM4.5 7H10.9952V8.5625H6.06134V17.9997H15.4294V12.9997H16.9907V19.4991L4.5 19.4997V7Z" fill="\#CBE1EA"></path>
        </svg>
        </template></mw-icon-link-out>

        </span>
        </a>
        </div>
        </template></mw-menu-item>
        </li><!---->
        </ul>

        </div>

        <div class="pb-5 bg-[\#0F3850] flex pb-30">
        <!--?lit$063844234$--><ul class="flex flex-col gap-1 list-none pb-10 w-full">
        <!--?lit$063844234$-->
        <li class="pb-10 pt-3">
        <mw-login><template shadowrootmode="open"><!---->
        <div part="container" class="login-nav w-full">
        <!--?lit$063844234$-->
        <!--?lit$063844234$-->
        <div part="login-container" class="w-full mt-2 flex flex-row gap-2 justify-center self-center items-center content-center">
        <mw-button-menu-login part="login-left" label="Log in" id=""><template shadowrootmode="open"><!---->
        <button part="button" type="button" id="" class="min-w-[100px] w-full border-black border-2 bg-gradient-to-b from-[\#4A7D95] to-[\#225F73] border font-['Open_Sans'] text-white text-center font-bold py-1 px-4 rounded @hover:from-transparent @hover:to-transparent @hover:bg-transparent">
        <!--?lit$063844234$-->Log in
        </button>
        </template></mw-button-menu-login>
        <mw-button-menu-login part="login-right" label="Sign up" id=""><template shadowrootmode="open"><!---->
        <button part="button" type="button" id="" class="min-w-[100px] w-full border-black border-2 bg-gradient-to-b from-[\#4A7D95] to-[\#225F73] border font-['Open_Sans'] text-white text-center font-bold py-1 px-4 rounded @hover:from-transparent @hover:to-transparent @hover:bg-transparent">
        <!--?lit$063844234$-->Sign up
        </button>
        </template></mw-button-menu-login>
        </div>

        </div>
        </template></mw-login>
        </li>

        </ul>
        </div>
        </div>
        </div>
        </template></mw-menu-games-mobile>

        <script>
        if (window.mwdata.importModules === undefined) {
        window.mwdata.importModules  = [];
        }
        window.mwdata.importModules.push(["games/menu", function(m) { m.init('menu-games-mobile', '/dist-cross-dungarees/2025-12-05--21-50-13-sc56n'); }]);
        let menu = document.getElementById('menu-games-mobile');
        if (menu) {
        menu.gamesSubmenuOpen = false;
        }
        </script>


        <div class="outer-container ">
        <div class="main-container">

        <div id="main-banner-ad-container" class="container-top-ads bg-mw-black w-100 position-relative ">
        <div class="container"><div class="cafemedia-ad-slot-top h-100-px d-flex justify-content-center align-items-center"><div id="AdThrive_Header_1_desktop" class="adthrive-ad adthrive-header adthrive-header-1 adthrive-ad-cls adthrive-header-desktop" data-google-query-id="CNrp-JmPrJEDFRpLkQUdNfw3qA"><div id="google_ads_iframe_/18190176,15510053/AdThrive_Header_1/61575e8e934c48ea554b3caa_2__container__" style="border: 0pt none;"><iframe id="google_ads_iframe_/18190176,15510053/AdThrive_Header_1/61575e8e934c48ea554b3caa_2" name="google_ads_iframe_/18190176,15510053/AdThrive_Header_1/61575e8e934c48ea554b3caa_2" title="3rd party ad content" width="1" height="1" scrolling="no" marginwidth="0" marginheight="0" frameborder="0" aria-label="Advertisement" tabindex="0" allow="private-state-token-redemption;attribution-reporting" data-load-complete="true" data-google-container-id="b" style="border: 0px; vertical-align: bottom;"></iframe></div></div></div></div>
        </div>

        <div class="main-wrapper clearfix wrapper-1136  wordfinder-search-results-container">
        <div class="lr-cols-area sticky-column d-flex">
        <div class="left-content">
        <div class="row">
        <div class="col">
        <div class="row">
        <div class="col">
        <div class="mt-4 mt-md-5">
        <nav aria-label="breadcrumb" style="--bs-breadcrumb-divider:'';">
        <ol class="breadcrumb mb-0">
        <li class="breadcrumb-item"><a href="/wordfinder">Word Finder</a></li>
        <li class="breadcrumb-item active" aria-current="page">Results</li>
        </ol>
        </nav>                </div>
        </div>
        </div>

        <div>
        <h1>6-Letter Words Including <span class="ps-2">A_&nbsp;_&nbsp;_E_</span></h1>
        </div>

        <div class="row">
        <div class="col left-content-well-col">
        <div class="wordfinder-well wordfinder-main-form-section pt-4 pb-4">
        <div class="fill-in-blanks-form-container">
        <div class="text-center">
        <h3 id="fill-in-blanks-label" class="form-container-header fw-bold m-0">
        Fill-in-the-Blanks Search
        </h3>
        </div>
        <form name="fill-in-blanks-search-form" class="wf-form" autocomplete="off" aria-labelledby="fill-in-blanks-label">
        <div class="d-flex align-items-center flex-wrap justify-content-center">
        <div class="form-select-container mt-3 me-3">
        <span id="number-of-letters-label" class="visually-hidden">Choose number of letters</span>
        <select id="number-of-letters-fill-in-blanks-select" class="form-select w-auto" aria-labelledby="number-of-letters-label">
        <option value="2">2 Letters</option>
        <option value="3">3 Letters</option>
        <option value="4">4 Letters</option>
        <option value="5" selected="">5 Letters</option>
        <option value="6">6 Letters</option>
        <option value="7">7 Letters</option>
        <option value="8">8 Letters</option>
        <option value="9">9 Letters</option>
        <option value="10">10 Letters</option>
        <option value="11">11 Letters</option>
        <option value="12">12 Letters</option>
        <option value="13">13 Letters</option>
        <option value="14">14 Letters</option>
        <option value="15">15 Letters</option>
        </select>
        </div>
        <div class="fill-in-blanks-overflow-container">
        <span id="fill-in-blanks-input-group-label" class="visually-hidden">Fill in the blanks</span>
        <div id="fill-in-blanks-input-group" class="d-flex flex-nowrap mt-3 ms-2 me-3">
        <input id="fill-in-blanks-input-1" type="text" class="form-control fill-in-blanks-input text-uppercase focus-first" autocomplete="off" spellcheck="false" aria-labelledby="fill-in-blanks-input-group-label">
        <input id="fill-in-blanks-input-2" type="text" class="form-control fill-in-blanks-input text-uppercase" autocomplete="off" spellcheck="false" aria-labelledby="fill-in-blanks-input-group-label">
        <input id="fill-in-blanks-input-3" type="text" class="form-control fill-in-blanks-input text-uppercase" autocomplete="off" spellcheck="false" aria-labelledby="fill-in-blanks-input-group-label">
        <input id="fill-in-blanks-input-4" type="text" class="form-control fill-in-blanks-input text-uppercase" autocomplete="off" spellcheck="false" aria-labelledby="fill-in-blanks-input-group-label">
        <input id="fill-in-blanks-input-5" type="text" class="form-control fill-in-blanks-input text-uppercase" autocomplete="off" spellcheck="false" aria-labelledby="fill-in-blanks-input-group-label">
        <input type="text" class="form-control fill-in-blanks-input text-uppercase" autocomplete="off" spellcheck="false" aria-labelledby="fill-in-blanks-input-group-label" id="fill-in-blanks-input-6"></div>
        </div>
        <div class="form-submit-btn-row ms-2">
        <button id="fill-in-blanks-form-submit-btn" type="button" class="btn form-submit-btn border-rounded">
        <svg width="34" height="34" viewBox="0 0 34 34" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_1720_87172)">
        <path d="M22.8129 20.6449L30.2266 27.7934C30.6361 28.1882 30.6361 28.8283 30.2266 29.2231L28.7439 30.6528C28.3344 31.0476 27.6706 31.0476 27.2611 30.6528L19.8474 23.5043C19.4379 23.1095 19.4379 22.4694 19.8474 22.0746L21.3301 20.6449C21.7396 20.2501 22.4034 20.2501 22.8129 20.6449ZM15.4542 24.2769C9.37414 24.2769 4.44531 19.5244 4.44531 13.6619C4.44531 7.79937 9.37414 3.04688 15.4542 3.04688C21.5342 3.04688 26.463 7.79937 26.463 13.6619C26.463 19.5244 21.5342 24.2769 15.4542 24.2769ZM15.4542 21.244C19.797 21.244 23.3176 17.8494 23.3176 13.6619C23.3176 9.47437 19.797 6.07973 15.4542 6.07973C11.1113 6.07973 7.5907 9.47437 7.5907 13.6619C7.5907 17.8494 11.1113 21.244 15.4542 21.244Z" fill="currentColor"></path>
        </g>
        <defs>
        <clipPath id="clip0_1720_87172">
        <rect width="26.0884" height="27.902" fill="currentColor" transform="translate(4.44531 3.04688)"></rect>
        </clipPath>
        </defs>
        </svg>
        <span class="visually-hidden">Search</span>
        </button>
        </div>
        </div>
        </form>
        </div>
        <div class="mt-5">
        <div class="wordfinder-panel">
        <div class="panel-body">
        <div class="d-flex justify-content-center">
        <div id="dataset-radio-btns-input" data-disable-alternate-input="" class="radio radio-btn-group btn-group" role="group" aria-label="All or common words only">
        <input type="radio" class="btn-check" name="fill-in-blanks-search-words-radio-input" id="fill-in-blanks-search-all-words-radio-input" value="all" autocomplete="off">
        <label class="btn btn-outline-primary fw-bold" for="fill-in-blanks-search-all-words-radio-input">
        All words <span class="badge mw-badge-blue-25 no-hover-effect" title="235">235</span>
        </label>
        <input type="radio" class="btn-check" name="fill-in-blanks-search-words-radio-input" id="fill-in-blanks-search-common-words-radio-input" value="common" autocomplete="off">
        <label class="btn btn-outline-primary fw-bold" for="fill-in-blanks-search-common-words-radio-input">
        Common <span class="badge mw-badge-blue-25 no-hover-effect" title="30">30</span>
        </label>
        </div>                </div>
        <ul class="paginated-list-results lt-10-words">
        <li class="pb-4 d-flex">
        <a href="/dictionary/abased" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abased</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abases" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abases</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abated" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abated</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abater" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abater</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abates" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abates</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abazes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abazes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abeles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abeles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abided" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abided</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abider" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abider</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abides" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abides</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abodes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abodes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aboves" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aboves</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abreed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abreed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abused" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abused</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abusee" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abusee</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abuser" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abuser</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/abuses" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">abuses</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acater" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acater</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acates" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acates</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Acaxee" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Acaxee</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/achier" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">achier</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/achter" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">achter</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ackees" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ackees</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acknew" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acknew</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acmaea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acmaea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acquet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acquet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Actaea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Actaea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aculea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aculea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aculei" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aculei</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acumen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acumen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/acuter" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">acuter</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adages" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adages</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adawed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adawed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/addies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">addies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/addled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">addled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/addles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">addles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Adelea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Adelea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adipes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adipes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adives" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adives</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adobes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adobes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adored" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adored</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adorer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adorer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/adores" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">adores</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aerier" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aerier</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aeries" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aeries</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aether" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aether</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/affeer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">affeer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/affied" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">affied</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/affies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">affies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/afreet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">afreet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Afroed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Afroed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agaces" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agaces</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agapes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agapes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agates" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agates</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agaves" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agaves</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agazed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agazed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agenes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agenes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aggies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aggies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agoges" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agoges</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agones" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agones</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agreed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agreed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/agrees" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">agrees</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aholes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aholes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aidmen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aidmen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aiglet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aiglet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ailded" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ailded</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/airier" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">airier</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/airmen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">airmen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/AIRMET" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">AIRMET</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/airted" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">airted</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aisled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aisled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aisles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aisles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aizles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aizles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ajimez" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ajimez</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/akeley" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">akeley</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/akules" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">akules</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alares" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alares</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alates" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alates</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Alfven" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Alfven</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alined" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alined</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alines" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alines</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aliped" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aliped</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aliter" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aliter</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alites" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alites</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alkies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alkies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/allees" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">allees</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/allied" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">allied</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/allies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">allies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Alopex" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Alopex</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alpeen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alpeen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/althea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">althea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aludel" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aludel</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alulet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alulet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alumen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alumen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/alures" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">alures</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Alytes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Alytes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amated" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amated</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amates" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amates</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amazed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amazed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amazes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amazes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ambeer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ambeer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ambled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ambled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ambler" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ambler</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ambles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ambles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amices" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amices</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amides" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amides</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amines" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amines</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amoles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amoles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amoved" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amoved</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amoves" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amoves</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ampler" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ampler</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Amsler" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Amsler</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amulet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amulet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amused" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amused</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amuser" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amuser</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/amuses" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">amuses</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anadem" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anadem</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anagen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anagen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ancred" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ancred</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ancree" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ancree</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aneled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aneled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aneles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aneles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Anezeh" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Anezeh</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/angled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">angled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/angler" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">angler</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/angles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">angles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/animes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">animes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anises" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anises</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anisey" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anisey</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ankles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ankles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anklet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anklet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anodes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anodes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anoles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anoles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anomer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anomer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/answer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">answer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anthem" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anthem</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anther" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anther</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/antler" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">antler</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/antres" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">antres</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/anuses" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">anuses</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apelet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apelet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aperea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aperea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apexed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apexed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apexes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apexes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apices" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apices</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apnoea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apnoea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apodes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apodes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apogee" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apogee</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apozem" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apozem</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Appies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Appies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apples" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apples</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/applet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">applet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/appley" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">appley</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/apuses" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">apuses</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Arales" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Arales</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Aranea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Aranea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arched" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arched</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/archei" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">archei</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/archer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">archer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arches" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arches</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arenes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arenes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/areres" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">areres</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aretes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aretes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/argued" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">argued</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arguer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arguer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/argues" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">argues</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arider" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arider</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arisen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arisen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arises" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arises</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Arkies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Arkies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/armies" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">armies</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/armlet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">armlet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arnees" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arnees</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arries" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arries</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arsled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arsled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/arsles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">arsles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/artier" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">artier</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ashier" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ashier</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/ashmen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">ashmen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/asiden" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">asiden</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/asides" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">asides</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/asleep" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">asleep</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/asteep" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">asteep</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/asteer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">asteer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/astrer" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">astrer</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/astres" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">astres</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Astrex" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Astrex</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/asylee" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">asylee</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atabeg" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atabeg</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atabek" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atabek</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Ateles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Ateles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atises" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atises</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atlees" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atlees</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atokes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atokes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atoles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atoles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atoned" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atoned</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atones" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atones</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atopen" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atopen</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atules" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atules</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atweel" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atweel</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/atween" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">atween</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/auklet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">auklet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aulder" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aulder</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/auncel" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">auncel</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/auspex" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">auspex</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/autoed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">autoed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/avaled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">avaled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/avales" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">avales</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aviled" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aviled</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/aviles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">aviles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/avocet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">avocet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/avowed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">avowed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/avower" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">avower</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Avoyel" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Avoyel</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/awaked" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">awaked</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/awaken" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">awaken</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/awakes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">awakes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/awheel" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">awheel</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/awnlet" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">awnlet</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/awoken" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">awoken</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/axised" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">axised</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/axites" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">axites</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/axones" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">axones</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/axseed" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">axseed</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azalea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azalea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Azazel" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Azazel</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azides" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azides</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azines" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azines</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azoles" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azoles</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azotea" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azotea</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azotes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azotes</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/Azrael" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">Azrael</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azured" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azured</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azures" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azures</a>
        </li>
        <li class="pb-4 d-flex">
        <a href="/dictionary/azymes" target="_blank" class="mw-badge-blue-25 badge text-decoration-none">azymes</a>
        </li>
        </ul>
        </div>
        </div>                                            </div>
        </div>
        <div class="d-flex mt-4 wordfinder-well wordfinder-classic-search-well wordfinder-alternate-form-section flex-column justify-content-center pb-4 pt-4 internal-search-tool-form-blue">
        <div class="classic-search-form-container">
        <form name="classic-search-form" class="wf-form" autocomplete="off">
        <div class="d-flex flex-wrap align-items-center justify-content-center">
        <div class="d-flex flex-wrap form-select-container-row mt-3">
        <div class="form-select-container">
        <span id="number-of-letters-label" class="visually-hidden">Choose number of letters</span>
        <select id="number-of-letters-classic-search" class="form-select me-2" aria-labelledby="number-of-letters-label">
        <option value="-1">Any Words</option>
        <option value="2">2-Letter Words</option>
        <option value="3">3-Letter Words</option>
        <option value="4">4-Letter Words</option>
        <option value="5" selected="">5-Letter Words</option>
        <option value="6">6-Letter Words</option>
        <option value="7">7-Letter Words</option>
        <option value="8">8-Letter Words</option>
        <option value="9">9-Letter Words</option>
        <option value="10">10-Letter Words</option>
        <option value="11">11-Letter Words</option>
        <option value="12">12-Letter Words</option>
        <option value="13">13-Letter Words</option>
        <option value="14">14-Letter Words</option>
        <option value="15">15-Letter Words</option>
        </select>
        </div>
        <div id="search-constraints-select-container" class="form-select-container">
        <span id="letter-placement-label" class="visually-hidden">Select letter placement in the word</span>
        <select id="search-constraints-select" class="form-select me-2" aria-labelledby="letter-placement-label">
        <option value="begins">Starting with</option>
        <option value="ends">Ending with</option>
        <option value="any-order">Including</option>
        <option value="contains">Containing in order</option>
        </select>
        <span class="select-helper-element position-absolute" aria-hidden="true"></span>
        </div>
        </div>
        <div class="letter-search-input-group input-group d-flex flex-nowrap justify-content-center ms-0 mt-3">
        <span id="letters-search-label" class="visually-hidden">Add the letters for the search</span>
        <input id="classic-search-form-input" type="text" class="form-control text-uppercase focus-first " autocomplete="off" spellcheck="false" placeholder="These letters" aria-labelledby="letters-search-label" required="">
        <button id="classic-search-form-submit-btn" type="button" class="btn form-submit-btn">
        <svg width="34" height="34" viewBox="0 0 34 34" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(\#clip0_1720_87172)">
        <path d="M22.8129 20.6449L30.2266 27.7934C30.6361 28.1882 30.6361 28.8283 30.2266 29.2231L28.7439 30.6528C28.3344 31.0476 27.6706 31.0476 27.2611 30.6528L19.8474 23.5043C19.4379 23.1095 19.4379 22.4694 19.8474 22.0746L21.3301 20.6449C21.7396 20.2501 22.4034 20.2501 22.8129 20.6449ZM15.4542 24.2769C9.37414 24.2769 4.44531 19.5244 4.44531 13.6619C4.44531 7.79937 9.37414 3.04688 15.4542 3.04688C21.5342 3.04688 26.463 7.79937 26.463 13.6619C26.463 19.5244 21.5342 24.2769 15.4542 24.2769ZM15.4542 21.244C19.797 21.244 23.3176 17.8494 23.3176 13.6619C23.3176 9.47437 19.797 6.07973 15.4542 6.07973C11.1113 6.07973 7.5907 9.47437 7.5907 13.6619C7.5907 17.8494 11.1113 21.244 15.4542 21.244Z" fill="currentColor"></path>
        </g>
        <defs>
        <clipPath id="clip0_1720_87172">
        <rect width="26.0884" height="27.902" fill="currentColor" transform="translate(4.44531 3.04688)"></rect>
        </clipPath>
        </defs>
        </svg>
        <span class="visually-hidden">Search</span>
        </button>
        </div>
        </div>
        </form>
        </div>                </div>
        </div>
        </div>
        </div>
        </div>        </div>

        <div class="right-rail position-relative d-none d-md-flex align-items-center flex-column">
        <div id="mw-ad-slot-right-1" class="abl abl-auto-250-nr d-flex align-items-center justify-content-center"><div id="AdThrive_Sidebar_1_desktop" class="adthrive-ad adthrive-sidebar adthrive-sidebar-1 adthrive-ad-cls" data-google-query-id="CMmo9ZmPrJEDFRjrjgkd6uwtyQ"><div id="google_ads_iframe_/18190176,15510053/AdThrive_Sidebar_1/61575e8e934c48ea554b3caa_2__container__" style="border: 0pt none;"><iframe id="google_ads_iframe_/18190176,15510053/AdThrive_Sidebar_1/61575e8e934c48ea554b3caa_2" name="google_ads_iframe_/18190176,15510053/AdThrive_Sidebar_1/61575e8e934c48ea554b3caa_2" title="3rd party ad content" width="1" height="1" scrolling="no" marginwidth="0" marginheight="0" frameborder="0" aria-label="Advertisement" tabindex="0" allow="private-state-token-redemption;attribution-reporting" data-load-complete="true" data-google-container-id="a" style="border: 0px; vertical-align: bottom;"></iframe></div></div></div>
        <div class="wgt-side">
        <div class="games-landing-page-redesign-container" style="clear: both">
        <div class="games-landing-section">
        <div class="games-landing-section-row px-0">
        <div class="row">
        <div class="w-100 large-view">
        <a href="https://www.merriam-webster.com/games/quordle/" data-nosnippet="" class="text-decoration-none games-landing-section-col games-landing-section-col-responsive games-landing-section-col-featured  position-relative mb-0 py-3 py-sm-0 py-md-3 py-lg-0 ">

        <div class="img-link">




        <span class="lazyload-container ratio-4-3"><img data-sizes="auto" src="https://merriam-webster.com/assets/mw/static/images/games/external/quordle/485x364@1x.jpg" srcset="https://merriam-webster.com/assets/mw/static/images/games/external/quordle/485x364@1x.jpg 1x, https://merriam-webster.com/assets/mw/static/images/games/external/quordle/485x364@2x.jpg 2x" alt="Play Quordle: Guess all four words in a limited number of tries.  Each of your guesses must be a real 5-letter word." data-src="https://merriam-webster.com/assets/mw/static/images/games/external/quordle/485x364@1x.jpg" data-srcset="https://merriam-webster.com/assets/mw/static/images/games/external/quordle/485x364@1x.jpg 1x, https://merriam-webster.com/assets/mw/static/images/games/external/quordle/485x364@2x.jpg 2x" data-dim="485x364" class=""></span>




        </div>
        <div class="games-landing-section-img-panel-overlay">
        <div>
        <h3 class="games-landing-section-col-text fw-normal font-weight-normal">Can you solve 4 words at once?</h3>
        <div class="games-landing-section-col-nav-btn-container ">
        <button class="games-landing-d-sm-hide gold">Play</button>
        <button class="games-landing-d-sm-only gold">Play</button>
        </div>
        </div>
        </div>
        <h3 class="games-landing-section-col-text my-3 games-landing-d-sm-hide">Can you solve 4 words at once?    </h3>
        <div class="games-landing-section-col-nav-btn-container games-landing-d-sm-hide">
        <button class="games-landing-d-sm-hide gold">Play</button>
        <button class="games-landing-d-sm-only gold">Play</button>
        </div>


        </a>
        </div>
        </div>
        </div>
        </div>
        </div>                    </div>
        <div class="wotd-side wgt-side wgt-wod-side top-location">

        <div class="wotd-header d-flex align-items-center justify-content-evenly">
        <div class="d-flex flex-column text-center justify-content-center me-2 mr-2">
        <a href="/" title="Merriam Webster" class="mw-logo text-decoration-none">
        <svg width="57" height="57" viewBox="0 0 57 57" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M0 28.5C0 12.7559 12.8041 0 28.6006 0C44.397 0 57 12.762 57 28.5C57 44.2441 44.397 57 28.6006 57C12.8102 57.0061 0 44.2441 0 28.5Z" fill="white"></path>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M28.5 56.6686C44.0571 56.6686 56.6686 44.0571 56.6686 28.5C56.6686 12.9429 44.0571 0.331395 28.5 0.331395C12.9429 0.331395 0.331395 12.9429 0.331395 28.5C0.331395 44.0571 12.9429 56.6686 28.5 56.6686ZM28.5 57C44.2401 57 57 44.2401 57 28.5C57 12.7599 44.2401 0 28.5 0C12.7599 0 0 12.7599 0 28.5C0 44.2401 12.7599 57 28.5 57Z" fill="\#D71920"></path>
        <path d="M2.08537 28.4971C2.08537 13.9089 13.9149 2.08545 28.503 2.08545C43.0911 2.08545 54.9146 13.9089 54.9146 28.4971C54.9146 43.0852 43.0911 54.9087 28.503 54.9147C13.9149 54.9147 2.09139 43.0852 2.08537 28.4971ZM3.81402 28.4971C3.81402 35.3153 6.57865 41.483 11.0479 45.9522C15.517 50.4214 21.6848 53.18 28.503 53.1861C35.3213 53.1861 41.489 50.4214 45.9582 45.9522C50.4274 41.483 53.192 35.3153 53.192 28.4971C53.192 21.6788 50.4274 15.5111 45.9582 11.0419C41.489 6.57272 35.3213 3.80808 28.503 3.80808C21.6788 3.80808 15.517 6.56669 11.0479 11.0419C6.57865 15.5111 3.81402 21.6788 3.81402 28.4971Z" fill="\#D71920"></path>
        <path d="M31.3917 34.9741C31.3917 34.3001 31.8774 33.8872 32.4421 33.8811C33.2193 33.8811 33.5958 34.4823 33.5958 35.2716L34.0147 35.2959L34.0633 34.0815H34.039L34.0572 34.0633C33.5775 33.6322 33.0371 33.5168 32.4239 33.5168C31.4706 33.5168 30.5841 34.1726 30.5841 35.2837C30.5962 37.5911 33.5897 36.6014 33.5654 38.174C33.5654 38.8965 33.11 39.4126 32.4057 39.4126H32.3753C31.4888 39.3944 30.9363 38.6658 30.912 37.7125L30.4444 37.6882V37.7125L30.4566 37.8279L30.578 39.4308C31.1184 39.7405 31.7135 39.8195 32.2903 39.8195C33.3954 39.8195 34.5005 39.2062 34.5005 37.8704C34.4823 35.6602 31.3613 36.3038 31.3917 34.9741ZM27.6817 33.5775C27.4874 33.5775 27.3053 33.614 27.111 33.6383L26.097 34.2819L26.2427 34.4033C26.5159 34.3183 26.7892 34.2697 27.0503 34.2697C28.4589 34.2697 29.0843 35.5813 29.0904 36.911C29.0904 38.2893 28.4893 39.4551 27.2446 39.4551C26.4309 39.4551 25.9391 38.8904 25.9391 38.0768V29.9951L25.6902 29.8858C25.1801 30.1105 24.5608 30.3109 23.99 30.3837L23.9779 30.6145C24.3483 30.6388 24.6154 30.6995 24.9372 30.8573L24.913 37.3057C24.913 37.5971 24.913 37.8825 24.913 38.174C24.913 38.6901 24.9069 39.2062 24.8705 39.7466L25.1012 39.8741L25.5687 39.4248C25.9573 39.6555 26.4431 39.8377 26.8863 39.8437C29.0904 39.8377 30.0437 38.259 30.0437 36.5467C30.0437 34.6037 29.1754 33.5775 27.6817 33.5775ZM20.6809 31.3067L20.6991 30.9909L17.7481 30.9666L17.7238 31.2824L18.8775 31.592L18.7075 32.0839L16.7766 37.5425C16.5641 36.911 14.9854 32.1446 14.8761 31.7378L15.0219 31.507L15.9812 31.3613L15.9994 31.0152L12.8117 30.997V31.0213H12.7874V31.3431L13.6617 31.5435L13.8196 31.8349C13.7103 32.2417 12.0588 36.9414 11.8341 37.5668L9.92146 31.6103L9.97611 31.592L10.9051 31.3613L10.9476 31.0152H10.9233H10.808L7.63236 30.9849L7.602 31.3067L8.73745 31.586L11.3544 39.8559L11.7552 39.8741L14.1414 33.2982L16.3516 39.8377L16.7766 39.8559L19.6668 31.5738L20.6809 31.3067ZM9.90325 31.6224L9.98218 31.8653L9.90325 31.6224ZM18.8836 31.5738L18.8107 31.5556L18.8836 31.5738ZM38.344 38.4472L38.3075 38.4776C38.0343 38.7508 37.6093 39.1758 37.2692 39.1758C36.6196 39.1637 36.3706 38.7144 36.3645 37.9007C36.3645 37.8643 36.3645 37.8279 36.3645 37.7914V34.5491H36.407L38.1315 34.5733L38.1557 33.9722L36.3403 33.9479L36.3645 32.3996L36.0549 32.3753C35.8059 33.2436 35.1198 33.7536 34.4519 34.2394L34.4397 34.5491H34.464V34.5733H35.3262L35.302 38.2893C35.302 39.4491 36.0488 39.868 36.7228 39.8741C36.7471 39.8741 36.7774 39.8741 36.8017 39.8741C37.755 39.8255 38.4047 38.8783 38.4108 38.8783L38.35 38.4897L38.344 38.4472ZM35.1623 27.8518H35.1927C35.9152 27.8335 36.4981 27.5603 37.0507 27.1292L37.0203 27.8639C37.0203 27.8639 37.0264 27.8639 37.0324 27.8639H37.0446C37.0446 27.8639 37.0628 27.8578 37.0689 27.8578C37.2935 27.8335 38.4533 27.5725 38.9755 27.4753L39.0119 27.4692L38.8297 27.1899L37.9857 27.1049V23.48C37.9857 22.4296 37.5303 21.956 36.7956 21.956C36.7592 21.956 36.7288 21.956 36.6924 21.956C36.5892 21.956 36.4799 21.962 36.3767 22.0167L35.0469 22.8121C34.9134 22.8971 34.4337 23.1582 34.4155 23.3889L34.373 24.0508L34.464 24.1236L35.1562 23.984C35.2412 23.9658 35.3687 23.9718 35.3687 23.8261V23.8079C35.3627 23.6925 35.3566 23.5832 35.3566 23.48C35.3627 23.0185 35.4659 22.6724 36.3038 22.551C36.3342 22.5449 36.3645 22.5449 36.3949 22.5449C36.8139 22.5449 37.0628 22.9882 37.0628 23.8443V24.7187C36.492 24.8947 35.8666 25.1437 35.1987 25.3623C34.6948 25.5262 33.9115 25.7691 33.9115 26.771C33.9115 27.2264 34.4519 27.8518 35.1623 27.8518ZM37.0446 27.8396H37.0385L37.0507 27.6635L37.0446 27.8396ZM37.0871 25.0162V25.0223C36.9778 25.0587 36.8806 25.089 36.7774 25.1194C36.8806 25.083 36.9778 25.0526 37.0871 25.0162ZM37.0871 25.0405V25.4291L37.0628 26.7285C36.7046 26.9531 36.1459 27.2385 35.8181 27.2385C35.3748 27.2385 34.9194 26.9288 34.9194 26.522C34.9134 25.7205 35.7695 25.4898 37.0871 25.0405ZM32.2903 20.5291C32.5696 20.5291 32.8307 20.3044 32.8368 20.0008C32.8368 19.6911 32.606 19.4118 32.2903 19.4058C31.9867 19.4058 31.7681 19.7094 31.7681 20.0008C31.7681 20.2801 32.0231 20.5291 32.2903 20.5291ZM31.8956 27.3296L30.8148 27.5057L30.7966 27.7971L33.8993 27.8214V27.7971H33.9236V27.5299L32.8428 27.3053L32.8611 22.1017L32.6303 21.9924C32.1506 22.2049 31.4767 22.3931 30.9423 22.466L30.9181 22.7574C31.2642 22.7817 31.6103 22.8425 31.9138 22.9942L31.8956 27.3296ZM23.3221 27.2871L22.2838 27.4632L22.2656 27.8032L25.3319 27.8275L25.3562 27.4874L24.2997 27.2628L24.3179 24.3544C24.3179 24.154 24.8644 22.8121 25.5019 22.8121C25.6416 22.8121 25.842 22.9153 25.9634 23.0064L26.2791 23.0368L26.5038 22.2413C26.2973 22.0652 25.8237 22.0349 25.5748 22.0349C24.9433 22.041 24.7551 22.7635 24.5061 23.2432L24.3483 23.4982V22.1078L24.069 21.9985L24.0629 22.0227L24.0508 21.9985C23.5832 22.211 22.891 22.3992 22.3445 22.4721L22.3203 22.7635C22.6785 22.7878 23.0307 22.8485 23.3343 23.0003L23.3221 27.2871ZM9.81217 27.4814L8.7253 27.2567L9.33249 20.6991L9.38714 20.5351L12.3077 27.5846L12.4717 27.5967L15.3133 20.608L15.3922 20.7537L15.9509 27.2749L14.785 27.451L14.7668 27.791L18.1124 27.8153L18.1367 27.4753L17.1227 27.2506L16.3941 20.013L17.5356 19.8369L17.5295 19.8126H17.5538L17.5417 19.5211L15.0583 19.4968L12.5749 25.5323L10.055 19.509L7.52307 19.4968L7.48664 19.8126L8.7253 20.0433L8.02704 27.281L7.06768 27.4571L7.04946 27.7971L9.78788 27.8214L9.81217 27.4814ZM48.0529 23.8747C48.059 22.6907 47.4579 22.0227 46.201 21.9863C46.1949 21.9863 46.1889 21.9863 46.1828 21.9863C46.031 21.9863 45.7881 22.047 45.4481 22.3142L44.5677 23.0671C44.2519 22.4721 43.6508 22.0045 42.9465 21.9985C42.485 21.9985 42.315 22.0592 41.9507 22.3446L41.1128 22.891V22.0956L40.8395 21.9863C40.372 22.1988 39.7101 22.3871 39.188 22.4599L39.1637 22.6846C39.5037 22.7089 39.8437 22.7696 40.1352 22.9214L40.1291 27.2628L39.2123 27.4389L39.194 27.7485L42.0296 27.7728L42.0539 27.4935L41.1006 27.2385L41.1188 23.3464C41.5378 23.0307 42.0114 22.6482 42.5093 22.6482C43.5719 22.6482 43.5962 23.7229 43.6022 24.6762V27.2628L42.6854 27.4692L42.6672 27.7485L45.5027 27.7728L45.527 27.4935L44.5737 27.2385L44.5919 23.3343C44.9988 23.0368 45.5452 22.721 46.0492 22.721C47.0086 22.721 47.0693 23.5286 47.0753 24.3847C47.0753 24.5304 47.0753 24.6701 47.0753 24.8158V27.2628L46.1585 27.4692L46.1403 27.7485L48.9333 27.7728L48.9576 27.4935L48.0468 27.2385L48.0529 23.8747ZM20.0979 27.955C20.6869 27.955 21.0998 27.876 21.1909 27.8214L22.5753 26.3824L22.1867 26.352C21.7374 26.8195 21.3002 27.2142 20.6626 27.2142C20.6323 27.2142 20.6019 27.2142 20.5716 27.2142C18.7682 27.1049 18.5193 25.7448 18.5132 24.1904H18.5496L21.6827 24.2147C22.0288 24.2147 22.1685 24.1722 22.1685 23.7168C22.1685 22.7028 21.3002 21.9985 20.0919 21.9985C18.665 21.9985 17.5174 23.0671 17.5174 24.9615C17.5235 26.8863 18.6589 27.9489 20.0979 27.955ZM19.9279 22.3506C20.7962 22.3506 21.1666 22.97 21.1666 23.7715C21.1666 23.814 21.1666 23.8625 21.1666 23.9111L18.5435 23.8868C18.6043 23.1278 19.1143 22.3506 19.9279 22.3506ZM30.8452 22.2353C30.6387 22.0592 30.1651 22.0288 29.9162 22.0288C29.2847 22.0349 29.0965 22.7574 28.8475 23.2371L28.6897 23.4921V22.1017L28.4104 21.9924L28.4043 22.0167L28.3921 21.9924C27.9246 22.2049 27.2324 22.3931 26.6859 22.466L26.6677 22.7574C27.026 22.7817 27.3781 22.8425 27.6817 22.9942L27.6696 27.2871L26.6313 27.4632L26.607 27.7971H26.6313H26.7102L29.6733 27.8214L29.6976 27.4814L28.6411 27.2567L28.6654 24.3483C28.6654 24.1479 29.2119 22.806 29.8433 22.806C29.983 22.806 30.1834 22.9092 30.3048 23.0003L30.6205 23.0307L30.8452 22.2353ZM45.6545 34.8162L45.4845 35.0894V33.6201L45.187 33.5108L45.1809 33.535L45.1688 33.5108C44.677 33.7354 43.9544 33.9358 43.3776 34.0087L43.3533 34.3183C43.7297 34.3426 44.1001 34.4033 44.4159 34.5673L44.4037 39.0787L43.3108 39.2669L43.2926 39.6191L46.5107 39.6434L46.535 39.2912L45.4299 39.0544L45.4481 35.9942C45.4481 35.7877 46.0188 34.373 46.6928 34.373C46.8385 34.373 47.0511 34.4823 47.1786 34.5794L47.5065 34.6098L47.7433 33.7718C47.5307 33.5897 47.0268 33.5593 46.7718 33.5593C46.116 33.5472 45.9217 34.3122 45.6545 34.8162ZM22.302 39.0726C22.2656 39.0726 22.2353 39.0726 22.2049 39.0726C20.2983 38.9572 20.0372 37.5182 20.0312 35.8788L23.3828 35.9031C23.7472 35.9031 23.8929 35.8545 23.8929 35.3748C23.8929 34.3062 22.976 33.5593 21.707 33.5593C20.2012 33.5654 18.9929 34.6887 18.9868 36.6864C18.9929 38.7204 20.189 39.8498 21.707 39.8498C22.3324 39.8498 22.7635 39.7648 22.8607 39.7102L24.324 38.1922L23.9172 38.1558C23.4375 38.6597 22.976 39.0726 22.302 39.0726ZM21.5248 33.9419C22.4417 33.9419 22.8364 34.5976 22.8364 35.4477C22.8364 35.4963 22.8364 35.5448 22.8364 35.5934L20.0615 35.5691C20.1283 34.7555 20.6626 33.9419 21.5248 33.9419ZM43.3776 38.1193C42.904 38.6172 42.4425 39.0301 41.7685 39.0301C41.7321 39.0301 41.7017 39.0301 41.6653 39.0301C39.7587 38.9147 39.4976 37.4757 39.4916 35.8363L42.8432 35.8606C43.2076 35.8606 43.3533 35.812 43.3533 35.3323C43.3533 34.2637 42.4364 33.5168 41.1674 33.5168C39.6616 33.5229 38.4472 34.6462 38.4472 36.6439C38.4472 38.684 39.6494 39.8073 41.1674 39.8073C41.7867 39.8073 42.2239 39.7223 42.3211 39.6677L43.7844 38.1497L43.3776 38.1193ZM40.9913 33.8994C41.9082 33.8994 42.3028 34.5551 42.3028 35.4052C42.3028 35.4538 42.3028 35.5023 42.2968 35.5509L39.5219 35.5266C39.5948 34.7191 40.1352 33.8994 40.9913 33.8994ZM48.9091 24.5729V25.3926H51.1374V24.5729H48.9091Z" fill="\#004990"></path>
        </svg>
        </a>
        <h4 class="wotd-side__title margin-t-0p9375em fs-6 m-0 p-0 mt-3">
        <a class="fw-bold text-uppercase wotd-header" style="letter-spacing:3px;" href="/word-of-the-day">Word of the Day</a>
        </h4>
        </div>
        </div>

        <div class="wotd-side__headword-wrapper d-flex justify-content-center align-items-center mt-4 mb-4">
        <h4 class="wotd-side__headword d-flex mt-0 mb-0 mr-3 me-2">
        <a style="letter-spacing:1.5px;" class="fs-2 fw-bold font-logo lh-base text-break text-wrap text-truncate" href="/word-of-the-day">enigmatic</a>
        </h4>
        <a class="wotd-side__headword__pron play-pron d-flex m-0 hoverable" data-lang="en_us" data-dir="e" data-file="enigma02" href="https://www.merriam-webster.com/wordfinder/fill-in-blanks/all/6/a___e_/1?pronunciation&amp;lang=en_us&amp;dir=e&amp;file=enigma02" title="Listen to the pronunciation of enigmatic">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="22px" height="22px" viewBox="0 0 26 26" version="1.1" data-inject-url="https://www.merriam-webster.com/dist-cross-dungarees/2025-12-05--21-50-13-sc56n/images/svg/audio.svg" class="svg replaced-svg"><title>Listen to the pronunciation of enigmatic</title>
        <defs>
        <linearGradient id="grad3" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" style="stop-color:rgb(38,86,103);stop-opacity:1"></stop>
        <stop offset="100%" style="stop-color:rgb(15,56,80);stop-opacity:1"></stop>
        </linearGradient>
        <linearGradient id="grad1" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" style="stop-color:rgb(231,240,244);stop-opacity:1"></stop>
        <stop offset="100%" style="stop-color:rgb(203,225,234);stop-opacity:1"></stop>
        </linearGradient>
        </defs>
        <g class="Audio" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
        <g class="Icon/AudioPron">
        <g class="Audio">
        <circle class="outline Oval" fill="url(\#grad3) \#265667" cx="13" cy="13" r="13"></circle>
        <polygon class="logo Path-2" stroke="\#FFFFFF" fill="\#FFFFFF" points="6 10.5031797 6 15.4563465 9.37102011 15.4563465 13 18 13 8.02789307 9.5 10.5031797"></polygon>
        <path class="logo2 Path-3" d="M16,8.5 C17.4887382,10.0718463 18.2331073,11.569707 18.2331073,12.993582 C18.2331073,15.1293945 17.1165537,16.4498291 16,17.5894775" stroke="\#FFFFFF"></path>
        <path class="logo2 Path-4" d="M18.0189209,6.5 C20.0989176,8.3652264 21.138916,10.5318931 21.138916,13 C21.138916,15.4681069 20.0723674,17.6756672 17.93927,19.6226807" stroke="\#FFFFFF"></path>
        </g>
        </g>
        </g>
        </svg>
        </a>
        </div>

        <p class="wotd-side__link mt-3 mb-5 p-0">
        <a href="/word-of-the-day">See Definitions and Examples</a> »
        </p>

        <div class="wotd-side__subscribe pt-2">
        <p class="wotd-side__subscribe__lead p-0 mt-3 mb-0">Get Word of the Day daily email!</p>
        <form class="js-wod-subscribe-frm wotd-side__subscribe__form" action="/word-of-the-day" method="post">
        <input type="submit" class="wod-submit wotd-side__subscribe__form__submit" name="wod-submit" value="SUBSCRIBE">
        <input type="text" class="wod-subscribe wotd-side__subscribe__form__input" name="wod-subscribe" placeholder="Your email address" aria-label="Subscribe to Word of the Day">
        </form>
        </div>

        </div>
        <div id="mw-ad-slot-sticky-container" class="position-sticky d-flex align-items-center justify-content-center abl abl-auto-250-sticky ">
        <div id="mw-ad-slot-sticky" class="mx-auto d-none d-md-inline-flex align-items-center justify-content-center"><div id="AdThrive_Sidebar_9_desktop" class="adthrive-ad adthrive-sidebar adthrive-sidebar-9 adthrive-ad-cls" data-google-query-id="CPv2r_yKrJEDFa70jgkdSIgdRg"><div id="google_ads_iframe_/18190176,15510053/AdThrive_Sidebar_9/61575e8e934c48ea554b3caa_2__container__" style="border: 0pt none;"><iframe id="google_ads_iframe_/18190176,15510053/AdThrive_Sidebar_9/61575e8e934c48ea554b3caa_2" name="google_ads_iframe_/18190176,15510053/AdThrive_Sidebar_9/61575e8e934c48ea554b3caa_2" title="3rd party ad content" width="1" height="1" scrolling="no" marginwidth="0" marginheight="0" frameborder="0" aria-label="Advertisement" tabindex="0" allow="private-state-token-redemption;attribution-reporting" data-load-complete="true" data-google-container-id="c" style="border: 0px; vertical-align: bottom;"></iframe></div></div></div>
        </div>
        </div>
        </div>
        </div>
        <!-- adhesion -->


        <footer class="global-footer ">
        <div class="footer-mast clearfix">
        <div class="mw-logo-section lazyload">
        <a href="/" class="footer-logo">Merriam Webster</a>
        </div>
        <div class="footer-mast-content clearfix">
        <div class="footer-subscribe-block">
        <div class="footer-subscribe-msg">
        <p>Learn a new word every day. Delivered to your inbox!</p>
        </div>
        <form data-medium="footer" class="footer-subscribe js-wod-subscribe-frm">
        <input class="footer-subscribe-field" type="text" name="email" value="" aria-label="Sign up for Merriam-Webster's Word of the Day newsletter" placeholder="Your email address">
        <input class="footer-subscribe-button" type="submit" name="submit" aria-label="Subscribe" value="SUBSCRIBE">
        <input class="footer-subscribe-button hidden arrow" type="submit" name="submit" value="&gt;">
        </form>
        </div>
        <div class="footer-internal-links">
        <ul class="footer-menu">
        <li><a href="/help">Help <span class="rborder"></span></a></li>
        <li><a href="/about-us">About Us <span class="rborder"></span></a></li>
        <li><a href="https://corporate.britannica.com/advertise-with-us" rel="noopener" target="_blank">Advertising Info <span class="rborder"></span></a></li>
        <li><a href="/contact-us">Contact Us <span class="rborder"></span></a></li>
        <li><a href="/privacy-policy">Privacy Policy <span class="rborder"></span></a></li>
        <li class="last"><a href="/terms-of-use">Terms of Use</a></li>
        </ul>
        </div>
        <div class="follow-us lazyload">
        <div class="follow-links">
        <ul>
        <li><a href="https://www.facebook.com/merriamwebster" rel="noopener" target="_blank" class="social-link social-fb">Facebook</a></li>
        <li><a href="https://twitter.com/merriamwebster" rel="noopener" target="_blank" class="social-link social-tw">Twitter</a></li>
        <li><a href="https://www.youtube.com/user/MerriamWebsterOnline" rel="noopener" target="_blank" class="social-link social-play">YouTube</a></li>
        <li><a href="https://www.instagram.com/merriamwebster/" rel="noopener" target="_blank" class="social-link social-ig">Instagram</a></li>
        </ul>
        </div>
        </div>
        </div>
        </div>

        <div class="footer-foot clearfix footer-oa cafemedia-footer-foot">
        <div class="footer-foot-content clearfix internal-page">
        <p class="copyright">© 2025 Merriam-Webster, Incorporated</p>
        </div>
        </div>
        </footer>
        </div>




        </div><iframe marginwidth="0" marginheight="0" scrolling="no" frameborder="0" id="1f7d2b235e66bd" width="0" height="0" src="about:blank" name="__pb_locator__" style="display: none; height: 0px; width: 0px; border: 0px;"></iframe>
        <script type="text/javascript" id="" charset="">!function(b,e,f,g,a,c,d){b.fbq||(a=b.fbq=function(){a.callMethod?a.callMethod.apply(a,arguments):a.queue.push(arguments)},b._fbq||(b._fbq=a),a.push=a,a.loaded=!0,a.version="2.0",a.queue=[],c=e.createElement(f),c.async=!0,c.src=g,d=e.getElementsByTagName(f)[0],d.parentNode.insertBefore(c,d))}(window,document,"script","https://connect.facebook.net/en_US/fbevents.js");typeof window.geoCountry==="undefined"&&console.warn("geoCountry is undefined. Fix - this is unexpected");
        typeof window.geoRegion==="undefined"&&console.warn("geoRegion is undefined. Fix - this is unexpected");typeof window.geoCountry==="string"&&window.geoCountry==="US"&&(window.geoRegion==="CA"?fbq("dataProcessingOptions",["LDU"],1,1E3):window.geoRegion==="CO"?fbq("dataProcessingOptions",["LDU"],1,1001):window.geoRegion==="CT"&&fbq("dataProcessingOptions",["LDU"],1,1002));fbq("init","673022290083244");fbq("track","PageView");</script>
        <script type="text/javascript" id="" charset="">!function(b,c,d,e,a,f,g){b.fbq||(a=b.fbq=function(){a.callMethod?a.callMethod.apply(a,arguments):a.queue.push(arguments)},b._fbq||(b._fbq=a),a.push=a,a.loaded=!0,a.version="2.0",a.queue=[])}(window,document);</script>
        <script type="text/javascript" id="" charset="">(function(a,e,b,f,g,c,d){a[b]=a[b]||function(){(a[b].q=a[b].q||[]).push(arguments)};c=e.createElement(f);c.async=1;c.src="https://www.clarity.ms/tag/"+g+"?ref\x3dgtm2";d=e.getElementsByTagName(f)[0];d.parentNode.insertBefore(c,d)})(window,document,"clarity","script","fq2f5zdaqe");</script>
        <script type="text/javascript" id="" charset="">window.isGdprCountry!==void 0&&window.isGdprCountry===!1&&window.clarity("consent");</script><script type="text/javascript" id="" charset="">(function(){function f(b){for(var c={},a;b&&b.tagName!="BODY";){if(b.tagName.toLowerCase()==="a")for(b.hasAttribute("id")&&b.id?c.id=b.id:c.id="",a=Array.from(b.classList),a.length>0&&(c.className=a.join(" ")),a=b.getAttributeNames(),i=0;i<a.length;i++)a[i]==="href"?c.href=b.getAttribute(a[i]):a[i]==="target"?c.target=b.getAttribute(a[i]):a[i]==="label"?c.text=b.getAttribute(a[i]):a[i]==="textContent"&&(c.text=b.getAttribute(a[i]));b=b.parentElement?b.parentElement:b.getRootNode().host}return c}var h=
        function(b){if("composed"in b&&typeof b.composedPath==="function"){var c=b.composedPath(),a=c[0],d=c.length?c.filter(function(g){return!a.shadowRoot&&!!g.shadowRoot}).length>0:!1;d&&(a!==null&&(a=f(a),console.log(JSON.stringify(a))),a!==null&&a.id&&a.id.startsWith("gtm-")&&!a.href.includes("javascript:void")&&(console.log("clicked"),dataLayer.push({event:"webComponentClickGTM","gtm.element":c[0],"gtm.elementText":a.text||"","gtm.elementClasses":a.className||"","gtm.elementId":a.id||"","gtm.elementTarget":a.target||
        "","gtm.elementUrl":a.href||e.target.action||"","gtm.originalEvent":b,"gtm.inShadowDom":d})))}};document.addEventListener("click",h,!0)})();</script><iframe src="https://ads.adthrive.com/builds/core/79a03dd/html/i.html" id="adt-ii" style="display: none;"></iframe><iframe src="https://ads.adthrive.com/builds/core/79a03dd/html/rnf.html" id="adthrive-mcmp" style="display: none;"></iframe><iframe id="criteo-ig" src="https://gpsb-reims.criteo.com/paapi/join_ig?advertiser_id=500002_500329&amp;ig_name=r6NnpiEx" style="display: none;"></iframe><img src="https://pixel.rubiconproject.com/token?pid=49096&amp;us_privacy=1YNY" alt="" style="display: none;"><iframe id="__uspapiLocator" name="__uspapiLocator" style="display: none;"></iframe><div id="adthrive-ccpa-modal" class="adthrive-ccpa-modal"><div id="adthrive-ccpa-modal-content" class="adthrive-ccpa-modal-content"><div id="adthrive-ccpa-modal-close-btn-container"><span>✕</span></div><div id="adthrive-ccpa-modal-title">Do not sell or share my personal information.</div><span id="adthrive-ccpa-modal-language">You have chosen to opt-out of the sale or sharing of your information from this site and any of its affiliates. To opt back in please click the "Customize my ad experience" link.<br>
        <br>This site collects information through the use of cookies and other tracking tools. Cookies and these tools do not contain any information that personally identifies a user, but personal information that would be stored about you may be linked to the information stored in and obtained from them. This information would be used and shared for Analytics, Ad Serving, Interest Based Advertising, among other purposes.<br>
        <br>For more information please visit this site's Privacy Policy.</span><div class="adthrive-ccpa-lower-buttons-container"><div id="adthrive-ccpa-modal-cancel-btn" class="adthrive-ccpa-modal-btn">CANCEL</div><div id="adthrive-ccpa-modal-continue-btn" class="adthrive-ccpa-modal-btn">CONTINUE</div></div></div></div><div class="adthrive-comscore adthrive-footer-message"><div id="adthrive-ccpa-link" class="adthrive-ccpa-link" style="display: none;">Information from your device can be used to personalize your ad experience. <br><br><a id="ccpaTag" href="/">Do not sell or share my personal information.</a></div></div><iframe name="__launchpadLocator" style="display: none;"></iframe><iframe style="width: 0px; height: 0px; display: none; position: fixed; left: -999px; top: -999px;"></iframe><iframe style="width: 0px; height: 0px; display: none; position: fixed; left: -999px; top: -999px;"></iframe><iframe name="cnftComm" style="width: 0px; height: 0px; display: none; position: fixed; left: -999px; top: -999px;"></iframe><div id="confiant_tag_holder" style="display:none"></div><script type="text/javascript" async="true" src="https://cdn.brandmetrics.com/scripts/bundle/65568.js?sid=f9816ecc-b51b-4747-bc3e-1ea86a0677a2&amp;toploc=www.merriam-webster.com"></script><iframe id="__bm_locator" name="__bm_locator" width="0" height="0" scrolling="no" src="about:blank" aria-hidden="true" tabindex="-1" style="display: none; width: 0px; height: 0px;"></iframe><iframe name="google_ads_top_frame" id="google_ads_top_frame" style="display: none; position: fixed; left: -999px; top: -999px; width: 0px; height: 0px;"></iframe><iframe height="0" width="0" frameborder="0" src="https://feed.pghub.io/tag?us_privacy=1YNY&amp;referrer_url=https%3A%2F%2Fwww.merriam-webster.com%2Fwordfinder%2Ffill-in-blanks%2Fcommon%2F6%2Fa___e_%2F1&amp;page_url=https%3A%2F%2Fwww.merriam-webster.com%2Fwordfinder%2Ffill-in-blanks%2Fall%2F6%2Fa___e_%2F1&amp;owner=P%26G&amp;bp_id=cafemedia&amp;ch=%7B%22architecture%22%3A%22x86%22%2C%22bitness%22%3A%2264%22%2C%22brands%22%3A%5B%7B%22brand%22%3A%22Chromium%22%2C%22version%22%3A%22142%22%7D%2C%7B%22brand%22%3A%22Google%20Chrome%22%2C%22version%22%3A%22142%22%7D%2C%7B%22brand%22%3A%22Not_A%20Brand%22%2C%22version%22%3A%2299%22%7D%5D%2C%22fullVersionList%22%3A%5B%7B%22brand%22%3A%22Chromium%22%2C%22version%22%3A%22142.0.7444.175%22%7D%2C%7B%22brand%22%3A%22Google%20Chrome%22%2C%22version%22%3A%22142.0.7444.175%22%7D%2C%7B%22brand%22%3A%22Not_A%20Brand%22%2C%22version%22%3A%2299.0.0.0%22%7D%5D%2C%22mobile%22%3Afalse%2C%22model%22%3A%22%22%2C%22platform%22%3A%22Linux%22%2C%22platformVersion%22%3A%22%22%7D&amp;initiator=js&amp;data=%7B%22iabc%22%3A%5B%5D%2C%22iaba%22%3A%5B%5D%2C%22lotamePanoramaId%22%3A%222ef87cbed6b4eb9e378230020180185ca02cac570c3328ac63084e85b9977b5a%22%7D" style="display: none;"></iframe><script type="text/javascript" src="https://pghub.io/js/pandg-sdk.js"></script><iframe src="https://www.google.com/recaptcha/api2/aframe" width="0" height="0" style="display: none;"></iframe><div id="AdThrive_Footer_1_desktop" class="adthrive-ad adthrive-footer adthrive-footer-1 adthrive-ad-cls adthrive-footer-desktop adthrive-sticky" data-google-query-id="CNvM7pmPrJEDFRbVjgkdfOowrA"><div id="google_ads_iframe_/18190176,15510053/AdThrive_Footer_1/61575e8e934c48ea554b3caa_2__container__" style="border: 0pt none;"><iframe id="google_ads_iframe_/18190176,15510053/AdThrive_Footer_1/61575e8e934c48ea554b3caa_2" name="google_ads_iframe_/18190176,15510053/AdThrive_Footer_1/61575e8e934c48ea554b3caa_2" title="3rd party ad content" width="1" height="1" scrolling="no" marginwidth="0" marginheight="0" frameborder="0" aria-label="Advertisement" tabindex="0" allow="private-state-token-redemption;attribution-reporting" data-load-complete="true" data-google-container-id="9" style="border: 0px; vertical-align: bottom;"></iframe></div></div></body><iframe name="ifrm_pubmatic" src="https://ads.pubmatic.com/AdServer/js/topics/topics_frame.html?bidder=pubmatic" style="display: none;"></iframe><iframe sandbox="allow-scripts allow-same-origin" id="3082a371c888cc6b8" frameborder="0" allowtransparency="true" marginheight="0" marginwidth="0" width="0" hspace="0" vspace="0" height="0" style="height:0px;width:0px;display:none;" scrolling="no" src="https://ssp-sync.criteo.com/user-sync/iframe?gdprapplies=&amp;gdpr=&amp;ccpa=1YNY&amp;gpp=&amp;gpp_sid=&amp;redir=https%3A%2F%2Fpbs-raptive-eu.ay.delivery%2Fsetuid%3Fbidder%3Dcriteo%26gdpr%3D%26gdpr_consent%3D%26gpp%3D%26gpp_sid%3D%26f%3Db%26uid%3D%24%7BCRITEO_USER_ID%7D&amp;profile=230">
        </iframe></html>
        "#;
        let words = Document::find_words(response.to_string());
        match words {
            Ok(words) => {
                assert_eq!(words.len(), 235)
            }
            Err(err) => {
                panic!("Empty state unexpected - {}", err)
            }
        }
    }
}
