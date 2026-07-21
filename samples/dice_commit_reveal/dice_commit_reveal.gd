extends Node
## Offline dice commit–reveal sample (Liar's Dice–shaped).
## Private nonces stay local until open; only commitment_hex is "on the wire".

func _ready() -> void:
	run_demo()
	get_tree().quit(0)


func run_demo() -> void:
	var api = ClassDB.instantiate("FairPlayApi")
	if api == null:
		push_error("FairPlayApi not registered — is the GDExtension loaded?")
		get_tree().quit(1)
		return

	print("FairPlay version: ", api.call("version"))

	# Commit five dice faces (local secrets: faces + nonce)
	var faces := PackedByteArray([1, 2, 3, 4, 5])
	var c: Dictionary = api.call("commit_dice_hand", faces)
	var commitment_hex: String = c["commitment_hex"]
	var nonce_hex: String = c["nonce_hex"]
	print("COMMIT public: ", commitment_hex)
	# Wire-safe public message would only include commitment_hex (not nonce / faces)

	var ok: bool = api.call("verify_commitment", commitment_hex, faces, nonce_hex)
	print("OPEN correct: ", ok)
	if not ok:
		push_error("expected verify success")
		get_tree().quit(2)
		return

	var wrong := PackedByteArray([1, 2, 3, 4, 6])
	var bad: bool = api.call("verify_commitment", commitment_hex, wrong, nonce_hex)
	print("OPEN wrong faces: ", bad)
	if bad:
		push_error("expected verify failure for wrong faces")
		get_tree().quit(3)
		return

	var die: Dictionary = api.call("commit_die_face", 4)
	var die_ok: bool = api.call("verify_die_face", die["commitment_hex"], 4, die["nonce_hex"])
	var die_bad: bool = api.call("verify_die_face", die["commitment_hex"], 5, die["nonce_hex"])
	print("single die ok=", die_ok, " wrong=", die_bad)
	if not die_ok or die_bad:
		push_error("die face verify mismatch")
		get_tree().quit(4)
		return

	print("DICE_COMMIT_REVEAL_OK")
