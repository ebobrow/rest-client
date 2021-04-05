# REST Cli

Send HTTP requests from the command line.

### Usage

```
rest_cli <file>
```

where `file` contains your HTTP requests.

Supported methods:
- GET
- POST
- PUT
- PATCH
- DELETE
- HEAD

### Example

```
### GET slash route ###
http://localhost:3000
GET /

### POST to /auth/register ###
http://localhost:3000

# Headers
Content-Type: application/json
Authorization: bearer alghdlaiusdflhsadfkjsadf

# Body
{
    "username": "name",
    "password": "password123"
}

POST /auth/register
```
