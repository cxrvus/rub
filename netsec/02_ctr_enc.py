from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes


def xor_bytes(a, b):
    if len(a) != len(b):
        raise ValueError("Die Bytefolgen muessen gleich lang sein.")
    result = bytearray()
    for i in range(len(a)):
        result.append(a[i] ^ b[i])
    return bytes(result)


def aes_encrypt_block(key, block):
    cipher = Cipher(algorithms.AES(key), modes.ECB())
    encryptor = cipher.encryptor()
    return encryptor.update(block) + encryptor.finalize()


def ctr_crypt(key, nonce, data):
    # Implement here
    if len(data) == 0:
        return b""
    
    block_count = (len(data) // 16) + 1
    ciphertext = b""

    for i in range(block_count):
        counter = i.to_bytes(8, "big")
        block_input = nonce + counter
        block_output = aes_encrypt_block(key, block_input)
        offset = i * 16
        msg_block = data[offset : offset + 16]
        encrypted_block = xor_bytes(block_output[:len(msg_block)], msg_block)
        ciphertext += encrypted_block

    return ciphertext


def main():
    key = bytes.fromhex("00112233445566778899aabbccddeeff")
    nonce = bytes.fromhex("0102030405060708")
    message = b"Netzsicherheit 1!"
    expected = bytes.fromhex("ef1ef24773af6e3c004591365f4f63ea31")

    for length in [0, 1, 16, 17]:
        data = message[:length]
        ciphertext = ctr_crypt(key, nonce, data)
        assert ciphertext == expected[:length]
        assert ctr_crypt(key, nonce, ciphertext) == data

    print("Alle Tests erfolgreich.")
    print("Chiffretext:", ctr_crypt(key, nonce, message).hex())



if __name__ == "__main__":
    main()
