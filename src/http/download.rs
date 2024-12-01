pub mod download {
    use error_chain::error_chain;
    // use std::io::copy;
    // use std::fs::File;
    use bytes::Bytes;
    // use tempfile::{Builder, TempDir};

    error_chain! {
         foreign_links {
             Io(std::io::Error);
             HttpRequest(reqwest::Error);
         }
    }
    pub async fn get(target: &str) -> Result<Bytes> {
        // let tmp_dir = Builder::new().prefix("music").tempdir()?;
        let response = reqwest::get(target).await?;

        /*let f_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        */
        // println!("file to download: '{}'", f_name);
        // let f_name = tmp_dir.path().join(f_name);
        // println!("will be located under: '{:?}'", f_name);
        let content: Bytes = response.bytes().await?;
        // copy(&mut content, &mut File::create(&f_name)?)?;
        // let name = f_name.to_str().unwrap();
        // Ok((name.to_string(), content))
        Ok(content)


    }
}