Quick Guide
--------------------------------------------------------------------------------

`cargo run`

`curllocalhost:8080`
 {"server_id":0,"request_count":1,"messages":[]}

`curl -X POST -H "Content-Type: application/json" -d '{"message": "hello"}' localhost:8080/send`
 {"server_id":1,"request_count":1,"message":"hello"}

`curl -X POST -H "Content-Type: application/json" -d '{"message": "helloagain"}' localhost:8080/send`
{"server_id":2,"request_count":1,"message":"helloagain"}

`curl -X POST localhost:8080/clear`
{"server_id":5,"request_count":1,"messages":[]}

`curllocalhost:8080`
{"server_id":6,"request_count":1,"messages":[]}

`curl -X POST -H "Content-Type: application/json" -d '{"bad": "hello"}' localhost:8080/send`
NOT WORKING


`curl localhost:8080/lookup/2`

`curl localhost:8080/lookup/foo`
-> 404

