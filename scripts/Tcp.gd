extends Node

var server := TCP_Server.new()
var clients = []

func _ready():
    server.listen(8080)
    print("TCP server running")

func _process(delta):
    if server.is_connection_available():
        var client = server.take_connection()
        clients.append(client)
        print("Client connected")

    # Handle existing clients
    for client in clients:
        if client.get_available_bytes() > 0:
            var msg = client.get_utf8_string(client.get_available_bytes())
            print("Client says:", msg)

            # Echo back
            client.put_data(("Server received: " + msg).to_utf8())