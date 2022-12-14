# ImOk

<p>An API (REST architecture) to track and save your night adventures.<br>
This project was made for fun.
We do not condone excessive drinking.

Things you can track and save for your nights:
 - Drunkness levels (0-5)
 - Where or not you did the dirty(coitus)
 - A brief description of your night
 - Where you drove or not
 - What was your location
 - Whether you texted your ex-girlfriend/boyfriend

## Prerequisites
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Docker](https://docs.docker.com/engine/install/)
3. [Diesel](https://diesel.rs/guides/getting-started)

 ## Setup
### Run a postgres database from docker
### Copy .env.example to .env and populate it with the correct values
*Note: if running postgres from docker don't edit `DATABASE_URL`.*
### Run database setup & migrations

```sh
diesel setup
diesel migrations generate
```
## Flow
- Register with a user
- Login with the user (to get the head token for authorization)
- Make your entry

## Example
### Register

```
{
  "password": "string",
  "username": "string"
}
```


### Login

```
{
  "password": "string",
  "username": "string"
}
```


**This returns the authorization token to be used for every "night" activity (create, delete etc.)**

### Create Night

```
{
  "coitus": true,
  "description": "string",
  "drive": true,
  "drunkness": "Cool",
  "location": "string",
  "talked_2x": true,
  "user_id": 0
}
```

***More features are to be added soon. Contact us if you have an idea to be added or any kind of feedback***
