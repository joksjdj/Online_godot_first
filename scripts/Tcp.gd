extends Node

var server := TCP_Server.new()
var clients = []

func _ready():
    server.listen(8081)
    print("TCP server running")

func _process(delta):
    if server.is_connection_available():
        var client = server.take_connection()
        clients.append(client)
        print("Client connected")

    # Handle existing clients
    for client in clients:
        if client.get_available_bytes() > 0:
            var data = client.get_utf8_string(client.get_available_bytes())
            var msg = JSON.parse(data).result
            print(msg)

            if msg.req == "login":
                print("login request received")
                _http_request(client, "/login", JSON.print({"username": msg.username, "password": msg.password}), "login")

            if msg.req == "signup":
                print("signup request received")
                _http_request(client, "/signup", JSON.print({"username": msg.username, "password": msg.password}), "signup")
            

func _http_request(client, url, body, type):
    var http_client := HTTPClient.new()

    var err = http_client.connect_to_host("alexanderpi", 8080)
    if err != OK:
        print("Failed to connect")
        return

    while http_client.get_status() in [
        HTTPClient.STATUS_CONNECTING,
        HTTPClient.STATUS_RESOLVING
    ]:
        http_client.poll()
        OS.delay_msec(10)

    http_client.request(
        HTTPClient.METHOD_POST,
        url,
        ["Content-Type: application/json"],
        body
    )

    while http_client.get_status() == HTTPClient.STATUS_REQUESTING:
        http_client.poll()
        OS.delay_msec(10)

    var response = http_client.read_response_body_chunk()
    var code = http_client.get_response_code()
    var text = response.get_string_from_utf8()

    print("Rust says:", code, text)

    responde(client, text, code, type)

func responde(client, body, status, type):
    var response = JSON.print({"status": status, "type": type, "body": body})
    client.put_data(response.to_utf8())