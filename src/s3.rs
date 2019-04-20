use crate::config::Config;
use crate::tourdates::TourdateSource;
use failure::Error;
use futures::Stream;
use rusoto_core::{Region::SaEast1, RusotoError};
use rusoto_s3::{
    GetObjectError, GetObjectOutput, GetObjectRequest, PutObjectRequest, S3Client, S3,
};

pub struct Client {
    client: S3Client,
    bucket: String,
}

pub struct ContextualClient<'s3> {
    client: &'s3 S3Client,
    bucket: &'s3 str,
    key: &'static str,
}

pub fn new_client(config: &Config) -> Client {
    let client = S3Client::new(SaEast1);

    Client {
        client: client,
        bucket: config.bucket.clone(),
    }
}

impl Client {
    pub fn tourdate_client<S: TourdateSource>(&self) -> ContextualClient {
        ContextualClient {
            client: &self.client,
            bucket: &self.bucket,
            key: S::S3_KEY,
        }
    }
}

impl<'s3> ContextualClient<'s3> {
    pub fn matches_existing_hash(&self, new_hash: &[u8]) -> Result<bool, Error> {
        match self.get_current_hash()? {
            Some(old_hash) => Ok(old_hash == new_hash),

            None => Ok(false),
        }
    }

    pub fn get_current_hash(&self) -> Result<Option<Vec<u8>>, Error> {
        let get_request = GetObjectRequest {
            bucket: self.bucket.to_string(),
            key: self.key.to_string(),
            ..Default::default()
        };

        let request = self.client.get_object(get_request).sync();

        let body = match request {
            Ok(GetObjectOutput {
                body: Some(body), ..
            }) => body,

            Ok(_) | Err(RusotoError::Service(GetObjectError::NoSuchKey(_))) => return Ok(None),

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
            bucket: self.bucket.to_string(),
            key: self.key.to_string(),
            body: Some(new_hash.into()),
            ..Default::default()
        };

        self.client.put_object(put_request).sync()?;

        Ok(())
    }
}
