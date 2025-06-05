use tokio::task::JoinHandle;

use anyhow::Result;
use rss::Channel;
use soup::prelude::*;



async fn read_rss(url: &str) -> Result<Channel> {
    // Using reqwest, read the bytes from the rss channel
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    
    // Write to the channel Struct
    let channel = Channel::read_from(&content[..])?;
    
    Ok(channel)
}

async fn read_link_from_rss(channel: Channel) -> Result<Vec<String>> {
    // Get links form channel
    let mut links: Vec<String> = vec![];
    channel.items.iter()
        .for_each(|item| links.push(item.link().unwrap().to_string()));
    
    
    Ok(links)
}

pub async fn get_content_from_feed(channel: Channel) -> Result<Vec<String>> {
    let links = read_link_from_rss(channel).await.unwrap();
    
    let mut join_handle: Vec<JoinHandle<String>> = vec![];
    
    let mut bodies: Vec<String> = vec![];
    
    for link in links {
        let handle = tokio::spawn(async move{
            let body = reqwest::get(link.clone())
                .await.unwrap()
                .text()
                .await.unwrap();
            
            body
        });
        
        join_handle.push(handle);
    }
    
    for handle in join_handle {
        bodies.push(handle.await?);
    }
    
    Ok(bodies)
}

#[derive(Debug)]
pub struct RssClient<'a> {
    pub url: &'a str,
    pub channel: Channel,
}

impl <'a>RssClient<'a> {
    pub async fn new(url: &'a str) -> Self {
        let channel = read_rss(url).await.unwrap();
    
        Self {
            url,
            channel,
        }
    }
    

    
    pub async fn join_content_into_html(self) -> Result<Vec<String>> {
        let content = get_content_from_feed(self.channel).await.unwrap();
        
        let mut join_handle: Vec<JoinHandle<String>> = vec![];
        
        let mut resp: Vec<String> = vec![];
        
        for c in content {
            let handle = tokio::spawn(async move {
                let soup = Soup::new(c.as_str());
                
                let ap_all = soup.tag(true)
                    .find_all()
                    .filter(|tag| tag.name() == "article");
                
                let body = ap_all
                    .map(|p| p.display())
                    .collect::<Vec<_>>()
                    .join("\n");
                
                body
            });  
            
            join_handle.push(handle);
        }
        
        for handle in join_handle {
            resp.push(handle.await?);
        }
        
        Ok(resp)
    }
    
    async fn html_to_md(html: Vec<String>) -> Result<Vec<String>> {
        let mut mds: Vec<String> = vec![];
        
        for h in html {
            let parsed = html2md::parse_html(&h);
            let clean = parsed.replace("\n\n", "");
            
            mds.push(clean);
        }
        
        Ok(mds)
    }
}


