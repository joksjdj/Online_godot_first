extends Node

@onready var game_tracking = {
	"enemies_left": 0,
	"enemies": {},
}
var playing_clients: Array[StreamPeerTCP] = []
var world_active: bool = false
var frame_passed: int = 0

var world_scene := preload("res://scenes/physics_world.tscn")
var world_script := preload("res://scripts/physics_world.gd")

var server := TCPServer.new()
var clients: Array[StreamPeerTCP] = []


func _ready():
    server.listen(8081)
    print("TCP server running")

func _process(delta):
    frame_passed += 1
    
    if server.is_connection_available():
        var client = server.take_connection()
        client.set_no_delay(true)
        clients.append(client)
        print("Client connected")

    for client in clients:
        if client.get_available_bytes() > 0:
            var data = client.get_utf8_string(client.get_available_bytes())

            var msg = JSON.parse_string(data)
            if msg == null:
                print("Invalid JSON:", data)
                return

            match msg.req:
                "login":
                    print("login request received")
                    _http_request(
                        client,
                        "/login",
                        JSON.stringify({"username": msg.username, "password": msg.password}),
                        "login"
                    )

                "signup":
                    print("signup request received")
                    _http_request(
                        client,
                        "/signup",
                        JSON.stringify({"username": msg.username, "password": msg.password}),
                        "signup"
                    )
                
                "play":
                    print("play request received")
                    if !world_active:
                        world_active = true
                        var world = world_scene.instantiate()
                        world.set_script(world_script)
                        get_tree().root.add_child(world)
                        world.tcp = self
                    playing_clients.append(client)

                "exit":
                    print("exit request received")
                    if client in playing_clients:
                        playing_clients.erase(client)

        if frame_passed >= 3:
            frame_passed = 0
            if client in playing_clients and game_tracking.enemies.size() >= 7:
                var response = JSON.stringify(game_tracking)
                responde(client, response, "success", "game_update")

func _http_request(client: StreamPeerTCP, url: String, body: String, req_type: String):
    var http := HTTPClient.new()

    var err = http.connect_to_host("alexanderpi", 8080)
    if err != OK:
        print("Failed to connect:", err)
        return

    while http.get_status() in [
        HTTPClient.STATUS_CONNECTING,
        HTTPClient.STATUS_RESOLVING
    ]:
        http.poll()
        OS.delay_msec(10)

    http.request(
        HTTPClient.METHOD_POST,
        url,
        ["Content-Type: application/json"],
        body
    )

    while http.get_status() == HTTPClient.STATUS_REQUESTING:
        http.poll()
        OS.delay_msec(10)

    var response = http.read_response_body_chunk()
    var code = http.get_response_code()
    var text = response.get_string_from_utf8()

    print("Rust says:", code, text)

    responde(client, text, code, req_type)

func responde(client: StreamPeerTCP, body: String, status, req_type: String):
    var response = JSON.stringify({
        "status": status,
        "type": req_type,
        "body": body
    })

    client.put_data(response.to_utf8_buffer())
