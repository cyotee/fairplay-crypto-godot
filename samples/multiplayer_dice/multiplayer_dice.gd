extends Node
## Documented MultiplayerAPI-shaped path for dice commit–reveal.
## This sample runs offline with a local "host" simulation of RPC-style messages.
## Private keys / nonces never go into the public message dicts.

signal public_message(from_seat: String, payload: Dictionary)

var _local_secrets: Dictionary = {} # seat -> { faces, nonce_hex, commitment_hex }


func _ready() -> void:
	run_demo()
	get_tree().quit(0)


func run_demo() -> void:
	var api = ClassDB.instantiate("FairPlayApi")
	if api == null:
		push_error("FairPlayApi missing")
		get_tree().quit(1)
		return

	# Seat A commits locally
	var faces_a := PackedByteArray([6, 6, 1, 2, 3])
	var ca: Dictionary = api.call("commit_dice_hand", faces_a)
	_local_secrets["A"] = {
		"faces": faces_a,
		"nonce_hex": ca["nonce_hex"],
		"commitment_hex": ca["commitment_hex"],
	}
	# Public wire (RPC-equivalent) — no private_key / no nonce yet
	var wire_commit := {
		"type": "commitment",
		"label": "dice_hand",
		"commitment_hex": ca["commitment_hex"],
	}
	_assert_no_secrets(wire_commit)
	public_message.emit("A", wire_commit)
	print("WIRE_COMMIT ", wire_commit)

	# Later open
	var wire_open := {
		"type": "commitment_open",
		"label": "dice_hand",
		"commitment_hex": ca["commitment_hex"],
		"message_hex": ca["message_hex"],
		"nonce_hex": ca["nonce_hex"],
	}
	_assert_no_secrets(wire_open)
	var ok: bool = api.call(
		"verify_commitment",
		wire_open["commitment_hex"],
		faces_a,
		wire_open["nonce_hex"]
	)
	print("PEER_VERIFY ", ok)
	if not ok:
		get_tree().quit(2)
		return
	print("MULTIPLAYER_DICE_OK")


func _assert_no_secrets(d: Dictionary) -> void:
	for k in d.keys():
		var kl := str(k).to_lower()
		if "private" in kl or kl == "sk" or "secret_key" in kl:
			push_error("secret field on wire: " + str(k))
			get_tree().quit(9)
