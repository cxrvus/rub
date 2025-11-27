# testing for 36:

addi t0 zero 36
addi t1 zero 1
blt t0 t1 end

square:
    mul t2 t1 t1
    bgt t2 t0 end
    addi t1 t1 1
    j square

end:
    addi t1 t1 -1
    mv t0 t1
