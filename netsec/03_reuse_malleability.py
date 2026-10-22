def xor_bytes(a, b):
    if len(a) != len(b):
        raise ValueError("Die Bytefolgen muessen gleich lang sein.")
    result = bytearray()
    for i in range(len(a)):
        result.append(a[i] ^ b[i])
    return bytes(result)


def recover_message(known_plaintext, known_ciphertext, target_ciphertext):
    # Implement here
    full_key_stream = xor_bytes(known_plaintext, known_ciphertext)
    key_stream = full_key_stream[:len(target_ciphertext)]
    target_plaintext = xor_bytes(target_ciphertext, key_stream)
    return target_plaintext


def change_message(ciphertext, offset, old, new):
    # Implement here
    mask = xor_bytes(old, new)
    result = bytearray(ciphertext)
    for i in range(len(result) - offset):
        result[offset + i] = mask[i]
    return bytes(result)


def main():
    known_plaintext = b"Dies ist eine bekannte Nachricht."
    known_ciphertext = bytes.fromhex("684ddf7dc341afff962e5fa4f0918833828095073bf7b45d11281484c4eb43a262")
    target_ciphertext = bytes.fromhex("5856df688558a9e5dd3f0ba2fad49825888097367fa1")

    recovered = recover_message(known_plaintext, known_ciphertext, target_ciphertext)
    print("Rekonstruierter Klartext:", recovered.decode("utf-8"))

    ciphertext = bytes.fromhex("6c9d07211c47deb17acaa2ddc1a892e328d866786d37b8af7da4")
    changed = change_message(ciphertext, 22, b"0100", b"0900")
    print("Manipulierter Chiffretext:", changed.hex())



if __name__ == "__main__":
    main()
