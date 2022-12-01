


## Get all urls stored in Redis

### Request
`GET /`

```
curl -X GET http://localhost:8080/
```
### Response
```
curl -X GET http://localhost:8080/
```

## Get a specified url from a hashed one.

### Request
`GET /[hashed value]`

```
curl -X  GET http://localhost:8080/56b86e475b6d677e1ceb62a4c68cfe9a
```
### Response
```
curl -X GET http://localhost:8080/
```
## Post a new url.

### Request
`POST /`

```
url -X POST -H "Content-Type: application/json" -d '{"user_name":"username","url":"https://www.microsoft.com/"}'  http://localhost:8080
```
### Response

## Delete a stored url.

### Request
`DELETE /[hashed value]`

```
curl -X DELETE http://localhost:8080/99999ebcfdb78df077ad2727fd00969f
```
### Response
```
curl -X GET http://localhost:8080/
```
