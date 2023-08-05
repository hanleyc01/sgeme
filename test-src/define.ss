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