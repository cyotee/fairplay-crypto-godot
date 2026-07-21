## Thin GDScript facade — no crypto math; calls FairPlayApi GDExtension class.
class_name FairPlay
extends RefCounted

var _api: RefCounted


func _init() -> void:
	# Class registered by manamesh_fairplay GDExtension
	_api = ClassDB.instantiate("FairPlayApi")


func version() -> String:
	return str(_api.call("version"))


func sha256_hex(text: String) -> String:
	return str(_api.call("sha256_hex", text))


func generate_keypair() -> Dictionary:
	return _api.call("generate_keypair")


func validate_public_key(public_key_hex: String) -> bool:
	return bool(_api.call("validate_public_key", public_key_hex))


func commit_die_face(face: int) -> Dictionary:
	return _api.call("commit_die_face", face)


func commit_dice_hand(faces: PackedByteArray) -> Dictionary:
	return _api.call("commit_dice_hand", faces)


func verify_die_face(commitment_hex: String, face: int, nonce_hex: String) -> bool:
	return bool(_api.call("verify_die_face", commitment_hex, face, nonce_hex))


func verify_commitment(commitment_hex: String, message: PackedByteArray, nonce_hex: String) -> bool:
	return bool(_api.call("verify_commitment", commitment_hex, message, nonce_hex))


func encrypt_payload(payload_id: String, private_key_hex: String) -> Dictionary:
	return _api.call("encrypt_payload", payload_id, private_key_hex)


func encrypt_layer(ciphertext: String, layers: int, private_key_hex: String) -> Dictionary:
	return _api.call("encrypt_layer", ciphertext, layers, private_key_hex)


func peel_layer(ciphertext: String, layers: int, private_key_hex: String) -> Dictionary:
	return _api.call("peel_layer", ciphertext, layers, private_key_hex)


func merkle_root_utf8(leaves: PackedStringArray) -> String:
	return str(_api.call("merkle_root_utf8", leaves))
