# API

## Index

- `GET /signin/oauth`
- `GET /signin/oauth/consent`
- `GET /signin/oauth/authorize`
- `POST /api/v1/oauth`
- `POST /api/v1/oauth/token`

## Overview

> OAuth 2.0의 Authorization Code Grant 형식의 인증을 사용함

1. [로그인]() 버튼 클릭(`state` 생성)
2. `GET /signin/oauth`(`state` 전달) 페이지(로그인 폼 페이지)로 이동
3. 사용자 정보를 입력받으면 `POST /api/v1/oauth`에 정보를 전송하고 HttpOnly 쿠키로 `incalo_sid`(session_id)를 받음
4. 정상적(200)으로 응답을 받았다면 `GET /signin/oauth/consent`(`incalo_sid` 전송)페이지로 이동해 사용자에게 권한과 관련된 내용을 확인받음(2회차부터는 확인 없이 다음 단계로 넘어감)
5. `GET /signin/oauth/authorize` 페이지로 이동(302 Found 반환)
6. 이어서 초기에 지정한 리다이렉션 URI(`state` 전달)로 이동함(여기서 `state` 비교함)
7. URI의 쿼리스트링에서 `code`을 찾아 `POST /api/v1/oauth/token`에 전송함(반환된 `access_token`은 자동으로 HttpOnly 쿠키에 저장)
8. 정상적(200)으로 응답을 받았다면 로그인이 완료된 것으로 간주
9. 이제 API 사용 시 `access_token`는 자동으로 요청 헤더에 들어감

## Reference

### GET /signin/oauth

### GET /signin/oauth/consent

### GET /signin/oauth/authorize

### POST /api/v1/oauth

### POST /api/v1/oauth/token
