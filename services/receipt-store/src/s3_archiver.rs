use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use anyhow::Result;
use tracing::{info, error};
use chrono::{Utc, Datelike, Timelike};

pub struct S3Archiver {
    client: S3Client,
    bucket: String,
}

impl S3Archiver {
    pub async fn new(bucket: &str, region: &str) -> Result<Self> {
        let config = aws_config::from_env()
            .region(aws_sdk_s3::config::Region::new(region.to_string()))
            .load()
            .await;
        
        let client = S3Client::new(&config);

        Ok(Self {
            client,
            bucket: bucket.to_string(),
        })
    }

    pub async fn archive_receipt(&self, receipt_json: &str) -> Result<()> {
        // Create partition path: year/month/day/hour
        let now = Utc::now();
        let partition_path = format!(
            "receipts/{:04}/{:02}/{:02}/{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour()
        );

        // Generate object key with timestamp
        let key = format!(
            "{}/receipt_{}.json",
            partition_path,
            now.timestamp()
        );

        let body = ByteStream::from(receipt_json.as_bytes().to_vec());

        match self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(body)
            .send()
            .await
        {
            Ok(_) => {
                info!("Receipt archived to S3: s3://{}/{}", self.bucket, key);
                Ok(())
            }
            Err(e) => {
                error!("Failed to archive receipt to S3: {}", e);
                Err(anyhow::anyhow!("S3 archive error: {}", e))
            }
        }
    }

    pub async fn batch_archive_receipts(&self, receipts: Vec<String>) -> Result<()> {
        // For batch operations, we could use S3 multipart upload
        // For MVP, we'll archive them individually
        for receipt_json in receipts {
            if let Err(e) = self.archive_receipt(&receipt_json).await {
                error!("Failed to archive receipt in batch: {}", e);
                // Continue with other receipts
            }
        }
        Ok(())
    }
}

