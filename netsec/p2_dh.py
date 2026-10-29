def dh_values(p, g, a, b):
    a_pub = pow(g, a, p)
    b_pub = pow(g, b, p)
    key = pow(a_pub, b, p)  # key_a == key_b
    return (a_pub, b_pub, key, key)


def discrete_log(p, g, value):
    for x in range(0, p-1):
        if pow(g, x, p) == value:
            return x
    return None


def passive_key(p, g, A, B):
    a = discrete_log(p, g, A)
    b = discrete_log(p, g, B)
    (_, _, key, _) = dh_values(p, g, a, b)
    return key


if __name__ == "__main__":
    assert dh_values(23, 5, 6, 15) == (8, 19, 2, 2)
    assert discrete_log(23, 5, 8) == 6
    assert discrete_log(23, 5, 1) == 0
    assert discrete_log(23, 4, 5) is None
    assert passive_key(23, 5, 8, 19) == 2
    assert dh_values(29, 2, 3, 4) == (8, 16, 7, 7)
    print("Alle Selbsttests bestanden.")
