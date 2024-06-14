# imdn

> A simple API in Rust to serve images and generate thumbnails.

## Environment variables

- `ROOT_DIR`: Where your original images are stored
- `CACHE_DIR`: Where the cached thumbnails should be stored

## Example usage

```yaml
services:
  image_cdn:
    image: ghcr.io/quozul/imdn:master
    environment:
      ROOT_DIR: "/usr/src/imdn/images"
      CACHE_DIR: "/usr/src/imdn/cache"
    volumes:
      - "./images:/usr/src/imdn/images"
      - "cache:/usr/src/imdn/cache"
    ports:
      - "8000:8000"

volumes:
  cache: { }
```
