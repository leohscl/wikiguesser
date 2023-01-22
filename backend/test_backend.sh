# # testing game creation
# str_post='{"cat":"None","email":"None"}'
# curl_get_game="curl -X POST -d $str_post -H Content-Type:application/json localhost:8000/games/get_or_create"
# $curl_get_game
# id=$($curl_get_game | jq '.game.id')
# echo $id
# curl -X DELETE localhost:8000/games/$id

# testing report
article_report_id=736095 

curl -d '{"article_id":"736095","report_cat":"Bug","description":"there is no bug this is a test"}' -H "Content-Type: application/json" localhost:8000/reports
# id=$(curl localhost:8000/games/get_or_create/none | jq '.game.id')
# echo $id
# # curl -d '{"string":"test1"}' -H "Content-Type: application/json" localhost:8000/games/update/$id | jq '.word'
# # curl -d '{"string":"glshghaziohgshdl"}' -H "Content-Type: application/json" localhost:8000/games/update/$id
# # curl localhost:8000/games/get_or_create/none
# curl -X DELETE localhost:8000/games/$id
