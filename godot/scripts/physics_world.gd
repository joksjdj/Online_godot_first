extends Node3D

@onready var game_tracking = {
	"enemies_left": 0,
	"enemies": {},
}

# PackedScene
@export var enemy_to_spawn: PackedScene = preload("res://assets/enemy.tscn")

# Global tracking
var spawning_enemies: bool = false
var frame_passed: int = 0

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	frame_passed += 1
	
	for path in get_tree().get_nodes_in_group("paths_to_follow"):
		if path is PathFollow3D:
			path.progress += 4 * delta
	
	for character in get_tree().get_nodes_in_group("affected_by_gravity"):
		if character is CharacterBody3D:
			if not character.is_on_floor():
				character.velocity += character.get_gravity() * 2 * delta
			character.move_and_slide()
			
	for enemy in get_tree().get_nodes_in_group("enemies"):
		var t = enemy.walk_path.global_transform
		var pos = enemy.global_transform.origin
		pos.x = t.origin.x
		pos.z = t.origin.z
		enemy.global_transform.origin = pos
			
		enemy.get_node("Head").global_rotation.y = enemy.walk_path.global_rotation.y
		enemy.global_rotation.y = enemy.get_node("Head").global_rotation.y
		
		game_tracking["enemies"][enemy.id] = {
			"pos": enemy.global_position,
			"rot": enemy.get_node("Head").global_rotation
		}
		
	for player in get_tree().get_nodes_in_group("player"):
		game_tracking["enemies"][player.id] = {
			"pos": player.global_position,
			"rot": player.get_node("Head").global_rotation
		}
			
	if game_tracking.enemies_left <= 0 and !spawning_enemies:
		spawning_enemies = true
		var points = get_node("/root/Main/Area3D/SpawnPoint").get_children()
		spawn_enemies(points)
	
	if frame_passed >= 120:
		frame_passed = 0
		print(game_tracking, "\n")
	
func spawn_enemies(points):
	points.shuffle()
	for spawn in points:
		if game_tracking.enemies_left >= 7:
			break
		game_tracking.enemies_left += 1
		var enemy = enemy_to_spawn.instantiate()
		enemy.add_to_group("enemies")
		enemy.add_to_group("affected_by_gravity")
		
		enemy.walk_path = spawn.get_node("Path3D/PathFollow3D")
		enemy.id = game_tracking.enemies_left
		
		enemy.global_position = spawn.get_node("Path3D/PathFollow3D").global_position
		enemy.set_collision_layer_value(5, true)
		spawn.add_child(enemy)
		
	spawning_enemies = false
	
	
