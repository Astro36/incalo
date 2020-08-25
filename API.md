# API

## Index

- `GET /signin/oauth`
- `GET /signin/oauth/consent`
- `GET /signin/oauth/authorize`
- `POST /api/oauth`
- `POST /api/oauth/token`
- `POST /api/user/logout`

## Overview

> OAuth 2.0의 Authorization Code Grant 형식의 인증을 사용함

1. [로그인]() 버튼 클릭(`state` 생성)
2. (브라우저) `GET /signin/oauth`(`state` 전달) 페이지(로그인 폼 페이지)로 이동
3. (브라우저) 사용자 정보를 입력받으면 `POST /api/v1/oauth`에 정보를 전송하고 서버로부터 HttpOnly 쿠키로 `incalo_sid`(session_id)를 받음
4. 정상적(200)으로 응답을 받았다면 `GET /signin/oauth/consent`(`incalo_sid` 전송)페이지로 이동해 사용자에게 권한과 관련된 내용을 확인받음(2회차부터는 확인 없이 다음 단계로 넘어감)
5. `GET /signin/oauth/authorize` 페이지로 이동(302 Found 반환)
6. 이어서 초기에 지정한 리다이렉션 URI(`state` 전달)로 이동함(여기서 `state` 비교함)
7. URI의 쿼리스트링에서 `code`을 찾아 `POST /api/v1/oauth/token`에 전송함(반환된 `access_token`은 `localstorage`에 저장)
8. 정상적(200)으로 응답을 받았다면 로그인이 완료된 것으로 간주
9. 이제 API 사용 시 `access_token`는 자동으로 요청 헤더에 들어감

## Reference

### GET /signin/oauth

로그인 페이지

사용자에게 `id`와 `password`를 입력받아 `POST /api/v1/oauth`에 Ajax로 전송해 로그인을 시도한다.
로그인 성공(200 Ok)를 받았다면 `GET /signin/oauth/consent`으로 이동한다.

#### Input

Query String Parameters:

- `response_type`: (필수) `code`만 허용
- `client_id`: (필수) 클라이언트 식별자
- `redirect_uri`: (선택) 로그인 성공시 리다이렉트될 URI
- `state`: (필수) [CSRF 공격](https://en.wikipedia.org/wiki/Cross-site_request_forgery) 보호용 값

### GET /signin/oauth/consent

개인정보 이용권한 동의 페이지

`incalo_sid`가 가리키는 사용자의 개인정보 이용권한 동의 상태를 확인한다.
만약 이미 허락한 상태라면, 자동으로 `GET /signin/oauth/authorize` 페이지로 리다이렉트(302 Found)한다.

개인정보 이용권한을 받지 않은 상태에서 `GET /signin/oauth/authorize`으로 이동하면 이용권한을 승인한 것으로 간주한다.

#### Input

Query String Parameters:

- `response_type`: (필수) `code`만 허용
- `client_id`: (필수) 클라이언트 식별자
- `redirect_uri`: (선택) 로그인 성공시 리다이렉트될 URI
- `state`: (필수) [CSRF 공격](https://en.wikipedia.org/wiki/Cross-site_request_forgery) 보호용 값

Cookies:

- `incalo_sid`: 로그인 세션 아이디

### GET /signin/oauth/authorize

`access_token` 발급에 필요한 `authorize_code`를 전달하는 페이지

로그인 단계가 모두 잘 이루어졌다면 `{redirect_uri}?code={authorize_code}`으로 리다이렉트한다.

`redirect_uri`으로 돌아오면 브라우저는 `authorize_code`와 함께 받은 `state`가 처음 서버에 전달한 `state`와 같은지 확인한다.

#### Input

Query String Parameters:

- `response_type`: (필수) `code`만 허용
- `client_id`: (필수) 클라이언트 식별자
- `redirect_uri`: (선택) 로그인 성공시 리다이렉트될 URI
- `state`: (필수) [CSRF 공격](https://en.wikipedia.org/wiki/Cross-site_request_forgery) 보호용 값

Cookies:

- `incalo_sid`: 로그인 세션 아이디

### POST /api/v1/oauth

`id`와 `password`로 로그인을 하고 `incalo_sid`를 HttpOnly 쿠키로 반환한다.

#### Request

```text
POST /token HTTP/1.1
Host: server.example.com
Content-Type: application/x-www-form-urlencoded

id=hello&password=1q2w3e4r
```

#### Response(Success)

```text
HTTP/1.1 200 Ok
Set-Cookie: incalo_sid=aaa.bbb.ccc; HttpOnly
```

### POST /api/v1/oauth/token

#### Request

```text
POST /token HTTP/1.1
Host: server.example.com
Content-Type: application/x-www-form-urlencoded

grant_type=authorization_code&code=SplxlOBeZQQYbYS6WxSbIA&redirect_uri=https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb
```

Query String Parameters:

- `grant_type`: (필수) `authorization_code`만 허용
- `code`: (필수) 최대 수명 10분의 일회용 인증 코드
- `redirect_uri`: (선택) 로그인 성공시 리다이렉트될 URI
- `client_id`: (필수) 클라이언트 식별자

#### Response(Success)

```text
HTTP/1.1 200 OK
Content-Type: application/json;charset=UTF-8
Cache-Control: no-store
Pragma: no-cache

{
  "access_token": "2YotnFZFEjr1zCsicMWpAA",
  "token_type": "bearer",
  "expires_in": 3600
}
```

- `access_token`: (필수) 리소스 접근 토큰
- `token_type`: (필수) 토큰 종류(ex. Bearer)
- `expires_in`: (권장) 리소스 접근 토큰의 수명(초)
