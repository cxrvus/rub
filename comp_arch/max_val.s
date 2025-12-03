start:
    # check if t0 < t1, or t0 == t1:
    beq t0 t1 vals_eq
    bltu t0 t1 vals_lt
    # else do nothing:
    # => t0 will automacally be the greater value
    j end

vals_lt:
    # if t0 < t1, return t1:
    mv t0 t1
    j end

vals_eq:
    # if t0 == t1, return -1:
    li t0 -1
    
end:
    nop
