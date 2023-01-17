curl localhost:8000/games/get_or_create/none
id=$(curl localhost:8000/games/get_or_create/none | jq '.game.id')
echo $id
# curl -d '{"string":"test1"}' -H "Content-Type: application/json" localhost:8000/games/update/$id | jq '.word'
# curl -d '{"string":"glshghaziohgshdl"}' -H "Content-Type: application/json" localhost:8000/games/update/$id
# curl localhost:8000/games/get_or_create/none
curl -X DELETE localhost:8000/games/$id
