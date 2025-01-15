# IMage Delivery (kindof) Network

> A simple API in NodeJS to serve images and generate thumbnails.

## Usage

First, you will have to export three environment variables:

```dotenv
AWS_ACCESS_KEY_ID=xxx
AWS_SECRET_ACCESS_KEY=xxx
# Both secret and port are optionnal, however changing the secret 
# from the default value is highly encouraged for security reasons.
SECRET=super-secret-secret
PORT=3000
```

Then you can start the project using CLI:

```shell
node index.js --s3-bucket=my-bucket \
  --s3-endpoint=https://s3.example.com \
  --s3-region=eu-west-1 \
  --cache-location=/var/cache/image-proxy \
  --origin=http://localhost:3000
```

## Authentication

This CDN implements authentication, you can find a sample function to generated an authenticated URL for your CDN
instance in [generateSignedUrl.js](generateSignedUrl/nodejs/generateSignedUrl.js).

## Development Quick Start

```
npm install
npm run dev
```

```
open http://localhost:3000
```

### NodeJS rewrite

This project was originally written in Rust, but due to poor performance, this was re-written in NodeJS to take
advantages of the streams and LibVips which is used by the Sharp library for image processing.
