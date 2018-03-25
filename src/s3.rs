use rusoto_core::Region::SaEast1;
use rusoto_s3::{GetObjectError, GetObjectOutput, GetObjectRequest, PutObjectRequest, S3, S3Client};
use futures::Stream;
use failure::Error;

pub struct Client(S3Client);

pub fn new_client() -> Client {
    let client = S3Client::simple(SaEast1);
    Client(client)
}

const BUCKET: &'static str = "renato-zannon-lotr-watcher";
const KEY: &'static str = "last-hash";

impl Client {
    pub fn matches_existing_hash(&self, new_hash: &[u8]) -> Result<bool, Error> {
        match self.get_current_hash()? {
            Some(old_hash) => Ok(old_hash == new_hash),

            None => Ok(false),
        }
    }

    pub fn get_current_hash(&self) -> Result<Option<Vec<u8>>, Error> {
        let get_request = GetObjectRequest {
            bucket: BUCKET.to_string(),
            key: KEY.to_string(),
            ..Default::default()
        };

        let request = self.0.get_object(&get_request).sync();

        let body = match request {
            Ok(GetObjectOutput {
                body: Some(body), ..
            }) => body,

            Ok(_) | Err(GetObjectError::NoSuchKey(_)) => return Ok(None),

            Err(e) => {
                return Err(Error::from(e));
            }
        };

        let mut hash = Vec::new();

        for part in body.wait() {
            hash.extend(part?);
        }

        Ok(Some(hash))
    }

    pub fn update_hash(&self, new_hash: Vec<u8>) -> Result<(), Error> {
        let put_request = PutObjectRequest {
            bucket: BUCKET.to_string(),
            key: KEY.to_string(),
            body: Some(new_hash),
            ..Default::default()
        };

        self.0.put_object(&put_request).sync()?;

        Ok(())
    }
}
