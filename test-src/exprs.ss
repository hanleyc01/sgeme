;; exprs
symbol
#t
#f
32
#(1 2)
#\c
"fart"
'asdf
(add1 2)
(lambda () fart)
(lambda)
(lambda ())
(lambda (asdf asdf) asdf asdf)
(lambda (asdf) asd)
(if test 
    test 
    test)
(cond [test test] [else test])
(cond)
(cond [test test test] test)

(case fart ['asdf asdf])
(case fart)
(case fart ['asdf asdf] [else fart])

(and)
(and as as)
(and as as as as )

(or)
(or as as)
(or as as as as)

(when test fart)
(when)
(when test fart fart)

(unless test fart)
(unless)
(unless test fart fart)

(let ([fart fart]) fart)
(let () fart)
(let fart)
(let)
(let ([test test] [test test]) test)

(letrec ([fart fart]) fart)
(letrec () fart)
(letrec fart)
(letrec)
(letrec ([test test] [test test]) test)

(begin)
(begin test test)
(begin test etste testset)