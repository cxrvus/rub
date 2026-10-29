def rsa_keys(p, q, e):
    n = p * q
    d = pow(e, -1, (p-1) * (q-1))
    return (n, d)


def encrypt(m, e, n):
    return pow(m, e, n)


def decrypt(c, d, n):
    return pow(c, d, n)


def recover_message(c, e, n, r, oracle):
    c = c * pow(r, e, n) % n
    m = oracle(c)
    return m * pow(r, -1, n) % n


def decrypt_oracle(c):
    if c == 15:
        raise ValueError("Der Zielchiffretext ist gesperrt.")
    return pow(c, 23, 187)


if __name__ == "__main__":
    assert rsa_keys(11, 17, 7) == (187, 23)
    assert encrypt(42, 7, 187) == 15
    assert decrypt(15, 23, 187) == 42
    assert decrypt(encrypt(11, 7, 187), 23, 187) == 11
    assert recover_message(15, 7, 187, 2, decrypt_oracle) == 42
    assert recover_message(15, 7, 187, 3, decrypt_oracle) == 42
    print("Alle Selbsttests bestanden.")
