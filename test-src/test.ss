;; this is a test of all the available pre-built special forms
;; that come with the `sgeme` bootstrapping compiler

;; illegal import of nothing
(import)

;; a single import
(import "sgeme.ss")

;; import multiple things
(import "sgeme.ss"
        "sgeme.ss")

(export test)
(export (test test))
(export)

;; `define` variations
(define)
(define test)
(define test 1)
(define test 1 2)

;; `(define (<ident> ...) ...)` variations
(define ())
(define () 1)
(define (main) 1)
(define (main asdf) 1)
(define (main asdf) 1 2)

;; `(define-record ...)`
(define-record)
(define-record test)
(define-record test (test test test))
(define-record test (test test test) test)
(define-record test ("tesst" test) test)

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