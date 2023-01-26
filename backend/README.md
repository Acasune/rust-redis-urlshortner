
## Activate backend.

```
  docker compose up backend -d
```


## Get all urls stored in Redis

### Request
`GET /`

```
curl -X GET http://localhost:8080/
```
### Response
```
{"urls":[{"hashed":"99999ebcfdb78df077ad2727fd00969f","raw":"https://google.com"},{"hashed":"56b86e475b6d677e1ceb62a4c68cfe9a","raw":"https://www.microsoft.com/"}]}
```

## Get a specified url from a hashed one.

### Request
`GET /[hashed value]`

```
curl -X  GET http://localhost:8080/56b86e475b6d677e1ceb62a4c68cfe9a
```
### Response
```
{"url":"https://www.microsoft.com/"}
```
## Post a new url.

### Request
`POST /`

```
curl -X POST -H "Content-Type: application/json" -d '{"user_name":"username","url":"https://www.microsoft.com/"}'  http://localhost:8080
```
### Response
```
{"hashed":"56b86e475b6d677e1ceb62a4c68cfe9a"}
```
## Delete a stored url.

### Request
`DELETE /[hashed value]`

```
curl -X DELETE http://localhost:8080/56b86e475b6d677e1ceb62a4c68cfe9a
```
### Response
```
{"hashed":"56b86e475b6d677e1ceb62a4c68cfe9a"}
```
