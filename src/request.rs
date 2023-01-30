use std::error::Error;

pub(crate) fn req_with_sql_inject(sql_data: [&str; 5], url:&str) -> Result<(), Box<dyn Error>> {
    for sq in sql_data {
        let request_url = format!("{}{}", url, sq);
        let resp: String = reqwest::blocking::get(request_url)?.text()?;
        println!("{:#?}", resp);
    }
    Ok(())
}