use elasticsearch::{Elasticsearch,
                    Error,
                    http::transport::Transport,
};



pub fn es_connection() -> Result<Elasticsearch, Error>{

    let transport = Transport::single_node("http://10.0.0.101:9200")?;
    Ok(Elasticsearch::new(transport))
}