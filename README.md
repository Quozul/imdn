# IMage Delivery (kindof) Network

> A simple API in Rust to serve images and generate thumbnails.

## Example Usage

### Local Volume

Here is an example to serve your home Pictures directory:

```shell
imdn -c /path/to/cache local $HOME/Pictures
```

### S3

For S3 usage, the env variables must be set before running the program.

```shell
AWS_ACCESS_KEY_ID=xxx AWS_SECRET_ACCESS_KEY=xxx imdn -c /path/to/cache s3 \
  --bucket images \
  --region eu-west-1 \
  --endpoint https://s3.example.com/
```

Note that the cache cannot be stored on a S3 bucket... yet.

### Docker

For easy installation/deployment, see [docker-compose.yml](docker-compose.yml).

### Documentation

For endpoint documentation, see [api-docs.yaml](assets/docs.yaml). Import the file into https://editor.swagger.io/ for
testing.