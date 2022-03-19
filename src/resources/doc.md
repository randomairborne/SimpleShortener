<!---
This file is not needed to compile the app. It's generated with https://dillinger.io
-->

# SimpleShortener Docs

### If you just wish to add links, and not automate things, see [the panel](/simpleshortener/)  .

## Authentication

All requests require `username` and a `password` header. These should match some of the credentials in the configuration file.  You also need a `Content-Type` header set to `application/json`

### Errors

For an error response, the server will return a JSON with a single key- `error`, which has a short message describing the error. On a 2xx error code, the key is instead `message`.

## Get existing links

To get the existing links, make a `GET` request to `/simpleshortener/api/list`. This returns a JSON file with the `links` field having a key:value list of shortening:destination.

## Add new link

You can add new links by making a `PUT` request to `/simpleshortener/api/add` with the JSON data

```json
{
    "link": "shorturl",
    "destination": "https://example.com"
}
```

## Delete link

You can delete links by making a `DELETE` request to `/simpleshortener/api/delete/<short url>`

## Edit existing link

You can edit existing links by making a `PATCH` request to `/simpleshortener/api/edit/<short url>` with the JSON data

```json
{
  "destination": "https://example.org"  
}
```

## Generate QR code

You can generate a QR code by making a `POST` request to `/simpleshortener/api/qr` with the JSON data

```json
{
  "destination": "https://example.net"
}
```
