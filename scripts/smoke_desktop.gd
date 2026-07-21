extends SceneTree
## Headless smoke: load extension and exercise commit-reveal.

func _initialize() -> void:
	call_deferred("_run")


func _run() -> void:
	var api = ClassDB.instantiate("FairPlayApi")
	if api == null:
		print("SMOKE_FAIL FairPlayApi not registered")
		quit(1)
		return
	print("SMOKE version=", api.call("version"))
	var c = api.call("commit_die_face", 3)
	var ok = api.call("verify_die_face", c["commitment_hex"], 3, c["nonce_hex"])
	var bad = api.call("verify_die_face", c["commitment_hex"], 4, c["nonce_hex"])
	print("SMOKE open_ok=", ok, " open_wrong=", bad)
	if ok and not bad:
		print("SMOKE_DESKTOP_OK")
		quit(0)
	else:
		print("SMOKE_FAIL verify logic")
		quit(2)
