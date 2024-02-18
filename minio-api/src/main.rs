use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio_api::{ACCESS_KEY, MINIO_SERVER, SECRET_KEY};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let base_url = MINIO_SERVER.parse::<BaseUrl>()?;

    let static_provider = StaticProvider::new(ACCESS_KEY, SECRET_KEY, None);

    let client = Client::new(
        base_url.clone(),
        Some(Box::new(static_provider)),
        None,
        None,
    )
    .unwrap();

    let bucket_name = "ota-files";

    // Check 'asiatrip' bucket exist or not.
    let exists = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await
        .unwrap();

    // Make 'asiatrip' bucket if not exist.
    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();
    }

    // Upload '/home/user/Photos/asiaphotos.zip' as object name
    // 'asiaphotos-2015.zip' to bucket 'asiatrip'.
    client
        .upload_object(
            &mut UploadObjectArgs::new(
                &bucket_name,
                "1987-0.2.0.bin",
                "/root/proj/mcu-ota-platform/file/1987-0.2.0.bin",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    println!("'/home/user/Photos/asiaphotos.zip' is successfully uploaded as object 'asiaphotos-2015.zip' to bucket 'asiatrip'.");
    Ok(())
}
