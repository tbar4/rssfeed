use anyhow::Result;

mod rss_puller;
use rss_puller::extractor::RssClient;

#[tokio::main]
async fn main() -> Result<()> {
    
    let rss = RssClient::new("https://spacenews.com/feed/").await.join_content_into_html().await;
    println!("{rss:#?}");
    /*
    //let url = "https://www.darpa.mil/rss.xml";
    let url = "https://spacenews.com/feed/";
    let feed = read_rss(url).await?;
    
    let link = read_link_from_rss(feed).await?;
    
    println!("{link:#?}");
    
    let bodies = get_content_from_feed(link).await?;
    
    let resp = join_content(bodies).await?;
    
    resp.iter()
        .for_each(|article| println!("{:#?}", article));
    
    //let mds = html_to_md(resp).await?;
    
        //mds.iter()
        //.for_each(|article| println!("{:#?}", article));
    */
    Ok(())
}
