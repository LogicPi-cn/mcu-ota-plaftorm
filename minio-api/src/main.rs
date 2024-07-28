use minio_api::{ACCESS_KEY, SECRET_KEY};

use minio_rsc::client::{BucketArgs, KeyArgs};
use minio_rsc::error::Result;
use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;

use std::fs::File;
use std::io::Read;

async fn example() -> Result<()> {
    let provider = StaticProvider::new(ACCESS_KEY, SECRET_KEY, None);
    let minio = Minio::builder()
        .endpoint("ota.logicpi.cn:9000")
        .provider(provider)
        .secure(false)
        .build()
        .unwrap();
    let (_buckets, _owner) = minio.list_buckets().await?;

    minio.make_bucket(BucketArgs::new("bucket1"), false).await?;
    minio.make_bucket("bucket2", true).await?;

    minio
        .put_object("bucket1", "hello.txt", "hello minio!".into())
        .await?;
    minio.stat_object("bucket1", "hello.txt").await?;
    minio.get_object("bucket1", "hello.txt").await?;
    let key = KeyArgs::new("hello.txt")
        .version_id(Some("cdabf31a-9752-4265-b137-6b3961fbaf9b".to_string()));
    minio.get_object("bucket1", key).await?;
    minio.remove_object("bucket1", "hello.txt").await?;

    let bucket2 = minio.bucket("bucket2");
    bucket2
        .put_object("hello.txt", "hello minio!".into())
        .await?;
    bucket2.stat_object("hello.txt").await?;
    bucket2.get_object("hello.txt").await?;
    bucket2.remove_object("hello.txt").await?;

    // if fs-tokio feature enabled
    // download file to local
    minio
        .fget_object("bucket1", "hello.txt", "local.txt")
        .await?;
    // // upload file to minio
    minio
        .fput_object("bucket1", "hello.txt", "local.txt")
        .await?;

    // minio.remove_bucket("bucket1").await?;
    // minio.remove_bucket("bucket2").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let provider = StaticProvider::new(ACCESS_KEY, SECRET_KEY, None);
    let minio = Minio::builder()
        .endpoint("ota.logicpi.cn:9000")
        .provider(provider)
        .secure(false)
        .build()
        .unwrap();

    // 打开文件
    let mut file =
        File::open("/home/craftor/mcu-ota-platform/file/1987-0.2.0.bin").expect("无法打开文件");

    // 读取文件内容到一个字节数组中
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("无法读取文件");

    minio
        .put_object("ota-debug", "1987-0.2.0.bin", buffer.into())
        .await?;

    minio
        .fget_object("ota-debug", "1987-0.2.0.bin", "download.bin")
        .await?;

    Ok(())
}
