# SPEC

## JWT

## API

* http get ":8080/api/server/time"

=>

{
    "ok":true,
    "code":0,
    "message":"",
    "data":{"now":"2016-07-03 10:22:21"}
}

* http get ":8080/api/server/mode"

=>

{
    "ok":true,
    "code":0,
    "message":"",
    "data":"development"
}

* http post ":8080/api/users/login"

{
    login_name: name,
    password: password
}

=>

{
    "code": 0,
    "data": {
        "token": "eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6ImFkbWluIiwiYXVkIjpudWxsLCJleHAiOm51bGwsIm5iZiI6bnVsbCwiaWF0IjpudWxsLCJqdGkiOm51bGx9.y3T0C4mSiRMjVR+glcnZrmes+vis65z2uFkpUtdA6b0",
        "user": {
            "id": 1,
            "login_name": "admin",
            "valid": true
        }
    },
    "message": "",
    "ok": true
}
